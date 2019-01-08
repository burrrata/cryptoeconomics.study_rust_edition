<h1 align="center">
    TX: making stuff happen.
</h1>

<br>

## Words

Core Concepts
- tx as things that request a change state
- state transition funciton as a way to confirm that state change
- fun facts that come with having a centralized operator do this for you

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
```rust
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
    
    // USEFUL FUNCTIONS
    
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
    
    
    // INIT STATE
    
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
    
    // ACCOUNT STUFF
    
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
    
    // TX STUFF
    
    // Add funds to an account
    pub fn add_funds(&mut self,
                     account_id: String,
                     amount: i32) {
        
        // A very important function for any private and seldom audited
        // for-profit enterprise. What could go wrong?
        // https://en.wikipedia.org/wiki/Enron_scandal
        
        if let Some(x) = self.accounts.get_mut(&account_id) {
            x.balance += amount;
        }
    }
    
    // Create a new bank TX
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
            sender: self.accounts.get(&bank_debt).unwrap(),
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

    // Verify pending user TX
    // Notice how the bank (or any hacker) gets to bypass this check
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

    // Simulate some TX
    for i in 0..10 {
        
        let sender = &bank.account_ids[thread_rng().gen_range(0, bank.account_ids.len())];
        let receiver = &bank.account_ids[thread_rng().gen_range(0, bank.account_ids.len())];
        
        if sender != receiver {
        
            bank.new_bank_tx(sender.to_string(),
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
}
```

<br>

## Resources

<br>
