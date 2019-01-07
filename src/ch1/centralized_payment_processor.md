## Bringing it all together

## Words

When you use a centralized operator like a bank:
- trust bank has correct state
- bank controls accounts
- bank controls tx
- bank controls history
- bank creates new money

## Code

```rust, ignore
extern crate rand;
use rand::prelude::*;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;


#[derive(Debug)]
struct State {
    account_ids: Vec<String>,
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
            account_ids: Vec::new(),
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
            password: thread_rng().gen_range(0, 1000000),
            nonce: 0,
            balance: 0,
        };
        
        self.account_ids.push(account_id.clone());
        self.accounts.insert(account_id, account_data);
    }
    
    // Create multiple new accounts
    pub fn new_accounts(&mut self,
                        num_accounts: i32) {
        
        for i in 0..num_accounts {
            self.new_account()
        }
    }
    
    // Print Account Stats
    pub fn print_account(&mut self,
                         account_id: String) {
        
        if let Some(x) = self.accounts.get(&account_id) {
            println!("Your Account:\n{:#?}", self.accounts.get(&account_id));
        }
        println!("Account not found");
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
        
        // check pending tx
        for i in & self.pending_tx {
            
            // check that sender is legit
            if !(self.accounts.contains_key(&i.sender)) {
                println!("TX ERROR: sender not found.");
                continue;
            }
 
            // check that receiver is legit
            if !(self.accounts.contains_key(&i.receiver)) {
                println!("TX ERROR: receiver not found.");
                continue;
            }           
            
            // check that tx is signed by sender password
            if !(i.sender_password == self.accounts
                                     .get(&i.sender)
                                     .unwrap()
                                     .password) {
                println!("TX ERROR: tx and sender passwords do not match.");
                continue;
            }
            
            // check that the TX nonce matches the sender nonce
            if !(i.sender_nonce == self.accounts
                                  .get(&i.sender)
                                  .unwrap()
                                  .nonce) {
                println!("TX ERROR: tx and sender nonces do not match.");
                continue;
            }

            // check that the TX amount is >= the sender's balance 
            if !(i.amount <= self.accounts
                                    .get(&i.sender)
                                    .unwrap()
                                    .balance) {
                println!("TX ERROR: sender has insufficient balance");
                continue;
            } 
            
            // Tx is legit so let's process it
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
                
            // add processed TX to history
            self.history.push(i.clone());
        }
        
        // clear pending tx
        self.pending_tx = Vec::new();
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
            if i.receiver == account_id {
                account_history.push(i.clone());
            }
        }
        
        account_history
    }
    
    // Print the history for an account
    pub fn print_account_history(&mut self,
                                 account_id: String,) {
        
        let mut account_history = Vec::new();
        let list = self.history.clone();
        for i in list {
            if i.sender == account_id {
                account_history.push(i.clone());
            }
            if i.receiver == account_id {
                account_history.push(i.clone());
            }
        }
        
        println!("\nAccount {} ", account_id);
        println!("{:#?}", self.accounts.get(&account_id));
        println!("History:\n{:#?}", account_history);
    }
}


fn main() {
    
    // Roll your own bank!
    let mut bank = State::new_state();
    println!("\n/// Initialized Bank State ///");
    println!("{:#?}", &bank);
    
    // Create some new accounts
    bank.new_accounts(10);
    println!("\n/// Created Some Accounts ///");
    println!("{:#?}", bank);
    
    // Add some funds to those accounts
    for i in bank.accounts.values_mut() {
        i.balance += 10000;
    }
    println!("\n/// Added Funds To Accounts ///");
    println!("{:#?}", bank);

    // Simulate some TX
    for i in 0..10 {
        
        let sender = &bank.account_ids[thread_rng().gen_range(0, bank.account_ids.len())];
        let receiver = &bank.account_ids[thread_rng().gen_range(0, bank.account_ids.len())];
        
        if sender != receiver {
        
            bank.new_tx(sender.to_string(),
                        bank.accounts.get(sender).unwrap().password,
                        bank.accounts.get(sender).unwrap().nonce,
                        receiver.to_string(),
                        thread_rng().gen_range(100, 1000))
        }
    }
    println!("\n/// Simulated Some TX ///");
    println!("{:#?}", bank);
    
    // Process pending TX
    bank.process_pending_tx();
    println!("\n/// Processed Pending TX ///");
    println!("{:#?}", bank);
    
    // Init some variables for testing
    let test_account0 = bank.account_ids[0].clone();
    let test_account1 = bank.account_ids[1].clone();
    let test_account2 = bank.account_ids[2].clone();
    
    // Get the history for an account
    bank.print_account_history(test_account0.clone());
    
    // Freeze an account
    bank.freeze_account(test_account0.clone().to_string());
    println!("\n/// Froze Account {} ///", &test_account0);
    println!("{:#?}", bank);
    
    // Try checking the balance of a frozen account
    println!("\n/// Checking Frozen Account ///");
    bank.print_account(test_account0.clone().to_string());
    
    // Try sending from a frozen account to a regular account
    /*
    // CURRENT WIP
    // These fail, and rightfully so, but they need to
    // fail gracefully and return helpful errors rather 
    // than just halting the program.
    println!("/// Frozen Account TX Test ///");
    bank.new_tx(test_account0.to_string(),
                bank.accounts.get(&test_account0).unwrap().password,
                bank.accounts.get(&test_account0).unwrap().nonce,
                test_account1,
                100);
    bank.new_tx(test_account1.to_string(),
                bank.accounts.get(&test_account1).unwrap().password,
                bank.accounts.get(&test_account1).unwrap().nonce,
                test_account0.clone(),
                100);
    // Try sending to a frozen account from a regular account
    bank.new_tx(test_account2.to_string(),
                bank.accounts.get(&test_account2).unwrap().password,
                bank.accounts.get(&test_account2).unwrap().nonce,
                test_account0,
                100);
    */
    println!("//////////////////////////////");
}

/*
MAIN
- show the bank's view vs the user's view
*/
```
