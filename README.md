# Meticulous - better unwrapping

[![crates.io](https://img.shields.io/crates/v/meticulous.svg)](https://crates.io/crates/meticulous)
[![Documentation](https://docs.rs/meticulous/badge.svg)](https://docs.rs/meticulous)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/meticulous.svg)](./LICENSE-APACHE)
[![CI](https://github.com/glebpom/meticulous/workflows/CI/badge.svg)](https://github.com/glebpom/meticulous/actions?query=workflow%3ACI)

```toml
[dependencies]
meticulous = "0.2.0"
```

This crate provides extensions to `Result` and `Option` with additional unwrapping methods, which have more meaning compared to
standard `expect` and `unwrap`.

Different "unwrapping" cases may have different meaning. Some of them need to be fixed later, others don't. You may also
want to easily find different types of unwrapping using a simple code search. Using different unwrapping methods from
this crate helps writing and maintaining the code.

## todo

At the early stage, you don't want to care about handling all the errors, happy-path scenarios may be enough. [todo] can
be used in such cases.

### disallow-todo-on-release

The optional `disallow-todo-on-release` feature prevents temporary unwrapping from remaining in release builds.
It is disabled by default. Enable it in your dependency declaration:

```toml
[dependencies]
meticulous = { version = "0.2.0", features = ["disallow-todo-on-release"] }
```

With this feature enabled, `ResultExt::todo` and `OptionExt::todo` are unavailable when `meticulous` is compiled
with `debug_assertions` disabled, as in Cargo's default release profile. Calls to either method then fail to compile,
even for an `Ok` or `Some` value. Code using only `assured` and `verified` continues to compile.

The condition uses the library's `debug_assertions` setting, rather than the caller's setting or the profile's name.
Profile overrides that enable debug assertions for `meticulous` keep these methods available in release builds;
overrides that disable them remove these methods even in development builds when the feature is enabled.

This feature applies only to this crate's `.todo()` extension methods. It does not check Rust's built-in `todo!()` macro
or TODO comments.

## assured

[assured] can be used when you are sure that a `Result` will never be `Err` or an `Option` will never be `None`.
For example, some type conversions cannot fail on the architecture you target.

## verified

Sometimes you check all conditions which may lead to failure, before doing the particular operation. In this
case [verified] can be used.

[todo]: https://docs.rs/meticulous/latest/meticulous/trait.ResultExt.html#tymethod.todo
[assured]: https://docs.rs/meticulous/latest/meticulous/trait.ResultExt.html#tymethod.assured
[verified]: https://docs.rs/meticulous/latest/meticulous/trait.ResultExt.html#tymethod.verified
