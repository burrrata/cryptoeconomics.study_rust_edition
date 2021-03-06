<h1 align="center">
    NOTES
</h1>
<br><br><br>

1! MVPoS

2! Networking/Threading

- https://users.rust-lang.org/t/fearless-shared-state-concurrency/24514/4

3! Testnet

<br>

THEN explore crypto upgrades towards specific architectures like Bitcoin, Ethereum, etc...
- and then add references and explanations along with each

<br><br><br>

Blockchains are databases. Glorious, beautiful, databases.

Like any database, their current state is called the state.

Who gets to change this state and under what circumstances is defined in the state transition function.

Who is on the database is defined by the account creation system.

And whether or not the database has been tampered with is verified via the history.

:)

<br><br><br>


# Better Explanations & References

These are ALL full of amazing references, and they need to be used as inspiration for the chapter descriptions and linked to as references if someone wants to learn more
- aantonop videos
- Mastering Bitcoin
- Mastering Ethereum
- https://github.com/jpantunes/awesome-cryptoeconomics
- https://github.com/L4ventures/awesome-cryptoeconomics
- Bitcoin Whitepaper
- Ethereum White/Beige/Yellow papers

Explore New Resources
- everything here: https://www.reddit.com/r/Bitcoin/comments/ag03jf/understanding_bitcoin/
- also everything here: https://www.reddit.com/r/Bitcoin/comments/afxg6t/what_impacted_your_learning_experience_the_most/

Code Tutorials
- https://github.com/justinmoon/digital-cash
- BUIDL Bootcamp: https://www.youtube.com/playlist?list=PLQ56Yiu6lEaxIPm9-GB5M393CmtYRZFGY

<br><br><br>

# Current Book: CDM, PoW, PoS

TODO
- clean up chapters 1 & 2
- build chapter 3 code
- THEN go back and do the sections

General Template
- create intro that explains the topic/goal
- create code summary shows what we're doing
- clean up code so that it's readable/usable
- copy code sections into book sections and add comments
    - note that code comments are for the sections, but less for the final chapter summary
- then add resources and links to production code examples

<br><br><br>

# Upgrading The Code

Less Cloning, More Borrowing: 
- update code to use borrows and lifetimes rather than cloning everything.

Lifetimes
- It would be great to learn more about lifetimes so that every function can take in a reference rather than requiring a String that has to be cloned over and over and over...

Errors
- It would also be nice to have better errors so that when functions fail we (or the users) know why.

Enums
- enums and how they interact with structs and implementations

Pub/Priv Functions
- pub vs priv functions

Structs and Traits
- they're useful, but I'm not using them

External crates in mdBook:
- https://users.rust-lang.org/t/diy-digital-signatures/23739/5

<br><br><br>

# UX / UI
- reformat the Resources sections to look like Awesome Lists, and then add relevant stuff to the relevant lists
- rename all pictures to be the same name as the section they're featured in
- pictures for each chapter intro
- ayu as default theme
- better feedback flow
- encourage PRs so people can upgrade code or propose alternative versions of functions (ideally leading to modular systems people can play with)

<br><br><br>

# Feedback

There needs to be a clear and easy way for people to provide feedback.
- email: rustycryptoeconomics@protonmail.com
- forum? 
- reddit?

People to reach out to once the first draft is ready
- Tensor Programming (they rolled the Rust blockchain tutorial this is based on)
- Cryptoeconomics.Study forum (so far 0 engagement)
- r/ethereum
- r/cryptoeconomics
- TWIE newsletter
- other crypto newsletters

<br><br><br>

# Networking

Real
- this means a CLI application rather than a browser sandbox

Simulated
- Build in concurrent threads to simulate network activity and forks rather than having everything in a main() function
- https://doc.rust-lang.org/book/ch16-00-concurrency.html

<br><br><br>

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

# Ideas From C.S
- https://github.com/K-Ho/code/tree/networkVisualization
- https://docs.google.com/document/d/1R85zczC1-nklLXEFx-dZfQdlZexRAk8S9G9Hc3Zjkx4/edit

### PoW "Blockchain"

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

### PoS "Blockchain"

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

<br>
