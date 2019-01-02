```rust
// Here's an example

// Struct that holds transaction data
struct TX {
    sender: String,
    receiver: String,
    tx_amount: f32,
    nonce: i32,
}

// Struct that holds a transaction
// and a transaction signature making it valid
struct Signed_TX {
    tx: TX,
    signature: String,
}

// I need a function that can perform digital signatures by
// hashing the TX Struct with a private key, and that also
// allows people to verify that signature against the private
// key's corresponding public key.
// The keys can be very very small.
// The signature can be insecure.
// It just needs to use standard Rust code :)
```
