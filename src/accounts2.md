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


fn main() {

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
        
    //////////////////////////////
    // ACCOUNT DATABASE TESTING //
    //////////////////////////////

    fn new_account() -> (String, f32) {
    
        let priv_key = key_gen();
        let pub_key = hash(& priv_key.clone());
        let balance = 100.0;
        
        println!("\nThis is your private key: {:#?}", priv_key);
        println!("This is your public key: {:#?}", pub_key);
        
        (pub_key, balance)
    }

    let mut accounts: HashMap<String, f32> = HashMap::new();
    for i in 0..3 {
        let (x, y) = new_account();
        accounts.insert(x, y);
    }
    
    for i in accounts.values_mut() {
        *i += 100.0
    }
    
    println!("\nACCOUNTS:\n{:#?}\n", accounts);
}
