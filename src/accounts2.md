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


fn tx(mut accounts: HashMap<String, f32>,
      sender: String,
      receiver: String,
      tx_amount: f32) -> HashMap<String, f32> {
    
    if !accounts.contains_key(&sender) {
        println!("\nTX Sender Not Found.");
    } 
    
    if !(tx_amount > 0.0) {
        println!("\nTX Amount Invalid");
    } 
    
    else {
        if accounts.get(&sender).unwrap() > &tx_amount {
            *accounts.get_mut(&sender).unwrap() -= tx_amount;
            *accounts.get_mut(&receiver).unwrap() += tx_amount;
            println!("\nValid TX Success!");
            println!("{} sent {} to {}", sender, tx_amount, receiver);
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
    accounts.insert(String::from("0x000000000000000"), 500.0);
    accounts.insert(String::from("0x000000000000001"), 500.0);
    println!("\nACCOUNTS:\n{:#?}\n", &accounts);
    
    
    // add 100.0 to each account
    for i in accounts.values_mut() {
        *i += 100.0
    }
    println!("\nACCOUNTS:\n{:#?}\n", &accounts);
    
    
    // test tx function
    let accounts = tx(accounts,
       String::from("0x000000000000000"),
       String::from("0x000000000000001"),
       100.0);
       
    let accounts = tx(accounts,
       String::from("0x000000000000000"),
       String::from("0x000000000000001"),
       -10.0);
    
    
    // end of testing    
    println!("\nACCOUNTS FINAL:\n{:#?}\n", &accounts);

}
```
