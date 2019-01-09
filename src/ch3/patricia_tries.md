# Patricia Tries
Perhaps the most important data structure in Ethereum. 

## Words

### Patricia tree (or trie):
A data structure which stores the state of every account. The trie is built by starting from each individual node, then splitting the nodes into groups of up to 16 and hashing each group, then making hashes of hashes and so forth until there is one final "root hash" for the entire trie. The trie has the important properties that: 

1. there is exactly one possible trie and therefore one possible root hash for each set of data,
2. it is very easy to update, add or remove nodes in the trie and generate the new root hash,
3. there is no way to modify any part of the tree without changing the root hash, so if the root hash is included in a signed document or a valid block the signature or proof of work secures the entire tree,
4. one can provide just the "branch" of a tree going down to a particular node as cryptographic proof that that node is indeed in the tree with that exact content. 

Patricia trees are also used to store the internal storage of accounts as well as transactions and ommers. See [here](https://easythereentropy.wordpress.com/2014/06/04/understanding-the-ethereum-trie/) for a more detailed description.

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
- https://easythereentropy.wordpress.com/2014/06/04/understanding-the-ethereum-trie/
- https://github.com/ethereum/wiki/wiki/Glossary
- Merklize this! Merkle Trees & Patricia Tries: https://www.zeroknowledge.fm/57
