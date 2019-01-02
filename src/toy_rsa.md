Following the wikipedia page:
- https://en.wikipedia.org/wiki/RSA_(cryptosystem)#Key_generation

```rust
// function that calculates pow() with a mod option like
// python does (but Rust does not)
// https://docs.python.org/3/library/functions.html#pow
// https://doc.rust-lang.org/nightly/std/primitive.i32.html#method.pow
fn exp_mod(input: i32,
           power: i32,
           modulo: i32) -> i32 {
    
    let mut out = (input * input) % modulo;
    // because the first iter of out took 2 off the base
    for i in 0..power-2 {
        out = (out * input) % modulo;
    }
    
    out
}

// toy RSA function
fn toy_rsa(input: Vec<i32>,
           key: i32,
           modulo: i32) -> Vec<i32> {
    
    let output = input.iter()
                      .map(|x| exp_mod(*x, key, modulo))
                      .collect();
    output
}

// convert string to Vec<i32>
fn s2v(input: String) -> Vec<i32> {
    
    let output: Vec<i32> = input.as_bytes()
                                .iter()
                                .map(|x| *x as i32)
                                .collect();
    
    output
}

// convert Vec<i32> to string
fn v2s(input: Vec<i32>) -> String {
    
    let output_u8: Vec<u8> = input.iter()
                                  .map(|x| *x as u8)
                                  .collect();
    let output_string = String::from_utf8(output_u8).unwrap();
    
    output_string
}


// Let's roll it
fn main() {
    
    println!("\n// PARAMS //");
    
    // pick 2 primes
    let p: i32 = 61;
    let q: i32 = 53;
    println!("p: {}\nq: {}", p, q);
    
    // product of primes
    // (the part that's usually hard to reverse) 
    let m: i32 = p * q; // 3233
    println!("m: {}", m);
    
    // TODO: create function that computes LCM of p and q
    let lcm: i32 = 780; // lcm((p - 1.0), (q - 1.0));
    println!("lcm: {}", lcm);
    
    // TODO: explain why this works
    let pub_key: i32 = 17;
    println!("public key: {}", pub_key);
    
    // TODO: explain why this works
    let priv_key: i32 = 413;
    println!("private key: {}", priv_key);


    println!("\n// TESTING FUNCTION //");
    // Create a message String
    let message = "thepasswordispassword".to_string();
    println!("message string: {}", message);
    
    // Convert message to Vec<i32>
    let m1 = s2v(message);
    println!("message before encryption: {:?}", m1);
    
    // Encrypt the messages using the public key: e
    let e = toy_rsa(m1, pub_key, m);
    println!("encrypted message: {:?}", &e);
    
    // Decrypt the messages using the private key: d
    let m2 = toy_rsa(e, priv_key, m);
    println!("message after decryption: {:?}", &m2);
    
    // Convert decrypted Vec<i32> back to message String
    let message2 = v2s(m2);
    println!("message string: {}", message2);
    
}
```
