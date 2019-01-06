Structs
- State {accounts, frozen_accounts, pending_tx, history}
- Account {balance, password, nonce}
- TX {sender, password, nonce, receiver}

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

HashMaps
- accounts: (key: id, value: struct Account)
- frozen_accounts: (key: id, value: struct Account )
- pending_tx: (key: tx#, value: struct TX)
- history: (key: tx#, value: struct TX)

MAIN
- create new state
- create new accounts
- simulate TX
- show the bank's view vs the user's view

```rust, ignore
// Code TBD
```
