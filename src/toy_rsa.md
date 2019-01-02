Following these turorials/suggestions:
- https://users.rust-lang.org/t/diy-digital-signatures/23739
- https://crypto.stackexchange.com/questions/24808/toy-public-private-key-example-open-to-frequency-analysis
- https://wpollock.com/AUnixSec/PublicKeyDemo.htm
- https://www.reddit.com/r/codes/comments/abg9tj/diy_digital_signatures/

```rust
#![allow(warnings)]

fn main() {
    
    // create a message
    let message = "abcd".to_string();
    
    // pick 2 primes
    let p: f32 = 3.0;
    let q: f32 = 11.0;
    
    // product of primes (the part that's usually hard to reverse) 
    let n = p * q; // 33
    
    // Compute the value of Euler's totient function of n: φ(n)
    let m: f32 = (p - 1.0) * (q - 1.0); // 20.0
    
    // Pick any number less than and relatively prime to φ(n)
    // in this case any prime number except 2 or 5 will do.
    // This is one of your two keys: the public/encryption key e.
    let e: f32 = 7.0;
    
    // Compute the matching private/decryption key d, 
    // as the inverse of e modulus φ(n).
    // In this case the inverse means a number such that 
    // e × d mod φ(n) = 1:
    let d: f32 = 3.0;
    
    // The two keys are: (7,33) and (3,33).
 
 
    // Encrypt the messages using the public key: e
    //let mut b = message.as_bytes();
    let mut b = vec![1, 2, 3, 4]; // [a, b, c, d]
    println!("b: {:?}", &b);
    
    let mut x = Vec::new();
    for i in b {
        x.push((i as f32).powf(e) % n);
    }
    println!("x: {:?}", &x);
    
    // Decrypt the messages using the private key: d
    let o: Vec<f32> = x.iter_mut().map(|i| i.powf(d) % n).collect();
    println!("o: {:?}", &o);
    
}
```
