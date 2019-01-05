# Randomness
and why it matters

## Words
If you want to see how important randomness is, just skip ahead to the [centralized payment processor demo](https://github.com/burrrata/cryptoeconomics.study_rust_edition/blob/master/src/ch1/centralized_payment_processor.md). Everything commented out relies on randomness. It's nealry impossible to create new accounts and generate public or private keys without a reliable and secure form of randomness. 

We'll explore this more in the public key crypto tutorial, but the [Rust Rand Book](https://rust-random.github.io/book/intro.html) is also a great resource. Not only do they explain how the library works, but they provide a foundation of knowledge for why their library works the way it does. It's honestly better than anything I could write, so I'll let you explore that for yourself.

Also, from Wikipedia is some warnings on how weak randomness can lead to hacked keys and crypto systems. This isn't a concern with our toy demo, but for any live working blockchain it's mission critical.
- https://en.wikipedia.org/wiki/RSA_(cryptosystem)

A cryptographically strong random number generator, which has been properly seeded with adequate entropy, must be used to generate the primes p and q. An analysis comparing millions of public keys gathered from the Internet was carried out in early 2012 by Arjen K. Lenstra, James P. Hughes, Maxime Augier, Joppe W. Bos, Thorsten Kleinjung and Christophe Wachter. They were able to factor 0.2% of the keys using only Euclid's algorithm.

They exploited a weakness unique to cryptosystems based on integer factorization. If n = pq is one public key and n′ = p′q′ is another, then if by chance p = p′ (but q is not equal to q'), then a simple computation of gcd(n,n′) = p factors both n and n′, totally compromising both keys. Lenstra et al. note that this problem can be minimized by using a strong random seed of bit-length twice the intended security level, or by employing a deterministic function to choose q given p, instead of choosing p and q independently.

Nadia Heninger was part of a group that did a similar experiment. They used an idea of Daniel J. Bernstein to compute the GCD of each RSA key n against the product of all the other keys n′ they had found (a 729 million digit number), instead of computing each gcd(n,n′) separately, thereby achieving a very significant speedup since after one large division the GCD problem is of normal size.

Heninger says in her blog that the bad keys occurred almost entirely in embedded applications, including "firewalls, routers, VPN devices, remote server administration devices, printers, projectors, and VOIP phones" from over 30 manufacturers. Heninger explains that the one-shared-prime problem uncovered by the two groups results from situations where the pseudorandom number generator is poorly seeded initially and then reseeded between the generation of the first and second primes. Using seeds of sufficiently high entropy obtained from key stroke timings or electronic diode noise or atmospheric noise from a radio receiver tuned between stations should solve the problem.[34]

Strong random number generation is important throughout every phase of public key cryptography. For instance, if a weak generator is used for the symmetric keys that are being distributed by RSA, then an eavesdropper could bypass RSA and guess the symmetric keys directly.


## Code
If you want to explore this code, copy the commented part into the Rust Playground:
- https://play.rust-lang.org

```rust

/*

extern crate rand;

use rand::prelude::*;

// generate a new key
fn key_gen() -> String {
    
    let rn: i32 = thread_rng().gen_range(100000, 1000000);
    
    rn.to_string()
}

fn main() {

    let key = key_gen();
    println!("key: {}", key);
    
}

*/
```

# Resources

### Rust Rand Book
- https://rust-random.github.io/book/intro.html

### Other
- https://blog.cloudflare.com/why-randomness-matters/
- https://en.wikipedia.org/wiki/Entropy_(information_theory)
- https://en.wikipedia.org/wiki/Cryptographically_secure_pseudorandom_number_generator
- https://www.quantamagazine.org/a-unified-theory-of-randomness-20160802/

### Ethereum Specific Randomness
- https://en.ethereum.wiki/Alternative-blockchains,-randomness,-economics,-and-other-research-topics
- https://vitalik.ca/files/randomness.html
- https://ethresear.ch/t/posw-random-beacon/1814
