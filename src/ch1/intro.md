# Chapter 1: centralized payment processors
They're not as boring when you roll your own! 
  - [State](./ch1/state.md): all the things
  - [Randomness](.ch1/randomness.md): and why it matters
  - [Public Key Crypto](.ch1/public_key_crypto.md): not just an account/password system
  - [Digital Signatures](.ch1/digital_signatures.md): the proof is in the pudding
  - [Accounts](.ch1/accounts.md): combining these things into something useful!
  - [TX](.ch1/tx.md): using your account to do stuff!
  - [Hashing](.ch1/hashing.md) (and Merkle Trees/Tries): keeping track of what happened
  - [Centralize Payment Processor](.ch1/centralized_payment_processor): putting it all together into a working example
  - [Inspo](.ch1/inspo.md): standing on the shoulders of giants

<br>

# GOALS
- For education purposes build a fully functional (but not secure) blockchain using standard Rust code so that the main concepts can be understood as simply as possible in mdBook or the Rust Playground.
- Then create a CLI tutorial that shows how to iteratively ugrade each component and function in the standard model to make it more secure
- Stetch Goal: work towards recreating the Parity Ethereum Client and/or arbitrary blockchain architectures like Substrate, but that's probably not realistic lol

Each chapter in the book can then evolve like this:
- goal/thing to build a toy version of (w links examples of the thing IRL)
- each page explains a part (w links to wikipedia article and production library version of the tutorial example)
- summary putting it all together (editable code you can run and play with)

Bonus points if there's a story to explain the code and concepts.
