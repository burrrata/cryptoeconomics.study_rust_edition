<p align="center">
    <img src="chapter_summary.jpg">  
</p>

<h1 align="center">
    PoS "Blockchain"
</h1>
<p align="center">
  You get a stake! You get a stake! You get a Stake!
</p>

<br>

## Words

Core Concepts
- RIP the Lorax: save the trees
- PoW is doomed to specialized ASIIC domination whereas PoS allows for more democratized access to blockchain security
- PoS also allows you to create much more interesting and compelling incentivization mechanisms such as slashing and correlated slashing whereas with PoW the attacker doesn't ever lose their compute power and they can launch multiple attacks at little to no increased cost
- PoS also lowers the barriers to entry of becomming a validator and contributing to the network :)

<br>

## Videos

<br>

## Code
```rust
extern crate rand;
use rand::prelude::*;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

/* GOAL
A modular architecture where you can change any of the modules,
say changing PoW to PoS, and it still runs.
*/

/* QUESTIONS
Why does this panic?
*/


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
            tx_sig_check_pub_signed.push(Keys::exp_mod(self, i, tx.data.sender))
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


// This struct holds all the data needed for 
// the chosen state transition protocol.
// In this case we're doign PoS, but if you
// wanted to impliment PoW you would write a new
// STF struct and new verify_pending_tx and proof
// functions.
#[derive(Debug)]
pub struct STF {
    version: String, // PoA, PoW, PoS, etc...
    difficulty: i32,
    validator: i32,
}

impl STF {
    
    // A "random beacon"
    pub fn random_validator_selection(state: &mut State) {
        
        let validator: i32 = thread_rng().gen_range(0, state.validators.len() as i32);
        state.stf.validator = validator;
    }
    
    // This function encodes the rules of what qualifies as a "valid tx"
    pub fn verify_pending_tx(state: &mut State) -> Vec<TX> {
        
        let mut verified_tx = Vec::new();
        
        for i in &state.pending_tx {
        
            if !(state.accounts.contains_key(&i.data.sender)) {
                println!("Invalid TX: sender not found.");
                continue
            }
            
            if !(state.accounts.contains_key(&i.data.receiver)) {
                println!("Invalid TX: receiver not found.");
                continue
            }
            
            if !(i.data.amount > 0) {
                println!("Invalid TX: negative amount error.");
                println!("{} cannot send {} to {}", i.data.sender, i.data.amount, i.data.receiver);
                continue
            }
            
            if !(state.accounts.get(&i.data.sender).unwrap().balance > i.data.amount) {
                println!("Invalid TX: insufficient funds.");
                println!("{} cannot send {} to {}", i.data.sender, i.data.amount, i.data.receiver);
                continue         
            }
            
            if !(i.data.sender_nonce == state.accounts.get(&i.data.sender).unwrap().nonce) {
                println!("Invalid TX: potential replay tx.");
                println!("{} has nonce {}, but submitted a tx with nonce {}", i.data.sender, state.accounts.get(&i.data.sender).unwrap().nonce, i.data.sender_nonce);
                continue
            }
            
            if !(Keys::check_tx_signature(state.keys, i.clone())) {
                println!("Invalid TX: signature check failed");
                continue
            }
            
            verified_tx.push(i.clone());
        }
        
        verified_tx
    }

    // This function creates a proof that authorizes the state transition
    // This is a variation of PoW that's easy enough that it runs in the Rust Playground 
    // You could change the logic of this function to satisfy PoS or PoA as well.
    pub fn proof(state: &State) -> String {
    
        let hash = Hash::hash(&state.stf.validator);
        
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
                       block: &mut Block) -> bool {
        
        // proof to check
        let submitted_proof = &block.proof;
        
        // check that validator matches randomly chosen STF validator
        if submitted_proof != &Hash::hash(&state.stf.validator) {
            println!("\nProof Error: invalid PoS validator.");
            return false
        }
        
        // check validator account has enough state
        let validator_balance = state.accounts.get(&state.stf.validator).unwrap().balance;
        if !(validator_balance > state.stf.difficulty) {
            println!("ERROR: block proof does not meet difficulty requirements.");
            return false
        }
        
        // if tests are passed, return true
        true
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

// TODO
// - does it make sense to add more data to the State?
//   STF (type, difficulty, etc...)
//   KEY_PARAMS (type, p, q, modulo, etc..)
//   or maybe CRYPTO (KEY_PARAMS, hash function, hash tree function, etc...)
#[derive(Debug)]
pub struct State {
    keys: Keys,
    stf: STF,
    accounts: HashMap<i32, Account>,
    validators: Vec<i32>,
    pending_tx: Vec<TX>,
    history: Vec<Block>,
}

impl State {

    // Create a new state
    pub fn create_state() -> State {
        
        let rsa_params = Keys {
            min: 0,
            max: 1000000,
            p: 61,
            q: 53,
            modulo: 3233,
            ctf_pq: 780,
        };

        let stf_data = STF {
            version: String::from("PoS"),
            difficulty: 10,
            validator: 0,
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
            validators: Vec::new(),
            pending_tx: Vec::new(),
            history: vec![genesis_block],
        };
        
        new_state
    }

    // Create a new account
    pub fn create_account(&mut self) {
        
        // TODO
        // - How can I make Keys::generator_keypair() not
        //   take in anything as input and have all the params
        //   stored within the Keys library?
        let (priv_key, pub_key) = Keys::generate_keypair(self.keys);
        let new_account = Account {
            balance: 0,
            nonce: 0,
        };
        
        if self.accounts.contains_key(&pub_key) {
            println!("Bummer... account collision.");
            return
        }
        
        self.accounts.insert(pub_key, new_account);
        //println!("\nThis is your public key: {:#?}", &pub_key);
        //println!("This is your private key: {:#?}", &priv_key);
        //println!("This is your account: {:#?}", self.accounts.get(&pub_key).unwrap());
    }
    
    // Create a new TX
    pub fn create_tx(&mut self,
                     sender_pub_key: i32,
                     sender_priv_key: i32,
                     receiver_pub_key: i32,
                     amount: i32) {
        
        
        let data = TxData {
            sender: sender_pub_key,
            sender_nonce: self.accounts.get(&sender_pub_key).unwrap().nonce,
            receiver: receiver_pub_key,
            amount: amount,
        };
        
        let signature = Keys::sign(self.keys, &data, sender_priv_key);
        
        let tx = TX {
            data: data,
            signature: signature,
        };
        
        self.pending_tx.push(tx);
    }

    // function to transition the state to a new state
    pub fn create_new_state(&mut self) {
        
        // "publicly" select a random validator
        STF::random_validator_selection(self);
    
        // check tx and put valid ones into a block
        let mut block = STF::create_block(self);
        
        // check that the block proof is valid
        if !(STF::check_block(&self, &mut block)) {
            println!("\nERROR: block not valid.");
            return
        }
        
        // transition the state by incorporating the
        // information in the new block
        for i in &block.data.transactions {
            self.accounts.get_mut(&i.data.sender).unwrap().balance -= i.data.amount;
            self.accounts.get_mut(&i.data.receiver).unwrap().balance += i.data.amount;
            self.accounts.get_mut(&i.data.sender).unwrap().nonce += 1;
        }
        
        // add the block to the history
        self.history.push(block);
    }
}



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

```

<br>

## Resources

Ethereum PoS FAQ
- https://github.com/ethereum/wiki/wiki/Proof-of-Stake-FAQs

<br>
