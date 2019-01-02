Following the wikipedia page:
- https://en.wikipedia.org/wiki/RSA_(cryptosystem)#Key_generation

TODO
- figure out how to take in a message String and turn it into a Vec<f32>

```rust
// because Rust isn't perfect and pow() or powf() don't
// have options for integration of modulo operations
fn exp_mod(input: i32,
           power: i32,
           modulo: i32) -> i32 {
    
    let mut out = (input * input) % modulo;
    //println!("0: {}", out);
    for i in 1..power-1 { //because the first iter of out took 2 off the base
        out = (out * input) % modulo;
        //println!("{}: {}", i, out);
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

fn main() {
    
    println!("// PARAMS //");
    
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
    let message = "abcd".to_string();
    println!("message string: {}", message);
    
    // Convert message to Vec<i32> because 
    let m1: Vec<i32> = message.as_bytes().iter().map(|x| *x as i32).collect();
    println!("message before encryption: {:?}", m1);
    
    // Encrypt the messages using the public key: e
    let e = toy_rsa(m1, pub_key, m);
    println!("encrypted message: {:?}", &e);
    
    // Decrypt the messages using the private key: d
    let m2 = toy_rsa(e, priv_key, m);
    println!("message after decryption: {:?}", &m2);
    
    // Convert decrypted Vec<i32> back to a Vec<u8>
    let message2_bytes: Vec<u8> = m2.iter().map(|x| *x as u8).collect();
    // Covert u8 bytes back to original String
    let message2 = String::from_utf8(message2_bytes).unwrap();
    println!("message string: {}", message2);
    
}
```
