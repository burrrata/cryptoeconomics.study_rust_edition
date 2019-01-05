# Public Key Crypto
(mostly RSA)

## Words
- mainly explain how you can create a pair of numbers such that it's easy to find one from the other, but not the other way around
- then exmplain how you can sign stuff with your private key, and others can verify that signature with your public key

## Code
This recreates the RSA style protocol explained in the [RSA wiki page](https://en.wikipedia.org/wiki/RSA_(cryptosystem)#Key_generation), randomly generating primes for p and q and demonstrating that the protocol works for various inputs :)

Haven't figured out how to import external crates to mdBook yet, so just copypasta the commented code into the Rust Playground if you want to experiment with it.

```rust

/*

extern crate rand;
use rand::prelude::*;

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

// slowly check if a number is prime
fn slow_prime_check(num: i32) -> bool {
    
    if num < 0 {
        println!("number must be greater than 0");
    }
    
    if num > 1000000 {
        println!("number cannot be greater than 1000000");
    }
    
    for i in 2..num{
        if num % i == 0 {
            return false
        }
    }
    true
}

// slowly yet randomly generate a prime number within a range
fn prime_gen(low: i32,
             high: i32) -> i32 {
    
    for i in 0..1000000 {
        let p = thread_rng().gen_range(low, high);
        if slow_prime_check(p) {
            return p
        }
    }
    0
}

// generate a public key within a range
fn pub_key_gen(min: i32,
               max: i32) -> i32 {
    
    let pub_key = prime_gen(min, max);
    assert!(max % pub_key != 0);
    
    pub_key
}

// slowly find the modular multiplicative inverse of a prime 
fn slow_mmi(ctf_pq: i32,
            pub_key: i32,
            max: i32)-> i32 {
    
    for i in 2..max {
        if (i * pub_key) % ctf_pq == 1 {
            return i
        }
    }
    println!("Try larger search?");
    0
}

// create a private key from a public key and other data
fn priv_key_gen(ctf_pq: i32,
                pub_key: i32) -> i32 {
    
    let priv_key = slow_mmi(ctf_pq, pub_key, 100000);
    
    priv_key
}

// Because... Rust.
// exp_mod() is like pow() with a mod option
// (like python does natively, but not Rust)
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



// Rollin, rollin, rollin... !
fn main() {

    /*
    // the numbers from wiki work
    // https://en.wikipedia.org/wiki/RSA_(cryptosystem)
    let p = 61;
    let q = 53;
    let m = 3233;
    let ctf_pq = 780;
    let pub_key = 17;
    let priv_key = 413;
    */

    // usually works on Rust Playground when p and q are < 500
    let p = prime_gen(5, 100);
    let q = prime_gen(5, 100);
    let m = p * q; 
    let ctf_pq = ctf(p, q);
    let pub_key = pub_key_gen(1, ctf_pq);
    let priv_key = priv_key_gen(ctf_pq, pub_key);
    println!("\n// Params //");
    assert!(p > 0);
    assert!(q > 0);
    println!("p: {}", &p);
    println!("q: {}", &q);
    println!("m: {}", &m);
    println!("ctf_pq: {}", &ctf_pq);
    println!("pub_key: {}", &pub_key);
    println!("priv_key: {}", &priv_key);
    
    let message = "thepasswordispassword".to_string();
    let m2nums = s2v(message.clone());
    let ciphertext = toy_rsa(m2nums.clone(), pub_key, m);
    let decrypted = toy_rsa(ciphertext.clone(), priv_key, m);
    let message2 = v2s(decrypted.clone());
    assert_eq!(message, message2);
    println!("\n// Testing //");
    println!("message: {:?}", &message);
    println!("message as nums: {:?}", &m2nums);
    println!("ciphertext: {:?}", &ciphertext);
    println!("decrypted nums: {:?}", &decrypted);
    println!("decrypted message: {}", &message2);
    
    println!("DONE!");

}

*/
```

## Resources
- https://en.wikipedia.org/wiki/Public-key_cryptography
- https://en.wikipedia.org/wiki/RSA_(cryptosystem)
- https://en.wikipedia.org/wiki/Least_common_multiple
- https://en.wikipedia.org/wiki/Modular_exponentiation
