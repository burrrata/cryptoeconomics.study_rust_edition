# Ch1: Centralized Payment Processors
They're not as boring when you roll your own! 
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


### TODO

General:
- would it make sense to have a separate module for data serialization (turning stuff into &[u8] bytes so that RSA and the hash function can operate on them?)
- then you could have an upgrade path to Ethereum's RLP: https://github.com/ethereum/wiki/wiki/RLP

UX: make it useable so people can actually learn stuff or provide feedback
- build out individual sections as a tutorial leading up to the full demo
- add relevant resources to each section
- improve comments in the centralize_payment_processor finale to better explain what's going on for those who jump ahead

Feedback: making it actually usable
- update Cryptoeconomics.Study post
- hit up the Tensor Programming guy who rolled the template Rust blockchain tutorial
- share to /r/ethereum
