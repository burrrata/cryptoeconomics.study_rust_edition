/* GOAL
A modular architecture where you can change any of the modules,
say changing PoW to PoS, and it still runs.
*/


/* QUESTIONS / TODOS
QUESTION
- does it make more sense to incriment the nonce every time a 
  tx is submitted, or everytime a tx is sucessfully processed?

PROBLEM
- concurrent threading is a weak way to simulate network
  activity.
TODO
- create a way for the state to be shared rather than confirmed
  on the main() thread
  - can we create collective coin flipping or MPC style randomness
    just with multiple threads?
- or do the best we can with this setup but move towards a 
  CLI tutorial that allows for networking
*/

/////////////////////////////////////////////////////////


extern crate rand;
use rand::prelude::*;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;


// Data Encoding
/////////////////////////////////////////////////////////

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
    // Hashes that data into a hex or an integer (you choose)
    fn hash<T>(preimage: &T) -> String {
        
        // convert to u8
        let stuff_as_u8 = unsafe {
            DataEncoding::to_u8(preimage)
        };
        
        // hash u8 to u64
        let mut hasher = DefaultHasher::new();
        hasher.write(stuff_as_u8);
        
        // format u64 hash as String
        let string_digest = format!("{}", hasher.finish());
        string_digest
        
        // hex String
        //let digest = hasher.finish();
        //let hex_digest = format!("{:#X}", digest);
        //hex_digest
        
        // i32
        //let digest = hasher.finish() as i32;
        //digest 
        
        // f64
        //let digest = hasher.finish() as f64;
        //digest 
     
        // u64
        //let digest = hasher.finish();
        //digest
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


// Keys
/////////////////////////////////////////////////////////

// This struct holds all the data for the key generation
// and signing. If you want to use a different key
// protocol, change the data in the Keys struct as well
// as the functions in the Keys impl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Keys {
    min: i32,
    max: i32,
    p: i32,
    q: i32,
    modulo: i32,
    ctf_pq: i32, 
}

/// "RSA" Key Generation and Signing ///
impl Keys {
    
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
    
    // slowly check if a number is prime
    pub fn slow_prime_check(self,
                            num: i32) -> bool {
        
        if num < self.min {
            println!("number must be greater than {}", self.min);
            return false
        }
        
        if num > self.max {
            println!("number cannot be greater than {}", self.max);
            return false
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
        
        for _i in 0..1000000 {
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
        //assert!(max % priv_key != 0);
        
        priv_key
    }
    
    // slowly find the modular multiplicative inverse of a prime 
    pub fn slow_mmi(self,
                    priv_key: i32)-> i32 {
        
        for i in 1..self.max {
            if (i * priv_key) % self.ctf_pq == 1 {
                return i
            }
        }
        //println!("Try larger search?");
        return 0
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
                   signing_key: i32) -> Vec<i32> {
        
        let hashed_thing = Hash::hash(thing_to_be_signed);
        
        let mut hashed_thing_vec = Vec::new();
        for i in hashed_thing.chars() {
            hashed_thing_vec.push(i.to_string().parse::<i32>().unwrap())
        }
        
        let mut signed_vec = Vec::new();
        for i in hashed_thing_vec {
            signed_vec.push(Keys::exp_mod(self, i, signing_key,));
        }

        signed_vec
    }
    
    // Check signature on a TX
    pub fn check_tx_signature(self,
                              tx: TX) -> bool {
        
        let tx_sig_check: Vec<i32> = tx.clone().signature;
        
        let mut tx_sig_check_pub_signed = Vec::new();
        for i in tx_sig_check {
            tx_sig_check_pub_signed.push(Keys::exp_mod(self, i, tx.data.sender_pub_key))
        }
        
        let mut tx_sig_check_string = String::new();
        for i in tx_sig_check_pub_signed {
            tx_sig_check_string.push_str(&i.to_string())
        }
        
        let hashed_tx = Hash::hash(&tx.data);
        
        if tx_sig_check_string == hashed_tx {
            return true
        } else {
            return false
        }
    }
}


// State Transition Function
/////////////////////////////////////////////////////////

// This struct holds all the data needed for 
// the chosen state transition protocol.
// In this case we're doign PoS, but if you
// wanted to impliment PoW you would write a new
// STF struct and new verify_pending_tx and proof
// functions.
#[derive(Debug, Clone)]
pub struct STF {
    version: String, // PoA, PoW, PoS, etc...
    difficulty: i32,
    current_validator: i32,
}

impl STF {
    
    // Standin for a "random beacon"
    pub fn random_validator_selection(state: &mut State) {
        
        // kick out any validators who don't meet the 
        // difficulty requirements
        // TODO: find a more "rust like" way to do this
        // without clone()
        let state_copy = state.clone();
        let mut count = 0;
        for i in state_copy.validators {
            if state.accounts.get(&i).unwrap().balance < state.stf.difficulty {
                state.validators.remove(count as usize);
            }
            count += 1;
        }
        
        // check that there are validators
        if state.validators.len() <= 0 {
            println!("ERROR: no known validators.");
            return
        }
        
        // randomly select a validator from the validator Vec
        let validator_num = thread_rng().gen_range(0, state.validators.len());
        state.stf.current_validator = state.validators[validator_num];
    }
    
    // This function encodes the rules of what qualifies as a "valid tx"
    pub fn verify_pending_tx(state: &mut State) -> Vec<TX> {
        
        let mut new_state = state.clone();
        let mut verified_tx = Vec::new();
        
        for i in &new_state.pending_tx {
        
            if !(new_state.accounts.contains_key(&i.data.sender_pub_key)) {
                println!("Invalid TX: sender_pub_key not found.");
                continue
            }
            
            if !(new_state.accounts.contains_key(&i.data.receiver)) {
                println!("Invalid TX: receiver not found.");
                continue
            }
            
            if !(i.data.amount > 0) {
                println!("Invalid TX: negative amount error.");
                println!("{} cannot send {} to {}", i.data.sender_pub_key, i.data.amount, i.data.receiver);
                continue
            }
            
            if !(new_state.accounts.get(&i.data.sender_pub_key).unwrap().balance > i.data.amount) {
                println!("Invalid TX: insufficient funds.");
                println!("{} has {} and cannot send {} to {}", i.data.sender_pub_key, new_state.accounts.get(&i.data.sender_pub_key).unwrap().balance, i.data.amount, i.data.receiver);
                continue   
            }
            
            if !(i.data.sender_pub_key_nonce == new_state.accounts.get(&i.data.sender_pub_key).unwrap().nonce) {
                println!("Invalid TX: potential replay tx.");
                println!("{} has nonce {}, but submitted a tx with nonce {}", i.data.sender_pub_key, new_state.accounts.get(&i.data.sender_pub_key).unwrap().nonce, i.data.sender_pub_key_nonce);
                continue
            }
            
            if !(Keys::check_tx_signature(new_state.keys, i.clone())) {
                println!("Invalid TX: signature check failed");
                continue
            }
            
            verified_tx.push(i.clone());
            new_state.accounts.get_mut(&i.data.sender_pub_key).unwrap().balance -= i.data.amount;
            new_state.accounts.get_mut(&i.data.receiver).unwrap().balance += i.data.amount;
        }
        
        verified_tx
    }

    // This function creates a proof that authorizes the state transition
    // This can be as complex as desired such as in a PoW setting 
    // Or it can simply hash the publicly announced validator address like
    // it does here :)
    pub fn proof(state: &State) -> String {
    
        let hash = Hash::hash(&state.stf.current_validator);
        
        hash
    }
    
    // Create A New Block With Valid Transactions
    pub fn create_block(state: &mut State) -> Block {
    
        let verified_tx = STF::verify_pending_tx(state);
        let header = BlockHeader {
            nonce: 0,
            timestamp: time::now().to_timespec().sec as i32,
            block_number: state.history.last().unwrap().data.header.block_number + 1,
            previous_block_hash: Hash::hash(&state.history.last().unwrap().data.header.current_block_hash),
            current_block_hash: Hash::hash_tree(verified_tx.clone()),
        };
        
        let data = BlockData {
            header: header,
            transactions: verified_tx, 
        };
        let proof = STF::proof(state);
        
        let block = Block {
            proof: proof,
            data: data,
        };
        
        block
    }
    
    // function to transition the state
    pub fn check_block(state: &State,
                       block: &Block) -> bool {
        
        // proof to check
        let submitted_proof = &block.proof;
        
        // check that validator matches currently chosen STF validator
        if submitted_proof != &Hash::hash(&state.stf.current_validator) {
            println!("\nProof Error: invalid PoS validator.");
            return false
        }
        
        // check validator account has enough state
        let validator_balance = state.accounts.get(&state.stf.current_validator).unwrap().balance;
        if !(validator_balance > state.stf.difficulty) {
            println!("ERROR: block proof does not meet difficulty requirements.");
            return false
        }
        
        // if tests are passed, return true
        true
    }
    
}


// State
/////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Account {
    balance: i32,
    nonce: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TxData {
    sender_pub_key: i32,
    sender_pub_key_nonce: i32,
    amount: i32,
    receiver: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TX {
    data: TxData,
    signature: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockHeader {
    nonce: i32,
    timestamp: i32,
    block_number: i32,
    previous_block_hash: String,  
    current_block_hash: String,  
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockData {
    header: BlockHeader,
    transactions: Vec<TX>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    proof: String,
    data: BlockData,
}

#[derive(Debug, Clone)]
pub struct State {
    keys: Keys,
    stf: STF,
    accounts: HashMap<i32, Account>,
    priv_keys: HashMap<i32, i32>, // this is for testing only to simulate network activity
    validators: Vec<i32>,
    pending_tx: Vec<TX>,
    history: Vec<Block>,
}

impl State {

    // Create a new state
    pub fn create_state() -> State {
        
        // Key Generation Parameters
        // account key range
        let min = 1;
        let max = 100000; 
        // set p and q for RSA system
        // p and q must be prime, the larger the better
        // - for Rust Playground: p && q < 300
        // https://en.wikipedia.org/wiki/List_of_prime_numbers
        // https://en.wikipedia.org/wiki/RSA_(cryptosystem)#Key_generation
        let p = 61; //173; //Keys::prime_gen(min, max); // we want between 5 and 1000000
        let q = 53; //223; //Keys::prime_gen(min, max); // we want between 5 and 1000000
        let m = p * q; 
        let ctf_pq = Keys::ctf(p, q);
        // set up Keys struct to hold the RSA parameters
        let rsa_params = Keys {
            min: min,
            max: max,
            p: p,
            q: q,
            modulo: m,
            ctf_pq: ctf_pq,
        };

        // State Transition Function data
        let stf_data = STF {
            version: String::from("PoS"),
            difficulty: 100,
            current_validator: 0, // changes every block
        };

        let genesis_block = Block {
                proof: String::from("GENESIS BLOCK"),
                data: BlockData {
                    header: BlockHeader {
                        nonce: 0,
                        timestamp: time::now().to_timespec().sec as i32,
                        block_number: 0,
                        previous_block_hash: String::from("N/A"),  
                        current_block_hash: Hash::hash(&String::from("")),  
                    },
                    transactions: Vec::new(),
                }
            };
        
        let new_state = State {
            keys: rsa_params,
            stf: stf_data,
            accounts: HashMap::new(),
            priv_keys: HashMap::new(),
            validators: Vec::new(),
            pending_tx: Vec::new(),
            history: vec![genesis_block],
        };
        
        new_state
    }

    // Create a new account
    pub fn create_account(&mut self) {
        
        // create a new priv/pub key pair
        let (priv_key, pub_key) = Keys::generate_keypair(self.keys);
        
        // check to make sure we're not duplicating an existing account
        // - this is important when we're using small numbers for key 
        // generation while testing
        if self.accounts.contains_key(&pub_key) {
            println!("ERROR: account generation collision.");
            return
        }
        
        // if no collisions, create a new account
        let new_account = Account {
            balance: 10000, // init account with testnet tokens
            nonce: 0,
        };
        
        // Add public keys and account to the "blockchain"
        self.accounts.insert(pub_key, new_account);
        
        // This to simulate tx and network activity while testing.
        // Remove if you want to print out and store your keys.
        self.priv_keys.insert(pub_key, priv_key);
        
        // Uncomment if you want to keep your private keys offline
        //println!("\nThis is your public key: {:#?}", &pub_key);
        //println!("This is your private key: {:#?}", &priv_key);
        //println!("This is your account: {:#?}", self.accounts.get(&pub_key).unwrap());
    

    }
    
    // Create a new TX
    pub fn create_tx(&mut self,
                     sender_pub_key: i32,
                     sender_pub_key_priv_key: i32,
                     receiver_pub_key: i32,
                     amount: i32) {
        
        // increase sender's nonce by 1 to prevent replay attacks
        self.accounts.get_mut(&sender_pub_key).unwrap().nonce += 1;
        
        let data = TxData {
            sender_pub_key: sender_pub_key,
            sender_pub_key_nonce: self.accounts.get(&sender_pub_key).unwrap().nonce,
            receiver: receiver_pub_key,
            amount: amount,
        };
        let signature = Keys::sign(self.keys, &data, sender_pub_key_priv_key);
        let tx = TX {
            data: data,
            signature: signature,
        };
        
        // push tx to the pending_tx pool
        self.pending_tx.push(tx);
    }
    
    // Testing function to generate a tx with random
    // sender, reciever, and amount
    pub fn create_random_tx(&mut self) {
        
        // get random keys from the account pool
        let keys: Vec<i32> = self.accounts.iter().map(|x| *x.0).collect();
        let sender_pub_key = keys[thread_rng().gen_range(0, keys.len())];
        let sender_priv_key = self.priv_keys.get(&sender_pub_key).unwrap();
        let receiver = keys[thread_rng().gen_range(0, keys.len())];
        
        // increase sender's nonce by 1 to prevent replay attacks
        self.accounts.get_mut(&sender_pub_key).unwrap().nonce += 1;
        
        // create a tx from the randomly chosen keys
        let data = TxData {
            sender_pub_key: sender_pub_key,
            sender_pub_key_nonce: self.accounts.get(&sender_pub_key).unwrap().nonce,
            amount: thread_rng().gen_range(1, self.accounts.get(&sender_pub_key).unwrap().balance),
            receiver: receiver,
        };
        let signature = Keys::sign(self.keys, &data, *sender_priv_key);
        let tx = TX {
            data: data,
            signature: signature,
        };

        // push tx to the pending_tx pool
        self.pending_tx.push(tx);
    }
    
    // function to add an account to the validator Vec
    pub fn create_validator(&mut self,
                            account: i32) {
        
        self.validators.push(account);
    }
    
    // function to iterate through all accounts eligible
    // to participate in the validator pool and randomly
    // "flip a coin" to decide if they join the validator
    // pool
    pub fn create_random_validators(&mut self) {
        
        let keys: Vec<i32> = self.accounts.iter().map(|x| *x.0).collect();
        for i in keys {
            match random() {
                true => self.create_validator(i),
                false => continue
            }
        }
    }
    
    // function to transition the state to a new state
    pub fn create_new_state(&mut self) {
        
        // "publicly" select a random validator
        STF::random_validator_selection(self);
        
        // check tx and put valid ones into a block
        let block = STF::create_block(self);
        
        // check that the block proof is valid
        if !(STF::check_block(&self, &block)) {
            // if block is not valid slash the current validator's funds
            println!("\nERROR: block not valid.");
            self.accounts.get_mut(&self.stf.current_validator).unwrap().balance -= self.stf.difficulty;
            return
        }
        
        // if block is valid add reward to current validator's balance
        self.accounts.get_mut(&self.stf.current_validator).unwrap().balance += self.stf.difficulty;
        
        // transition the state by incorporating the
        // information in the new block
        for i in &block.data.transactions {
            self.accounts.get_mut(&i.data.sender_pub_key).unwrap().balance -= i.data.amount;
            self.accounts.get_mut(&i.data.receiver).unwrap().balance += i.data.amount;
        }
        
        // add the block to the history and clear pending tx pool
        self.history.push(block);
        self.pending_tx.clear();
    }
}



fn main() {
    
    // Init The "Blockchain"
    let mut blockchain = State::create_state();
    //println!("\nBLOCKCHAIN:\n{:#?}", &blockchain);
    
    // Create New Accounts
    // create some new accounts
    for _ in 0..100 {
        blockchain.create_account();
    }
    // randomly add accounts to the validator pool
    blockchain.create_random_validators();
    //println!("\nBLOCKCHAIN:\n{:#?}", &blockchain);
    
    // Test TX and State Transition Function
    // simulate 10 blocks
    for _ in 0..10 {
        // simulate 10 tx per block
        for _ in 0..10 {
            blockchain.create_random_tx();
        }
        // create new block and transition the state
        blockchain.create_new_state();
    }
    println!("\n\n\nBLOCKCHAIN:\n{:#?}", &blockchain);
}
