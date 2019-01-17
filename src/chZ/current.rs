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




#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Keys {
    min: i32,
    max: i32,
    p: i32,
    q: i32,
    modulo: i32,
    ctf_pq: i32, 
}

// "RSA" Key Generation and Signing
impl Keys {
    
    // Set range for keys
    // - note: greater than 1000000 tends to break the Rust Playground
    pub const min: i32 = 0;
    pub const max: i32 = 1000000;
    
    // Set toy "RSA" parameters
    pub const p: i32 = 61;
    pub const q: i32 = 53;
    pub const modulo: i32 = 3233; // Keys::p * Keys::q;
    pub const ctf_pq: i32 = 780; // Keys::ctf(Keys::p, Keys::q);
    
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
    // - change thing_to_be_singed from Vec<i32> to any arbitrary
    //   type that is encoded to a standard format by the
    //   data_encode() function
    // toy RSA function for creating digital signatures
    pub fn toy_rsa_sig(self,
                       thing_to_be_signed: Vec<i32>,
                       private_key: i32) -> Vec<i32> {
        
        let signature = thing_to_be_signed.iter()
                                          .map(|x| Keys::exp_mod(self, *x, private_key))
                                          .collect();
        
        signature
    }
    
}

#[derive(Debug, Clone, PartialEq)]
struct TX {
    sender: i32,
    sender_nonce: i32,
    sender_signature: i32, // sender priv key signs a hash of the sending address and nonce
    amount: i32,
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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Account {
    balance: i32,
    nonce: i32,
}

#[derive(Debug)]
struct State {
    key_params: Keys,
    accounts: HashMap<i32, Account>,
    pending_tx: Vec<TX>,
    history: Vec<Block>,
}

impl State {
    
    
    // TODO
    // upgrade to something like what Substrate uses
    // - https://github.com/paritytech/substrate/tree/master/core/serializer
    // Turn stuff into an &[u8] slice
    pub unsafe fn data_encode<T: Sized>(p: &T) -> &[u8] {
        ::std::slice::from_raw_parts(
            (p as *const T) as *const u8,
            ::std::mem::size_of::<T>(),
        )
    }
    
    // Create a new account
    pub fn create_account(&mut self) {
        
        let priv_key = Keys::priv_key_gen(self.key_params);
        let pub_key = Keys::pub_key_gen(self.key_params, priv_key);
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
    
    
    
    
}
