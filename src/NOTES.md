# Flow

### CPP State

Accounts
- id: hash of randomized numbers 
- password: another set of randomized numbers
- balance: u8
- nonce: u8

tx
- check that account exists
- check account is not frozen
- check account balance is > tx amount
- check tx nonce = account nonce to prevent double spend glitches/hacks

history
- hashmap of processed tx
- function to search hashmap for an account (key) and return associated tx (value)

### P2P State
- agreed upon pub key crypto params
- accounts
- tx & signatures
- blocks and merkle tries 
- PoW

Better P2P Network
- ECDSA
- patricia tries
- RSL data encoding
- PoS

L2: state channels

L2: plasma

Cryptoeconomic Applications
- So... if we want to do interesting things with cryptoecnomics... we need a scripting language, and that means replicating the EVM... and that's a lot...
- Would it make more sense to dive into solidity at this point?

<br>

# TODO

UX: make it useable so people can actually learn stuff or provide feedback
- build out individual sections as a tutorial leading up to the full demo
- add relevant resources to each section
- improve comments in the centralize_payment_processor finale to better explain what's going on for those who jump ahead

Feedback: making it actually usable
- update Cryptoeconomics.Study post
- hit up the Tensor Programming guy who rolled the template Rust blockchain tutorial
- share to /r/ethereum


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

### Chapter Framework:
- goal/thing to build a toy version of (w links examples of the thing IRL)
- each page explains a part (w links to wikipedia article and production library version of the tutorial example)
- summary putting it all together (editable code you can run and play with)

Bonus points if there's a story to explain the code and concepts.

### Why this book?

Wanted to build something that runs in the [Rust Playground](https://play.rust-lang.org) and [mdBook](https://rust-lang-nursery.github.io/mdBook/index.html). This means no external Ethereum or crypto libraries. The goal is to explain the core concepts as simply as possible with working Rust code.
- Cryptoeconomics.Study code is written in JS and references external Ethereum libraries
- there's no functionality for accounts, keys, or tx signatures in the Rust tutorial

So here we are!
