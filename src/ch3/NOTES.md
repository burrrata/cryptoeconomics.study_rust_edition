# Minimal Viable PoS

<br>

### Things We'll Need

A source of randomness
  - collective coin flipping
  - randomness beacon
  - Rust Rand thread_rng() until we have networking?
  
A slashing/rewarding mechanism
  - reward if state transition is valid/uncontested
  - slash of stake if state transition is flawed

A mechanism that chooses nodes to produce the next state transition
  - coin age
  - purely random selection
  - etc...

A way to check that any proposed state transitions are valid
  - proof of proposing node's proof of stake
  - proof of proposing node's eligibility to produce the next state transition
  - proof that state transition only includes valid tx

<br>

### Building Blocks

Roll your own PoS blockchain! (Go)
- https://github.com/mycoralhealth/blockchain-tutorial
- https://medium.com/@mycoralhealth/code-your-own-proof-of-stake-blockchain-in-go-610cd99aa658

Vlad had some python sketches drawn up for some of his ideas right?
- tbd

99% Fault Tolerant Consensus
- https://vitalik.ca/general/2018/08/07/99_fault_tolerant.html
- https://www.youtube.com/watch?v=l0AQ0UJAvM8

Pyethereum Serenity Draft (very extra, much sharding)
- https://github.com/ethereum/pyethereum/tree/serenity/

EIP 1011 (lots of extra stuff we don't need yet)
- https://github.com/ethereum/EIPs/blob/master/EIPS/eip-1011.md

ethresear.ch (useful, but not for MVPoS)
- https://ethresear.ch/c/casper

Substrate Staking (very extra)
- https://github.com/paritytech/substrate/tree/master/srml/staking/src

Casper FFG Testnet (very web3.js, not very general)
- https://hackmd.io/s/Hk6UiFU7z

Sharding (probably a little extra for a MVPoS draft)
- https://github.com/ethereum/wiki/wiki/Sharding-roadmap
