// TODO
// How do we not use rand as an external crate?
// Or at least figure out how to import it into mdBook?
extern crate rand;
use rand::prelude::*;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;


// Old State Struct
/*
#[derive(Debug)]
struct State {
    modulo: i32,
    accounts: HashMap<i32, Account>,
    pending_tx: Vec<SignedTX>,
    verified_tx: Vec<SignedTX>,
    history: Vec<Vec<SignedTX>>,
}
*/

#[derive(Debug)]
struct State {
    modulo: i32,
    accounts: HashMap<i32, Account>,
    pending_tx: Vec<SignedTX>,
    chain: Vec<block>,
}

#[derive(Debug, Clone)]
struct Account {
    balance: f32,
    nonce: i32,
}

#[derive(Debug, Clone)]
struct TX {
    sender: i32,
    receiver: i32,
    amount: f32,
    nonce: i32,
}

#[derive(Debug, Clone)]
struct SignedTX {
    tx: TX,
    signature: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct Blockheader {
    timestamp: i64,
    nonce: i32, 
    pre_hash: String,  
    merkle: String,  
}

#[derive(Debug, Clone)]
pub struct Block {
    header: Blockheader,
    transactions: Vec<TX>
}


impl State {

    /// "RSA" KEY GENERATION STUFF ///
    // Would it make more sense to have the RSA stuff in 
    // it's own impl ?
    
    // variable names based off Euclidean divison equation: a = b · q + r
    // https://crates.io/crates/gcd
    // https://en.wikipedia.org/wiki/Greatest_common_divisor
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
    // https://en.wikipedia.org/wiki/Least_common_multiple
    pub fn lcm(a: i32,
           b: i32) -> i32 {
        
        let lcm = (a * b) / State::gcd(a, b);
        
        lcm
    }
    
    // Carmichael's totient function
    // https://en.wikipedia.org/wiki/Carmichael_function
    pub fn ctf(a: i32,
           b: i32) -> i32 {
        
        State::lcm((a - 1), (b - 1))
    }
    
    // slowly check if a number is prime
    pub fn slow_prime_check(num: i32) -> bool {
        
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
    pub fn prime_gen(low: i32,
                 high: i32) -> i32 {
        
        for i in 0..1000000 {
            let p = thread_rng().gen_range(low, high);
            if State::slow_prime_check(p) {
                return p
            }
        }
        0
    }
    
    // slowly find the modular multiplicative inverse of a prime 
    pub fn slow_mmi(ctf_pq: i32,
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
    
    // generate a public key within a range
    pub fn pub_key_gen(min: i32,
                   max: i32) -> i32 {
        
        let pub_key = State::prime_gen(min, max);
        assert!(max % pub_key != 0);
        
        pub_key
    }
    
    // create a private key from a public key and other data
    pub fn priv_key_gen(ctf_pq: i32,
                    pub_key: i32) -> i32 {
        
        let priv_key = State::slow_mmi(ctf_pq, pub_key, 100000);
        
        priv_key
    }
    
    // Because... Rust.
    // exp_mod() is like pow() with a mod option
    // (like python does natively, but not Rust)
    // https://docs.python.org/3/library/functions.html#pow
    // https://doc.rust-lang.org/nightly/std/primitive.i32.html#method.pow
    // https://en.wikipedia.org/wiki/Modular_exponentiation
    pub fn exp_mod(input: i32,
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
    pub fn toy_rsa(input: Vec<i32>,
               key: i32,
               modulo: i32) -> Vec<i32> {
        
        let output = input.iter()
                          .map(|x| State::exp_mod(*x, key, modulo))
                          .collect();
        output
    }

    /// "BLOCKCHAIN" STUFF ///
    
    // Initialize A "Blockchain"
    pub fn new_blockchain() -> State {
        let mut state = State {
            modulo: 0,
            accounts: HashMap::new(),
            pending_tx: Vec::new(),
            verified_tx: Vec::new(),
            history: Vec::new(),
        };
    
        state
    }    
    
    // Create New Account
    pub fn new_account(&mut self, ctf_pq: i32) {
        
        let pub_key = State::pub_key_gen(1, ctf_pq);
        let priv_key = State::priv_key_gen(ctf_pq, pub_key);
        let new_account = Account {
            balance: 100.0,
            nonce: 0,
        };
        
        if self.accounts.contains_key(&pub_key) {
            println!("Bummer... account collision.");
        }
        self.accounts.insert(pub_key.clone(), new_account);
        
        println!("\nThis is your public key (address): {:#?}", &pub_key);
        println!("This is your private key (signing key): {:#?}", &priv_key);
        println!("This is your account: {:#?}", self.accounts.get(&pub_key).unwrap());
    }
    
    // Turn Arbitrary Stuff Into &[u8] Slice
    pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
        ::std::slice::from_raw_parts(
            (p as *const T) as *const u8,
            ::std::mem::size_of::<T>(),
        )
    }

    // Hash &[u8] Into Hex String
    pub fn hash_u8(stuff: &[u8]) -> String {
        
        let mut hasher = DefaultHasher::new();
        hasher.write(stuff);
        let digest = hasher.finish();
        let hex_digest = format!("{:#X}", digest);
            
        hex_digest
    }    
    
    // Takes in stuff
    // Turns it to a u8 slice
    // Hashes that slice into a hex string
    pub fn hash_any<T>(stuff: &T) -> String {
        
        let u8_stuff = unsafe {
            any_as_u8_slice(stuff)
        };
        let hash_of_stuff = hash_u8(u8_stuff);
        
        hash_of_stuff
    }
    
    // convert string to Vec<i32>
    pub fn s2v(input: String) -> Vec<i32> {
        
        let output: Vec<i32> = input.as_bytes()
                                    .iter()
                                    .map(|x| *x as i32)
                                    .collect();
        
        output
    }
    
    // convert Vec<i32> to string
    pub fn v2s(input: Vec<i32>) -> String {
        
        let output_u8: Vec<u8> = input.iter()
                                      .map(|x| *x as u8)
                                      .collect();
        let output_string = String::from_utf8(output_u8).unwrap();
        
        output_string
    }    
    
    // Create A TX And Add It To The pending_tx Pool
    pub fn new_signed_tx(&mut self,
                  sender_pub_key: i32,
                  sender_priv_key: i32,
                  receiver: i32,
                  amount: f32,
                  m: i32) {
        
        // Create TX
        let tx = TX {
            sender: sender_pub_key,
            receiver: receiver,
            amount: amount,
            nonce: self.accounts.get(&sender_pub_key).unwrap().nonce,
        };
        
        // Create Signature
        let tx_bytes: &[u8] = unsafe {
            State::any_as_u8_slice(&tx)
        };
        let tx_hash = State::hash_u8(tx_bytes);
        let signature = State::toy_rsa(State::s2v(tx_hash), sender_priv_key, m);
        
        // Create Signed TX
        let signed_tx = SignedTX {
            tx: tx,
            signature: signature,
        };
        
        // Add SignedTX to pending TX pool
        self.pending_tx.push(signed_tx);
    }
    
    // Check The Signature Of A SignedTX Matches The Sender
    // NOTE: 
    //   if the TX uses an invalid signature
    //   there is a high likelihood that it will produce
    //   invalid utf8, and thus this function will crash
    //   when v2s() tries to turn the Vec<i32> into a String
    // TODO:
    //   make the TX just fail rather than crashing the entire program  
    pub fn check_signed_tx(signed_tx: SignedTX,
                           modulo: i32) -> bool {
    
        let tx_as_bytes = unsafe {
            State::any_as_u8_slice(&signed_tx.tx)
        };
        let tx_hash = State::hash_u8(tx_as_bytes);
        println!("tx hash: {}", tx_hash);
        
        let decrypted_tx_hash_sig = State::toy_rsa(signed_tx.signature,
                                            signed_tx.tx.sender,
                                            modulo);
        let decrypted_tx_hash = State::v2s(decrypted_tx_hash_sig);
        println!("decrypted tx hash: {}", decrypted_tx_hash);
        
        match tx_hash == decrypted_tx_hash {
            true => true,
            false => {
                println!("not valid tx");
                return false
            },
        }
    }
    
    // Verify TX In The pending_tx Pool
    pub fn verify_tx(&mut self,
                     block: Block) -> Block {
        
        //println!("\nVerifying TX:");
        
        for i in & self.pending_tx {
        
            //println!("{:#?}", &i);
            
            if !self.accounts.contains_key(&i.tx.sender) {
                println!("Invalid TX: sender not found.");
                break
            } 
            
            if !self.accounts.contains_key(&i.tx.receiver) {
                println!("Invalid TX: receiver not found.");
                break
            }
            
            if !(i.tx.amount > 0.0) {
                println!("Invalid TX: negative amount error.");
                println!("{} cannot send {} to {}", i.tx.sender, i.tx.amount, i.tx.receiver);
                break
            } 
            
            if !(self.accounts.get(&i.tx.sender).unwrap().balance > i.tx.amount) {
                println!("Invalid TX: insufficient funds.");
                println!("{} cannot send {} to {}", i.tx.sender, i.tx.amount, i.tx.receiver);
                break            
            }
            
            if !(i.tx.nonce == self.accounts.get(&i.tx.sender).unwrap().nonce) {
                println!("Invalid TX: potential replay tx.");
                println!("{} has nonce {}, but submitted a tx with nonce {}", i.tx.sender, self.accounts.get(&i.tx.sender).unwrap().nonce, i.tx.nonce);
                break
            }
            
            if !(State::check_signed_tx(i.clone(), self.modulo)) {
                println!("TX No Good!");
                break
            }
            
            println!("Valid TX.");
            block.push(i.clone());
        }
        
        self.pending_tx = Vec::new();
        valid_block
    }
    
    // Confirm TX in valid_tx Pool And Add Them To The History
    pub fn confirm_tx(&mut self) {
        
        println!("\nConfirming TX:");
        
        let mut block = Vec::new();
        
        for i in & self.verified_tx {
            
            self.accounts.get_mut(&i.tx.sender).unwrap().balance -= i.tx.amount;
            self.accounts.get_mut(&i.tx.receiver).unwrap().balance += i.tx.amount;
            self.accounts.get_mut(&i.tx.sender).unwrap().nonce += 1;
            println!("{} sent {} to {}", &i.tx.sender, &i.tx.amount, &i.tx.receiver);
            
            block.push(i.clone())
        }
        
        self.history.push(block);
        self.verified_tx = Vec::new();
    }
    
    // Hash Previous Block
    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return vec![48; 64].unwrap()
        };
        hash_u8(&block.header)
    }
    
    // This should do a few things:
    // - verify tx
    // - add verified tx to a block
    // - confirm and process the block
    pub fn generate_new_block(&mut self) -> bool {
    
        let header = Blockheader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            pre_hash: hash_any(self.chain.last()),
            merkle: String::from("tbd"),
        };

        let mut block = Block {
            header: header,
            transactions: vec![]
        };

        // Confirm TX
        //block.transactions.append(&mut self.verified_tx;
        //block.header.merkle = State::get_merkle(block.transactions.clone());
        //println!("{:#?}", &block);
        
        //self.chain.push(block);
        true
    }

    // NOT public
    /*
    fn get_merkle(curr_trans: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for t in &curr_trans {
            let hash = Chain::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }
        merkle.pop().unwrap()
    }
    */
    
}




// Rollin, rollin, rollin...
fn main() {


    // Init Blockchain State
    let mut state = State::new_blockchain();
    
    
    // Init "RSA" Params and Create Account Keys
    // with fixed p and q to generate deterministic accounts
    let p = 61; // State::prime_gen(5, 100);
    let q = 53; // State::prime_gen(5, 100);
    assert!(p > 0);
    assert!(q > 0);
    // m (3233) is now a constant we can use for all keys that share the same p and q setup
    // Could we also use 65537 ?
    let m = p * q;
    state.modulo = m;
    let ctf_pq = State::ctf(p, q);
    // manually create testing account from previous keys
    let acc_0_pub_key = 773;
    let acc_0_priv_key = 557;
    let acc_0 = Account {
        balance: 10000.0,
        nonce: 0,
    };
    state.accounts.insert(acc_0_pub_key.clone(), acc_0);
    // Manually create testing account from previous keys
    let acc_1_pub_key = 179;
    let acc_1_priv_key = 719;
    let acc_1 = Account {
        balance: 10000.0,
        nonce: 0,        
    };
    state.accounts.insert(acc_1_pub_key.clone(), acc_1);
    // Uncomment if you want to generate more keys
    // and see their params
    /*
    let pub_key = State::pub_key_gen(1, ctf_pq);
    let priv_key = State::priv_key_gen(ctf_pq, pub_key);
    println!("p: {}", &p);
    println!("q: {}", &q);
    println!("m: {}", &m);
    println!("ctf_pq: {}", &ctf_pq);
    println!("pub_key: {}", &pub_key);
    println!("priv_key: {}", &priv_key);
    */
    // Create 3 random accounts
    for i in 0..3 {
        state.new_account(ctf_pq)
    }
    // check results
    println!("\nInitial {:#?}", state);

    
    // Create TX
    state.new_signed_tx(acc_0_pub_key,
                        acc_0_priv_key,
                        acc_1_pub_key,
                        50.0,
                        m);
    println!("\n{:#?}", state);
    
    // Verify TX
    state.verify_tx();
    println!("\n{:#?}", state);
    
    // Confirm Verified TX
    state.confirm_tx();
    println!("\n{:#?}", state);
}
