# Centralized Payment Processors
What are they and how do they work? Let's roll our own to find out! 

<br>

# TODO
- remix the Javascript as editable, runnable, Rust code
- move the english words into comments in the code
- bonus points if you can craft a story to explain the code and concepts

<br>

## Lectures
- https://cryptoeconomics.study/lectures/chapter-01-0.html

```How do I embed YouTube videos into mdBooks?```
- shots fired: https://users.rust-lang.org/t/embedding-youtube-in-mdbook/23555/3

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
- https://github.com/cryptoeconomics-study/code/blob/master/c1_CentralPaymentOperator/paypalWithSigs.js
- https://codepen.io/karlfloersch/pen/YaEoYy?editors=0012
```
var EthCrypto = require('eth-crypto')

var initialState = {}

/* Signed transaction format
tx = {
  contents: {
    type: string,  // either 'mint' or 'send'
    amount: int,   // some quantity of coins
    from: string,  // the address of the sender
    to: string,    // the address of the recipient
  },
  sig: string      // the signature of the sender
}
*/

var accounts = {
  'paypal': EthCrypto.createIdentity(),
  'aparna': EthCrypto.createIdentity(),
  'jing': EthCrypto.createIdentity()
}

var unsignedTxs = [
  {
    type: 'mint',
    amount: 100,
    from: accounts.paypal.address,
    to: accounts.paypal.address,
    nonce: 0
  },
  {
    type: 'send',
    amount: 65,
    from: accounts.paypal.address,
    to: accounts.aparna.address,
    nonce: 1
  },
  {
    type: 'send',
    amount: 10,
    from: accounts.aparna.address,
    to: accounts.jing.address,
    nonce: 0
  }
]

function getTxHash (tx) {
  return EthCrypto.hash.keccak256(JSON.stringify(tx))
}

var signedTxs = [
  {
    contents: unsignedTxs[0],
    sig: EthCrypto.sign(accounts.paypal.privateKey, getTxHash(unsignedTxs[0]))
  },
  {
    contents: unsignedTxs[1],
    sig: EthCrypto.sign(accounts.paypal.privateKey, getTxHash(unsignedTxs[1]))
  },
  {
    contents: unsignedTxs[2],
    sig: EthCrypto.sign(accounts.aparna.privateKey, getTxHash(unsignedTxs[2]))
  }
]

function applyTransaction (state, tx) {
  // Check the from address matches the signature
  const signer = EthCrypto.recover(tx.sig, getTxHash(tx.contents))
  if (signer !== tx.contents.from) {
    throw new Error('Invalid signature!')
  }
  // If we don't have a record for this address, create one
  if (!(tx.contents.to in state)) {
    state[[tx.contents.to]] = {
      balance: 0,
      nonce: 0
    }
  }
  // Check that the nonce is correct for replay protection
  if (tx.contents.nonce !== state[[tx.contents.from]].nonce) {
    throw new Error('Invalid nonce!')
  }
  // Mint coins **only if identity is PayPal**
  if (tx.contents.type === 'mint' && tx.contents.from === accounts.paypal.address) {
    state[[tx.contents.to]].balance += tx.contents.amount
  } else if (tx.contents.type === 'send') { // Send coins
    if (state[[tx.contents.from]].balance - tx.contents.amount < 0) {
      throw new Error('Not enough money!')
    }
    state[[tx.contents.from]].balance -= tx.contents.amount
    state[[tx.contents.to]].balance += tx.contents.amount
  }
  state[[tx.contents.from]].nonce += 1
  return state
}

// Apply all transactions and print out all intermediate state
let state = initialState
for (let i = 0; i < signedTxs.length; i++) {
  state = applyTransaction(state, signedTxs[i])
  console.log(('State at time ' + i), state)
}

// Just for fun, let's try signing aparna's transaction with jing's privatekey and see if we catch it
const invalidSigTx = {
  contents: unsignedTxs[2], // aparna sending jing 10
  sig: EthCrypto.sign(accounts.jing.privateKey, getTxHash(unsignedTxs[2]))
}

try {
  applyTransaction(state, invalidSigTx)
} catch (err) {
  console.log('We caught the error!', err)
}

// Now let's try replaying a tx and see if we catch it
try {
  applyTransaction(state, signedTxs[2])
} catch (err) {
  console.log('We caught the error!', err)
}
// Woot!
console.log('Success!')
```
