[![GitHub Actions
results](https://github.com/peter-lyons-kehl/prudent/actions/workflows/main.yml/badge.svg)](https://github.com/peter-lyons-kehl/prudent/actions)

# Summary

`prudent`` helps you minimize the amount of Rust code that is marked as `unsafe`.

# API and examples
All the following examples are also run as
[doctests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html).

# unsafe_fn
```rust
# use prudent::unsafe_fn;
const unsafe fn unsafe_fn_no_args() {}
const unsafe fn unsafe_fn_one_arg(b: bool) -> bool { b }
const unsafe fn unsafe_fn_two_args(_: bool, u: u8) -> u8 { u }

unsafe_fn!(unsafe_fn_no_args);
unsafe_fn!(unsafe_fn_one_arg, true);
unsafe_fn!(unsafe_fn_two_args, true, 0);

const UNIT: () = unsafe_fn!(unsafe_fn_no_args);
const B: bool = unsafe_fn!(unsafe_fn_one_arg, true);
const U8: u8 = unsafe_fn!(unsafe_fn_two_args, true, 0);
```

# unsafe_method_ref
```rust
# use prudent::unsafe_method_ref;
let _ = unsafe_method_ref!(1u8, unchecked_add, 0);

struct S {} // intentionally NOT Copy
impl S {
    fn unsafe_method_no_args(&self) {}
    fn unsafe_method_one_arg(&self, _: bool) {}
    fn unsafe_method_two_args(&self, _: bool, _: bool) {}
}

let s = S {};
unsafe_method_ref!(s, unsafe_method_no_args);
unsafe_method_ref!(s, unsafe_method_one_arg, true);
unsafe_method_ref!(s, unsafe_method_two_args, true, false);
```

# unsafe_method_mut
```rust
# use prudent::unsafe_method_mut;
struct S {}
impl S {
    fn unsafe_method_no_args(&mut self) {}
    fn unsafe_method_one_arg(&mut self, _: bool) {}
    fn unsafe_method_two_args(&mut self, _: bool, _: bool) {}
}

let mut s = S {};
unsafe_method_mut!(s, unsafe_method_no_args);
unsafe_method_mut!(s, unsafe_method_one_arg, true);
unsafe_method_mut!(s, unsafe_method_two_args, true, false);
```

```rust
struct S {}
impl S {
    fn method_no_args(&mut self) {}
}
let mut s = S {};

// (s) does NOT move s. So good.
(s).method_no_args();
(s).method_no_args();

{s}.method_no_args(); // {s}} moves s!
// This then fails:
//
// {s}.method_no_args();

/*(|x: &mut S| {
  x.method_no_args()
})(s);*/
```

```rust
let mut u = 0u8;
#[allow(unsafe_code)]
unsafe {
  #[deny(unsafe_code)]
  u.unchecked_add(1);
}
```

# unsafe_ref
## unsafe_ref - one arg, basic reference
```rust
# use prudent::unsafe_ref;
# use core::fmt::Display;
const B: bool = true;
let pt: *const bool = &B as *const bool;

let _: &bool = unsafe_ref!(pt);
let _ = unsafe_ref!(pt);
```

## unsafe_ref - one arg, slice
```rust
# use prudent::unsafe_ref;
# use core::fmt::Display;
const BS: [bool; 2] = [true, false];
let pt: *const [bool] = &BS as *const [bool];

let _: &[bool] = unsafe_ref!(pt);
let _ = unsafe_ref!(pt);
```

## unsafe_ref - one arg, dyn reference
```rust
# use prudent::unsafe_ref;
# use core::fmt::Display;
const B: bool = true;
let pt: *const bool = &B as *const bool;

let _: &dyn Display = unsafe_ref!(pt);
```

## unsafe_ref - two args, lifetimed reference
```rust
# use prudent::unsafe_ref;
# use core::fmt::Display;
const B: bool = true;
let pt: *const bool = &B as *const bool;

let _: &'static bool = unsafe_ref!(pt, 'static);
let _ = unsafe_ref!(pt, 'static);
```

## unsafe_ref - two args, lifetimed dyn reference
```rust
# use prudent::unsafe_ref;
# use core::fmt::Display;
const B: bool = true;
let pt: *const bool = &B as *const bool;

let _: &'static dyn Display = unsafe_ref!(pt, 'static);
```

## unsafe_ref - two args, lifetimed slice
```rust
# use prudent::unsafe_ref;
# use core::fmt::Display;
const BS: [bool; 2] = [true, false];
let pt: *const [bool] = &BS as *const [bool];

let _: &'static [bool] = unsafe_ref!(pt, 'static);
let _ = unsafe_ref!(pt, 'static);
```

## unsafe_ref - two args, typed basic reference
```rust
# use prudent::unsafe_ref;
# use core::fmt::Display;
const B: bool = true;
let pt: *const bool = &B as *const bool;

let _: &bool = unsafe_ref!(pt, bool);
let _ = unsafe_ref!(pt, bool);
```

## unsafe_ref - two args, typed slice
```rust
# use prudent::unsafe_ref;
# use core::fmt::Display;
const BS: [bool; 2] = [true, false];
let pt: *const [bool] = &BS as *const [bool];

let _: &[bool] = unsafe_ref!(pt, [bool]);
let _ = unsafe_ref!(pt, [bool]);
```

## unsafe_ref - two args, typed dyn reference
```rust
# use prudent::unsafe_ref;
# use core::fmt::Display;
const B: bool = true;
let pt: *const dyn Display = &B as *const dyn Display;

let _: &dyn Display = unsafe_ref!(pt, dyn Display);
let _ = unsafe_ref!(pt, dyn Display);
```

<!-- ------- -->

# unsafe_mut
## unsafe_mut - one arg, basic reference
```rust
# use prudent::unsafe_ref;
# use core::fmt::Display;
const B: bool = true;
const _BS: [bool; 2] = [true, false];
```


<!-- This is independent of [`![feature(as_ref_unchecked)]` rust-lang/rust#122034](https://github.com/rust-lang/rust/issues/122034). -->

# Details

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
      - TODO: in expressions whose deref is `unsafe`
- Reviewers: Save your time by making the unsafe parts shorter. Focus on what matters.
- All of us:
  - Prevent accidental invocation of functions (3rd party, or even your own) that
    1. have been called as a part of (larger) `unsafe {...}` block, and
    2. they used to be safe, but
    3. later they were changed to `unsafe`. (Of course, such a change is a breaking change, but
       mistakes happen.)
  - Make our libraries and applications safer.

However,

- `prudent` **cannot** make/guarantee your `unsafe` code to be "safe". No tool can fully do that
  (because of nature of `unsafe`).
- Using `prudent` doesn't mean you can ignore/skip reviewing/blindly trust any **safe** code that
  calls/interacts with `unsafe` code or with data used by both. ["Unsafe Rust cannot trust Safe Rust
  without care."](https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html) ["Unsafe code must
  trust some Safe code, but shouldn't trust generic Safe
  code."](https://doc.rust-lang.org/nomicon/working-with-unsafe.html)

<!-- ## Use -->
<!-- # Details -->
<!-- ## Problem -->

## Scope

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
