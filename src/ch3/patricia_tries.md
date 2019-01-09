# Patricia Tries
Perhaps the most important data structure in Ethereum. 

## Words

"Perhaps the most important data structure in Ethereum is the Patricia tree. The Patricia tree is a data structure that, like the standard binary Merkle tree, allows any piece of data inside the trie to be securely authenticated against a root hash using a logarithmically sized (ie. relatively short) hash chain, but also has the important property that data can be added, removed or modified in the tree extremely quickly, only making a small number of changes to the entire structure. The trie is used in Ethereum to store transactions, receipts, accounts and particularly importantly the storage of each account."
- source: https://blog.ethereum.org/2015/07/05/on-abstraction/

## Code
```rust, ignore
// TBD
```

## Resources
- https://blog.ethereum.org/2015/07/05/on-abstraction/
- https://github.com/ethereum/wiki/wiki/Patricia-Tree
- https://github.com/paritytech/parity-common/tree/master/patricia_trie
