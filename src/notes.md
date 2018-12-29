## GOALS
- For education purposes build a fully functional (but not secure) blockchain using standard Rust code so that the main concepts can be understood as simply as possible in mdBook or the Rust Playground.
- Then show create a library and tutorial that is meant to be explored through the CLI that shows how to iteratively ugrade each component and function in the standard model to make it more secure
- Stetch Goal: work towards recreating the Parity Ethereum Client, but that's probably not realistic lol


## Simple Rust Blockchain Template
Uses mostly standard Rust without calling to external crypto libraries that mdBook can't handle.
- https://steemit.com/technology/@tensor/rust-project-cli-toy-blockchain
- https://github.com/tensor-programming/Rust_block_chain

The above example uses sha256, which is great, but also calls an external library 
This is a function to replace ```pub fn hash``` with a standard Rust hasher

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

fn main() {

    fn hash(input: &[u8]) -> String {
        
        let mut hasher = DefaultHasher::new();
        hasher.write(input);
        let digest = hasher.finish();
        let hex_digest = format!("{:#X}", digest);
        
        hex_digest
    }
    
    let hw = "Hello World";
    let hw_bytes = hw.as_bytes();
    println!("hw_bytes: {:?}", hw_bytes);
    
    let hash_test = hash(hw_bytes);
    println!("hash test: {}", hash_test);
    
}
```

## For the latest version that works in the Rust Playground, see current.md
