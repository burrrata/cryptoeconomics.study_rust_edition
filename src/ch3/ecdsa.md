<h1 align="center">
    ECDSA: math is beautiful.
</h1>

<br>

## Words

secp256k1
- https://github.com/paritytech/parity-ethereum/blob/master/accounts/ethkey/src/crypto.rs

Edwards-Curve Ed25519
- wikipedia:
  - https://en.wikipedia.org/wiki/Curve25519
- Substrate: 
  - https://www.parity.io/substrate/
- Dalek Crypto:
  - https://dalek.rs/
  - https://docs.rs/x25519-dalek/0.1.0/x25519_dalek/index.html#examples

In the chapter after this maybe expand to ZK proofs and stuff?
- heard rollup is cool: https://github.com/barryWhiteHat/roll_up


### Tiny Bits of Elliptic Curves

An Elliptic Curve for the purpose of cryptography is simply a large set of points that
we will call _C_. These points can be added, subtracted, or multiplied by integers (also called scalars).
Given an integer _k_ and
using the scalar multiplication operation we can compute `k*H`, which is also a point on
curve _C_. Given another integer _j_ we can also calculate `(k+j)*H`, which equals
`k*H + j*H`. The addition and scalar multiplication operations on an elliptic curve
maintain the commutative and associative properties of addition and multiplication:

### Cloudflare's "relatively" easy primer on ECC and ECDSA
- [https://blog.cloudflare.com/a-relatively-easy-to-understand-primer-on-elliptic-curve-cryptography/](https://blog.cloudflare.com/a-relatively-easy-to-understand-primer-on-elliptic-curve-cryptography/)
- [https://blog.cloudflare.com/ecdsa-the-digital-signature-algorithm-of-a-better-internet/](https://blog.cloudflare.com/ecdsa-the-digital-signature-algorithm-of-a-better-internet/)

Historically, in order to securely share messages people encrypted them. This was better than nothing, but it was annoying because:
- you had to meet in person or use a trusted intermediary to agree on the encryption protocol being used,
- then you had to share and keep track of the encryption/decryption keys,
- and even then if anyone intercepted your messages and figured out what protocol you used they could often decrypt your messages later on. 

This all changed with public private key cryptography. Rather than sharing secret codebooks, people were able to share their public keys, literally in public, for anyone to see. This acted like an address, and then people who wanted to send messages to that address would encrypt information via that public key, and the only way to decrypt that data was with the private key that corresponded to the public key. It's kind of like how mail works today where you can post your address publicly and people can send you mail, but only you have the key that can open the mailbox to read the mail. 

Today (2018), the majority of cryptography used for securing and verifying  message communication is founded on this idea that the key that you use to encrypt your data can be made public while the key that is used to to decrypt your data can be kept private. The first of these systems, is known as RSA — named  after the initials of the three people who first publicly described the algorithm.

So how does this work? The foundation of public private key crypto is a set of algorithms that are easy to process in one direction, but difficult to undo. Algorithms that have this characteristic — easy in one direction, hard the other — are known as trapdoor door functions. This means that anyone can easily encrypt messages for a specific public key, but decrypting that message is very hard unless you have the corresponding private key. In the case of RSA, this is done by multiplying two prime numbers. It's easy to take two numbers and multiply them, but it is difficult to take a (large) prime number and factor it to figure out what two primes were used to create it. 

<br>

## Videos

<br>

## Code
```rust, ignore
extern crate rand;
use rand::prelude::*;

// multiplies an i32 number X
// a certain number of times Y 
// and returns an i32 number
fn power(x: i32,
         y: i32) -> i32 {
    
    //println!("x: {}", x);
    //println!("y: {}", y);
    
    let mut output = x*x; 
    //println!("output: {}", output);
    for i in 0..(y-2) {
        output = output * x;
        //println!("output: {}", output);
    };
    
    output
} 

// like power(), but with a modulo operation 
// thrown in between every multiplication 
fn power_mod(x: i32,
             y: i32,
             m: i32) -> i32 {
    
    //println!("x: {}", x);
    //println!("y: {}", y);
    //println!("m: {}", m);
    
    let mut output = (x * x) % m; 
    //println!("output: {}", output);
    for i in 0..(y-2) {
        output = (output * x) % m;
        //println!("iter {}: {}", i, output);
    };
    
    output
} 

// TODO! rewrite this or find a decent native Rust function
// returns log of num
// or 0 if it's not divisible by an integer
fn logg(num: i32,
        log: i32) -> i32 {
    
    let mut count = 0;
    let mut m = log.clone();
    
    for i in 0..100000 { // so that it doesn't run forever
        m = m * log;
        count += 1;        
        if m > num {
            return 0
        }        
    }
    
    count
}

// determines if a number is probably prime
// returns a boolean
fn rabin_miller(n: i32) -> bool {

    let mut s = n-1;
    let mut t = 0;
    // in python: & is a binary AND gate
    // copies a bit to the result if exists in both operands
    // https://www.tutorialspoint.com/python/bitwise_operators_example.html
    // same in Rust: 
    // https://doc.rust-lang.org/book/appendix-02-operators.html
    while s & 1 == 0 { 
        s = s/2;
        t +=1;
    }
    
    let mut k = 0;
    while k < 128 {
        //println!("k: {}", k);
        let a = thread_rng().gen_range(2, n-1);
        //println!("a: {}", a);
        let mut v = power_mod(a, s, n);
        //println!("v: {}", v);
        if v != 1 {
            let mut i = 0;
            while v != (n-1) {
                if i == t-1 {
                    //println!("i: {}", i);
                    return false
                } else {
                    i += 1;
                    v = (v^2)%n;
                    //println!("i: {}", i);
                    //println!("v: {}", v);
                }
            }
        }
        k += 2;
    }
    return true
}

// lowPrimes is all primes (sans 2, which is covered by the bitwise and operator)
// under 1000. taking n modulo each lowPrime allows us to remove a huge chunk
// of composite numbers from our potential pool without resorting to Rabin-Miller
// returns boolean
fn is_prime(n: i32) -> bool {

    let low_primes = vec![3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,
                        101,103,107,109,113,127,131,137,139,149,151,157,163,167,173,179,
                        181,191,193,197,199,211,223,227,229,233,239,241,251,257,263,269,
                        271,277,281,283,293,307,311,313,317,331,337,347,349,353,359,367,
                        373,379,383,389,397,401,409,419,421,431,433,439,443,449,457,461,
                        463,467,479,487,491,499,503,509,521,523,541,547,557,563,569,571,
                        577,587,593,599,601,607,613,617,619,631,641,643,647,653,659,661,
                        673,677,683,691,701,709,719,727,733,739,743,751,757,761,769,773,
                        787,797,809,811,821,823,827,829,839,853,857,859,863,877,881,883,
                        887,907,911,919,929,937,941,947,953,967,971,977,983,991,997];
    
    if n >= 3 {
        if n & 1 != 0 {
            for p in low_primes {
                if n == p {
                    return true
                } else if n % p == 0 {
                    return false
                }
            return rabin_miller(n)                  
            }
        }
    }
    return false
}

// generates (potentially) prime numbers
// the input k is the desired bit length
// on the Rust Playground: https://play.rust-lang.org
// 15 is the highest value of k that does not overflow
fn prime_gen(k: i32) -> i32 {
    
    let mut r = 100 * (logg(k, 2) + 1); //number of attempts max
    let r_ = r.clone(); 
    while r > 0 {
        let n = thread_rng().gen_range(power(2, k-1), power(2, k));
        r -= 1;
        if is_prime(n) {
            return n
        }       
    }
    return 0
}


fn main() {
    
    let number = 15; // highest you can use without overflowing the Rust Playground
    println!("prime_gen: {}", prime_gen(15));
    
}
```

<br>

### Toy RSA algorithm:
- todo: add simplified description from Cloudflare blog
- https://blog.cloudflare.com/a-relatively-easy-to-understand-primer-on-elliptic-curve-cryptography/

```rust, ignore
// An extremely simple program to illustrate the basic principles of RSA crypto.
// Uncomment the print statements within the RSA function to see it in action! 
// Try changing the message value to see that it works with any number.

fn main() {
 
    // set up RSA function
    fn rsa(m: i32, 
           max: i32,
           key: i32) -> i32 {
        
        let mut new_m = (m * m) % max;
        println!("\niteration 0 and 1: {}", new_m);
        
        for i in 2..key {
            new_m = (new_m * m) % max;
            println!("iteration {}: {}", i, new_m);
        };
        
        new_m
    }
    
    // set up params
    let p1 = 13; // prime 1
    let p2 = 7; // prime 2
    let max = p1 * p2; // 91
    let pub_key = 5; // public key
    let priv_key = 29; // private key
    let m = 5; // message
    let enc_m = rsa(m, max, pub_key); // encrypted message
    let dec_m = rsa(enc_m, max, priv_key); // decrypted message
    

    // print and check results
    println!("\noriginal message: {}", m);    
    println!("\npublic key encrypted message: {}", enc_m);
    println!("\nprivate key decrypted message: {}", dec_m);
    assert!(m == dec_m);
    
}
```


<br>

## Resources

Computerphile Youtube Video 
- A visual introduction to ECC: https://www.youtube.com/watch?v=NF1pwjL9-DE

Andrea Corbellini's "gentle" introduction to ECC
- part 1 http://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/
- part 2 http://andrea.corbellini.name/2015/05/23/elliptic-curve-cryptography-finite-fields-and-discrete-logarithms/
- part 3 http://andrea.corbellini.name/2015/05/30/elliptic-curve-cryptography-ecdh-and-ecdsa/
- part 4 http://andrea.corbellini.name/2015/06/08/elliptic-curve-cryptography-breaking-security-and-a-comparison-with-rsa/

Adrea Corbellini on ECDSA
- http://andrea.corbellini.name/2015/05/30/elliptic-curve-cryptography-ecdh-and-ecdsa/

Wikipedia
- Elliptic Curve Cryptography: https://en.wikipedia.org/wiki/Elliptic-curve_cryptography
- Elliptic Curve Digital Signature Algorithms: https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm

Haven't read these yet ¯\_(ツ)_/¯
- https://www.johannes-bauer.com/compsci/ecc/
- https://www.instructables.com/id/Understanding-how-ECDSA-protects-your-data/

Libraries and Implementations of ECDSA
- https://dalek.rs
- https://github.com/DaGenix/rust-crypto/blob/master/src/ed25519.rs


<br>
