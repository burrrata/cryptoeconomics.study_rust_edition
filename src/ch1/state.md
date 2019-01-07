# State
All the things...

## Video

## Words

## Code
```rust, ignore
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;


// This structure keeps track of all the data we need
#[derive(Debug)]
struct State {

    // Accounts are stored in a HashMap 
    // with the account ID String the key
    // and the Account Struct as the value.
    accounts: HashMap<String, Account>,
    // Frozen accounts are decoupled from the main system
    // so that they're non-funcitonal, but they are not
    // deleted so that you can unfreeze them if needed.
    // This can be useful for regulatory compliance or 
    // savings accounts.
    frozen_accounts: HashMap<String, Account>,
    // This is just a Vec that stores all account IDs.
    // This is useful to look up account data in the 
    // accounts HashMap.
    account_ids: Vec<String>,
    // This is churning pool of TX that have been submitted
    // by users, but not verified and processed by the bank. 
    pending_tx: Vec<TX>,
    // This is a history of all TX the bank has processed
    // correctly.
    history: Vec<TX>,
    // This is a history of all the moneys the bank has
    // created via fractional reserve banking. The bank can
    // sell this history to collectors or other investors.
    debt_history: Vec<TX>,
    // This is the amount of capital the bank has as debt on
    // it's balance sheet. While this is a liability for the
    // users who owe money, it's an asset for the bank that
    // they can trade at a discount to other parties like
    // collectors or investment funds.
    debt_pool: i32,
}

// This structure keeps track of the information in a user
// account.
#[derive(Debug, Clone)]
struct Account {

    // The password is how the user authorizes TX and proves
    // that they were sent by the user and not another party
    // like the receiver creating fake TX. This data is 
    // stored in the bank's centralized account database, 
    // and the bank (or anyone who gains access to it) can
    // change it at any time. This can be on purpose due to
    // compliance, regular operations, or intentional fraud.
    // This can also occur if the bank is hacked or if the user
    // reuses a password from another site for their bank
    // account, and that site then gets hacked.
    password: i32,
    // This number is incrimented every time an account
    // creates a valid TX. This is to prevent accidental 
    // glitches that might replay TX if the pending_tx pool
    // is not cleared properly.
    nonce: i32,
    // This is the users balance. Funny how so much of a 
    // person's access to resources, opportunities, and
    // survival depend on this number being accurate...
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


impl State {
    
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
}
```
