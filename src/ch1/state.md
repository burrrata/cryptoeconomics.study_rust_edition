<h1 align="center" size="24">
    State: all the things...
</h1>

## Words

Some people say data is the new oil. Why? Probably because people like metaphors, but also... they both have one thing in common: they allow you to do stuff. Our world is becoming more and more digitized, and this means that data = access to:
- money,
- social connections,
- and identity.

This data is mostly stored in centralized databases. The "state" of these databases is what we reference to determine what is "true" or not. Ever gone to a restaurant and been told they can't find your reservation, or tried to register to vote and been told that you're not in the system? Data. 

In this example we'll explore some of the common data structures that banks keep track of, and how they might go about maintaining and updating the state of that data. 

## Video

[![Cryptoeconomics - 1.5 - Properties of Centralized Systems](https://img.youtube.com/vi/XIsn8-5Xekc/0.jpg)](https://www.youtube.com/watch?v=XIsn8-5Xekc)

<p>
    <a href="https://cryptoeconomics.study/lectures/chapter-01-2.html">Cryptoeconomics - 1.2 - State Transitions & Payment Processor Implementation</a>.
</p>

## Code
```rust, editable
// This is live editable Rust code. That means 
// that you can push the play button (little white triangle)
// in the top right of this box to see run it, or 
// you can edit whatever you like to see how that
// changes things. Explore!

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;


// In Rust, structs are a way to organize data. You can 
// learn more about them here:
// https://doc.rust-lang.org/book/ch05-00-structs.html

// This structure keeps track of all the bank's data.
// The state is simply a record of what's what, and when
// things change, like users doing stuff or the bank doing
// stuff, the state will (hopefully )change to reflect that.
// Theoretically it's in everyone's best interest to make
// sure that the state accurate.
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

// This structure keeps track of all the TX information
// the bank cares about from users.
#[derive(Debug, Clone)]
struct TX {
    // Account to take money from.
    sender: String,
    // Add a password so we know the account to take money
    // from is the one that submitted the TX.
    sender_password: i32,
    // Check to make sure we're not processing duplicate TX.
    sender_nonce: i32,
    // Account to add money to.
    receiver: String,
    // Amount of moneys to "move around". Actually, the money
    // doesn't exist because the bank makes it up when they
    // "loan" money. It's just a number in a database. The only
    // thing verifying it's existance is the banks internal 
    // ledger, and maybe the ledgers of other banks. Good 
    // thing those banks are all secure, honest, and don't
    // collude 👍
    amount: i32,
}


// An implimentation is a structure that links functions
// together. You can learn more about them here:
// https://doc.rust-lang.org/book/ch05-03-method-syntax.html
impl State {
    
    // This function create a new state for the bank.
    pub fn new_state() -> State {
    
        // Ah... a blank canvas. So clean. So pure. So beautiful.
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

// In Rust the main() function is where the program runs.
// You can store functions and stuff anywhere, but main()
// is the function has it's own state that keeps track of
// variables and computation. You can learn more about it
// here: https://doc.rust-lang.org/book/
fn main() {
    
    // Let's roll our own "bank"!
    let mut bank = State::new_state();
    println!("{:#?}", bank);
    
    // Let the games begin...
}
```
