[![GitHub Actions
results](https://github.com/peter-lyons-kehl/prudent/actions/workflows/main.yml/badge.svg)](https://github.com/peter-lyons-kehl/prudent/actions)

# Helps you make Rust code safer

# Summary

## Purpose

`prudent`` helps you minimize the amount of code that is marked as `unsafe`.

- ergonomic (short)
- simple
- and obvious (easy to search for and review).

That helps both authors, reviewers and all of us:

- Authors/maintainers:
  - Notice/prevent accidental (unintended), or unnecessary, `unsafe` code:
    - function/method calls:
      - in parameters to calls to an `unsafe` function or method
      - in an expression that evaluates to an (`unsafe`) function (that is to be evaluated)
      - in an expression that evaluates to the receiver (`self`) of an `unsafe` method
    - variable access:
      - TODO: `static mut` variables
      - TODO: fields of `union` types
    - value cast (to a different type):
      - TODO: in expressions whose cast or defer is `unsafe`
- Reviewers: Save your time by making the unsafe parts shorter. Focus on what matters.
- All of us:
  - Prevent accidental invocation of functions (3rd party, or even your own) that
    1. have been called as a part of (larger) `unsafe {...}` block, and
    2. they used to be safe, but
    3. later they were changed to `unsafe`. (Of course, such a change is a breaking change, but
       mistakes happen.)
  - Make our libraries and applications safer.
<!-- ## Use -->
<!-- # Details -->
<!-- ## Problem -->

# Scope

Rust is a rich language and it allows complex statements/expressions. `prudent` tries to be flexible, but it also needs to be manageable and testable. So, there may be code that `prudent` doesn't accept. Most likely if it involves advanced pattern matching.

`prudent` is to help you make `unsafe` code stand out more. Mixing `unsafe` with advanced or complex
syntax may sound exciting, but it makes reading the code difficult. Can that be an opportunity to
refactor?

# Zero cost

`prudent` is a zero-cost abstraction (for both binary size/speed and memory). Rust/LLVM easily
optimizes it out.

Current features of `prudent` don't use procedural macros, but use `macro_rules!` (macros by
example), so it compiles fast.

# Compatibility

`prudent` is `no-std`-compatible. It has no dependencies.

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
