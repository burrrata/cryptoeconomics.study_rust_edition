# Centralized Payment Processors
What are they and how do they work? Let's roll our own to find out! 

<br>

# GOALS
- For education purposes build a fully functional (but not secure) blockchain using standard Rust code so that the main concepts can be understood as simply as possible in mdBook or the Rust Playground.
- Then create a CLI tutorial that shows how to iteratively ugrade each component and function in the standard model to make it more secure
- Stetch Goal: work towards recreating the Parity Ethereum Client, but that's probably not realistic lol

# TODO
- remix the Javascript as editable, runnable, Rust code
- move the english words into comments in the code
- bonus points if you can craft a story to explain the code and concepts

<br>

## Lectures
- https://cryptoeconomics.study/lectures/chapter-01-0.html

QUESTION: how do I embed YouTube videos in mdBook so they play here rather than taking the user away from the page to YouTube?
- https://users.rust-lang.org/t/embedding-youtube-in-mdbook/23555/3

[![chapter_1.0](https://img.youtube.com/vi/VaUTTE5xb54/hqdefault.jpg)](https://youtu.be/VaUTTE5xb54)

[![chapter_1.1](https://img.youtube.com/vi/FLIo_ZjV--U/hqdefault.jpg)](https://youtu.be/FLIo_ZjV--U)

[![chapter_1.2](https://img.youtube.com/vi/XIsn8-5Xekc/hqdefault.jpg)](https://youtu.be/XIsn8-5Xekc)

[![chapter_1.3](https://img.youtube.com/vi/j7Mbx8laZwY/hqdefault.jpg)](https://youtu.be/j7Mbx8laZwY)

[![chapter_1.4](https://img.youtube.com/vi/-xoCoZGJ9AQ/hqdefault.jpg)](https://youtu.be/-xoCoZGJ9AQ)

[![chapter_1.5](https://img.youtube.com/vi/ckzi8iqGilE/hqdefault.jpg)](https://youtu.be/ckzi8iqGilE)

## Words
- https://github.com/cryptoeconomics-study/website/blob/master/overview.md

* Overview of basic crypto concepts
   * Hashes -- not how they work but instead what they do (collision resistance, second preimage resistance [given x and H(x), cannot find y such that H(y) = H(x)], preimage resistance, random oracle)
   * Public / private keys -- Sign with private key, verify with public key
* Overview of PayPal structure
   * State object which is simply a mapping of address->balance
   * Two types of state mutations: 1) `mint` and 2) `send` -- each with their own logic
   * We start with an initial state (ie `genesis block`) and apply transactions (ie `blocks`) to get our most recent state.
      * The list of transactions (blocks) is called the “history”
      * The result of computing all transactions is called the “state”
      * Note: In Ethereum the full history is approx 250 GB, while the state is approx 3 GB.
         * Fun aside: Rent proposals say that people should pay rent on the state storage which they take up. There is no direct incentive to store the history, and so nodes today often do prune or delete historical data. If this happens too much there is a risk that we can’t recreate the chain anymore!
   * Use nonces to provide replay protection. (nonce means you can’t resubmit the same transaction multiple times)
   * Code implementation: https://codepen.io/karlfloersch/pen/YaEoYy?editors=0012 
* Account model vs UTXO model
   * Briefly cover the differences
   * Account model (what we are using for our implementation):
      * A global mapping of `account->balance`
      * Sends reduce one account’s balance and increase another account's balance 
   * UTXO model (unspent transaction output model used in Bitcoin)
      * Same as the account model, however with three added rules:
         * 1) Every send must include the entire account balance.
         * 2) Sends can specify multiple recipients.
         * 3) Sends can originate from multiple senders.
   * Supposed to be privacy preserving, but these days the privacy can be broken. Only purely private way to send is zero knowledge proofs.
* Properties of centralized systems
   * Benefits:
      * Easy to build and reason about.
      * Simple to scale.
      * Privacy preserving. (if you trust the operator)
   * Downsides:
      * Single point of failure
         * If the operator is removed (eg. servers burn down, servers seized by authorities), the entire system breaks.
      * Censorship
         * The operator can censor users and change their balances, and it is very difficult for users to prove malfeasance.
            * This is because there is no client-side validation
      * Fraud
         * Because the operator has complete control, they can steal money directly from users.
         * The only safeguard against this kind of misbehavior is the legal system & social reputation.
            * Even these threats are not enough--see Bitconnect, Mt. Gox, and many other exchanges which have been hacked.
            * Also, theft is often unprovable
   * These downsides limit what can be built on top of these systems.
      * Clearly no illegal securities!
* Let’s decentralize :)

<br>

## Code

MUST run in the [Rust Playground](https://play.rust-lang.org) and [mdBook](https://rust-lang-nursery.github.io/mdBook/index.html). This means no external Ethereum or crypto libraries. The goal is to explain the core concepts as simply as possible with working Rust code. THEN explain how the user can upgrade the toy functions in order to move towards something like [Parity's Ethereum Client](https://github.com/paritytech/parity-ethereum) or [Substrate](https://github.com/paritytech/substrate).

Here's Karl's awesome code:
- https://github.com/cryptoeconomics-study/code/blob/master/c1_CentralPaymentOperator/paypalWithSigs.js
- https://codepen.io/karlfloersch/pen/YaEoYy?editors=0012

Here's an awesome Rust blockchain tutorial:
- https://steemit.com/technology/@tensor/rust-project-cli-toy-blockchain
- https://github.com/tensor-programming/Rust_block_chain

These are GREAT, but Cryptoeconomics.Study code is written in JS and references external Ethereum libraries, and there's no functionality for accounts, keys, or tx signatures in the Rust tutorial. 

Attempting to get the best of both worlds...

```rust
#![allow(warnings)]

extern crate rand;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use rand::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;


// INFO
//
// Accounts
// - the "bank" creates accounts for users because
//   the "bank" is the only one who manages the State, just
//   like with a real bank
// ? there's got to be a better way to structure them than
//   disjointed HashMaps with the key as the pub_key and 
//   value as the data?
//
// Private Keys
// - users keep their keys to verify their tx
// - "bank" (currently YOU in this tutorial) can change
//   the state arbitrarily, just like with a real bank
// 
// TODO: TX Signatures
// ! users hash their tx with their private key
// ! verify_tx() checks that hash against the sender's pub_key



// Structs

#[derive(Debug)]
struct State {
    balances: HashMap<String, f32>,
    nonces: HashMap<String, i32>,
    pending_tx: Vec<TX>,
    verified_tx: Vec<TX>,
    history: Vec<Vec<TX>>,
}

#[derive(Debug, Clone)]
struct TX {
    sender: String,
    receiver: String,
    tx_amount: f32,
    nonce: i32,
}


// State
impl State {
    
    // initialize new blockchain
    pub fn new_blockchain() -> State {
        let mut state = State {
            balances: HashMap::new(),
            nonces: HashMap::new(),
            pending_tx: Vec::new(),
            verified_tx: Vec::new(),
            history: Vec::new(),
        };
    
        state
    }    
    
    // generate a new key
    pub fn key_gen() -> String {
        
        let rn: i32 = thread_rng().gen_range(100000, 1000000);
        
        rn.to_string()
    }

    // hash an arbitrary input
    pub fn hash<T: serde::Serialize>(item: &T) -> String {
    
        let input = serde_json::to_string(&item).unwrap();
        let input_bytes = input.as_bytes();
        
        let mut hasher = DefaultHasher::new();
        hasher.write(input_bytes);
        let digest = hasher.finish();
        let hex_digest = format!("{:#X}", digest);
        
        hex_digest
    }    

    // create a new account
    pub fn new_account(&mut self) {
    
        let priv_key = State::key_gen();
        let pub_key = State::hash(& priv_key.clone());
        let balance = 100.0;
        let nonce = 0;
        
        println!("\nThis is your public key: {:#?}", pub_key);
        println!("This is your private key: {:#?}", priv_key);
        println!("This is your balance: {:#?}", balance);
        
        self.balances.insert(pub_key.clone(), balance.clone());
        self.nonces.insert(pub_key.clone(), nonce.clone());
    }
    
    // create a tx and add it to the pending_tx pool
    pub fn new_tx(&mut self,
                  priv_key: &str,
                  receiver: &str,
                  tx_amount: f32) {
        
        let tx = TX {
            sender: State::hash(&priv_key),
            receiver: receiver.to_string(),
            tx_amount: tx_amount,
            nonce: *self.nonces.get(&State::hash(&priv_key)).unwrap(),
        };

        self.pending_tx.push(tx);
    }
    
    // verify the tx in the pending_tx pool
    pub fn verify_tx(&mut self) {
        
        println!("\nVerifying TX:");
        
        for i in & self.pending_tx {
        
            println!("{:#?}", &i);
            
            if !self.balances.contains_key(&i.sender) {
                println!("Invalid TX: sender not found.");
                break
            } 
            
            if !self.balances.contains_key(&i.receiver) {
                println!("Invalid TX: receiver not found.");
                break
            }
            
            if !(i.tx_amount > 0.0) {
                println!("Invalid TX: negative amount error.");
                println!("{} cannot send {} to {}", i.sender, i.tx_amount, i.receiver);
                break
            } 
            
            if !(self.balances.get(&i.sender).unwrap() > &i.tx_amount) {
                println!("Invalid TX: insufficient funds.");
                println!("{} cannot send {} to {}", i.sender, i.tx_amount, i.receiver);
                break            
            }
            
            if !(i.nonce == *self.nonces.get(&i.sender).unwrap()) {
                println!("Invalid TX: potential replay tx.");
                println!("{} has nonce {}, but submitted a tx with nonce {}", i.sender, *self.nonces.get(&i.sender).unwrap(), i.nonce);
                break
            }
            
            println!("Valid TX.");
            self.verified_tx.push(i.clone());
        }
        
        self.pending_tx = Vec::new();
    }
    
    // apply and confirm valid_tx pool
    pub fn confirm_tx(&mut self) {
        
        println!("\nConfirming TX:");
        
        let mut block = Vec::new();
        
        for i in & self.verified_tx {
            
            *self.balances.get_mut(&i.sender).unwrap() -= i.tx_amount;
            *self.balances.get_mut(&i.receiver).unwrap() += i.tx_amount;
            *self.nonces.get_mut(&i.sender).unwrap() += 1;
            println!("{} sent {} to {}", &i.sender, &i.tx_amount, &i.receiver);
            
            block.push(i.clone())
        }
        
        self.history.push(block);
        self.verified_tx = Vec::new();
    }
}


// CENTRALIZED BANK "BLOCKCHAIN"
fn main() {

    // Init Blockchain
    // init blockchain state 
    let mut state = State::new_blockchain();
    // create 3 random accounts
    for i in 0..3 {state.new_account()}
    // create deterministic test account
    let t0_priv = String::from("693677"); // 693677
    let t0_pub = State::hash(&t0_priv); // 0xC31B6988D3A6A62B
    let t0_bal = 10000.0;
    state.balances.insert(t0_pub.clone(), t0_bal.clone());
    state.nonces.insert(t0_pub.clone(), 0);
    // create deterministic test account
    let t1_priv = String::from("172218"); // 172218
    let t1_pub = State::hash(&t1_priv); // 0x81C52538C70E98B7
    let t1_bal = 10000.0;
    state.balances.insert(t1_pub.clone(), t1_bal.clone());
    state.nonces.insert(t1_pub.clone(), 0);
    // check results
    println!("\n{:#?}", state);
    
    
    // Test TX 
    // add some tx to the pending_tx pool
    state.new_tx(&t0_priv, &t1_pub, 500.0);
    state.new_tx(&t1_priv, &t0_pub, 127.0);
    state.new_tx(&t0_priv, &t1_pub.clone(), 1000.0);
    // verify valid tx
    state.verify_tx();
    // cofirm tx and change state
    state.confirm_tx();
    // check results
    println!("\n\nCurrent State:\n{:#?}", state);
}
```
