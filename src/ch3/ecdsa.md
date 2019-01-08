<h1 align="center">
    ECDSA: math is beautiful.
</h1>

<br>

## Words

secp256k1
- https://github.com/paritytech/parity-ethereum/blob/master/accounts/ethkey/src/crypto.rs

Edwards-Curve Ed25519
- wikipedia:
  - https://en.wikipedia.org/wiki/Curve25519
- Substrate: 
  - https://www.parity.io/substrate/
- Dalek Crypto:
  - https://dalek.rs/
  - https://docs.rs/x25519-dalek/0.1.0/x25519_dalek/index.html#examples

In the chapter after this maybe expand to ZK proofs and stuff?
- heard rollup is cool: https://github.com/barryWhiteHat/roll_up


### Tiny Bits of Elliptic Curves

An Elliptic Curve for the purpose of cryptography is simply a large set of points that
we will call _C_. These points can be added, subtracted, or multiplied by integers (also called scalars).
Given an integer _k_ and
using the scalar multiplication operation we can compute `k*H`, which is also a point on
curve _C_. Given another integer _j_ we can also calculate `(k+j)*H`, which equals
`k*H + j*H`. The addition and scalar multiplication operations on an elliptic curve
maintain the commutative and associative properties of addition and multiplication:

<br>

## Videos

<br>

## Code
```rust

```

<br>

## Resources

<br>
