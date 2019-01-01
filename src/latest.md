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

use std::hash::Hash;

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
// HOW does one roll a simple DIY TX signature scheme in Rust?
// (without external libraries)
// https://doc.rust-lang.org/std/hash/trait.Hash.html
// https://github.com/pubkey/eth-crypto/blob/master/src/sign-transaction.js
// https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html#sign-and-verify-a-message-with-hmac-digest
//
// Stretch Goals
// - parameters to add fees/rewards and simulate ongoing tx to see how profitable central operators really are (network effects)


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

#[derive(Debug, Clone)]
struct Signed_TX {
    tx: TX,
    signature: String,
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

    // hash stuff
    pub fn hash<T: serde::Serialize>(item: &T) -> String {
    
        let input = serde_json::to_string(&item).unwrap();
        let input_bytes = input.as_bytes();
        
        let mut hasher = DefaultHasher::new();
        hasher.write(input_bytes);
        let digest = hasher.finish();
        let hex_digest = format!("{:#X}", digest);
        
        hex_digest
    }
    
    // UNDER CONSTRUCTION
    /*
    pub fn sign_tx(priv_key: String,
                   tx: TX) -> String {
        
        let x = Vec::new();
        let mut hasher = DefaultHasher::new();
        let hashed_tx = Hash::hash_tx(tx, hasher);
        x.push(hashed_tx);
        x.push(priv_key);
        let signed_tx_hash = x.join("");
        
        signed_tx_hash
    }
    */
    
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
