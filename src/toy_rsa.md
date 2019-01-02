Following these turorials/suggestions:
- https://users.rust-lang.org/t/diy-digital-signatures/23739
- https://crypto.stackexchange.com/questions/24808/toy-public-private-key-example-open-to-frequency-analysis
- https://wpollock.com/AUnixSec/PublicKeyDemo.htm
- https://www.reddit.com/r/codes/comments/abg9tj/diy_digital_signatures/

TODO
- figure out how to take in a message String and turn it into a Vec<f32>

```rust

fn main() {
    
    println!("// PARAMS //");
    // pick 2 primes
    let p: i32 = 61;
    let q: i32 = 53;
    println!("p: {}\nq: {}", p, q);
    
    // product of primes (the part that's usually hard to reverse) 
    let m: i32 = 3233; //p * q;
    println!("n: {}", m);
    
    // LCM of p and q
    // Compute φ(n): the value of Euler's totient function of n
    let lcm: i32 = 780; // lcm((p - 1.0), (q - 1.0));
    println!("lcm: {}", lcm);
    
    // Pick any number less than and relatively prime to φ(n)
    // in this case any prime number except 2 or 5 will do.
    // This is one of your two keys: the public/encryption key.
    let pub_key: i32 = 17;
    println!("public key: {}", pub_key);
    
    // Compute the matching private/decryption key d, 
    // as the inverse of pub_key modulus φ(n).
    // In this case the inverse means a number such that 
    let priv_key: i32 = 413;
    println!("private key: {}", priv_key);

    // let p = 311, let q = 223
    // let m = 310 * 222 = 68820
    // let pub_key = 313
    // x * 313 mod 68820 = 1
    // x = 9831 
    // remainder is 3
    
    // a modular multiplicative inverse of an integer a is
    // an integer x such that the product ax is congruent
    // (equal) to 1 with respect to the modulus m 
    //let priv_key: i32 = "TBD" // 3.0;
    //assert_eq!(priv_key, 3.0);
    
    
    
    // Does n (the modulus) have to be bigger than any of the numbers in the message? 

    fn exp_mod(b: i32,
               p: i32,
               m: i32) -> i32 {
        
        let mut out = (b * b) % m;
        //println!("0: {}", out);
        for i in 1..p-1 { //because the first iter of out took 2 off the base
            out = (out * b) % m;
            //println!("{}: {}", i, out);
        }
        out
    }
    
    
    println!("\n// TESTING FUNCTION //");
    // Create a message as a String
    let message = "abcd".to_string();
    println!("message1: {}", message);
    
    // Convert message to Vec<i32> because 
    // pow() creates an overflow when applied to u8 or i32
    let m1: Vec<i32> = message.as_bytes().iter().map(|x| *x as i32).collect();
    //let m1: Vec<i32> = vec![15, 32, 23, 14];
    println!("m1: {:?}", m1);
    
    
    // Encrypt the messages using the public key: e
    let e: Vec<i32> = m1.iter().map(|x| exp_mod(*x, pub_key, m)).collect();
    println!("encrypted message: {:?}", &e);
    
    // Decrypt the messages using the private key: d
    let d: Vec<i32> = e.iter().map(|x| exp_mod(*x, priv_key, m)).collect();
    println!("decrypted message: {:?}", &d);
    
    
    // Convert decrypted Vec<i32> back to a Vec<u8>
    let m2: Vec<u8> = d.iter().map(|x| *x as u8).collect();
    println!("m2: {:?}", m2);
    
    // Covert u8 bytes back to original String
    let message2 = String::from_utf8(m2).unwrap();
    println!("message2: {}", message2);
    
}
```
