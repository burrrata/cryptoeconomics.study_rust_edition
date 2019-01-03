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


#[derive(Debug)]
struct State {
    accounts: HashMap<String, Account>,
    pending_tx: Vec<TX>,
    verified_tx: Vec<TX>,
    history: Vec<Vec<TX>>,
}

#[derive(Debug)]
struct Account {
balance: f32,
nonce: i32,
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


impl State {

    // initialize new blockchain
    pub fn new_blockchain() -> State {
        let mut state = State {
            accounts: HashMap::new(),
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
    
    // create new account
    pub fn new_account(&mut self) {
        
        let priv_key = State::key_gen();
        let pub_key = State::hash(& priv_key.clone());
        let new_account = Account {
            balance: 100.0,
            nonce: 0,
        };
        
        self.accounts.insert(pub_key.clone(), new_account);
        
        println!("\nThis is your public key: {:#?}", pub_key);
        println!("This is your private key: {:#?}", priv_key);
        println!("This is your account: {:#?}", self.accounts.get(&pub_key).unwrap());
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
            nonce: self.accounts.get(&State::hash(&priv_key)).unwrap().nonce,
        };

        self.pending_tx.push(tx);
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
    
    // verify the tx in the pending_tx pool
    pub fn verify_tx(&mut self) {
        
        println!("\nVerifying TX:");
        
        for i in & self.pending_tx {
        
            println!("{:#?}", &i);
            
            if !self.accounts.contains_key(&i.sender) {
                println!("Invalid TX: sender not found.");
                break
            } 
            
            if !self.accounts.contains_key(&i.receiver) {
                println!("Invalid TX: receiver not found.");
                break
            }
            
            if !(i.tx_amount > 0.0) {
                println!("Invalid TX: negative amount error.");
                println!("{} cannot send {} to {}", i.sender, i.tx_amount, i.receiver);
                break
            } 
            
            if !(self.accounts.get(&i.sender).unwrap().balance > i.tx_amount) {
                println!("Invalid TX: insufficient funds.");
                println!("{} cannot send {} to {}", i.sender, i.tx_amount, i.receiver);
                break            
            }
            
            if !(i.nonce == self.accounts.get(&i.sender).unwrap().nonce) {
                println!("Invalid TX: potential replay tx.");
                println!("{} has nonce {}, but submitted a tx with nonce {}", i.sender, self.accounts.get(&i.sender).unwrap().nonce, i.nonce);
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
            
            self.accounts.get_mut(&i.sender).unwrap().balance -= i.tx_amount;
            self.accounts.get_mut(&i.receiver).unwrap().balance += i.tx_amount;
            self.accounts.get_mut(&i.sender).unwrap().nonce += 1;
            println!("{} sent {} to {}", &i.sender, &i.tx_amount, &i.receiver);
            
            block.push(i.clone())
        }
        
        self.history.push(block);
        self.verified_tx = Vec::new();
    }
}


fn main() {

    // Init Blockchain
    // init blockchain state 
    let mut state = State::new_blockchain();
    
    // Init Accounts
    // create 3 random accounts
    for i in 0..3 {
        state.new_account()
    }
    // manually create account for testing
    let t0_priv = String::from("693677"); // 693677
    let t0_pub = State::hash(&t0_priv); // 0xC31B6988D3A6A62B
    let t0 = Account {
        balance: 10000.0,
        nonce: 0,
    };
    state.accounts.insert(t0_pub.clone(), t0);
    // manually create account for testing
    let t1_priv = String::from("172218"); // 172218
    let t1_pub = State::hash(&t1_priv); // 0x81C52538C70E98B7
    let t1 = Account {
        balance: 10000.0,
        nonce: 0,        
    };
    state.accounts.insert(t1_pub.clone(), t1);
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
