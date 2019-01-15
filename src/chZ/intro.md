# IDEA

What if rather than structuring the chapters as
- b2c database manager
- p2p PoW blockchain
- p2p PoS blockchain

I built it to be as generic as possible so that you can plug in the consensus mechanism, or key signing function, or database type?
- essentially like Substrate

Minimal Viable Chain
- plugable (and chainable) modules for everything.
- ideally so modular you could have a chain of a chain of a chain of a...

    functions
    - state transition function (PoW, PoS, PoA, etc...)
    - hash function (default, blake, other...)
    - signing / key creation function (toy "RSA", ecdsa, ed25519, lamport, etc...)
    
    structs
    - address format (utxo, account based, other)
    - transaction format (nonces, balances, data, IDs, etc...)
    - state (user defines state by filling it with other structs)

```rust, ignore
impl Chain {

    impl STF {
    
        pub fn CDM() {}
    
        pub fn PoW() {}
        
        pub fn PoS() {}
        
        pub fn PoA() {}
        
        pub fn PoX() {}
    }
    
    impl hash {
    
        pub fn default() {}
        
        pub fn blake() {}
        
        pub fn other() {}
    
    }
    
    impl keys {
    
        pub fn rn() {}
    
        pub fn toy_rsa() {}
        
        pub fn ecdsa() {}
        
        pub fn ed25519() {}
    
    }
}
```

So in the "book" portion, each chapter would focus on an aspect of the blockchain, and each section in that chapter would explore variations within that aspect, for example: a chapter all about consensus mechanisms with each section exploring a minimal implimentation of various ones.

Then showcase different configurations of the blockchains modules for Bitcoin, Ethereum, etc showing what configurations lead to those properties and what the differences are

Then explore cryptoeconomic mechanisms built on or with these modules
- like how Numerai incentivizes high signal/noise ratios
- like how PoS creates stronger security
- like how voting systems can be gamed

<br><br><br>
