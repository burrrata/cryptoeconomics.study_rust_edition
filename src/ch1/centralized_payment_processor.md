```rust
extern crate rand;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use rand::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;


// USEFUL FUNCTIONS
// variable names based off Euclidean divison equation: a = b · q + r
// https://crates.io/crates/gcd
// https://en.wikipedia.org/wiki/Greatest_common_divisor
fn gcd(a: i32,
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
// https://en.wikipedia.org/wiki/Least_common_multiple
fn lcm(a: i32,
       b: i32) -> i32 {
    
    let lcm = (a * b) / gcd(a, b);
    
    lcm
}

// Carmichael's totient function
// https://en.wikipedia.org/wiki/Carmichael_function
fn ctf(a: i32,
       b: i32) -> i32 {
    
    lcm((a - 1), (b - 1))
}

// slowly check if a number is prime
fn slow_prime_check(num: i32) -> bool {
    
    if num < 0 {
        println!("number must be greater than 0");
    }
    
    if num > 1000000 {
        println!("number cannot be greater than 1000000");
    }
    
    for i in 2..num{
        if num % i == 0 {
            return false
        }
    }
    true
}

// slowly yet randomly generate a prime number within a range
fn prime_gen(low: i32,
             high: i32) -> i32 {
    
    for i in 0..1000000 {
        let p = thread_rng().gen_range(low, high);
        if slow_prime_check(p) {
            return p
        }
    }
    0
}

// generate a public key within a range
fn pub_key_gen(min: i32,
               max: i32) -> i32 {
    
    let pub_key = prime_gen(min, max);
    assert!(max % pub_key != 0);
    
    pub_key
}

// slowly find the modular multiplicative inverse of a prime 
fn slow_mmi(ctf_pq: i32,
            pub_key: i32,
            max: i32)-> i32 {
    
    for i in 2..max {
        if (i * pub_key) % ctf_pq == 1 {
            return i
        }
    }
    println!("Try larger search?");
    0
}

// create a private key from a public key and other data
fn priv_key_gen(ctf_pq: i32,
                pub_key: i32) -> i32 {
    
    let priv_key = slow_mmi(ctf_pq, pub_key, 100000);
    
    priv_key
}

// Because... Rust.
// exp_mod() is like pow() with a mod option
// (like python does natively, but not Rust)
// https://docs.python.org/3/library/functions.html#pow
// https://doc.rust-lang.org/nightly/std/primitive.i32.html#method.pow
// https://en.wikipedia.org/wiki/Modular_exponentiation
fn exp_mod(input: i32,
           power: i32,
           modulo: i32) -> i32 {
    
    let mut out = (input * input) % modulo;
    // because the first iter of out took 2 off the base
    for i in 0..power-2 {
        out = (out * input) % modulo;
    }
    
    out
}

// toy RSA function
fn toy_rsa(input: Vec<i32>,
           key: i32,
           modulo: i32) -> Vec<i32> {
    
    let output = input.iter()
                      .map(|x| exp_mod(*x, key, modulo))
                      .collect();
    output
}

// convert string to Vec<i32>
fn s2v(input: String) -> Vec<i32> {
    
    let output: Vec<i32> = input.as_bytes()
                                .iter()
                                .map(|x| *x as i32)
                                .collect();
    
    output
}

// convert Vec<i32> to string
fn v2s(input: Vec<i32>) -> String {
    
    let output_u8: Vec<u8> = input.iter()
                                  .map(|x| *x as u8)
                                  .collect();
    let output_string = String::from_utf8(output_u8).unwrap();
    
    output_string
}


// STRUCTS
#[derive(Debug)]
struct State {
    accounts: HashMap<String, Account>,
    pending_tx: Vec<TX>,
    verified_tx: Vec<TX>,
    history: Vec<Vec<TX>>,
}

#[derive(Debug)]
struct Account {
balance: f32,
nonce: i32,
}

#[derive(Debug, Clone)]
struct TX {
    sender: String,
    receiver: String,
    tx_amount: f32,
    nonce: i32,
}

#[derive(Debug, Clone)]
struct Signed_TX {
    tx: TX,
    signature: String,
}


// STATE
impl State {

    // initialize new blockchain
    pub fn new_blockchain() -> State {
        let mut state = State {
            accounts: HashMap::new(),
            pending_tx: Vec::new(),
            verified_tx: Vec::new(),
            history: Vec::new(),
        };
    
        state
    }    
    
    // generate a new key
    pub fn key_gen() -> String {
        
        let rn: i32 = thread_rng().gen_range(100000, 1000000);
        
        rn.to_string()
    }
    
    // hash stuff
    pub fn hash<T: serde::Serialize>(item: &T) -> String {
    
        let input = serde_json::to_string(&item).unwrap();
        let input_bytes = input.as_bytes();
        
        let mut hasher = DefaultHasher::new();
        hasher.write(input_bytes);
        let digest = hasher.finish();
        let hex_digest = format!("{:#X}", digest);
        
        hex_digest
    }
    
    // create new account
    pub fn new_account(&mut self) {
        
        let priv_key = State::key_gen();
        let pub_key = State::hash(& priv_key.clone());
        let new_account = Account {
            balance: 100.0,
            nonce: 0,
        };
        
        self.accounts.insert(pub_key.clone(), new_account);
        
        println!("\nThis is your public key: {:#?}", pub_key);
        println!("This is your private key: {:#?}", priv_key);
        println!("This is your account: {:#?}", self.accounts.get(&pub_key).unwrap());
    }

    // create a tx and add it to the pending_tx pool
    pub fn new_tx(&mut self,
                  priv_key: &str,
                  receiver: &str,
                  tx_amount: f32) {
        
        let tx = TX {
            sender: State::hash(&priv_key),
            receiver: receiver.to_string(),
            tx_amount: tx_amount,
            nonce: self.accounts.get(&State::hash(&priv_key)).unwrap().nonce,
        };

        self.pending_tx.push(tx);
    }
    
    // UNDER CONSTRUCTION
    /*
    pub fn sign_tx(priv_key: String,
                   tx: TX) -> String {
        
        let x = Vec::new();
        let mut hasher = DefaultHasher::new();
        let hashed_tx = Hash::hash_tx(tx, hasher);
        x.push(hashed_tx);
        x.push(priv_key);
        let signed_tx_hash = x.join("");
        
        signed_tx_hash
    }
    */
    
    // verify the tx in the pending_tx pool
    pub fn verify_tx(&mut self) {
        
        println!("\nVerifying TX:");
        
        for i in & self.pending_tx {
        
            println!("{:#?}", &i);
            
            if !self.accounts.contains_key(&i.sender) {
                println!("Invalid TX: sender not found.");
                break
            } 
            
            if !self.accounts.contains_key(&i.receiver) {
                println!("Invalid TX: receiver not found.");
                break
            }
            
            if !(i.tx_amount > 0.0) {
                println!("Invalid TX: negative amount error.");
                println!("{} cannot send {} to {}", i.sender, i.tx_amount, i.receiver);
                break
            } 
            
            if !(self.accounts.get(&i.sender).unwrap().balance > i.tx_amount) {
                println!("Invalid TX: insufficient funds.");
                println!("{} cannot send {} to {}", i.sender, i.tx_amount, i.receiver);
                break            
            }
            
            if !(i.nonce == self.accounts.get(&i.sender).unwrap().nonce) {
                println!("Invalid TX: potential replay tx.");
                println!("{} has nonce {}, but submitted a tx with nonce {}", i.sender, self.accounts.get(&i.sender).unwrap().nonce, i.nonce);
                break
            }
            
            println!("Valid TX.");
            self.verified_tx.push(i.clone());
        }
        
        self.pending_tx = Vec::new();
    }
    
    // apply and confirm valid_tx pool
    pub fn confirm_tx(&mut self) {
        
        println!("\nConfirming TX:");
        
        let mut block = Vec::new();
        
        for i in & self.verified_tx {
            
            self.accounts.get_mut(&i.sender).unwrap().balance -= i.tx_amount;
            self.accounts.get_mut(&i.receiver).unwrap().balance += i.tx_amount;
            self.accounts.get_mut(&i.sender).unwrap().nonce += 1;
            println!("{} sent {} to {}", &i.sender, &i.tx_amount, &i.receiver);
            
            block.push(i.clone())
        }
        
        self.history.push(block);
        self.verified_tx = Vec::new();
    }
}


// Rollin, rollin, rollin... Rolling our own blockchain! :)
fn main() {

    // TODO
    // - merge RSA functions into impl State
    // - generate keys from toy_rsa not the default hasher
    // - find a way to encrypt struct TX with toy_rsa()
    // - incorporate signatures into TX
    // - check TX to verify sender matches signature

    // Testing RSA Stuff
    // usually works on Rust Playground when p and q are < 500
    let p = prime_gen(5, 100);
    let q = prime_gen(5, 100);
    let m = p * q; 
    let ctf_pq = ctf(p, q);
    let pub_key = pub_key_gen(1, ctf_pq);
    let priv_key = priv_key_gen(ctf_pq, pub_key);
    println!("\n// Params //");
    assert!(p > 0);
    assert!(q > 0);
    println!("p: {}", &p);
    println!("q: {}", &q);
    println!("m: {}", &m);
    println!("ctf_pq: {}", &ctf_pq);
    println!("pub_key: {}", &pub_key);
    println!("priv_key: {}", &priv_key);
    let message = String::from("thepasswordispassword");
    let m2nums = s2v(message.clone());
    let ciphertext = toy_rsa(m2nums.clone(), pub_key, m);
    let decrypted = toy_rsa(ciphertext.clone(), priv_key, m);
    let message2 = v2s(decrypted.clone());
    assert_eq!(message, message2);
    println!("\n// Testing //");
    println!("message: {:?}", &message);
    println!("message as nums: {:?}", &m2nums);
    println!("ciphertext: {:?}", &ciphertext);
    println!("decrypted nums: {:?}", &decrypted);
    println!("decrypted message: {}", &message2);
    println!("DONE!");


    // Init Blockchain
    // init blockchain state 
    let mut state = State::new_blockchain();
    
    // Init Accounts
    // create 3 random accounts
    for i in 0..3 {
        state.new_account()
    }
    // manually create account for testing
    let t0_priv = String::from("693677"); // 693677
    let t0_pub = State::hash(&t0_priv); // 0xC31B6988D3A6A62B
    let t0 = Account {
        balance: 10000.0,
        nonce: 0,
    };
    state.accounts.insert(t0_pub.clone(), t0);
    // manually create account for testing
    let t1_priv = String::from("172218"); // 172218
    let t1_pub = State::hash(&t1_priv); // 0x81C52538C70E98B7
    let t1 = Account {
        balance: 10000.0,
        nonce: 0,        
    };
    state.accounts.insert(t1_pub.clone(), t1);
    // check results
    println!("\n{:#?}", state);
    
    // Test TX 
    // add some tx to the pending_tx pool
    state.new_tx(&t0_priv, &t1_pub, 500.0);
    state.new_tx(&t1_priv, &t0_pub, 127.0);
    state.new_tx(&t0_priv, &t1_pub.clone(), 1000.0);
    // verify valid tx
    state.verify_tx();
    // cofirm tx and change state
    state.confirm_tx();
    // check results
    println!("\n\nCurrent State:\n{:#?}", state);
    
}
```
