# Refactoring
- How to make the core architecture more modular and pluggable.

### structs and impl functions 
- for each core component of the blockchain

### trait objects 
- so that you can just specify the library vs swapping it
- for example rather than actually changing the functions within impl Keys or impl STF, you could just specify which version you want in the State struct via a keyword?
- https://doc.rust-lang.org/book/ch17-02-trait-objects.html
