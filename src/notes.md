# Notes

# mdBook and External Libraries

If you include any files that call external crates (rand) in the SUMMARY.md the mdBook build fails.

How did The Rust Cookbook do it?
- https://github.com/rust-lang-nursery/rust-cookbook
- they have a Cargo.toml with dependancies: https://github.com/rust-lang-nursery/rust-cookbook/blob/master/Cargo.toml
  - is that enough?
- their book.toml is standard: https://github.com/rust-lang-nursery/rust-cookbook/blob/master/book.toml
- their travis.yml is also standard: https://github.com/rust-lang-nursery/rust-cookbook/blob/master/.travis.yml

mdBook says nothing about adding depdancies to external crates
- https://rust-lang-nursery.github.io/mdBook/index.html
- https://github.com/rust-lang-nursery/mdBook

Tried adding a Cargo.toml and running `cargo build --release` and pushing that to the repo, but Travis still failed. Not sure what else to try at this point...

Asked on the Rust Forum and apparently there's a few workarounds:
- https://users.rust-lang.org/t/diy-digital-signatures/23739/5

# Goals
- For education purposes build a fully functional (but not secure) blockchain using standard Rust code so that the main concepts can be understood as simply as possible in mdBook or the Rust Playground.
- Maybe create a CLI tutorial that shows how to iteratively ugrade each component and function in the standard model to make it more secure
- Stetch Goal: work towards recreating the Parity Ethereum Client and/or arbitrary blockchain architectures like Substrate, but that's probably not realistic lol

Chapter Framework:
- goal/thing to build a toy version of (w links examples of the thing IRL)
- each page explains a part (w links to wikipedia article and production library version of the tutorial example)
- summary putting it all together (editable code you can run and play with)

Bonus points if there's a story to explain the code and concepts.
