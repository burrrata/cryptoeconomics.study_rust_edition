<br>

<div align="center">
    <p align="center">
        <img src="TBD" alt="TBD">
    </p>
    <h1 align="center">
        Crypto & Trust
    </h1>
    <p align="center">
        How we go from a b2c centralized database to a p2p decentralized database.
    </p>
</div>

<br><br><br>

Core Concepts:
- cryptographic hardness (a foundation of trust and security)
- randomness (key generation)
- hash functions (data on lock)

### Computational infeasibility:
A process is computationally infeasible if it would take an impracticably long time (eg. billions of years) to do it for anyone who might conceivably have an interest in carrying it out. Generally, 280 computational steps is considered the lower bound for computational infeasibility.

### Hash:
A hash function (or hash algorithm) is a process by which a piece of data of arbitrary size (could be anything; a piece of text, a picture, or even a list of other hashes) is processed into a small piece of data (usually 32 bytes) which looks completely random, and from which no meaningful data can be recovered about the document, but which has the important property that the result of hashing one particular document is always the same. Additionally, it is crucially important that it is computationally infeasible to find two documents that have the same hash. Generally, changing even one letter in a document will completely randomize the hash; for example, the SHA3 hash of "Saturday" is c38bbc8e93c09f6ed3fe39b5135da91ad1a99d397ef16948606cdcbd14929f9d, whereas the SHA3 hash of Caturday is b4013c0eed56d5a0b448b02ec1d10dd18c1b3832068fbbdc65b98fa9b14b6dbf. Hashes are usually used as a way of creating a globally agreed-upon identifier for a particular document that cannot be forged.

### Encryption:
Encryption is a process by which a document (plaintext) is combined with a shorter string of data, called a key (eg. c85ef7d79691fe79573b1a7064c19c1a9819ebdbd1faaab1a8ec92344438aaf4), to produce an output (ciphertext) which can be "decrypted" back into the original plaintext by someone else who has the key, but which is incomprehensible and computationally infeasible to decrypt for anyone who does not have the key.

### Merkle Trees
- effecient data verification
- touch on patricia trees, but explain/build them in ch2: https://github.com/ethereum/wiki/wiki/Patricia-Tree

### Randomness 

The [Rust Rand Book](https://rust-random.github.io/book/intro.html) is also a great resource. Not only do they explain how the library works, but they provide a foundation of knowledge for why their library works the way it does. It's honestly better than anything I could write, so I'll let you explore that for yourself.

Also, from Wikipedia is some warnings on how weak randomness can lead to hacked keys and crypto systems. This isn't a concern with our toy demo, but for any live working blockchain it's mission critical.
- https://en.wikipedia.org/wiki/RSA_(cryptosystem)

A cryptographically strong random number generator, which has been properly seeded with adequate entropy, must be used to generate the primes p and q. An analysis comparing millions of public keys gathered from the Internet was carried out in early 2012 by Arjen K. Lenstra, James P. Hughes, Maxime Augier, Joppe W. Bos, Thorsten Kleinjung and Christophe Wachter. They were able to factor 0.2% of the keys using only Euclid's algorithm.

They exploited a weakness unique to cryptosystems based on integer factorization. If n = pq is one public key and n′ = p′q′ is another, then if by chance p = p′ (but q is not equal to q'), then a simple computation of gcd(n,n′) = p factors both n and n′, totally compromising both keys. Lenstra et al. note that this problem can be minimized by using a strong random seed of bit-length twice the intended security level, or by employing a deterministic function to choose q given p, instead of choosing p and q independently.

Nadia Heninger was part of a group that did a similar experiment. They used an idea of Daniel J. Bernstein to compute the GCD of each RSA key n against the product of all the other keys n′ they had found (a 729 million digit number), instead of computing each gcd(n,n′) separately, thereby achieving a very significant speedup since after one large division the GCD problem is of normal size.

Heninger says in her blog that the bad keys occurred almost entirely in embedded applications, including "firewalls, routers, VPN devices, remote server administration devices, printers, projectors, and VOIP phones" from over 30 manufacturers. Heninger explains that the one-shared-prime problem uncovered by the two groups results from situations where the pseudorandom number generator is poorly seeded initially and then reseeded between the generation of the first and second primes. Using seeds of sufficiently high entropy obtained from key stroke timings or electronic diode noise or atmospheric noise from a radio receiver tuned between stations should solve the problem.[34]

Strong random number generation is important throughout every phase of public key cryptography. For instance, if a weak generator is used for the symmetric keys that are being distributed by RSA, then an eavesdropper could bypass RSA and guess the symmetric keys directly.

<br><br><br>

If you want to explore this code, copy the commented part into the Rust Playground:
- https://play.rust-lang.org

```rust, ignore

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
```

<br><br><br>

## Resources & References

### Ethereum Wiki Glossary
- https://github.com/ethereum/wiki/wiki/Glossary

### Rust Rand Book
- https://rust-random.github.io/book/intro.html

### Ethereum Specific Randomness
- https://en.ethereum.wiki/Alternative-blockchains,-randomness,-economics,-and-other-research-topics
- https://vitalik.ca/files/randomness.html
- https://ethresear.ch/t/posw-random-beacon/1814

### Other Randomness Resources
- https://blog.cloudflare.com/why-randomness-matters/
- https://en.wikipedia.org/wiki/Entropy_(information_theory)
- https://en.wikipedia.org/wiki/Cryptographically_secure_pseudorandom_number_generator
- https://www.quantamagazine.org/a-unified-theory-of-randomness-20160802/
- https://developer.mozilla.org/en-US/docs/Web/API/Web_Crypto_API

<br><br><br>
