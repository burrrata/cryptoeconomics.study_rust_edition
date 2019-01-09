# RLP
Turning stuff into other stuff.

## Words
The purpose of RLP (Recursive Length Prefix) is to encode arbitrarily nested arrays of binary data, and RLP is the main encoding method used to serialize objects in Ethereum. The only purpose of RLP is to encode structure; encoding specific data types (eg. strings, floats) is left up to higher-order protocols; but positive RLP integers must be represented in big endian binary form with no leading zeroes (thus making the integer value zero be equivalent to the empty byte array). Deserialised positive integers with leading zeroes must be treated as invalid. The integer representation of string length must also be encoded this way, as well as integers in the payload. Additional information can be found in the Ethereum yellow paper Appendix B.
- from the wiki: https://github.com/ethereum/wiki/wiki/RLP


## Code
```rust, ignore
TBD
```


## Resources
- https://github.com/ethereum/wiki/wiki/RLP
- https://github.com/jnnk/pyrlp/blob/master/docs/tutorial.rst
- https://github.com/paritytech/parity-common/tree/master/rlp
