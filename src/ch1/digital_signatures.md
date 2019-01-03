# Digital Signatures
The proof is in the pudding.

Talk about how when you sign/do stuff with a public key crypto system you prove that it came from your address.

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

// ^^ see toy_rsa.md example for WIP solution
```

# Resources
- https://en.wikipedia.org/wiki/Digital_signature
