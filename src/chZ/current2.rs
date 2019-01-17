extern crate rand;
use rand::prelude::*;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

// GOAL
// - Refactor the code so that you can change any of the modules and it still runs.

// NOTES
// - Currently all values are i32 to make sketching the draft easier.
// - The goal is to use the DataEncoding library to encode all
//   data into a fast and easy to use uniform format everywhere

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




pub struct DataEncoding;

impl DataEncoding {
    
    // TODO
    //
    // - Upgrade to something like what Substrate uses
    //   https://github.com/paritytech/substrate/tree/master/core/serializer
    // - Also, does it need it's own struct/impl or does it
    //   make sense to have it in the State impl?
    //
    // Turn stuff into an &[u8] slice
    pub unsafe fn to_u8<T: Sized>(p: &T) -> &[u8] {
        ::std::slice::from_raw_parts(
            (p as *const T) as *const u8,
            ::std::mem::size_of::<T>(),
        )
    }    

    // string => Vec<i32>
    pub fn s2v(input: String) -> Vec<i32> {
        
        let output: Vec<i32> = input.as_bytes()
                                    .iter()
                                    .map(|x| *x as i32)
                                    .collect();
        
        output
    }
    
    // Vec<i32> => string
    pub fn v2s(input: Vec<i32>) -> String {
        
        let output_u8: Vec<u8> = input.iter()
                                      .map(|x| *x as u8)
                                      .collect();
        let output_string = String::from_utf8(output_u8).unwrap();
        
        output_string
    }    
    
    
}


pub struct Hash;

impl Hash {
    
    // Takes a preimage ("preimage" = fancy word for input to a hash function)
    // Encodes it via the data_encode() function
    // Hashes that data into a hex string
    pub fn hash<T>(preimage: &T) -> String {
        
        let stuff_as_u8 = unsafe {
            DataEncoding::to_u8(preimage)
        };
        
        let mut hasher = DefaultHasher::new();
        hasher.write(stuff_as_u8);
        let digest = hasher.finish();
        let hex_digest = format!("{:#X}", digest);
        
        hex_digest
    }    
    
}




/// "RSA" Key Generation and Signing ///

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Keys {
    min: i32,
    max: i32,
    p: i32,
    q: i32,
    modulo: i32,
    ctf_pq: i32, 
}

pub static KEY_PARAMS: Keys = Keys {
    min: 0,
    max: 1000000,
    p: 61,
    q: 53,
    modulo: 3233,
    ctf_pq: 780,
};

impl Keys {
    
    // These functionsare not needed as we have hard coded
    // the modulo and ctf_pq values
    /*
    // greatest common divisor
    pub fn gcd(a: i32,
               b: i32) -> i32 {
        
        let (mut a, mut b) = if a > b {
            (a, b)
        } else {
            (b, a)
        };
    
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
    
        a
    }
    
    // lowest common multiple
    pub fn lcm(a: i32,
               b: i32) -> i32 {
        
        let lcm = (a * b) / Keys::gcd(a, b);
        
        lcm
    }
    
    // Carmichael's totient function
    pub fn ctf(a: i32,
               b: i32) -> i32 {
        
        Keys::lcm(a - 1, b - 1)
    }
    */
    
    // slowly check if a number is prime
    pub fn slow_prime_check(self,
                            num: i32) -> bool {
        
        if num < self.min {
            println!("number must be greater than {}", self.min);
        }
        
        if num > self.max {
            println!("number cannot be greater than {}", self.max);
        }
        
        for i in 2..num{
            if num % i == 0 {
                return false
            }
        }
        
        true
    }

    // slowly, yet randomly, generate a prime number within a range
    pub fn prime_gen(self) -> i32 {
        
        for _i in 0..self.max {
            let p = thread_rng().gen_range(self.min, self.max);
            if Keys::slow_prime_check(self, p) {
                return p
            }
        }
        
        0
    }

    // generate a private key within a range
    pub fn priv_key_gen(self) -> i32 {
        
        let priv_key = Keys::prime_gen(self);
        assert!(self.max % priv_key != 0);
        
        priv_key
    }
    
    // slowly find the modular multiplicative inverse of a prime 
    pub fn slow_mmi(self,
                    priv_key: i32)-> i32 {
        
        for i in 2..self.max {
            if (i * priv_key) % self.ctf_pq == 1 {
                return i
            }
        }
        println!("Try larger search?");
        
        0
    }
    
    // create a public key from a pricate key and RSA param data
    pub fn pub_key_gen(self,
                       priv_key: i32) -> i32 {
        
        let pub_key = Keys::slow_mmi(self, priv_key);
        
        pub_key
    }
    
    // generate a private/public key pair
    pub fn generate_keypair(self) -> (i32, i32){
        let priv_key = Keys::priv_key_gen(self);
        let pub_key = Keys::pub_key_gen(self, priv_key);
        (priv_key, pub_key)
    }
    
    // Because... Rust.
    pub fn exp_mod(self,
                   input: i32,
                   power: i32) -> i32 {
        
        let mut out = (input * input) % self.modulo;
        // because the first iter of out took 2 off the base
        for _i in 0..power-2 {
            out = (out * input) % self.modulo;
        }
        
        out
    }
    
    // TODO
    //
    // - change thing_to_be_singed from Vec<i32> to any arbitrary
    //   type that is encoded to a standard format by the
    //   data_encode() function
    // - output needs to return an i32, not a Vec<i32>
    
    // Toy RSA functions for creating digital signatures
    
    pub fn sign_i32(self,
                    thing_to_be_signed: i32,
                    priv_key: i32) -> i32 {
        
        let signature = Keys::exp_mod(self, thing_to_be_signed, priv_key);
        
        signature
    }
    
    pub fn sign_vec_i32(self,
                        thing_to_be_signed: Vec<i32>,
                        priv_key: i32) -> Vec<i32> {
        
        let output = thing_to_be_signed.iter()
                          .map(|x| Keys::exp_mod(self, *x, priv_key,))
                          .collect();
        output
    }
    
    pub fn sign<T>(self,
                   thing_to_be_signed: &T,
                   priv_key: i32) -> String {
        
        let hashed_thing = Hash::hash(thing_to_be_signed);
        let hashed_thing_vec = DataEncoding::s2v(hashed_thing);
        let signed_vec = hashed_thing_vec.iter()
                          .map(|x| Keys::exp_mod(self, *x, priv_key,))
                          .collect();
        let signature = DataEncoding::v2s(signed_vec);
        
        signature
    }
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



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Account {
    balance: i32,
    nonce: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TxData {
    sender: i32,
    sender_nonce: i32,
    amount: i32,
    receiver: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TX {
    tx_data: TxData,
    tx_signature: String,
}

#[derive(Debug)]
pub struct State {
    accounts: HashMap<i32, Account>,
    pending_tx: Vec<TX>,
    history: Vec<Block>,
}

impl State {

    // Create a new account
    pub fn create_account(&mut self) {
        
        // TODO
        // - How can I make Keys::generator_keypair() not
        //   take in anything as input and have all the params
        //   stored within the Keys library?
        let (priv_key, pub_key) = Keys::generate_keypair(KEY_PARAMS);
        let new_account = Account {
            balance: 0,
            nonce: 0,
        };
        
        if self.accounts.contains_key(&pub_key) {
            println!("Bummer... account collision.");
        }
        
        self.accounts.insert(pub_key, new_account);
        println!("\nThis is your public key: {:#?}", &pub_key);
        println!("This is your private key: {:#?}", &priv_key);
        println!("This is your account: {:#?}", self.accounts.get(&pub_key).unwrap());
    }
    
    // Create a new TX
    pub fn create_tx(&mut self,
                     sender_pub_key: i32,
                     sender_priv_key: i32,
                     receiver_pub_key: i32,
                     amount: i32) {
        
        let tx_data = TxData {
            sender: sender_pub_key,
            sender_nonce: sender_priv_key,
            receiver: receiver_pub_key,
            amount: amount,
        };
        
        let tx_signature = Keys::sign(KEY_PARAMS, &tx_data, sender_priv_key);
        
        let tx = TX {
            tx_data: tx_data,
            tx_signature: tx_signature,
        };
        
        self.pending_tx.push(tx);
    }
}
