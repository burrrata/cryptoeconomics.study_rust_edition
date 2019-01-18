// TODO
// When we encrypt a TX in the sign() function
// the resulting bytes are not valid utf8
// and thus do not convert to a String
// so v2s fails and the entire program shuts down



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

    // i32 -> String
    // https://doc.rust-lang.org/nightly/std/string/trait.ToString.html
    pub fn i2s(input: i32) -> String {
        
        let output = input.to_string();
        
        output
    }
    
    // String -> i32
    // https://stackoverflow.com/questions/27043268/convert-a-string-to-int-in-rust
    pub fn s2i(input: String) -> i32 {
        
        let output = input.parse::<i32>().unwrap();
        
        output
    }

    // string -> Vec<i32>
    pub fn s2v(input: String) -> Vec<i32> {
        
        let output: Vec<i32> = input.as_bytes()
                                    .iter()
                                    .map(|x| *x as i32)
                                    .collect();
        
        output
    }
 
    // Vec<i32> -> String
    // https://doc.rust-lang.org/nightly/std/string/trait.ToString.html
    pub fn v2s(input: Vec<i32>) -> String {
        
        let mut output_vec = Vec::new();
        for i in input {
            output_vec.push(i.to_string())
        }
        let output_string = output_vec.join("");
        
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
    
    // Create A Merkle Tree Of All TX In A Vec
    pub fn hash_tree<T>(stuff: Vec<T>) -> String {
        
        let mut v = Vec::new();

        for i in &stuff {
            let hashed = Hash::hash(&i);
            v.push(hashed);
        }

        if v.len() % 2 == 1 {
            let last = v.last().cloned().unwrap();
            v.push(last);
        }

        while v.len() > 1 {
            let mut h1 = v.remove(0);
            let mut h2 = v.remove(0);
            h1.push_str(&mut h2);
            let nh = Hash::hash(&h1);
            v.push(nh);
        }
        
        v.pop().unwrap()
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
    
    // Sign a TX with a toy RSA function
    pub fn sign<T>(self,
                   thing_to_be_signed: &T,
                   signing_key: i32) -> String {
        
        let hashed_thing = Hash::hash(thing_to_be_signed);
        let hashed_thing_vec = DataEncoding::s2v(hashed_thing);
        let signed_vec = hashed_thing_vec.iter()
                          .map(|x| Keys::exp_mod(self, *x, signing_key,))
                          .collect();
        let signature = DataEncoding::v2s(signed_vec);
        println!("signing");
        signature
    }
    
    // Check signature on a TX
    pub fn check_tx_signature(self,
                              tx: TX) -> bool {
        
        let cloned = tx.clone();
        let v = DataEncoding::s2v(cloned.signature);
        let counter_signed_v = v.iter()
                                .map(|x| Keys::exp_mod(self, *x, tx.data.sender,))
                                .collect();
        let counter_signed_hash = DataEncoding::v2s(counter_signed_v);
        
        let hashed_tx = Hash::hash(&tx.data);
        
        if counter_signed_hash == hashed_tx {
            return true
        } else {
            return false
        }
    }
}


pub struct STF;

impl STF {
    
    // This function encodes the rules of what qualifies as a "valid tx"
    pub fn verify_vec_of_tx(state: &mut State) -> Vec<TX> {
        
        let mut verified_tx = Vec::new();
        
        for i in &state.pending_tx {
        
            if !(state.accounts.contains_key(&i.data.sender)) {
                println!("Invalid TX: sender not found.");
                break
            }
            
            if !(state.accounts.contains_key(&i.data.receiver)) {
                println!("Invalid TX: receiver not found.");
                break
            }
            
            if !(i.data.amount > 0) {
                println!("Invalid TX: negative amount error.");
                println!("{} cannot send {} to {}", i.data.sender, i.data.amount, i.data.receiver);
                break
            }
            
            if !(state.accounts.get(&i.data.sender).unwrap().balance > i.data.amount) {
                println!("Invalid TX: insufficient funds.");
                println!("{} cannot send {} to {}", i.data.sender, i.data.amount, i.data.receiver);
                break            
            }
            
            if !(i.data.sender_nonce == state.accounts.get(&i.data.sender).unwrap().nonce) {
                println!("Invalid TX: potential replay tx.");
                println!("{} has nonce {}, but submitted a tx with nonce {}", i.data.sender, state.accounts.get(&i.data.sender).unwrap().nonce, i.data.sender_nonce);
                break
            }
            
            if !(Keys::check_tx_signature(KEY_PARAMS, i.clone())) {
                println!("TX No Good!");
                continue
            }
            
            verified_tx.push(i.clone());
        }
        
        verified_tx
    }

    // This function creates a proof that authorizes the state transition
    // This is a variation of PoW that's easy enough that it runs in the Rust Playground 
    pub fn proof(mut block: Block) -> (Blockheader, String) {
    
        let difficulty = 5;
        let max = 1000000;
        
        for i in 0..max {
        
            let mut count = 0;
            let hash = Hash::hash(&block);

            for i in hash.chars() {
                if i == '0' {
                    count += 1;
                }
            }
            
            if count > difficulty {
                // success
                return (block.blockheader, hash);
            }
            
            block.blockheader.nonce += 1;
        }
        
        // failure
        return (block.blockheader, String::from("!!! PoW ERROR !!!") )
    }
    
    // Create A New Block With Valid Transactions
    pub fn new_block(state: &mut State) -> Block {
    
        let verified_tx = STF::verify_vec_of_tx(state);
        
        let mut naive_header = Blockheader {
            nonce: 0,
            timestamp: time::now().to_timespec().sec as i32,
            block_number: state.history.last().unwrap().blockheader.block_number + 1,
            previous_block_hash: Hash::hash(&state.history.last().unwrap().blockheader.current_block_hash),
            current_block_hash: Hash::hash_tree(verified_tx.clone()),
        };
        
        let naive_block = Block {
            proof: String::from("TBD"),
            blockheader: naive_header,
            transactions: verified_tx.clone(),
        };
        
        let (blockheader, proof) = STF::proof(naive_block);
        let block = Block {
            proof: proof,
            blockheader: blockheader,
            transactions: verified_tx,
        };
        
        block
    }
    
    // check that PoW hash matches blockheader
    pub fn check_pow(mut block: Block) -> bool {

        let hash_check = Hash::hash(&block);
        
        if hash_check != block.proof {
            println!("\nPoW Error: Invalid PoW Hash.");
            return false
        }
        
        return true
    }
    
    pub fn push_block(state: &mut State,
                      mut block: Block) {
        
        if !(STF::check_pow(block.clone())) {
            println!("Block Error: check_pow() failed.");
            return
        }

        for i in &block.transactions {
            state.accounts.get_mut(&i.data.sender).unwrap().balance -= i.data.amount;
            state.accounts.get_mut(&i.data.receiver).unwrap().balance += i.data.amount;
            state.accounts.get_mut(&i.data.sender).unwrap().nonce += 1;
        }
        
        state.history.push(block);
    }
    
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
    data: TxData,
    signature: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Blockheader {
    nonce: i32,
    timestamp: i32,
    block_number: i32,
    previous_block_hash: String,  
    current_block_hash: String,  
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    proof: String,
    blockheader: Blockheader,
    transactions: Vec<TX>,
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
            return
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
        
        
        let data = TxData {
            sender: sender_pub_key,
            sender_nonce: sender_priv_key,
            receiver: receiver_pub_key,
            amount: amount,
        };
        
        let signature = Keys::sign(KEY_PARAMS, &data, sender_priv_key);
        
        println!("creating tx");
        
        let tx = TX {
            data: data,
            signature: signature,
        };
        
        self.pending_tx.push(tx);
    }
    
    // Create a new state transition
    pub fn state_transition_function(&mut self) {
        
        let new_block = STF::new_block(self);
        let state_transition = STF::push_block(self, new_block);
        
        state_transition
        
    }
}



fn main() {
    
    // Init "blockchain"
    let mut blockchain = State {
        accounts: HashMap::new(),
        pending_tx: Vec::new(),
        history: Vec::new(),
    };
    println!("blockchain:\n{:#?}", blockchain);
    
    // Create random accounts
    for _i in 0..3 {
        blockchain.create_account();
    }
    println!("blockchain:\n{:#?}", blockchain);
    
    // Manually create testing account 0
    let acc_0_pub_key = 773;
    let acc_0_priv_key = 557;
    let acc_0 = Account {
        balance: 10000,
        nonce: 0,
    };
    blockchain.accounts.insert(acc_0_pub_key.clone(), acc_0);
    println!("blockchain:\n{:#?}", blockchain);
    
    // Manually create testing account 1
    let acc_1_pub_key = 179;
    let acc_1_priv_key = 719;
    let acc_1 = Account {
        balance: 10000,
        nonce: 0,        
    };
    blockchain.accounts.insert(acc_1_pub_key.clone(), acc_1);
    println!("blockchain:\n{:#?}", blockchain);
    
    // test a tx
    blockchain.create_tx(acc_0_pub_key,
                        acc_0_priv_key,
                        acc_1_pub_key,
                        50);
    println!("blockchain:\n{:#?}", blockchain);
    
    // process the tx
    blockchain.state_transition_function();
    println!("blockchain:\n{:#?}", blockchain);
    
}
