# Accounts
- combining keys to make something useful

```rust
use std::collections::HashMap;

#[derive(Debug)]
struct Account {
    balance: f32,
    nonce: i32,
}


fn main() {

    let mut accounts = HashMap::new();
    
    accounts.insert("0x2135324", Account{balance: 100.0, nonce: 0});
    
    println!("accounts: {:#?}", accounts);
    
    println!("accounts 0 balance: {:#?}", accounts.get("0x2135324").unwrap().balance);
    
    
}
```
