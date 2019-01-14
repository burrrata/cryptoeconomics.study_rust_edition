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

In chapter 1 we explored how a generic database might work and how that database would be managed if we trusted a central operator to run it. What if we wanted to create a shared database though, where we could all verify things for ourselves? 

One option might be to keep the central operator, but make the database transparent. All participants would agree to a set of rules that they would all play by and they could all verify changes of state. If anything happened that was not in line with the agreed upon rules they could switch to a new network. This has a few downsides though... mainly that switching and coordination costs are high. If you catch someone cheating (either the central operator or another user), at what threshold is it worth raising an issue or switching to another network? What if that network is already integrated into other apps and services and you'd have to switch those over too? What if your identity, social connections, or reputation are tied to this network and starting on a new one would mean losing all that? At what point would a transgressino of the network validate the cost of switching? If it's anything similar to our current banking or social networks, the answer is pretty damn high. There must be a better way...

It turns out, there is! It's called cryptography. Roughly speaking, cryptography is the process of using mathematic functions to conceal or verify information. In this chapter we'll focus mainly on the later as a way to replace trust in a central operator with trust in mathematics. This means that we can define in code all the functions that the central operator performed:
- creating accounts
- verifying tx
- managing state transitions

In this section we'll cover some of the basic concepts in cryptography that allow us to do this, then throughout the chapter we'll apply those concepts to create the mechanisms that do that, and then in the chapter summary we'll have our own working network that we can use to simulate these things.  

<br><br><br>

Let's start with a few foundational concepts that will help us get started :)

Computational Infeasibility:
- A process is computationally infeasible if it would take an impracticably long time (eg. billions of years) to do it for anyone who might conceivably have an interest in carrying it out. 
- Why should we care? Well, without knowing the computational feasibility or infeasibility of a problem we cannot make any claims that people can't cheat on our network by overwriting data or creating data without following the rules. Say for example I know the password to my account and I think that it's secure. If it takes you 3 days to randomly guess all the possible combinations of characters that my password might be, then it's really only secure as long as someone doesn't decide to take the time to figure it out. If it takes you 3 billion years to guess all the possible combinations though, then the story is quite different and I can rest assured that I'm not going to wake up one day and have all my data changed or stolen.

Randomness:
- Random data does not have a pattern. If one were to take a series of random data and cut it in half, then ask someone to guess the second half given the first, any possible values would be equally likely and no one could do better than a wild guess. In computer science and information theory, we often say that if a random string of data is long enough it is computationally infeasable to guess it, and thus it is secure against attacks.
- Why should we care? Well we're actually going to use random data to create accounts in such a way that only the person who created the account can use it to send transactions or sign data. We're also going to use random data to create a random string of characters that people have to guess in order to earn a reward and create the next state transition. The longest chain of valid state transitions is the agreed upon valid state, and since people are competing to earn the rewards that come with processing valid state transitions, anyone who wants to spam or overwrite the network would have to solve more puzzles faster than everyone else competing to do so, which on networks like Bitcoin or Ethereum is many.   

Hash:
- A hash function (or hash algorithm) is a process by which a piece of data of arbitrary size (could be anything; a piece of text, a picture, or even a list of other hashes) is processed into a small piece of data (usually 32 bytes) which looks completely random, and from which no meaningful data can be recovered about the document, but which has the important property that the result of hashing one particular document is always the same. Additionally, it is crucially important that it is computationally infeasible to find two documents that have the same hash. Generally, changing even one letter in a document will completely randomize the hash; for example, the SHA3 hash of "Saturday" is c38bbc8e93c09f6ed3fe39b5135da91ad1a99d397ef16948606cdcbd14929f9d, whereas the SHA3 hash of Caturday is b4013c0eed56d5a0b448b02ec1d10dd18c1b3832068fbbdc65b98fa9b14b6dbf. Hashes are usually used as a way of creating a globally agreed-upon identifier for a particular document that cannot be forged.
- Why should we care? Well we're acually going us a hash function to create the random string of characters that need to be guessed in order to earn a reward and create the next state transition. We're also going to use a hash function to store the history of all those state transitions. Since every hash is completely deterministic, if someone were to change 1 datapoint in the past, it would literally change every piece of data that came after it. This means that in order to cheat and arbitrarily rewrite the state someone would have to re-solve all the puzzles after that change, and do so faster than everyone else working on the current puzzles. The more people are working to solve puzzles on the network the harder someone who wants to cheat would have to work as well. This means that the security of the network is tied to the amount of people working to earn rewards by creating valid state transitions, which is often tied to the monetary value of those rewards, which for networks like Bitcoin or Ethereum is generally quite high. 

Merkle Trees:
- This is how we're going to store the history of all state transitions. Everytime someone earns the right to create a new valid state transition they're going to look at all the pending transactions that were requested between the last state change. They'll then check that all the transaction requests are valid and update the state accordingly. They then publish the new state along with the transactions that were processed to get to that state, the hash of the previous state, and a hash of the new state combined with the previous state's hash. This is called a block, because it's a bunch of data that gets processed together in batches, or "blocks". Every time there's a state transition a new block is created, and since every block comes published with a hash of the previous blocks, they're all chained together such that if you change something in a past block you also change all the hashes of any blocks after that. This is why our p2p decentralized database is called a "blockchain". It's also called a merkle tree because the only way to get to that latest hash is to include all the previous data. The "root" is the latest hash, and the branches are all the data that went into that hash, and all the data that went into those hashes, and so on...
- this is really just a good high level explanation of state transitions, but a really poor explanation of merkle trees. Maybe put it somewhere else?

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
- https://en.wikipedia.org/wiki/RSA_(cryptosystem)#Security_and_practical_considerations
- https://en.wikipedia.org/wiki/Cryptographically_secure_pseudorandom_number_generator
- https://www.quantamagazine.org/a-unified-theory-of-randomness-20160802/
- https://developer.mozilla.org/en-US/docs/Web/API/Web_Crypto_API

<br><br><br>
