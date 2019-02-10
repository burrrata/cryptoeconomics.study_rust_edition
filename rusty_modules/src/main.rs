extern crate time;
use time::now;

extern crate rand;
use rand::prelude::*;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

include!("data_encoding.rs");
include!("hashing.rs");
include!("keys.rs");
include!("stf.rs");
include!("state.rs");

/*
mod data_encoding;
mod hashing;
mod keys;
mod stf;
mod state;

use data_encoding::DataEncoding;
use hashing::Hash;
use keys::Keys;
use stf::STF;
use state::{Account, TXData, TX, Blockheader, Blockdata, Block};
*/

fn main() {
    
    // Init "blockchain"
    let mut blockchain = State::create_state();
    //println!("\nBLOCKCHAIN:\n{:#?}", blockchain);
    
    // Create random accounts
    for _i in 0..3 {
        blockchain.create_account();
    }
    //println!("\nBLOCKCHAIN:\n{:#?}", blockchain);
    
    // Manually create testing account 0
    let acc_0_pub_key = 773;
    let acc_0_priv_key = 557;
    let acc_0 = Account {
        balance: 10000,
        nonce: 0,
    };
    blockchain.accounts.insert(acc_0_pub_key.clone(), acc_0);
    //println!("\nBLOCKCHAIN:\n{:#?}", blockchain);
    
    // Manually create testing account 1
    let acc_1_pub_key = 179;
    let acc_1_priv_key = 719;
    let acc_1 = Account {
        balance: 10000,
        nonce: 0,        
    };
    blockchain.accounts.insert(acc_1_pub_key.clone(), acc_1);
    //println!("\nBLOCKCHAIN:\n{:#?}", blockchain);
    
    // add testing account 0 and 1 to the validator pool
    blockchain.create_validator(acc_0_pub_key);
    blockchain.create_validator(acc_1_pub_key);
    //println!("\nBLOCKCHAIN:\n{:#?}", blockchain);
    
    // test a tx
    blockchain.create_tx(acc_0_pub_key,
                        acc_0_priv_key,
                        acc_1_pub_key,
                        50);
    //println!("blockchain:\n{:#?}", blockchain);
    
    // process the tx
    blockchain.create_new_state();
    println!("\nBLOCKCHAIN:\n{:#?}", blockchain);
}

