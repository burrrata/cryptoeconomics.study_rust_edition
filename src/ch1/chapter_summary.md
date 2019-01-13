<br>

<div align="center">
    <p align="center">
        <img src="TBD" alt="TBD">
    </p>
    <h1 align="center">
        Centralized Payment Processor
    </h1>
    <p align="center">
        Feel the power.
    </p>
</div>

<br><br><br>

So to recap...

The centralized database manager:
- gets to create new data and accounts out of thin air (or money and debt)
- controls read/write access to that data (ability to move/create money)
- has minimal downside when things go wrong due to bailouts and insurance (what is regulatory capture?)

The users:
- trust the central operator is showing them the correct state for their data or money
- are only allowed as much money or control as they can figure out how to convince the bank or anyone else to give them
- get their lives turned upside down if things go wrong

Seems a little odd that such an essential piece of modern living is so opaque and fragile... 🤔

Can we do better? Maybe! Some people have some ideas on how to at least make this process a little more secure. In the next chapter we'll explore how we can change the architecture of the system to make it better for users. This includes:
- giving everyone on the network the option to verify TX to make sure no one is cheating
- creating account IDs and passwords via cryptography so that they are not all located in a centralized database
- creating costs and rewards for managing state transitions to create reliability and security

Let's go to chapter 2 and see how that works!

<br><br><br>

[![Cryptoeconomics - 1.5 - Properties of Centralized Systems](https://img.youtube.com/vi/ckzi8iqGilE/0.jpg)](https://www.youtube.com/watch?v=ckzi8iqGilE)

<p>
    <a href="https://cryptoeconomics.study/lectures/chapter-01-5.html">Cryptoeconomics - 1.5 - Properties of Centralized Systems</a>.
</p>

<br><br><br>

Here is the code without extra commentary so you can explore and see how all the parts interact. Go ahead and try to modify or play with it :)

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
    account_ids: Vec<String>,
    pending_tx: Vec<TX>,
    history: Vec<TX>,
    debt_history: Vec<TX>,
    debt_pool: i32,
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
    
    
    /// GENERALLY USEFUL FUNCTIONS ///
    
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
    
    
    /// FUNCTION TO INIT THE STATE ///
    
    // Create a new state
    pub fn new_state() -> State {
    
        let mut new = State {
            accounts: HashMap::new(),
            frozen_accounts: HashMap::new(),
            account_ids: Vec::new(),
            pending_tx: Vec::new(),
            history: Vec::new(),
            debt_history: Vec::new(),
            debt_pool: 0,
        };
        
        new
    }
    
    
    /// ACCOUNT FUNCTIONS ///
    
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
    
    // Print account info
    pub fn print_account_info(&mut self,
                         account_id: String) {
        
        if let Some(x) = self.accounts.get(&account_id) {
            println!("Your Account:\n{:#?}", self.accounts.get(&account_id).unwrap());
        }
        println!("Account not found");
    }
    
    // Print account history
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
        println!("\n/// Getting Account History ///");
        println!("Account {} ", account_id);
        println!("{:#?}", self.accounts.get(&account_id).unwrap());
        println!("History:\n{:#?}", account_history);
    }
    
    // "Freeze" an account
    pub fn freeze_account(&mut self,
                          account_id: String) {
        
        let account = self.accounts.remove_entry(&account_id).unwrap();
    
        self.frozen_accounts.insert(account.0, account.1);
    }
    
    
    /// TX FUNCTIONS ///
        
    // Create a new bank TX
    pub fn new_bank_tx(&mut self,
                       receiver: String,
                       amount: i32) {

        // Tx is legit by default because it's from the bank so let's just process it.
        let tx = TX {
            sender: "bank".to_string(),
            sender_password: 0,
            sender_nonce: self.accounts.get("bank").unwrap().nonce,
            receiver: receiver,
            amount: amount, 
        };
        // decrease the balance in the bank's debt account
        self.debt_pool -= tx.amount;
        // increase the balance of the reciever's account
        self.accounts
            .get_mut(&tx.receiver)
            .unwrap()
            .balance += tx.amount;
            
        // add processed TX to history
        self.history.push(tx.clone());        
    }
    
    // Create a new TX
    pub fn new_user_tx(&mut self,
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

    
    /// STATE TRANSITION FUNCTIONS ///

    // Verify pending user TX
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
    
    // Add funds to an account
    pub fn add_funds(&mut self,
                     account_id: String,
                     amount: i32) {
        
        if let Some(x) = self.accounts.get_mut(&account_id) {
            x.balance += amount;
        }
    }
}


fn main() {
    
    // Init bank state
    let mut bank = State::new_state();
    println!("\n/// Initialized Bank State ///");
    println!("{:#?}", &bank);
    
    // Create some new accounts
    bank.new_accounts(10);
    println!("\n/// Created Some Accounts ///");
    println!("{:#?}", bank);

    // Init some variables for testing accounts
    let test_account0 = bank.account_ids[0].clone();
    let test_account1 = bank.account_ids[1].clone();
    let test_account2 = bank.account_ids[2].clone();

    // Add some funds to those accounts
    for i in bank.accounts.values_mut() {
        i.balance += 10000;
    }
    println!("\n/// Added Funds To Accounts ///");
    println!("{:#?}", bank);

    // Let's make some TX requests
    for i in 0..10 {
        
        let sender = &bank.account_ids[thread_rng().gen_range(0, bank.account_ids.len())];
        let receiver = &bank.account_ids[thread_rng().gen_range(0, bank.account_ids.len())];
        
        if sender != receiver {
            bank.new_user_tx(sender.to_string(),
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
    
    // Get the history for an account
    bank.print_account_history(test_account0.clone());
    
    // Freeze an account
    bank.freeze_account(test_account0.clone());
    println!("\n/// Froze Account {} ///", &test_account0);
    println!("{:#?}", bank);
    
    // Try checking the balance of a frozen account
    println!("\n/// Checking Frozen Account ///");
    bank.print_account_info(test_account0.clone().to_string());
}
```

<br><br><br>

## Resources To Learn More
- TBD

<br><br><br>
