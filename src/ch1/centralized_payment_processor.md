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
- show the bank's view vs the user's view


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
    pending_tx: HashMap<i32, TX>,
    history: HashMap<i32, TX>,
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
    sender_password: String,
    sender_nonce: i32,
    receiver: String,
}


// Central Payment Processor
impl State {
    
    // Create a new state
    pub fn new_state() -> State {
    
        /*
        HashMaps
        - accounts: (key: id, value: struct Account)
        - frozen_accounts: (key: id, value: struct Account )
        - pending_tx: (key: tx#, value: struct TX)
        - history: (key: tx#, value: struct TX)
        */
        
        let new = State {
            accounts: HashMap::new(),
            frozen_accounts: HashMap::new(),
            pending_tx: HashMap::new(),
            history: HashMap::new(),
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
        /*
        let mut account = self.accounts.get_mut(&account_id).unwrap();
        account.1.balance += amount;
        self.accounts.insert(account.0, account.1);
        */
    }
    
    // Check that an account exists
    pub fn does_account_exist(&mut self,
                              account_id: String) -> bool {
        
        if let Some(x) = self.accounts.get(&account_id) {
            return true
        }
        return false
    }
    
/*
Functions
- check account password = tx_password
- check account nonce = tx_nonce
- check account balance > tx amount
- check history of account
*/   
}




/*
MAIN
- create new state
- create new accounts
- simulate TX
- show the bank's view vs the user's view
*/
```
