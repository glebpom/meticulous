# Meticulous - better unwrapping

This crate provides extensions to `Result` type with additional unwrapping methods, which have more meaning compared to
standard `expect` and `unwrap`.

Different "unwrapping" cases may have different meaning. Some of them need to be fixed later, others don't. You may also
want to easily find different types of unwrapping using a simple code search. This helps helps with that.

# todo

At the early stage, you don't want to care about handling all the errors, happy-path scenarios may be enough. `todo` can
be used in such cases.

## assured

This may be used when you are sure that `Result` will never fail. For example, some type conversions is never intend to
fail on operating system you target.

## verified

Sometimes you check all conditions which may lead to failure, before doing the particular operation. In this
case `verified` may be used