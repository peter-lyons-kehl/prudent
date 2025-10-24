[![GitHub Actions
results](https://github.com/peter-lyons-kehl/prudent/actions/workflows/main.yml/badge.svg)](https://github.com/peter-lyons-kehl/prudent/actions)

# Helps you make Rust code safer

# Summary

## Purpose

Prudent helps you minimize the amount of code that is marked as `unsafe`. It is ergonomic (short),
simple and obvious (easy to search for and review).

That helps both authors, reviewers and all of us:

- authors/maintainers:
  - Notice/prevent accidental/unnecessary unsafe code
    - in parameters to function calls, or
    - TODO: in parameters or receiver `self` of a method call, or
    - TODO: in values stored to `static mut` variables, or
    - TODO: in values stored to fields of `union` types, or
    - TODO: in expressions whose cast or defer need `unsafe`
- reviewers: Save your time by making the unsafe parts shorter.
- all of us:
  - Prevent accidental invocation of functions (3rd party, or even your own) that
    1. have been called as a part of (larger) `unsafe {...}` block, and
    2. they used to be safe, but
    3. later they were changed to `unsafe`. (Of course, such a change is a breaking change, but
    `unsafe {...}` doesn't discriminate...)
  - Make our libraries and applications safer.
<!-- ## Use -->
<!-- # Details -->
<!-- ## Problem -->

# Zero cost

Prudent is a zero-cost abstraction (for both binary size/speed and memory). Rust/LLVM easily
optimizes it out.

Current features of Prudent don't use procedural macros, but use `macro_rules!` (macros by example),
which compiles fast.

# Compatibility

Prudent is `no-std`-compatible. It has no dependencies.

## Always forward compatible

`ndd` is planned to be always below version `1.0`. So stable (**even**-numbered) versions will be
forward compatible. (If a need ever arises for big incompatibility, that can go to a new crate.)

That allows you to specify `ndd` as a dependency with version `0.*`, which will match ANY **major**
versions (below `1.0`, of course). That will match the newest (**even**-numbered major) stable
version (available for your Rust) automatically.

This is special only to `0.*` - it is **not** possible to have a wildcard matching various **major**
versions `1.0` or higher.

# Quality assurance

Checks and tests are run by [GitHub Actions]. See
[results](https://github.com/peter-lyons-kehl/prudent/actions). All scripts run on Alpine Linux
(without `libc`, in a `rust:1.87-alpine` container) and are POSIX-compliant:

- `rustup component add clippy rustfmt`
- `cargo clippy`
- `cargo fmt --check`
- `cargo doc --no-deps --quiet`
- `cargo test`
- `cargo test --release`
- with [`MIRI`]
  - `rustup install nightly --profile minimal`
  - `rustup +nightly component add miri`
  - `cargo +nightly miri test`

# Updates

Please subscribe for low frequency updates at
[peter-lyons-kehl/prudent/issues#1](https://github.com/peter-lyons-kehl/prudent/issues/1).

<!-- 1. Link URLs to be used on GitHub.
     2. Relative links also work auto-magically on https://crates.io/crates/ndd.
     3. Keep them in the same order as used above.
-->
[`MIRI`]: https://github.com/rust-lang/miri
