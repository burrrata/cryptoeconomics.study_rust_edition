<br>

<div align="center">
    <p align="center">
        <img src="TBD">
    </p>
    <h1 align="center">
        Attacks Chapter Sketch
    </h1>
    <p align="center">
        Exploring attacks on various cryptoeconomic systems (applications or core blockchain protocols).
    </p>
</div>

<br><br><br>

# Goal
Each section will explore attack as well as any defenses or architectures that can prevent it. Users will be able to see how various design decisions create trade-offs between security, livliness, and functionality. At the end of the chapter will be a chart of various designs/architectures, their goals, and what they're vulnerable to. For example:
- PoW: security, but only works if you're the largest chain
- PoS: better carrots and sticks like slashing, but vulnerable to consolidation if a party aquires too much stake

<br><br><br>

### Long Range (Secret Mining) Attack 
Could be reproduced many times using PoW, but only once using PoS
- https://www.youtube.com/watch?v=jUp6LzZxhOg
- https://blog.ethereum.org/2014/05/15/long-range-attacks-the-serious-problem-with-adaptive-proof-of-work/

<br><br><br>

### Ethfinger Attack
Someone acquires a large volume of a token, then creates a dPoS node with a small portion of those tokens which looks like a lot to most people and people join because they think the validator has a lot of skin in the game. To appear honest, the validator makes sure that their stake in the dPoS node is a large part of the total volume, and because it seems trustworthy lots of people join. This becomes a significant portion of the network over time, and then the dPoS validator intentionally makes an error that burns all the stake in the node. This burns a significant portion of the network's funds, but actually increases the attackers total funds relative to the network. For example
- network tokens before attack: 100000
- attacker tokens before attack: 10000 (10% of total network)
- attacker dPoS with 1000 tokens stakes (1% of total network and 10% of attackers total balance)
- people in the network join the dPoS node because it seems secure and it grows to 5000 tokens (5% of network)
- attacker adds an additional 1000 tokens and more people add more because it seems "too big to fail", totaling 10000 tokens (10% of the network) in the dPoS node (2000 from attacker, 8000 from the public)
- this goes on until the node has 30000 tokens (30% of the network), but only 3000 of those are from the attacker.
- the attacker intentionally destroys the dPoS node
- network funds after attack: 70000 tokens
- attacker tokens after attack: 7000 tokens (10% of total network)

Source: 
- [Cryptoeconomic Primitives and Staking](https://www.zeroknowledge.fm/49) Carrots, sticks, and attack vectors for PoS.

Open Question: 
- at what ratio would this be profitable?

<br><br><br>

### Resources
- resources will be included within each example, but links to generic aggregations of stuff can be down here
- https://github.com/jpantunes/awesome-cryptoeconomics
- https://github.com/L4ventures/awesome-cryptoeconomics

<br><br><br>
