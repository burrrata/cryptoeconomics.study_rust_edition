# Public Key Crypto
(mostly RSA)

```rust
// variable names based off Euclidean divison equation: a = b · q + r
// https://crates.io/crates/gcd
// https://en.wikipedia.org/wiki/Greatest_common_divisor
fn gcd(a: i32,
       b: i32) -> i32 {
    
    let (mut a, mut b) = if a > b {
        (a, b)
    } else {
        (b, a)
    };

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }

    a
}

// lowest common multiple
// https://en.wikipedia.org/wiki/Least_common_multiple
fn lcm(a: i32,
       b: i32) -> i32 {
    
    let lcm = (a * b) / gcd(a, b);
    
    lcm
}

// Carmichael's totient function
// https://en.wikipedia.org/wiki/Carmichael_function
fn ctf(a: i32,
       b: i32) -> i32 {
    
    lcm((a - 1), (b - 1))
}

// find primes in a range!
fn find_primes(low: i32,
               high: i32) -> i32 {
    0 // placeholder
} 


// function that calculates pow() with a mod option like
// python does (but Rust does not)
// https://docs.python.org/3/library/functions.html#pow
// https://doc.rust-lang.org/nightly/std/primitive.i32.html#method.pow
// https://en.wikipedia.org/wiki/Modular_exponentiation
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
    
    // Pick 2 primes > 0
    let p = 61;
    let q = 53;
    assert!(p > 0);
    assert!(q > 0);
    println!("p: {}", p);
    println!("q: {}", q);
    
    // Create the modulo group as a product of the primes
    // note: this must be shared between parties otherwise
    //       the protocol will not work
    let m = p * q; // 3233
    println!("m: {}", m);
    
    //  Carmichael's totient function
    let ctf_pq = ctf(p, q); // 780;
    assert_eq!(ctf_pq, 780);
    println!("ctf_pq: {}", ctf_pq);
    
    // Choose pub_key such that
    // 1 < pub_key < ctf_pq (780) 
    // that is coprime (not a divisor) to ctf_pq
    let pub_key = 17; // prime_gen(1, ctf_pq);
    assert!(ctf_pq / pub_key != 0);
    println!("public key: {}", pub_key);
    
    // TODO: explain why this works
    let priv_key = 413;
    println!("private key: {}", priv_key);
    
    
    println!("\n// TESTING FUNCTION //");
    // Create a message String
    let message = "thepasswordispassword".to_string();
    println!("message string: {}", message);
    
    // Convert message to Vec<i32>
    let m1 = s2v(message);
    println!("message before encryption: {:?}", m1);
    
    // Encrypt the messages using the public key
    let em = toy_rsa(m1, pub_key, m);
    println!("encrypted message: {:?}", &em);
    
    // Decrypt the messages using the private key
    let m2 = toy_rsa(em, priv_key, m);
    println!("message after decryption: {:?}", &m2);
    
    // Convert decrypted Vec<i32> back to message String
    let message2 = v2s(m2);
    println!("message string: {}", message2);
    
}
```

# Resources
- https://en.wikipedia.org/wiki/Public-key_cryptography
- https://en.wikipedia.org/wiki/RSA_(cryptosystem)
- https://en.wikipedia.org/wiki/Least_common_multiple
- https://en.wikipedia.org/wiki/Modular_exponentiation
