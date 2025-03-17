This crate exists as a playground for me to explore type-level programming in Rust, including:

- Lowering prolog-like logic into Rust traits, like [here](https://willcrichton.net/notes/type-level-programming/)
- Type-level recursion for defining HLists,
  like [here](https://beachape.com/blog/2017/03/12/gentle-intro-to-type-level-recursion-in-Rust-from-zero-to-frunk-hlist-sculpting/)
- Type-witnesses for creating "proofs" of having checked certain types, especially specialization-based witnesses