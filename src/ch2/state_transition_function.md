<h1 align="center">
    State Transitions: the proof is in the pudding.
</h1>

<br>

## Words

This is where we'll talk about upgrading our state transition function from a consensus model of "because the central operator says so" to "because someone did the work and proved that they earned the right to make these changes for the community".

?
- do people generally refer to the "consensus model" and the "state transition function" as the same thing, or are they different, or are they mostly the same but slightly different?

Core Concepts:
- pending_tx pool => blocks
- ledger history => hashed merkle tries

Resources:
- https://github.com/cryptoeconomics-study/code/blob/master/c3_ProofOfWork/proofOfWork.js
- https://github.com/tensor-programming/Rust_block_chain/blob/master/src/blockchain.rs





### Blocks, block headers, and a merkle tree of the history.

Yo what's up with those nonces?
- are they to keep track of block numbers?
- or are they for something else?

The Ethereum Whitepaper has a lot of dope knowledge on how blocks are processed and the state transition function rolls the chain. Would be nice to include that.
- https://github.com/ethereum/wiki/wiki/White-Paper#blockchain-and-mining

Note: there will be no block rewards in this demo because a centralized payment processor only needs blocks
to keep track of history, but does not need to incentivize miners to expend resources to secure the network.
It's a "blockchain" style database not a decentralize P2P blockchain, but that's for ch2!




<br>

## Videos



[![Cryptoeconomics - 1.1 - Hashes and Signatures](https://img.youtube.com/vi/FLIo_ZjV--U/0.jpg)](https://www.youtube.com/watch?v=FLIo_ZjV--U)

<p>
    <a href="https://cryptoeconomics.study/lectures/chapter-01-1.html">Cryptoeconomics - 1.1 - Hashes and Signatures</a>.
</p>

<br>

## Code

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

<br>

## Resources

Hashing and Merkle Trees
- https://en.wikipedia.org/wiki/Merkle_tree
- https://blog.ethereum.org/2015/11/15/merkling-in-ethereum/
- https://ethereum.stackexchange.com/questions/2100/what-is-a-block-hash

PoW
- TBD

<br>
