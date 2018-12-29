```rust
#![allow(warnings)]

extern crate rand;
extern crate time;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use rand::prelude::*;
use std::fmt::Write;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;



// TODOS
//
// DONE: Simple Hash Function 
// - Problem: SHA2 is not part of the standard library and does not work in the Rust Playground
// - Solution: Swapped it out for the default hasher
// 
// ~DONE BY DEFAULT: Centralized Operator = Genesis Address
// - only account that can "mine" new blocks (aka approve tx)
// - only account that can change params (block rewards, difficulty, etc)
//
// CURRENT
// 
// TX Problems!
// - does not update receiver account balance 
// - does not check sender signature 
// TX Solutions?
// - done: create a function that verifies tx before added to a new block
// - todo: better Accounts data structure
//   - HashMap?
//      - NO: HashMap only works for types that implement the Eq and Hash traits. See link below for details.
//      - https://doc.rust-lang.org/rust-by-example/std/hash/alt_key_types.html
//   - Vec of Vecs?
//      - MAYBE: where each Vec holds a pub_key and a struct for that account? 
// - todo: function that updates account balances based on verified tx
// 
// Addresses
// - DONE: add struct for addresses (public_key, sign_key, balance)
// - DONE: add function to create new addresses
// - ONGOING: add vector to store all addresses
// 
// TX Updates
// - prevent over spending by checking if balance is > tx
// - check if tx is signed by sender
//
// Stretch Goals
// - parameters to change central operator's fees / rewards
// - parameters to simulate tx to see how profitable central operators really are
// - parameters to make the system p2p and see what happens


#[derive(Debug)]
pub struct Account {
    name: String,
    priv_key: String,
    pub_key: String,
    balance: f32,
}

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,  
    amount: f32,
}

#[derive(Serialize, Debug)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32, 
    pre_hash: String,  
    merkle: String,  
    difficulty: u32,
}

#[derive(Serialize, Debug)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>
}

#[derive(Debug)]
pub struct Chain {
    chain: Vec<Block>,
    accounts: Vec<Account>,
    pending_tx: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String, 
    reward: f32,
}



impl Chain {

    pub fn new_blockchain(miner_addr: String, 
                          difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            pending_tx: Vec::new(),
            accounts: Vec::new(),
            difficulty,
            miner_addr,
            reward: 0.0,
        };

        chain.new_block();
        chain

    }
    
    
    pub fn key_gen() -> String {
        let rn: i32 = thread_rng().gen_range(100000, 1000000);
        rn.to_string()
    }
    
    pub fn new_account(&mut self,
                       name: String) {
        
        let pk = Chain::key_gen();
        let account = Account {
            name: name,
            priv_key: pk.clone(),
            pub_key: Chain::hash(&pk),
            balance: 100.0,
        };
        
        self.accounts.push(account);
    }
    
    pub fn new_tx(&mut self,
                  sender: String,
                  receiver: String,
                  amount: f32) -> bool {
    
        self.pending_tx
            .push(Transaction{sender,
                              receiver,
                              amount,});

        true
    }
    
    // UNDER CONSTRUCTION
    // sender.balance isn't a thing yet
    // need a better Accounts data structure
    /*
    pub fn verify_tx(&mut self,
                     pending_tx: Vec<Transaction>) -> Vec<Transaction> {
        
        let verified_tx = Vec::new();
        let failed_tx = Vec::new();
        
        for i in pending_tx {
            match {
                sender.balance >= amount => verified_tx.push(i),
                _ => failed_tx.push(i);
            }
        }
        
        println!("These tx failed: {:#?}", failed_tx);
        verified_tx
    }
    
    pub fn apply_tx(&mut self,
                    verified_tx: Vec<Transaction>) {
        
        for i in verified_tx {
            sender.balance -= i.amount;
            receiver.balance += i.amount;
        }
    }
    */

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap()
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    pub fn new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty
        };

        let block_reward_tx = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_addr.clone(),
            amount: self.reward
        };

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![]
        };
        
        // Verify Pending TX Here
        let mut verified_tx = Vec::new(); // swap for line below asap
        // let mut verified_tx = verify_tx(pending_tx);
        verified_tx.push(block_reward_tx);
        
        // Apply Verified TX To Update Account Balances Here
        block.transactions.append(&mut self.pending_tx); // swap for line below asap
        // apply_tx(verified_tx);
        
        // Add Stuff To The New Block
        block.transactions.append(&mut verified_tx);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);

        // Add New Block To The Chain
        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

    fn get_merkle(pending_tx: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for t in &pending_tx {
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

    pub fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                },
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            };
        }
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let input_bytes = input.as_bytes();
        
        let mut hasher = DefaultHasher::new();
        hasher.write(input_bytes);
        let digest = hasher.finish();
        let hex_digest = format!("{:#X}", digest);
        
        hex_digest
    }
}

fn main() {


    // Create The "Blockchain" 
    // (really PP's private data system)
    let pp_pk = Chain::key_gen();
    let mut PP = Account {
            name: String::from("PP"),
            priv_key: pp_pk.clone(),
            pub_key: Chain::hash(&pp_pk),
            balance: 1000000.0,
    };
    let mut chain = Chain::new_blockchain(PP.pub_key.clone(), 1);
    chain.accounts.push(PP);


    // Test Account Creation
    for i in 0..5 {
        chain.new_account(String::from("unique_person"));
    }
    chain.new_account(String::from("Jim"));
    
    println!("\nAccounts:\n{:#?}\n", chain.accounts);
    
    
    // Test Updating Block Reward
    chain.update_reward(10000.0);
    // PROBLEM!
    // - the chain creates a "new block", but the reward
    //   does not get added to PP's balance!
    
    // Test New Block Creation
    for i in 0..3 {
        chain.new_block();
    }
    
    
    // Test Hashing
    let hw = String::from("Hello World");
    let hash_test = Chain::hash(&hw);
    println!("hash test: {}", hash_test);
    
    
    // Print Result of Chain
    println!("\nCHAIN:\n{:#?}\n", chain);
    
}
```
