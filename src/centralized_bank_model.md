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


// USEFUL STUFF

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
    
fn new_bank_account() -> (String, String, f32) {

    let priv_key = key_gen();
    let pub_key = hash(& priv_key.clone());
    let balance = 100.0;
    
    println!("\nThis is your public key: {:#?}", pub_key);
    println!("This is your balance: {:#?}", balance);
    
    (pub_key, priv_key, balance)
}

#[derive(Debug)]
struct TX {
    sender: String,
    receiver: String,
    tx_amount: f32,
}

fn verify_tx(mut accounts: HashMap<String, f32>,
             mut history: Vec<TX>,
             pending_tx: Vec<TX>) -> (HashMap<String, f32>, Vec<TX>) {
    
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
                println!("{} sent {} to {}", &i.sender, &i.tx_amount, &i.receiver);
                history.push(i);
            }
        }
    }
    
    (accounts, history)
}


// CENTRALIZED BANK MODEL
fn main() {


    // Account Testing
    // set up data stores for account balances and keys
    let mut accounts: HashMap<String, f32> = HashMap::new();
    let mut keys: HashMap<String, String> = HashMap::new();
    
    // create 3 default accounts
    for i in 0..3 {
        let (x, y, z) = new_bank_account();
        keys.insert(x.clone(), y.clone());
        accounts.insert(x, z);
    }
    
    // create 2 accounts with fixed keys and balances
    // because we can't test the tx function without 
    // knowing the sender and reciever addresses, but
    // everytime we run the program it creates new
    // accounts with randomized keys
    accounts.insert(String::from("0x000"), 1000.0);
    keys.insert(String::from("0x000"), String::from("000"));
    accounts.insert(String::from("0x001"), 1000.0);
    keys.insert(String::from("0x001"), String::from("001"));
    
    // add 100.0 to each account
    for i in accounts.values_mut() {
        *i += 100.0
    }
    
    // check results
    println!("\nACCOUNTS:\n{:#?}", &accounts);
    println!("\nKEYS:\n{:#?}\n", &keys);
    
    
    // TX Testing 
    // set up data stores for pending tx 
    // and history of approved tx
    let mut pending_tx = Vec::new();
    let mut history = Vec::new();
    
    // valid tx
    let tx0 = TX {
        sender: String::from("0x000"),
        receiver: String::from("0x001"),
        tx_amount: 100.0,
    };
    pending_tx.push(tx0);
    // valid tx
    let tx1 = TX {
        sender: String::from("0x000"),
        receiver: String::from("0x001"),
        tx_amount: 500.0,
    };
    pending_tx.push(tx1);
    // invalid tx
    let tx2 = TX {
        sender: String::from("0x000"),
        receiver: String::from("0x001"),
        tx_amount: -100.0,
    };
    pending_tx.push(tx2);
    
    // check to see which tx are valid
    // and process valid tx
    let (accounts, history) = verify_tx(accounts, history, pending_tx);
    
    // print results
    println!("\nACCOUNTS:\n{:#?}", &accounts);
    println!("\nKEYS:\n{:#?}\n", &keys);
    println!("\nHISTORY:\n{:#?}", &history);

}
```
