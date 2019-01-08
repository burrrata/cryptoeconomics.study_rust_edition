<h1 align="center">
    PoW: the proof is in the pudding.
</h1>

<br>

## Words

This is where we'll talk about upgrading our state transition function from a consensus model of "because the central operator says so" to "because someone did the work and proved that they earned the right to make these changes for the community".

?
- do people generally refer to the "consensus model" and the "state transition function" as the same thing, or are they different, or are they mostly the same but slightly different?

<br>

## Videos

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
```

<br>

## Resources

<br>
