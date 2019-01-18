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

EIP 1011
- https://github.com/ethereum/EIPs/blob/master/EIPS/eip-1011.md

Vlad had some python sketches drawn up for some of his ideas right?
- tbd

ethresear.ch
- https://ethresear.ch/c/casper

Substrate Staking
- https://github.com/paritytech/substrate/tree/master/srml/staking/src

99% Fault Tolerant Consensus
- https://vitalik.ca/general/2018/08/07/99_fault_tolerant.html
- https://www.youtube.com/watch?v=l0AQ0UJAvM8

Casper FFG Testnet (PoS)
- https://hackmd.io/s/Hk6UiFU7z

Sharding
- https://github.com/ethereum/wiki/wiki/Sharding-roadmap

Pyethereum Serenity Draft
- https://github.com/ethereum/pyethereum/tree/serenity/
