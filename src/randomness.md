## External Crate Test

If you add this to the SUMMARY.md, the mdBook build fails.

The Rust Cookbook did it, but I don't know how
- https://rust-lang-nursery.github.io/rust-cookbook/
- https://github.com/rust-lang-nursery/rust-cookbook


```rust
extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}
```

[`rand::Rng`]: https://docs.rs/rand/*/rand/trait.Rng.html
[`rand::thread_rng`]: https://docs.rs/rand/*/rand/fn.thread_rng.html
