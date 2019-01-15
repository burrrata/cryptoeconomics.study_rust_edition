extern crate rand;
use rand::prelude::*;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

// GOAL
// - Refactor the code so that you can change any of the modules and it still runs.


// GENERIC BLOCKCHAIN ARCHITECTURE
//  - State Transition Function
//  - Data Encoding Function
//  - Hash Function
//  - Key Generation Function
//  - Account Data
//  - Transaction Data
//  - State Data: a user defined configuration of the various blockchain modules

// State Transition Function
//  - determines what is a valid state transition by verifying tx
//  - determines who is authorized to create a state change via PoA, PoW, PoS, etc...
//  - impliments the state change
//  - this needs to contain all params out of the box including the difficulty level
//    and/or any functions needed to upgrade/modify those params

// Data Encoding Function
//  - takes in arbitrary data and encodes it in a specific way
//  - the entire "blockchain" uses this in order to allow any function
//    to process arbitrary data inputs as well as sharing data between functions
//  - standard for now, but may become upgradable as Ethreum and Substrate data is explored

// Hash Function
//  - takes in arbitrary data and returns a string
//  - the way that data is hashes or the encoding of the string can be changed

// Key Generation Function
//  - the method to generate public and private key pairs
//  - can be a centralized system, RSA, elliptic curves, etc...
//  - contains all parmas neccessary to work out of the box

// Account Data
//  - these will ALWAYS be a key/value pair in a HashMap
//  - what you can change is the data that the account struct holds
//  - UTXOs TBD

// TX Data
//  - standard for now

// State Data
//  - accounts: HashMap<i32, Account>
//  - pending_tx: Vec<TX>
//  - history: Vec<Block>
//  - data encoding: user defined
//  - State transition function: user defined
//  - hash function: user defined
//  - key gen function: user defined


// STANDARD STRUCTS
// These will keep the same name throughout the program, but their underlying
// logic can be changed/upgraded.
// - Account
// - TX
// - Blockheader
// - Block
// - State

#[derive(Debug, Clone, PartialEq)]
struct Account {
    balance: f32,
    nonce: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct TX {
    sender: i32,
    sender_nonce: i32,
    sender_signature: i32, // sender priv key signs a hash of the sending address and nonce
    amount: f32,
    receiver: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Blockheader {
    timestamp: i32,
    block_number: i32,
    nonce: i32,
    previous_block_hash: String,  
    current_block_hash: String,  
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    proof: String,
    header: Blockheader,
    transactions: Vec<TX>,
}

#[derive(Debug)]
struct State {
    accounts: HashMap<i32, Account>,
    pending_tx: Vec<TX>,
    history: Vec<Block>,
}


// STANDARD FUNCTIONS
// These will keep the same name throughout the program, but their underlying
// logic can be changed/upgraded.
// - data_encode()
// - key_gen()
// - hash()
// - new_account()
// - new_tx()
// - new_state_transition() (checks pending tx and produces new block)
// - check_state_transition() (checks the most recently produced block)

impl State {
    
    // Turn Arbitrary Stuff Into &[u8] Slice
    pub unsafe fn data_encode<T: Sized>(p: &T) -> &[u8] {
        ::std::slice::from_raw_parts(
            (p as *const T) as *const u8,
            ::std::mem::size_of::<T>(),
        )
    }
    
}
