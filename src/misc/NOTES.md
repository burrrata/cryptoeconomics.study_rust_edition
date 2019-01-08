<h1 align="center">
    NOTES
</h1>
<br>
<br>
<br>

# IMMEDIATE TODO
- double check that ch1 central database is up to date from changes to tx and state transition function 
- add timestamps to the central database model to order tx 
- clean up ch1 template so it looks less like a rough draft and more like something you want to explore

<br>

# Interactive Art
- how can I simulate multiple users in an environment so that everything doesn't just run in the main() thread?
- can fearless concurrency be simulated in mdBook or the Rust Playground?
- https://doc.rust-lang.org/book/ch16-00-concurrency.html

<br>

# Generalizable Data Structures
- focus LESS on payments, and MORE on data and access to computation, information, identity, social networks, and services.

While banks or central operators can change the data in our bank accounts, they can also change our ability to communicate with our friends or access goods and services on social media platforms and marketplaces. When we move to more generalizable and abstract state machines (EVM) it's really about data and not money. Like the intro says: he who controls the data controls the world. Putting this back in the hands of users is the only way to do that. World peace comes about one person at a time, and world order comes about when power is evenly distributed. That's why this tutorial exists so that more people can learn about this stuff! Would be good to emphasize that :)


# Ideas From C.S
- https://docs.google.com/document/d/1R85zczC1-nklLXEFx-dZfQdlZexRAk8S9G9Hc3Zjkx4/edit


# Centralized Database
- users trust bank state
- bank creates and controls accounts
- users request tx to be approved by bank
- bank controls any state transitions including freezing accounts or new money creation


# PoW "Blockchain"
- centralized state database => new p2p database structure
- trust => agreed upon pub key crypto accounts and network params (randomness + crypto basics)
- accounts => accounts created via "secure" random key generation (accounts + pub key crypto)
- tx => signed tx (tx + digital signatures)
- history => blocks and merkle tries (blocks + hashing)
- bank approved state transition => PoW state transition 

It's really important here that we can simulate multiple miners creating blocks to showcase how we deal with forks and decide which chain is the main chain.

Ideas from C.S:
- Implement a new Client node class.
- Implement a new Miner node class. 
- Run their protocol in the sandbox and understand how Nakamoto consensus functions under different assumptions.
- Develop a new doubleSpend.js script to spend coins, revert history and mine a longer chain.
- Give one miner 51% of the hash power and execute the doubleSpend.js script in the sandbox.
- Implement a new Selfish Miner node class with logic to execute a selfish mining attack.
- Give one miner 25% of the hash power and execute a selfish mining attack in the sandbox.

UX Feature Ideas from C.S:
- Option to toggle nodes into miners and back again
- Clearly display the longest chain and which nodes agree on this chain.
- Easily expand a block and explore its contents in a separate view.
- Visually see miners iterating through nonces and checking hashes
- Controls to adjust amount of mining power each miner has
- User can click a button to attempt a double spend attack
- Users can also toggle a miner into a selfish miner.
- Simulation of cost of mining to demonstrate when miners are profitable (and show that they lose money under the attack of selfish mining)

# PoS "Blockchain"
- signed "RSA" tx => ECDSA
- generic merkle trie => patricia tries
- generic String/u8 conversion => RSL data encoding
- PoW => PoS

C.S Casper Beacon Chain Ideas:
- Add validator class
- Add deposit transaction which locks coins
- Add withdraw transaction which unlocks coins (after some delay)
- Add vote() which votes on the current epoch - if more than ⅔ vote then the block is finalized
- Update the fork choice rule to not revert finalized blocks, and accept a ‘starting block’ blockhash.
C.S Feature Ideas:
- Visualize when finality is reached
- Implement and attempt nothing at stake attacks (get slashed)
- Users can alter the # of honest nodes
- Implement and attempt long range revision attacks
- Controls to adjust the stake of each node


<hr>

So... if we want to do interesting things with cryptoecnomics... we need a scripting language, and that means replicating the EVM... and that's a lot... Would it make more sense to dive into solidity at this point, or is it possible to replicate aspects of the Parity Ethereum client if I can roll external libraries in mdBook or build it out as a CLI tutorial?

<hr>

### L2: state channels

### L2: plasma

### Cryptoeconomic Applications

<br>

# General TODOs

### Book
External crates
- https://users.rust-lang.org/t/diy-digital-signatures/23739/5

Fun > docs/tutorials
- pictures for each chapter intro
- ayu as default theme
- encourage PRs so people can upgrade code or propose alternative versions of functions (ideally leading to modular systems people can play with)

### Ch1
- better resources
- clean up sassy comments so that they're not repeated every section

<br>

### Ch2
- clean up organization
- add PoW

<br>

### Ch3
- literally everything

<br>

# Better Rust Code
- It would be great to learn more about lifetimes so that every function can take in a reference rather than requiring a String that has to be cloned over and over and over...
- It would also be nice to have better errors so that when functions fail we (or the users) know why.

<br>

# Feedback
- There needs to be a clear and easy way for people to provide feedback.
    - email: rustycryptoeconomics@protonmail.com
    - forum? 
    - reddit?
- Tensor Programming (they rolled the Rust blockchain tutorial this is based on)
- Cryptoeconomics.Study forum (so far 0 engagement)
- r/ethereum
- r/cryptoeconomics
- TWIE newsletter
- other crypto newsletters


<br>

# mdBook and External Libraries
- If you include any files that call external crates (rand) in the SUMMARY.md the mdBook build fails.

Asked on the Rust Forum and apparently there's a few workarounds:
- https://users.rust-lang.org/t/diy-digital-signatures/23739/5
- https://github.com/burrrata/cryptoeconomics.study_rust_edition/blob/master/.travis.yml
- https://github.com/burrrata/cryptoeconomics.study_rust_edition/tree/master/target/release
- https://rust-lang-nursery.github.io/mdBook/cli/test.html#a--library-path

How did The Rust Cookbook do it?
- https://github.com/rust-lang-nursery/rust-cookbook
- they have a Cargo.toml with dependancies: https://github.com/rust-lang-nursery/rust-cookbook/blob/master/Cargo.toml
  - is that enough?
- their book.toml is standard: https://github.com/rust-lang-nursery/rust-cookbook/blob/master/book.toml
- their travis.yml is also standard: https://github.com/rust-lang-nursery/rust-cookbook/blob/master/.travis.yml

mdBook says nothing about adding depdancies to external crates
- https://rust-lang-nursery.github.io/mdBook/index.html
- https://github.com/rust-lang-nursery/mdBook

<br>

# Goals
- For education purposes build a fully functional (but not secure) blockchain using standard Rust code so that the main concepts can be understood as simply as possible in mdBook or the Rust Playground.
- Maybe create a CLI tutorial that shows how to iteratively ugrade each component and function in the standard model to make it more secure
- Stetch Goal: work towards recreating the Parity Ethereum Client and/or arbitrary blockchain architectures like Substrate, but that's probably not realistic lol
- Add storylines to explain the code and concepts.

<br>

# Why this book?

Wanted to build something that runs in the [Rust Playground](https://play.rust-lang.org) and [mdBook](https://rust-lang-nursery.github.io/mdBook/index.html). This means no external Ethereum or crypto libraries. The goal is to explain the core concepts as simply as possible with working Rust code.
- Cryptoeconomics.Study code is written in JS and references external Ethereum libraries
- there's no functionality for accounts, keys, or tx signatures in the Rust tutorial

So here we are!
