<br>

<div align="center">
    <p align="center">
        <img src="TBD">
    </p>
    <h1 align="center">
        State Transitions Chapter Sketch
    </h1>
    <p align="center">
        Exploring architectures that will allow the user to swap PoW for PoS for PoA for whatever they want (aka pluggable consensus mechanisms). 
    </p>
</div>

<br><br><br>

This page is a sketch for a chapter overview. Individual sections will be needed to explore and build each model as well as it's core concepts. 

The general blockchain architecture will remain the same, but the state transition function will change. The STF will:
- define rules of what makes a valid state transitions,
- verify that state transition requests (tx) conform to those rules,
- and if so then process those changes and produce a new state. 

Core Concepts:
- centralized database (CDM)
- consortium database (PoA)
- decentralized database (PoW, PoS, dPoS)

<br><br><br>

Videos
- Cryptoeconomics.Study
- aantonop
- other

<br><br><br>

```rust, ignore
// code will be in each section,  building the STF and then plugging it into a generic blockchain architecture
```

<br><br><br>

### Resources
- https://en.wikipedia.org/wiki/Peer-to-peer
- https://en.wikipedia.org/wiki/Decentralization
- https://medium.com/@VitalikButerin/the-meaning-of-decentralization-a0c92b76a274
- https://en.wikipedia.org/wiki/Distributed_ledger
- https://en.wikipedia.org/wiki/Byzantine_fault_tolerance

<br><br><br>
