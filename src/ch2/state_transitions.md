<br>

<div align="center">
    <p align="center">
        <img src="state_transitions.jpg">
    </p>
    <h1 align="center">
        State Transitions
    </h1>
    <p align="center">
        The proof is in the pudding.
    </p>
</div>

<br><br><br>

Goal
- Upgrade our state transition function from a consensus model of "because the central operator says so" to "because someone did the work and proved that they earned the right to make these changes for the community".

Core Concepts:
- pending_tx pool => blocks (nonces, block headers, etc...)
- ledger history => hashed merkle tries
- anti-spam protection with proof of work on each block

Open Questions
- do people generally refer to the "consensus model" and the "state transition function" as the same thing, or are they different, or are they mostly the same but slightly different?
- How do block nonces differ from TX nonces?


### Idea To Add
- because accounts are not in a centralized database the user controls the account, and as long as the network is operational, the user's account is too. Just like if your bank went under (https://en.wikipedia.org/wiki/Lehman_Brothers), your account would be gone, if the P2P network you're on goes under same deal. 

### Block:
A block is a package of data that contains zero or more transactions, the hash of the previous block ("parent"), and optionally other data. Because each block (except for the initial "genesis block") points to the previous block, the data structure that they form is called a "blockchain".

### Proof of work:
One important property of a block in Bitcoin, Ethereum and many other crypto-ledgers is that the hash of the block must be smaller than some target value. The reason this is necessary is that in a decentralized system anyone can produce blocks, so in order to prevent the network from being flooded with blocks, and to provide a way of measuring how much consensus there is behind a particular version of the blockchain, it must in some way be hard to produce a block. Because hashes are pseudorandom, finding a block whose hash is less than 0000000100000000000000000000000000000000000000000000000000000000 takes an average of 4.3 billion attempts. In all such systems, the target value self-adjusts so that on average one node in the network finds a block every N minutes (eg. N = 10 for Bitcoin and 1 for Ethereum).

### Proof of work nonce:
A meaningless value in a block which can be adjusted in order to try to satisfy the proof of work condition

### Mining:
Mining is the process of repeatedly aggregating transactions, constructing a block and trying different nonces until a nonce is found that satisfies the proof of work condition. If a miner gets lucky and produces a valid block, they are granted a certain number of coins as a reward as well as all of the transaction fees in the block, and all miners start trying to create a new block containing the hash of the newly generated block as their parent.

### Stale:
A stale is a block that is created when there is already another block with the same parent out there; stales typically get discarded and are wasted effort.

### Fork:
A situation where two blocks are generated pointing to the same block as their parent, and some portion of miners see one block first and some see the other. This may lead to two blockchains growing at the same time. Generally, it is mathematically near-certain that a fork will resolve itself within four blocks as miners on one chain will eventually get lucky and that chain will grow longer and all miners switch to it; however, forks may last longer if miners disagree on whether or not a particular block is valid.

<br><br><br>

## Videos

[![Cryptoeconomics - 1.1 - Hashes and Signatures](https://img.youtube.com/vi/FLIo_ZjV--U/0.jpg)](https://www.youtube.com/watch?v=FLIo_ZjV--U)

<p>
    <a href="https://cryptoeconomics.study/lectures/chapter-01-1.html">Cryptoeconomics - 1.1 - Hashes and Signatures</a>.
</p>

<br><br><br>

Code Resources:
- https://github.com/cryptoeconomics-study/code/blob/master/c3_ProofOfWork/proofOfWork.js
- https://github.com/tensor-programming/Rust_block_chain/blob/master/src/blockchain.rs

```rust, ignore
// https://github.com/tensor-programming/Rust_block_chain/blob/master/src/blockchain.rs

impl Chain {

    pub fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                },
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            };
        }
    }
}

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;


struct MyStruct {
    id: u8,
    data: String,
}

#[derive(Debug, Clone)]
struct TX {
    sender: String,
    receiver: String,
    tx_amount: f32,
    nonce: i32,
}

#[derive(Debug, Clone, Hash)]
struct AltTX {
    sender: String,
    receiver: String,
    tx_amount: i32,
    nonce: i32,
}


unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

fn hash(stuff: &[u8]) -> String {
    
    let mut hasher = DefaultHasher::new();
    hasher.write(stuff);
    let digest = hasher.finish();
    let hex_digest = format!("{:#X}", digest);
        
    hex_digest
}


fn main() {

    // Using Example Struct
    let my_struct = MyStruct {
        id: 98,
        data: "Hello World".to_string(),
    };
    
    let bytes: &[u8] = unsafe { 
        any_as_u8_slice(&my_struct)
    };

    println!("{:?}", &bytes);
    
    
    // Using TX Struct
    let tx = TX {
        sender: "Your Mom".to_string(),
        receiver: "Yours truly".to_string(),
        tx_amount: 1000.0,
        nonce: 345,
    };
    
    let tx_bytes: &[u8] = unsafe {
        any_as_u8_slice(&tx)
    };
    println!("tx: {:?}", &tx_bytes);
    
    let tx_hash = hash(tx_bytes);
    println!("tx hash: {:#?}", tx_hash);
}
```

<br><br><br>

### But wait... there's more

Hashing and Merkle Trees
- https://en.wikipedia.org/wiki/Merkle_tree
- https://blog.ethereum.org/2015/11/15/merkling-in-ethereum/
- https://ethereum.stackexchange.com/questions/2100/what-is-a-block-hash
- Merklize this! Merkle Trees & Patricia Tries: https://www.zeroknowledge.fm/57

PoW & Blocks
- https://github.com/ethereum/wiki/wiki/White-Paper#blockchain-and-mining
- https://ethereum.stackexchange.com/questions/5833/why-do-we-need-both-nonce-and-mixhash-values-in-a-block

Terms
- https://github.com/ethereum/wiki/wiki/Glossary

<br>
