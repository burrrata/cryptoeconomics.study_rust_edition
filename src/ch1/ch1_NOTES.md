# TODO

General:
- would it make sense to have a separate module for data serialization (turning stuff into &[u8] bytes so that RSA and the hash function can operate on them?)
- then you could have an upgrade path to Ethereum's RLP: https://github.com/ethereum/wiki/wiki/RLP

UX: make it useable so people can actually learn stuff or provide feedback
- build out individual sections as a tutorial leading up to the full demo
- add relevant resources to each section
- improve comments in the centralize_payment_processor finale to better explain what's going on for those who jump ahead

Feedback: making it actually usable
- update Cryptoeconomics.Study post
- hit up the Tensor Programming guy who rolled the template Rust blockchain tutorial
- share to /r/ethereum
