# Blocks

This is where we'll create blocks, block headers, and a merkle tree of the history.

Yo what's up with those nonces?
- are they to keep track of block numbers?
- or are they for something else?

The Ethereum Whitepaper has a lot of dope knowledge on how blocks are processed and the state transition function rolls the chain. Would be nice to include that.
- https://github.com/ethereum/wiki/wiki/White-Paper#blockchain-and-mining

Note: there will be no block rewards in this demo because a centralized payment processor only needs blocks
to keep track of history, but does not need to incentivize miners to expend resources to secure the network.
It's a "blockchain" style database not a decentralize P2P blockchain, but that's for ch2!

```rust
// from: https://github.com/tensor-programming/Rust_block_chain/blob/master/src/blockchain.rs

impl State {

    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty
        };

        let reward_trans = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_addr.clone(),
            amount: self.reward
        };

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![]
        };

        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.curr_trans);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);

        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

    fn get_merkle(curr_trans: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for t in &curr_trans {
            let hash = Chain::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }
        merkle.pop().unwrap()
    }
}
```
