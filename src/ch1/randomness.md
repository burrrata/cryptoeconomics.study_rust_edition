# Randomness
and why it matters

```rust
extern crate rand;

use rand::prelude::*;

// generate a new key
pub fn key_gen() -> String {
    
    let rn: i32 = thread_rng().gen_range(100000, 1000000);
    
    rn.to_string()
}

fn main() {

    let key = key_gen();
    println!("key: {}", key);
    
}
```
