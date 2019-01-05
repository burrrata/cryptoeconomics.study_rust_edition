# State
All the things...

## Words

So as we mentioned, a blockchain is a glorified database. As such, at it's heart is some data. Like any database that changes, the current state of things is literally called the "state". With a centralized database it's often just a handful of admins who use their meatsticks to push buttons on a keyboard that tell a program what to do when. Being how this is the simplest example we'll start here, but in future chapters we'll see how just a few changes can transfer control from the admins to the users. This will result in a  decentralized blockchain where the state is agreed upon by a distributed group of verifiers based on some consensus rules. Working towards that, let's start with the simplest example possible to explore the basic concepts. 

## Code

```rust
use std::collections::HashMap;

// First we're going to create a struct that will hold the important state data we want to keep our database functioning:
// - accounts: this is where people's money and addresses live
// - pending_tx: a pool of pending tx that have not yet been verified as legit or not
// - verified_tx

#[derive(Debug)]
struct State {
    accounts: HashMap<String, Account>,
    pending_tx: Vec<TX>,
    verified_tx: Vec<TX>,
    history: Vec<Vec<TX>>,
}

#[derive(Debug)]
struct Account {
    balance: f32,
    nonce: i32,
}

#[derive(Debug, Clone)]
struct TX {
    sender: String,
    receiver: String,
    tx_amount: f32,
    nonce: i32,
}

#[derive(Debug, Clone)]
struct SignedTX {
    tx: TX,
    signature: String,
}


impl State {

    // initialize new blockchain
    pub fn new_blockchain() -> State {
        let mut state = State {
            accounts: HashMap::new(),
            pending_tx: Vec::new(),
            verified_tx: Vec::new(),
            history: Vec::new(),
        };
    
        state
    }
}


fn main() {

  let state = State::new_blockchain();
  println!("state: {:#?}", state);

}
```
