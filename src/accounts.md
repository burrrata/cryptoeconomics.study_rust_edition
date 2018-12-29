```rust
#![allow(warnings)]

extern crate rand;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use rand::prelude::*;
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
        
    
    // ACCOUNT DATABASE TESTING
    
    #[derive(Debug)]
    pub struct Account {
        pub_key: String,
        balance: f32,
    }

    // P2P
    fn new_p2p_account() -> Account {
        
        let priv_key = key_gen();
        let pub_key = hash(& priv_key.clone());
        println!("\nThis is your private key: {:#?}", priv_key);
        println!("Please keep it somewhere safe!\n");
        
        Account {
            pub_key: pub_key,
            balance: 100.0,
        }
    }

    let mut p2p_accounts: Vec<Account> = Vec::new();
    for i in 0..3 {
        p2p_accounts.push(new_p2p_account());
    }
    println!("accounts:\n{:#?}\n", accounts_vec);
    
    // UNDER CONSTRUCTION
    /*
    // Centralized
    fn new_centralized_account() -> Vec<> {
        
        let cav = Vec::new(); // centralized account vec
        let priv_key = key_gen();
        let pub_key = hash(& priv_key.clone());
        let account = Account {
            pub_key: pub_key,
            balance: 100.0,
        };
        
        cav.push(account);
        cav.push(priv_key);
        
        cav
    }
    
    let mut bank_keys: Vec<Vec> = Vec::new();
    for i in 0..3 {
        accounts_vec.push(new_centralized_account());
    }
    println!("accounts:\n{:#?}\n", accounts_vec);
    println!("bank keys:\n{:#?}\n", bank_keys);
    */
}
```
