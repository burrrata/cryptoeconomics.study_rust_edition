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


// TODOS
//
// TX Processing
// the current architecture can only process 1 tx per 
// account per block otherwise there's a nonce error
//
// TX Signatures and Verification
// What if the bank gave people their private keys to
// sign TX, but also kept a copy for themselves to
// override or reverse any TX at any time?





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
    
fn new_bank_account() -> (String, String, f32, i32) {

    let priv_key = key_gen();
    let pub_key = hash(& priv_key.clone());
    let balance = 100.0;
    let nonce = 0;
    
    println!("\nThis is your public key: {:#?}", pub_key);
    println!("This is your balance: {:#?}", balance);
    
    (pub_key, priv_key, balance, nonce)
}

#[derive(Debug)]
struct TX {
    sender: String,
    receiver: String,
    tx_amount: f32,
    nonce: i32,
}

fn new_tx(sender: String,
          receiver: String,
          tx_amount: f32,
          mut nonces: HashMap<String, i32>,
          mut pending_tx: Vec<TX>) -> (HashMap<String, i32>, Vec<TX>) {
    
    let tx = TX {
        sender: sender.clone(),
        receiver: receiver.clone(),
        tx_amount: tx_amount.clone(),
        nonce: *nonces.get(&sender).unwrap(),
    };
    
    *nonces.get_mut(&sender).unwrap() += 1;
    pending_tx.push(tx);
    
    (nonces, pending_tx)
}

fn verify_tx(mut balances: HashMap<String, f32>,
             mut nonces: HashMap<String, i32>,
             pending_tx: Vec<TX>,
             mut history: Vec<TX>) -> (HashMap<String, f32>, HashMap<String, i32>, Vec<TX>) {

    for i in pending_tx {
    
        println!("\nCurrent TX Being Verified:\n{:#?}", &i);
        
        if !balances.contains_key(&i.sender) {
            println!("Invalid TX: sender not found.");
            break
        } 
        
        if !balances.contains_key(&i.receiver) {
            println!("Invalid TX: receiver not found.");
            break
        }
        
        if !(i.tx_amount > 0.0) {
            println!("Invalid TX: insufficient balance.");
            println!("{} cannot send {} to {}", i.sender, i.tx_amount, i.receiver);
            break
        } 
        
        if !(i.nonce == *nonces.get(&i.sender).unwrap()) {
            println!("Invalid TX: potential replay tx.");
            println!("{} has nonce {}, but submitted a tx with nonce {}", i.sender, *nonces.get(&i.sender).unwrap(), i.nonce);
            break
        }
        
        else {
            if balances.get(&i.sender).unwrap() > &i.tx_amount {
                *balances.get_mut(&i.sender).unwrap() -= i.tx_amount;
                *balances.get_mut(&i.receiver).unwrap() += i.tx_amount;
                *nonces.get_mut(&i.sender).unwrap() += 1;
                println!("Valid TX.");
                println!("{} sent {} to {}", &i.sender, &i.tx_amount, &i.receiver);
                history.push(i);
            }
        }
    }
    
    (balances, nonces, history)
}


// CENTRALIZED BANK MODEL
fn main() {


    // Account Testing
    ///////////////////////////////////////////
    // set up data stores for account balances and keys
    let mut balances: HashMap<String, f32> = HashMap::new();
    let mut keys: HashMap<String, String> = HashMap::new();
    let mut nonces: HashMap<String, i32> = HashMap::new();
    
    // create 3 default accounts
    for i in 0..3 {
        let (pub_key, priv_key, balance, nonce) = new_bank_account();
        keys.insert(pub_key.clone(), priv_key.clone());
        balances.insert(pub_key.clone(), balance.clone());
        nonces.insert(pub_key.clone(), nonce.clone());
    }
    
    // create 2 accounts with fixed keys and balances
    // because we can't test the tx function without 
    // knowing the sender and reciever addresses, but
    // everytime we run the program it creates new
    // accounts with randomized keys
    balances.insert(String::from("0x000"), 1000.0);
    keys.insert(String::from("0x000"), String::from("000"));
    nonces.insert(String::from("0x000"), 0);
    
    balances.insert(String::from("0x001"), 1000.0);
    keys.insert(String::from("0x001"), String::from("001"));
    nonces.insert(String::from("0x001"), 0);
    
    // add 100.0 to each account
    for i in balances.values_mut() {
        *i += 100.0
    }
    
    // check results
    println!("\nBALANCES:\n{:#?}", &balances);
    println!("\nNONCES:\n{:#?}", &nonces);
    println!("\nKEYS:\n{:#?}\n", &keys);
    
    
    // TX Testing 
    ///////////////////////////////////////////
    // vec to store pending tx
    let mut pending_tx = Vec::new();
    // vec to store history of valid tx
    let mut history = Vec::new();

    // valid tx
    let tx1 = TX {
        sender: String::from("0x000"),
        receiver: String::from("0x001"),
        tx_amount: 500.0,
        nonce: *nonces.get("0x000").unwrap(),
    };
    pending_tx.push(tx1);
    
    // invalid tx
    let tx2 = TX {
        sender: String::from("0x000"),
        receiver: String::from("0x001"),
        tx_amount: -100.0,
        nonce: *nonces.get("0x000").unwrap(),
    };
    pending_tx.push(tx2);
    
    // check to see which pending tx are valid
    // if tx is valid, process accordingly
    let (balances, nonces, history) = verify_tx(balances, nonces, pending_tx, history);

    
    // print results
    println!("\nBALANCES:\n{:#?}", &balances);
    println!("\nNONCES:\n{:#?}", &nonces);
    println!("\nKEYS:\n{:#?}", &keys);
    println!("\nHISTORY:\n{:#?}", &history);

}
```
