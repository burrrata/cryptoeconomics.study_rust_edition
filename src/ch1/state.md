# State
All the things

## TODO
Explain how a State object is simply a mapping of data in the chain, mainly addresses -> balances

## Code

```rust
use std::collections::HashMap;


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
