/*
 TODO ASAP
 - Funciton to check block PoW and TX before updating state
 - Make check_signed_tx_signature() NOT crash the entire
   program if the tx signature does not match the sender.
 
 Nice To Have
 - Maybe use 65537 as the "RSA" modulo rather than
   the toy setup in the wikipedia article? 
 - RLP: 
    - How much faster would the program be if values were
      converted to a standard data format? 
    - Or is the ux better with i32 because the user just types
      a number without thinking about type like in Javascript?   
*/


extern crate rand;
use rand::prelude::*;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;


#[derive(Debug)]
struct State {
    modulo: i32,
    pow_difficulty: i32, // 0 is instantanious, but 1, 2, and 3 take a while
    accounts: HashMap<i32, Account>,
    pending_tx: Vec<SignedTX>,
    chain: Vec<Block>,
    block_height: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Account {
    balance: f32,
    nonce: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct TX {
    sender: i32,
    receiver: i32,
    amount: f32,
    nonce: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct SignedTX {
    tx: TX,
    signature: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Blockheader {
    timestamp: i64,
    block_number: i32,
    nonce: i32, // PoW difficulty
    previous_block_hash: String,  
    merkle: String,  
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    header: Blockheader,
    transactions: Vec<SignedTX>,
    PoW: String, // literally the proof of doing the work
}


impl State {

    /// "RSA" KEY GENERATION STUFF ///

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
        
        let lcm = (a * b) / State::gcd(a, b);
        
        lcm
    }
    
    // Carmichael's totient function
    pub fn ctf(a: i32,
           b: i32) -> i32 {
        
        State::lcm(a - 1, b - 1)
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
        
        for _i in 0..1000000 {
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
    pub fn exp_mod(input: i32,
               power: i32,
               modulo: i32) -> i32 {
        
        let mut out = (input * input) % modulo;
        // because the first iter of out took 2 off the base
        for _i in 0..power-2 {
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
            pow_difficulty: 0,
            accounts: HashMap::new(),
            pending_tx: Vec::new(),
            chain: Vec::new(),
            block_height: 0,
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
        
        //println!("\nThis is your public key (address): {:#?}", &pub_key);
        //println!("This is your private key (signing key): {:#?}", &priv_key);
        //println!("This is your account: {:#?}", self.accounts.get(&pub_key).unwrap());
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
            State::any_as_u8_slice(stuff)
        };
        let hash_of_stuff = State::hash_u8(u8_stuff);
        
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
    pub fn check_signed_tx_signature(signed_tx: SignedTX,
                           modulo: i32) -> bool {
    
        let tx_as_bytes = unsafe {
            State::any_as_u8_slice(&signed_tx.tx)
        };
        let tx_hash = State::hash_u8(tx_as_bytes);
        //println!("tx hash: {}", tx_hash);
        
        let decrypted_tx_hash_sig = State::toy_rsa(signed_tx.signature,
                                            signed_tx.tx.sender,
                                            modulo);
        let decrypted_tx_hash = State::v2s(decrypted_tx_hash_sig);
        //println!("decrypted tx hash: {}", decrypted_tx_hash);
        
        match tx_hash == decrypted_tx_hash {
            true => true,
            false => {
                println!("not valid tx");
                return false
            },
        }
    }
    
    // Verify a Vec of TX
    pub fn verify_tx(&mut self,
                     tx: Vec<SignedTX>) -> Vec<SignedTX> {
        
        //println!("\nVerifying TX:");
        
        let mut verified_tx = Vec::new();
        
        for i in tx {
        
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
            
            if !(State::check_signed_tx_signature(i.clone(), self.modulo)) {
                println!("TX No Good!");
                break
            }
            
            //println!("Valid TX.");
            verified_tx.push(i.clone());
        }
        
        verified_tx
    }
    
    // Create A Merkle Tree Of All TX In A Vec
    pub fn merklize_block(transactions: Vec<SignedTX>) -> String {
        
        let mut merkle = Vec::new();

        for i in &transactions {
            let hashed_tx = State::hash_any(&i);
            merkle.push(hashed_tx);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let nh = State::hash_any(&h1);
            merkle.push(nh);
        }
        
        merkle.pop().unwrap()
    }

    // Create a proof of work computed to earn the right
    // to submit a valid block
    pub fn proof_of_work(mut block_header: Blockheader) -> String {
    
        println!("\n/// PoW ///\n");
    
        let mut header = block_header.clone();
        header.nonce = 1; // length of slice hash must match
        println!("header: {:#?}\n", header);
    
        loop {
            println!("");
            
            let hash = State::hash_any(&header);
            println!("hash: {:#?}", hash);
            
            println!("header.nonce: {:#?}", header.nonce);
            let slice = &hash[2..2+header.nonce as usize];
            println!("slice: {:#?}", slice);
            
            println!("slice.parse::<u32>(): {:#?}", slice.parse::<u32>());
            match slice.parse::<u32>() {
            
                Ok(val) => {
                    if val != 0 {
                        continue
                        
                    } else {
                        println!("final PoW hash: {:#?}", &hash);
                        return hash;
                    }
                },
                
                Err(_) => {
                    println!("\n!!! PoW LOOP ERROR !!!\n{:#?}\n", header.nonce);
                    continue;
                }
            };
        }
        return String::from("!!! PoW ERROR !!!") 
    }

    // Create A New Block With Valid Transactions
    pub fn new_block(&mut self) -> Block {
    
        println!("\n/// Creating New Block ///\n");
    
        let pending_tx = self.pending_tx.clone();
        
        let transactions = State::verify_tx(self, pending_tx);
        let header = Blockheader {
            timestamp: time::now().to_timespec().sec,
            block_number: self.block_height + 1,
            nonce: self.pow_difficulty,
            previous_block_hash: State::hash_any(& self.chain.last()),
            merkle: State::merklize_block(transactions.clone()),
        };
        let pow= State::proof_of_work(header.clone());

        let block = Block {
            header: header,
            transactions: transactions,
            PoW: pow,
        };
        
        println!("\n/// Block Successfully Created! ///\n");
        block
    }
    
    // Confirm TX in valid_tx Pool And Add Them To The History
    pub fn push_block(&mut self,
                      block: Block) {
        
        // Check block tx
        let checked_tx = State::verify_tx(self, block.transactions.clone());
        for i in 0..checked_tx.len() {
            for j in 0..block.transactions.len() {
                if checked_tx[i] != block.transactions[j] {
                    println!("TX Error: {:#?} does not match {:#?}.", checked_tx[i], block.transactions[i]);
                    break
                }
            }
        }
        
        // Check block PoW
        //let checked_pow = State::
        
        // If PoW and TX are valid,
        // - process tx and change state
        // - push block to state history
        //println!("\nPushing Block To Blockchain:\n{:#?}", &block);
        for i in & block.transactions {
            self.accounts.get_mut(&i.tx.sender).unwrap().balance -= i.tx.amount;
            self.accounts.get_mut(&i.tx.receiver).unwrap().balance += i.tx.amount;
            self.accounts.get_mut(&i.tx.sender).unwrap().nonce += 1;
            //println!("{} sent {} to {}", &i.tx.sender, &i.tx.amount, &i.tx.receiver);
        }
        self.chain.push(block);
        self.block_height += 1;
        //println!("Block pushed to Chain");
    }
}




// Rollin, rollin, rollin...
fn main() {


    // Init Blockchain State
    let mut state = State::new_blockchain();
    
    
    // Init "RSA" Params and Create Account Keys
    
    // Randomized initialization
    // State::prime_gen(5, 100);
    // State::prime_gen(5, 100);
    
    // Fixed p and q initialization to generate 
    // deterministic accounts for testing
    let p = 61; 
    let q = 53; 
    assert!(p > 0);
    assert!(q > 0);
    // m (3233) is now a constant we can use for all keys
    // that share the same fixed p and q setup
    let m = p * q;
    state.modulo = m;
    // Manually create testing account 0
    let acc_0_pub_key = 773;
    let acc_0_priv_key = 557;
    let acc_0 = Account {
        balance: 10000.0,
        nonce: 0,
    };
    state.accounts.insert(acc_0_pub_key.clone(), acc_0);
    // Manually create testing account 1
    let acc_1_pub_key = 179;
    let acc_1_priv_key = 719;
    let acc_1 = Account {
        balance: 10000.0,
        nonce: 0,        
    };
    state.accounts.insert(acc_1_pub_key.clone(), acc_1);
    // Carmichael's totient function of p and q
    let ctf_pq = State::ctf(p, q);
   
    // rand!
    // Create 3 random accounts
    for _i in 0..3 {
        state.new_account(ctf_pq)
    }
    // Uncomment if you want to generate more keys
    // and see their params
    let pub_key = State::pub_key_gen(1, ctf_pq);
    let priv_key = State::priv_key_gen(ctf_pq, pub_key);
    /*
    println!("p: {}", &p);
    println!("q: {}", &q);
    println!("m: {}", &m);
    println!("ctf_pq: {}", &ctf_pq);
    println!("pub_key: {}", &pub_key);
    println!("priv_key: {}", &priv_key);
    // check results
    println!("\nInitial {:#?}", state);
    */
    
    // Create Test TX
    state.new_signed_tx(acc_0_pub_key,
                        acc_0_priv_key,
                        acc_1_pub_key,
                        50.0,
                        m);
    //println!("\nAdded Pending TX\n{:#?}", state);
    
    // Create New Block
    let pending_block = state.new_block();
    
    // Push New Block To The "Blockchain"
    state.push_block(pending_block);
    //println!("\nState With New Block\n{:#?}", state);
}
