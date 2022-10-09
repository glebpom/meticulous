# Meticulous - better unwrapping

[![crates.io](https://img.shields.io/crates/v/meticulous.svg)](https://crates.io/crates/meticulous)
[![Documentation](https://docs.rs/meticulous/badge.svg)](https://docs.rs/meticulous)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/meticulous.svg)](./LICENSE-APACHE)
[![CI](https://github.com/glebpom/meticulous/workflows/CI/badge.svg)](https://github.com/glebpom/meticulous/actions?query=workflow%3ACI)

```toml
[dependencies]
melticulous = "0.1.0"
```

This crate provides extensions to `Result` type with additional unwrapping methods, which have more meaning compared to
standard `expect` and `unwrap`.

Different "unwrapping" cases may have different meaning. Some of them need to be fixed later, others don't. You may also
want to easily find different types of unwrapping using a simple code search. Using different unwrapping methods from
this crate helps writing and maintaining the code. 

# todo

At the early stage, you don't want to care about handling all the errors, happy-path scenarios may be enough. [todo] can
be used in such cases.

## assured

[assured] can be used when you are sure that `Result` will never fail. For example, some type conversions is never intend to
fail on operating system you target.

## verified

Sometimes you check all conditions which may lead to failure, before doing the particular operation. In this
case [verified] can be used.

[todo]: https://docs.rs/meticulous/latest/meticulous/trait.ResultExt.html#tymethod.todo
[assured]: https://docs.rs/meticulous/latest/meticulous/trait.ResultExt.html#tymethod.assured
[verified]: https://docs.rs/meticulous/latest/meticulous/trait.ResultExt.html#tymethod.verified
