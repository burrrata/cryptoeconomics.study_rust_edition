<div align="center">
    <h1 align="center">
        RLP
    </h1>
    <p align="center">
        Turning data into easier to manage data.
    </p>
</div>
<br>

## Words

### Open Questions
- WHY is RLP faster than other data formats?
- Just presenting it is nice, but it's really not very interesting unless we show WHY it matters (aka what problem does it solve)

### Serialization:
The process of converting a data structure into a sequence of bytes. Ethereum internally uses an encoding format called recursive-length prefix encoding (RLP). The purpose of RLP (Recursive Length Prefix) is to encode arbitrarily nested arrays of binary data, and RLP is the main encoding method used to serialize objects in Ethereum. The only purpose of RLP is to encode structure; encoding specific data types (eg. strings, floats) is left up to higher-order protocols; but positive RLP integers must be represented in big endian binary form with no leading zeroes (thus making the integer value zero be equivalent to the empty byte array). Deserialised positive integers with leading zeroes must be treated as invalid. The integer representation of string length must also be encoded this way, as well as integers in the payload. Additional information can be found in the Ethereum yellow paper Appendix B.
- https://github.com/ethereum/wiki/wiki/Glossary
- https://github.com/ethereum/wiki/wiki/RLP

### 2.2.1. Recursive Length Prefix Encoding

Recursive Length Prefix Encoding (RLP) imposes a
structure on data that intrinsically considers a prefixed
hex value to position the data in the state database tree.
This hex value determines the depth of a certain piece
of data. There are two types of fundamental items one
can encode in RLP:
1. Strings of bytes
2. Lists of other items

RLP encodes arrays of nested binary data to an arbitrary depth; it is the main serialization method for
data in Ethereum. RLP encodes structure of data only,
so it does not pay heed to the particular types of data
being encoded.

Positive RLP integers are represented with the most
significant value stored at the lowest memory address
(big endian) and without any leading zeroes. As a result, the RLP integer value for 0 is represented by an
empty byte-array. If a non-empty deserialized integer
begins with leading zeroes it is invalid.

The global state database is encoded as RLP for fast
traversal and inspection of data. RLP encoding creates a mapping between addresses and account states.
Since it is stored on node operator’s computers, the
tree can be indexed and searched without network delay. RLP encodes values as byte-arrays, or as sequences
of further values, which are subsequently encoded as
byte-arrays.

- https://github.com/chronaeon/beigepaper/blob/master/beigepaper.pdf

## Code
```rust, ignore
TBD
```


## Resources
- https://github.com/ethereum/wiki/wiki/Glossary
- https://github.com/ethereum/wiki/wiki/RLP
- https://github.com/jnnk/pyrlp/blob/master/docs/tutorial.rst
- https://github.com/paritytech/parity-common/tree/master/rlp
- 2.2.1 RLP: https://github.com/chronaeon/beigepaper/blob/master/beigepaper.pdf
