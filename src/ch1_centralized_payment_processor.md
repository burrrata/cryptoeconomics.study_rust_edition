# Centralized Payment Processors
What are they and how do they work? Let's roll our own to find out! 

<br>

# GOALS
- For education purposes build a fully functional (but not secure) blockchain using standard Rust code so that the main concepts can be understood as simply as possible in mdBook or the Rust Playground.
- Then create a CLI tutorial that shows how to iteratively ugrade each component and function in the standard model to make it more secure
- Stetch Goal: work towards recreating the Parity Ethereum Client, but that's probably not realistic lol

# TODO
- remix the Javascript as editable, runnable, Rust code
- move the english words into comments in the code
- bonus points if you can craft a story to explain the code and concepts

<br>

## Lectures
- https://cryptoeconomics.study/lectures/chapter-01-0.html

QUESTION: how do I embed YouTube videos in mdBook so they play here rather than taking the user away from the page to YouTube?
- https://users.rust-lang.org/t/embedding-youtube-in-mdbook/23555/3

[![chapter_1.0](https://img.youtube.com/vi/VaUTTE5xb54/hqdefault.jpg)](https://youtu.be/VaUTTE5xb54)

[![chapter_1.1](https://img.youtube.com/vi/FLIo_ZjV--U/hqdefault.jpg)](https://youtu.be/FLIo_ZjV--U)

[![chapter_1.2](https://img.youtube.com/vi/XIsn8-5Xekc/hqdefault.jpg)](https://youtu.be/XIsn8-5Xekc)

[![chapter_1.3](https://img.youtube.com/vi/j7Mbx8laZwY/hqdefault.jpg)](https://youtu.be/j7Mbx8laZwY)

[![chapter_1.4](https://img.youtube.com/vi/-xoCoZGJ9AQ/hqdefault.jpg)](https://youtu.be/-xoCoZGJ9AQ)

[![chapter_1.5](https://img.youtube.com/vi/ckzi8iqGilE/hqdefault.jpg)](https://youtu.be/ckzi8iqGilE)

## Words
- https://github.com/cryptoeconomics-study/website/blob/master/overview.md

* Overview of basic crypto concepts
   * Hashes -- not how they work but instead what they do (collision resistance, second preimage resistance [given x and H(x), cannot find y such that H(y) = H(x)], preimage resistance, random oracle)
   * Public / private keys -- Sign with private key, verify with public key
* Overview of PayPal structure
   * State object which is simply a mapping of address->balance
   * Two types of state mutations: 1) `mint` and 2) `send` -- each with their own logic
   * We start with an initial state (ie `genesis block`) and apply transactions (ie `blocks`) to get our most recent state.
      * The list of transactions (blocks) is called the “history”
      * The result of computing all transactions is called the “state”
      * Note: In Ethereum the full history is approx 250 GB, while the state is approx 3 GB.
         * Fun aside: Rent proposals say that people should pay rent on the state storage which they take up. There is no direct incentive to store the history, and so nodes today often do prune or delete historical data. If this happens too much there is a risk that we can’t recreate the chain anymore!
   * Use nonces to provide replay protection. (nonce means you can’t resubmit the same transaction multiple times)
   * Code implementation: https://codepen.io/karlfloersch/pen/YaEoYy?editors=0012 
* Account model vs UTXO model
   * Briefly cover the differences
   * Account model (what we are using for our implementation):
      * A global mapping of `account->balance`
      * Sends reduce one account’s balance and increase another account's balance 
   * UTXO model (unspent transaction output model used in Bitcoin)
      * Same as the account model, however with three added rules:
         * 1) Every send must include the entire account balance.
         * 2) Sends can specify multiple recipients.
         * 3) Sends can originate from multiple senders.
   * Supposed to be privacy preserving, but these days the privacy can be broken. Only purely private way to send is zero knowledge proofs.
* Properties of centralized systems
   * Benefits:
      * Easy to build and reason about.
      * Simple to scale.
      * Privacy preserving. (if you trust the operator)
   * Downsides:
      * Single point of failure
         * If the operator is removed (eg. servers burn down, servers seized by authorities), the entire system breaks.
      * Censorship
         * The operator can censor users and change their balances, and it is very difficult for users to prove malfeasance.
            * This is because there is no client-side validation
      * Fraud
         * Because the operator has complete control, they can steal money directly from users.
         * The only safeguard against this kind of misbehavior is the legal system & social reputation.
            * Even these threats are not enough--see Bitconnect, Mt. Gox, and many other exchanges which have been hacked.
            * Also, theft is often unprovable
   * These downsides limit what can be built on top of these systems.
      * Clearly no illegal securities!
* Let’s decentralize :)

<br>

## Code

MUST run in the [Rust Playground](https://play.rust-lang.org) and [mdBook](https://rust-lang-nursery.github.io/mdBook/index.html). This means no external Ethereum or crypto libraries. The goal is to explain the core concepts as simply as possible with working Rust code. THEN explain how the user can upgrade the toy functions in order to move towards something like [Parity's Ethereum Client](https://github.com/paritytech/parity-ethereum).

Here's Karl's awesome code:
- https://github.com/cryptoeconomics-study/code/blob/master/c1_CentralPaymentOperator/paypalWithSigs.js
- https://codepen.io/karlfloersch/pen/YaEoYy?editors=0012

Here's an awesome Rust blockchain tutorial:
- https://steemit.com/technology/@tensor/rust-project-cli-toy-blockchain
- https://github.com/tensor-programming/Rust_block_chain

These are GREAT, but Cryptoeconomics.Study code is written in JS and references external Ethereum libraries, and there's no functionality for accounts or tx signatures in the Rust tutorial. 

Attempting to get the best of both worlds here: [current.md](current.md)
