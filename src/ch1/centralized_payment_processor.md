## Bringing it all together

## Words

Structs
- State {accounts, frozen_accounts, pending_tx, history}
- Account {balance, password, nonce}
- TX {sender, password, nonce, receiver}

HashMaps
- accounts: (key: id, value: struct Account)
- frozen_accounts: (key: id, value: struct Account )
- pending_tx: (key: tx#, value: struct TX)
- history: (key: tx#, value: struct TX)

Functions
- hash
- create password
- create new account
- freeze account (move from reg accounts HashMap to frozen HashMap)
- add moneys to account (like if the user paid the central operator from another bank account)
- check account exists
- check account password = tx_password
- check account nonce = tx_nonce
- check account balance > tx amount
- check history of account

MAIN
- create new state
- create new accounts
- simulate TX
- show the bank's view vs the user's view as well as the functions the bank can perform vs what the user can perform


## Code

```rust, ignore
extern crate rand;
use rand::prelude::*;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;


#[derive(Debug)]
struct State {
    accounts: HashMap<String, Account>,
    frozen_accounts: HashMap<String, Account>,
    pending_tx: Vec<TX>,
    history: Vec<TX>,
}

#[derive(Debug, Clone)]
struct Account {
    password: i32,
    nonce: i32,
    balance: i32,
}

#[derive(Debug, Clone)]
struct TX {
    sender: String,
    sender_password: i32,
    sender_nonce: i32,
    receiver: String,
    amount: i32,
}


// Central Payment Processor
impl State {
    
    // Create a new state
    pub fn new_state() -> State {
    
        let mut new = State {
            accounts: HashMap::new(),
            frozen_accounts: HashMap::new(),
            pending_tx: Vec::new(),
            history: Vec::new(),
        };
        
        new
    }
    
    // Turn stuff into &[u8] slice
    pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
        ::std::slice::from_raw_parts(
            (p as *const T) as *const u8,
            ::std::mem::size_of::<T>(),
        )
    }

    // Hash &[u8] slice into a hex String
    pub fn hash_u8(stuff: &[u8]) -> String {
        
        let mut hasher = DefaultHasher::new();
        hasher.write(stuff);
        let digest = hasher.finish();
        let hex_digest = format!("{:#X}", digest);
            
        hex_digest
    }    
    
    // Hash stuff into a hex string
    pub fn hash<T>(stuff: &T) -> String {
        
        let u8_stuff = unsafe {
            State::any_as_u8_slice(stuff)
        };
        let hash_of_stuff = State::hash_u8(u8_stuff);
        
        hash_of_stuff
    }
    
    // Create a new account
    pub fn new_account(&mut self) {
        
        let account_id = State::hash(&thread_rng().gen_range(0, 1000000));
        let account_data = Account {
            password: thread_rng().gen(),
            nonce: 0,
            balance: 0,
        };
        
        self.accounts.insert(account_id, account_data);
    }
    
    // Create multiple new accounts
    pub fn new_accounts(&mut self,
                        num_accounts: i32) {
        
        for i in 0..num_accounts {
            self.new_account()
        }
    }
    
    // "Freeze" an account
    pub fn freeze_account(&mut self,
                          account_id: String) {
        
        let account = self.accounts.remove_entry(&account_id).unwrap();
    
        self.frozen_accounts.insert(account.0, account.1);
    }
    
    // Add funds to an account
    pub fn add_funds(&mut self,
                     account_id: String,
                     amount: i32) {
        
        if let Some(x) = self.accounts.get_mut(&account_id) {
            x.balance += amount;
        }
    }
    
    // Create a new TX
    pub fn new_tx(&mut self,
                  sender: String,
                  sender_password: i32,
                  sender_nonce: i32,
                  receiver: String,
                  amount: i32) {
        
        let tx = TX {
            sender: sender,
            sender_password: sender_password,
            sender_nonce: sender_nonce,
            receiver: receiver,
            amount: amount,
        };
        
        self.pending_tx.push(tx);
    }

    // Verify pending TX
    pub fn process_pending_tx(&mut self) {
        
        for i in & self.pending_tx {

            // check that sender is legit
            if !(self.accounts.contains_key(&i.sender)) {
                break
            }
 
            // check that receiver is legit
            if !(self.accounts.contains_key(&i.receiver)) {
                break
            }           
            
            // check that tx is signed by sender password
            if !(i.sender_password == self.accounts
                                     .get(&i.sender)
                                     .unwrap()
                                     .password) {
                break
            }
            
            // check that the TX nonce matches the sender nonce
            if !(i.sender_nonce == self.accounts
                                  .get(&i.sender)
                                  .unwrap()
                                  .nonce) {
                break
            }

            // check that the TX amount is >= the sender's balance 
            if !(i.amount >= self.accounts
                                    .get(&i.sender)
                                    .unwrap()
                                    .balance) {
                    break
                } 

            // decrease the balance from sender's account
            self.accounts
                .get_mut(&i.sender)
                .unwrap()
                .balance -= i.amount;
            // increase sender's nonce to prevent replay glitches
            self.accounts
                .get_mut(&i.sender)
                .unwrap()
                .nonce += 1;
            // increase the balance of the reciever's account
            self.accounts
                .get_mut(&i.receiver)
                .unwrap()
                .balance += i.amount;
        }
    }

    // Find the history for an account
    pub fn get_account_history(&mut self,
                               account_id: String,) -> Vec<TX> {
        
        let mut account_history = Vec::new();
        let list = self.history.clone();
        for i in list {
            if i.sender == account_id {
                account_history.push(i.clone());
            }
        }
        
        account_history
    }
}


/*
MAIN: todo
- simulate TX
- show the bank's view vs the user's view
*/

fn main() {
    
    // Roll your own bank!
    let mut bank = State::new_state();
    println!("bank: {:#?}", &bank);
    
    // Create some new accounts
    bank.new_accounts(10);
    println!("bank: {:#?}", bank);
    
    /*
    // Add some funds to those accounts
    for mut i in bank.accounts {
        i.1.balance += 5;
        //i.1.balance += thread_rng().gen_range(0, 1000);
    }
    println!("bank: {:#?}", &bank);
    */
}
```
