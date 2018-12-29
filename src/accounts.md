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
        
    
    //////////////////////////////
    // ACCOUNT DATABASE TESTING //
    //////////////////////////////
    
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
    
    println!("P2P ACCOUNTS:\n{:#?}\n", p2p_accounts);
    

    // Centralized Bank Accounts
    fn new_bank_account() -> (Account, String) {
        
        let priv_key = key_gen();
        let pub_key = hash(& priv_key.clone());
        let account = Account {
            pub_key: pub_key,
            balance: 0.0,
        };
        
        (account, priv_key)
    }

    #[derive(Debug)]
    struct Bank_Data {
        Accounts: Vec<Account>,
        Keys: Vec<String>,
    }
    
    let mut bank_accounts: Vec<Account> = Vec::new();
    let mut bank_keys: Vec<String> = Vec::new();
    let mut Bank_Data = Bank_Data {
        Accounts: bank_accounts,
        Keys: bank_keys,
    };  
    
    for i in 0..3 {
        let (account, key) = new_bank_account();
        Bank_Data.Accounts.push(account);
        Bank_Data.Keys.push(key);
    }
  
    println!("BANK DATA:\n{:#?}\n", Bank_Data);

}
