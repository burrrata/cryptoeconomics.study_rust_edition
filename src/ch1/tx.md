<div align="center">
    <p align="center">
        <img src="TBD" alt="TBD">
    </p>
    <h1 align="center">
        TX
    </h1>
    <p align="center">
        Making stuff happen.
    </p>
</div>

<br>

## Words

Core Concept: tx is a way to request a state change, but...
- the state transition function is how we make that state change
- the central operator controls the state and thus all state changes including freezing accounts or printing money

<br>

## Videos

[![Cryptoeconomics - 1.5 - Properties of Centralized Systems](https://img.youtube.com/vi/XIsn8-5Xekc/0.jpg)](https://www.youtube.com/watch?v=XIsn8-5Xekc)

<p>
    <a href="https://cryptoeconomics.study/lectures/chapter-01-2.html">Cryptoeconomics - 1.2 - State Transitions & Payment Processor Implementation</a>.
</p>

<br>

[![Cryptoeconomics - 1.3 - Replay Protection](https://img.youtube.com/vi/j7Mbx8laZwY/0.jpg)](https://www.youtube.com/watch?v=j7Mbx8laZwY)

<p>
    <a href="https://cryptoeconomics.study/lectures/chapter-01-3.html">Cryptoeconomics - 1.3 - Replay Protection</a>.
</p>

<br>

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
        
        new.accounts.insert(String::from("bank"), Account { password: 0, nonce: 0, balance: 1000000 });
        
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
        
        // The end of your life savings are just a click away...
        
        let account = self.accounts.remove_entry(&account_id).unwrap();
    
        self.frozen_accounts.insert(account.0, account.1);
    }


    /// TX FUNCTIONS ///
    
    // Create a new TX for the bank
    pub fn new_bank_tx(&mut self,
                       receiver: String,
                       amount: i32) {
        
        // When banks give people loans or credit it's actually processed
        // as debt which banks can then trade amongst each other at a market
        // rate based on how likely the debtor is likely to pay back in full
        // Yes you heard this right, they print money and profit from doing so.
        // Carpenters make cabinets, comedians make jokes, banks make money,
        // literaly...
        // Fun Fact: debt on a banks balance sheet is an ASSET to the bank and
        // not a liability. It's a liability to users, but banks can buy, sell, 
        // and trade this debt as a financial product. One of a banks primary 
        // products is loans, but as a user of a bank you're actually the product 
        // they're selling to other banks and investment funds. Kind of like how 
        // with social media platforms access to the users attention is the 
        // product that they sell to 3rd party advertisers.
        // https://en.wikipedia.org/wiki/Fractional-reserve_banking
        let tx = TX {
            sender: String::from("bank"),
            sender_password: 0,
            sender_nonce: self.accounts.get("bank").unwrap().nonce,
            receiver: receiver,
            amount: amount,
        };

        // Tx is legit by default because it's from the bank so let's process it.
        // decrease the balance from sender's account
        self.accounts
            .get_mut(&tx.sender)
            .unwrap()
            .balance -= tx.amount;
        // increase sender's nonce to prevent replay glitches
        self.accounts
            .get_mut(&tx.sender)
            .unwrap()
            .nonce += 1;
        // increase the balance of the reciever's account
        self.accounts
            .get_mut(&tx.receiver)
            .unwrap()
            .balance += tx.amount;
            
        // add processed TX to history
        self.history.push(tx.clone());        
    }
    
    // Create a new TX for a bank user
    pub fn new_user_tx(&mut self,
                       sender: String,
                       sender_password: i32,
                       sender_nonce: i32,
                       receiver: String,
                       amount: i32) {
        
        // This is really more of a TX request, but that's ok
        
        let tx = TX {
            sender: sender,
            sender_password: sender_password,
            sender_nonce: sender_nonce,
            receiver: receiver,
            amount: amount,
        };
        
        self.pending_tx.push(tx);
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

    println!("\n/// Created Pending TX ///");
    println!("{:#?}", bank);
}
```

<br>

## Resources

<br>
