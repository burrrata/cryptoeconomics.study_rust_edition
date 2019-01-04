# A Placeholder for SUMMARY.md

Because if you include any files that call external crates (rand) in the SUMMARY.md the mdBook build fails.

Soution TBD  ¯\_(ツ)_/¯ 

### How did The Rust Cookbook do it?
- https://github.com/rust-lang-nursery/rust-cookbook
- they have a Cargo.toml with dependancies: https://github.com/rust-lang-nursery/rust-cookbook/blob/master/Cargo.toml
  - is that enough?
- their book.toml is standard: https://github.com/rust-lang-nursery/rust-cookbook/blob/master/book.toml
- their travis.yml is also standard: https://github.com/rust-lang-nursery/rust-cookbook/blob/master/.travis.yml

mdBook says nothing about adding depdancies to external crates
- https://rust-lang-nursery.github.io/mdBook/index.html
- https://github.com/rust-lang-nursery/mdBook

Tried adding a Cargo.toml and running `cargo build --release` and pushing that to the repo, but Travis still failed. Not sure what else to try at this point...

# Summary

[Intro](./intro.md)

[Pregame w Crypto Wars](./crypto_wars.md)

[Centralized Payment Processor](./ch1/intro.md)
  - [State](./ch1/state.md): all the things
  - [Randomness](./ch1/randomness.md): and why it matters
  - [Public Key Crypto](./ch1/public_key_crypto.md): not just an account/password system
  - [Digital Signatures](./ch1/digital_signatures.md): the proof is in the pudding
  - [Accounts](./ch1/accounts.md): combining these things into something useful!
  - [TX](./ch1/tx.md): using your account to do stuff!
  - [Hashing](./ch1/hashing.md): deterministically making data compact
  - [Blocks](./ch1/blocks.md): how you know what happened
  - [Centralize Payment Processor](./ch1/centralized_payment_processor.md): putting it all together into a working example
  - [Inspo](./ch1/inspo.md): standing on the shoulders of giants

[Resources](./resources.md)