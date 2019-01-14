<br>

<div align="center">
    <p align="center">
        <img src="accounts.png">
    </p>
    <h1 align="center">
        Accounts
    </h1>
    <p align="center">
        You and your stuff.
    </p>
</div>

<br><br><br>

An account is just a way to store data to make it more useful. As we said before, he who controls the data controls the world. Let's put all our eggs in one basket via a centralized database and see how that works.

Some people like centralized databases and services because they're fast and there's someone to blame if/when things go wrong. This makes users feel safe because it's familiar, they don't have to think too much, and someone seems responsible. Time has told us however that this is merely a mirage to make us feel good and in reality centralized operators have most of the upside but limited downside, while for users it's reversed. Sound fun? Great! Let's go...

<br><br><br>

[![Cryptoeconomics - 1.0 - Chapter 1 Overview](https://img.youtube.com/vi/VaUTTE5xb54/0.jpg)](https://www.youtube.com/watch?v=VaUTTE5xb54)

<p>
    <a href="https://cryptoeconomics.study/lectures/chapter-01-0.html">Cryptoeconomics - 1.0 - Chapter 1 Overview</a>.
</p>

<br><br><br>

```rust, ignore
// This is Rust code.
// I haven't figured out how to get rand to play
// nicely with mdBook, so you'll have to copypasta
// this into the Rust Playground. Have fun!
// https://play.rust-lang.org

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
    
        // Ah... a blank canvas. So clean. So pure. So beautiful.
        // Let the games begin.
    
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
        
        // Notice how the only thing tying the account_id to the password
        // is that the bank stores them in the same database. If the bank
        // were to change this by accident, or a hacker were to get access to
        // that data via hacking the bank directly or a relevant 3rd party...
        // well... life would get very interesting very fast. Mostly for you 
        // though because the banks are insured so for them it's a write-off
        // that affects them minimally. 
        // https://en.wikipedia.org/wiki/Write-off
        // https://en.wikipedia.org/wiki/Equifax
        
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
        
        // Sock puppets ahoy!
        // Good thing banks are honest and would never create accounts to
        // simulate activity when there was none. Even better that crypto
        // exchanges are even more honest because, well... crypto! It's 
        // different this time right?
        // https://en.wikipedia.org/wiki/Sockpuppet_(Internet)
        // https://en.wikipedia.org/wiki/Wash_trade
        // https://medium.com/@bitfinexed/wash-trading-bitcoin-how-bitfinex-benefits-from-fraudulent-trading-8bd66be73215
        // https://medium.com/@bitfinexed/the-tether-truth-machine-the-wheels-of-justice-turn-slowly-but-grind-exceedingly-finely-8e3bd72ad011
        
        for i in 0..num_accounts {
            self.new_account()
        }
    }
    
    // Print account info
    pub fn print_account_info(&mut self,
                         account_id: String) {
        
        // If it's written down it must be true.
        
        if let Some(x) = self.accounts.get(&account_id) {
            println!("Your Account:\n{:#?}", self.accounts.get(&account_id).unwrap());
        }
        println!("Account not found");
    }
    
    // Print account history
    pub fn print_account_history(&mut self,
                                 account_id: String,) {
        
        // Assuming the bank's records are accurate and up to date, which
        // we assume they are, probably, but we don't know ¯\_(ツ)_/¯ 
        // https://www.bbc.com/news/business-43985233
        // https://www.cnet.com/news/commonwealth-bank-of-australia-financial-data-breach-20-million-accounts/
        
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
}
```

<br><br><br>

## But wait... there's more
- https://en.wikipedia.org/wiki/User_(computing)
- https://en.wikipedia.org/wiki/Bank_account

<br><br><br>
    
