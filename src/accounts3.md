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


// USEFUL FUNCTIONS
fn key_gen() -> String {
        let rn: i32 = thread_rng().gen_range(100000, 1000000);
        rn.to_string()
    }
    
fn hash<T: serde::Serialize>(item: &T) -> String {

    let input = serde_json::to_string(&item).unwrap();
    let input_bytes = input.as_bytes();
    
    let mut hasher = DefaultHasher::new();
    hasher.write(input_bytes);
    let digest = hasher.finish();
    let hex_digest = format!("{:#X}", digest);
    
    hex_digest
}
    
fn new_account() -> (String, f32) {

    let priv_key = key_gen();
    let pub_key = hash(& priv_key.clone());
    let balance = 100.0;
    
    println!("\nThis is your private key: {:#?}", priv_key);
    println!("This is your public key: {:#?}", pub_key);
    
    (pub_key, balance)
}

struct tx {
    sender: String,
    receiver: String,
    tx_amount: f32,
}

fn verify_tx(mut accounts: HashMap<String, f32>,
             pending_tx: Vec<tx>) -> HashMap<String, f32> {
    
    for i in pending_tx {
        
        if !accounts.contains_key(&i.sender) {
            println!("\nTX Sender Not Found.");
        } 
        
        if !(i.tx_amount > 0.0) {
            println!("\nInvalid TX Failed.");
            println!("{} cannot send {} to {}", i.sender, i.tx_amount, i.receiver);
        } 
        
        else {
            if accounts.get(&i.sender).unwrap() > &i.tx_amount {
                *accounts.get_mut(&i.sender).unwrap() -= i.tx_amount;
                *accounts.get_mut(&i.receiver).unwrap() += i.tx_amount;
                println!("\nValid TX Success.");
                println!("{} sent {} to {}", i.sender, i.tx_amount, i.receiver);
            }
        }
    }
    
    accounts
}


// TEST STUFF
fn main() {


    // create 3 accounts w 100.0 each
    let mut accounts: HashMap<String, f32> = HashMap::new();
    for i in 0..3 {
        let (x, y) = new_account();
        accounts.insert(x, y);
    }
    // create 2 accounts with fixed keys and balances
    // because everytime we run the program it creates
    // new accounts with randomized keys
    accounts.insert(String::from("0x000"), 1000.0);
    accounts.insert(String::from("0x001"), 1000.0);
    println!("\nACCOUNTS:\n{:#?}\n", &accounts);
    
    
    // add 100.0 to each account
    for i in accounts.values_mut() {
        *i += 100.0
    }
    println!("\nACCOUNTS:\n{:#?}\n", &accounts);
    
    
    // tx testing 
    let mut pending_tx = Vec::new();
    
    let tx0 = tx {
        sender: String::from("0x000"),
        receiver: String::from("0x001"),
        tx_amount: 100.0,
    };
    pending_tx.push(tx0);
    
    let tx1 = tx {
        sender: String::from("0x000"),
        receiver: String::from("0x001"),
        tx_amount: 500.0,
    };
    pending_tx.push(tx1);
    
    let tx2 = tx {
        sender: String::from("0x000"),
        receiver: String::from("0x001"),
        tx_amount: -100.0,
    };
    pending_tx.push(tx2);
    
    let accounts = verify_tx(accounts, pending_tx);
    
    
    // end of testing    
    println!("\nACCOUNTS FINAL:\n{:#?}\n", &accounts);

}
```
