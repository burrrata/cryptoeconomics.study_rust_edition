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

// TODO
//
// Addresses
// - add struct for addresses (public_key, sign_key, balance)
// - add function to create new addresses
// - add vector to store all addresses
// 
// TX Updates
// - check if tx is signed by sender
// - check if balance is > tx
// 
// Centralized Operator = Genesis Address
// - must approve all tx
// - only account that can change params (block rewards, difficulty, etc)
//
// Stretch Goals
// - parameters to change central operator's fees / rewards
// - parameters to simulate tx to see how profitable central operators really are
// - parameters to make the system p2p and see what happens


#[derive(Debug)]
pub struct Account {
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

pub struct Chain {
    chain: Vec<Block>,
    accounts: Vec<Account>,
    curr_trans: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String, 
    reward: f32,
}



impl Chain {

    pub fn new_blockchain(miner_addr: String, 
                          difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            accounts: Vec::new(),
            difficulty,
            miner_addr,
            reward: 0.0,
        };

        chain.generate_new_block();
        chain

    }
    
    
    pub fn key_gen() -> String {
        let rn: i32 = thread_rng().gen_range(100000, 1000000);
        rn.to_string()
    }
    
    pub fn new_account(&mut self) {
        
        let pk = Chain::key_gen();
        let account = Account {
            priv_key: pk.clone(),
            pub_key: Chain::hash(&pk),
            balance: 100.0,
        };
        
        self.accounts.push(account);
    }
    

    pub fn new_transaction(&mut self,
                           sender: String,
                           receiver: String,
                           amount: f32) -> bool {
    
        self.curr_trans
            .push(Transaction{sender,
                              receiver,
                              amount,});

        true
    }

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

    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty
        };

        let reward_trans = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_addr.clone(),
            amount: self.reward
        };

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![]
        };

        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.curr_trans);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);

        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

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

    // Blockchain Params
    let mut miner_addr = String::from("123");
    let mut dif = 1; // 0..3 are recommend for CLI, but only 1 works on Rust Playground
    
    // Create a "blockchain"
    let mut chain = Chain::new_blockchain(miner_addr, dif);
    
    // Test Account Creation
    for i in 0..5 {
        chain.new_account();
    }
    println!("\nAccounts:\n{:#?}\n", chain.accounts);
    
    // Test Hashing
    let hw = String::from("Hello World");
    let hash_test = Chain::hash(&hw);
    println!("hash test: {}", hash_test);
    
}
```
