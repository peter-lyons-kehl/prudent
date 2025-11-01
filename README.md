[![GitHub Actions
results](https://github.com/peter-lyons-kehl/prudent/actions/workflows/main.yml/badge.svg)](https://github.com/peter-lyons-kehl/prudent/actions)

# Summary

`prudent` helps you minimize the amount of Rust code that is marked as `unsafe`.

- ergonomic (short)
- clear
- obvious (easy to search for and review).

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

const fn return_fn_returning_bool() -> fn() -> bool {
    fn f() -> bool {
      true
    }
    f
}
```

# unsafe_method_***

Unfortunately, Rust language limitations don't allow us to have one macro for methods that with
receiver `self` being a shared reference `&self`, a mutable reference `&mut self` or a value `self`.
We need to use three different macros: `unsafe_method_ref`, `unsafe_method_mut` and
`unsafe_method_val`.

# unsafe_method_ref and unsafe_method_mut in const

As of late 2025, `const` traits are not stabilized in Rust. So, currently `unsafe_method_ref` and
`unsafe_method_mut` can't be used in `const` context. Please give thumbs up to
`feature(const_trait_impl)`
[rust-lang/rust#143874](https://github.com/rust-lang/rust/issues/143874).

<!--UNSURE: `unsafe_method_ref` and `unsafe_method_mut` in `const` **will** be supported by `prudent` on`nightly` Rust toolchain in late 2025.-->
 There are two workarounds. Both require you to pass in the receiver expression **typed exactly** as
 the receiver of the method. So, you can **not** hand a receiver expression with type `T` if the
 method's receiver is defined as `&T` or `&mut T`.

## More ergonomic alternative

1. This re-uses `unsafe_method_val`. 
2. You _could_ use `unsafe_method_val` directly. But, if you'd like to be able to search/easily
   notice these `const` use cases, (re)import `unsafe_method_val` under a different name, like
   `unsafe_method_for`.
3. However, suggest **not** to (re)import `unsafe_method_val` as `unsafe_method_const` or any other
   name implying `const`, because this macro on its own **cannot** give a `const` guarantee.

```rust
use prudent::unsafe_method_val;
use prudent::unsafe_method_val as unsafe_method_for;

// @TODO
//
//const ONE: u8 = unsafe_method_for!(1, u8::unchecked_add, );
```

## Less ergonomic
```rust
# use prudent::unsafe_fn;
const ONE: u8 = unsafe_fn!(u8::unchecked_add, 1, 0);
```

# unsafe_method_ref
```rust
# use prudent::unsafe_method_ref;
let _ = unsafe_method_ref!(1u8, unchecked_add, 0);

struct SNonCopy {}
impl SNonCopy {
    fn unsafe_method_no_args(&self) {}
    fn unsafe_method_one_arg(&self, _: bool) {}
    fn unsafe_method_two_args(&self, _: bool, _: bool) {}
}

let s = SNonCopy {};
unsafe_method_ref!(s, unsafe_method_no_args);
unsafe_method_ref!(s, unsafe_method_one_arg, true);
unsafe_method_ref!(s, unsafe_method_two_args, true, false);
```

# unsafe_method_mut
```rust
# use prudent::unsafe_method_mut;
struct SNonCopy {}
impl SNonCopy {
    fn unsafe_method_no_args(&mut self) {}
    fn unsafe_method_one_arg(&mut self, _: bool) {}
    fn unsafe_method_two_args(&mut self, _: bool, _: bool) {}
}

let mut s = SNonCopy {};
unsafe_method_mut!(s, unsafe_method_no_args);
unsafe_method_mut!(s, unsafe_method_one_arg, true);
unsafe_method_mut!(s, unsafe_method_two_args, true, false);
```

# unsafe_method_val
```rust
# use prudent::unsafe_method_val;
{
    struct SNonCopy {}
    impl SNonCopy {
        fn unsafe_method_no_args(self) {}
        fn unsafe_method_one_arg(self, _: bool) {}
        fn unsafe_method_two_args(self, _: bool, _: bool) {}
    }

    let sNonCopy = SNonCopy {};
    unsafe_method_val!(sNonCopy, unsafe_method_no_args);
    let sNonCopy = SNonCopy {};
    unsafe_method_val!(sNonCopy, unsafe_method_one_arg, true);
    let sNonCopy = SNonCopy {};
    unsafe_method_val!(sNonCopy, unsafe_method_two_args, true, false);
}
{
    #[derive(Clone, Copy)]
    struct SCopy {}
    impl SCopy {
        fn unsafe_method_no_args(self) {}
    }

    let sCopy = SCopy {};
    unsafe_method_val!(sCopy, unsafe_method_no_args);
    unsafe_method_val!(sCopy, unsafe_method_no_args);
    let _ = sCopy;
}
```
<!-- ------- -->

# unsafe_ref
## unsafe_ref - one arg, basic reference
```rust
# use prudent::unsafe_ref;
const B: bool = true;
const PT: *const bool = &B as *const bool;

const _: &bool = unsafe_ref!(PT);
let _ = unsafe_ref!(PT);
```

## unsafe_ref - one arg, slice
```rust
# use prudent::unsafe_ref;
const BS: [bool; 2] = [true, false];
const PT: *const [bool] = &BS as *const [bool];

const _: &[bool] = unsafe_ref!(PT);
let _ = unsafe_ref!(PT);
```

## unsafe_ref - one arg, dyn reference
```rust
# use prudent::unsafe_ref;
# use core::fmt::Display;
const B: bool = true;
const PT: *const bool = &B as *const bool;

const _: &dyn Display = unsafe_ref!(PT);
```

## unsafe_ref - two args, lifetimed reference
```rust
# use prudent::unsafe_ref;
const B: bool = true;
const PT: *const bool = &B as *const bool;

const _: &'static bool = unsafe_ref!(PT, 'static);
let _ = unsafe_ref!(PT, 'static);
```

## unsafe_ref - two args, lifetimed dyn reference
```rust
# use prudent::unsafe_ref;
# use core::fmt::Display;
const B: bool = true;
const PT: *const bool = &B as *const bool;

const _: &'static dyn Display = unsafe_ref!(PT, 'static);
```

## unsafe_ref - two args, lifetimed slice
```rust
# use prudent::unsafe_ref;
const BS: [bool; 2] = [true, false];
const PT: *const [bool] = &BS as *const [bool];

const _: &'static [bool] = unsafe_ref!(PT, 'static);
let _ = unsafe_ref!(PT, 'static);
```

## unsafe_ref - two args, typed basic reference
```rust
# use prudent::unsafe_ref;
const B: bool = true;
const PT: *const bool = &B as *const bool;

const _: &bool = unsafe_ref!(PT, bool);
let _ = unsafe_ref!(PT, bool);
```

## unsafe_ref - two args, typed slice
```rust
# use prudent::unsafe_ref;
const BS: [bool; 2] = [true, false];
const PT: *const [bool] = &BS as *const [bool];

const _: &[bool] = unsafe_ref!(PT, [bool]);
let _ = unsafe_ref!(PT, [bool]);
```

## unsafe_ref - two args, typed dyn reference
```rust
# use prudent::unsafe_ref;
# use core::fmt::Display;
const B: bool = true;
const PT: *const dyn Display = &B as *const dyn Display;

const _: &dyn Display = unsafe_ref!(PT, dyn Display);
let _ = unsafe_ref!(PT, dyn Display);
```
<!-- ------- -->

# unsafe_mut
## unsafe_mut - one arg, basic reference
```rust
# use prudent::unsafe_mut;
let mut b: bool = true;
let pt: *mut bool = &mut b as *mut bool;

let _: &bool = unsafe_mut!(pt);
let _ = unsafe_mut!(pt);
```

## unsafe_mut - one arg, slice
```rust
# use prudent::unsafe_mut;
let mut bs: [bool; 2] = [true, false];
let pt: *mut [bool] = &mut bs as *mut [bool];

let _: &[bool] = unsafe_mut!(pt);
let _ = unsafe_mut!(pt);
```

## unsafe_mut - one arg, dyn reference
```rust
# use prudent::unsafe_mut;
# use core::fmt::Display;
let mut b: bool = true;
let pt: *mut bool = &mut b as *mut bool;

let _: &dyn Display = unsafe_mut!(pt);
```

## unsafe_mut - two args, lifetimed reference
```rust
# use prudent::unsafe_mut;
let b: &'static mut bool = Box::leak( Box::new(true) );
let pt: *mut bool = b as *mut bool;

let _: &'static mut bool = unsafe_mut!(pt, 'static);
let _ = unsafe_mut!(pt, 'static);
```

## unsafe_mut - two args, lifetimed dyn reference
```rust
# use prudent::unsafe_mut;
# use core::fmt::Display;
let b: &'static mut bool = Box::leak( Box::new(true) );
let pt: *mut bool = b as *mut bool;

let _: &'static mut dyn Display = unsafe_mut!(pt, 'static);
let _ = unsafe_mut!(pt, 'static);
```

## unsafe_mut - two args, lifetimed slice
```rust
# use prudent::unsafe_mut;
let b: &'static mut [bool] = Box::leak( Box::new([true, false]) );
let pt: *mut [bool] = b as *mut [bool];

let _: &'static mut [bool] = unsafe_mut!(pt, 'static);
let _ = unsafe_mut!(pt, 'static);

```

## unsafe_mut - two args, typed basic reference
```rust
# use prudent::unsafe_mut;
let mut b: bool = true;
let pt: *mut bool = &mut b as *mut bool;

let _: &mut bool = unsafe_mut!(pt, bool);
let _ = unsafe_mut!(pt, bool);
```

## unsafe_mut - two args, typed slice
```rust
# use prudent::unsafe_mut;
let mut bs: &'static mut [bool] = Box::leak( Box::new([true, false]) );
let pt: *mut [bool] = bs as *mut [bool];

let _: &mut [bool] = unsafe_mut!(pt, [bool]);
let _ = unsafe_mut!(pt, [bool]);
```

## unsafe_mut - two args, typed dyn reference
```rust
# use prudent::unsafe_mut;
# use core::fmt::Display;
let mut b: bool = true;
let pt: *mut dyn Display = &mut b as *mut dyn Display;

let _: &mut dyn Display = unsafe_mut!(pt, dyn Display);
let _ = unsafe_mut!(pt, dyn Display);
```

<!-- This is independent of [`![feature(as_ref_unchecked)]` rust-lang/rust#122034](https://github.com/rust-lang/rust/issues/122034). -->

<!-- ------- -->
# unsafe_val

Only for types that implement/derive [core::marker::Copy].

## unsafe_val - one arg, basic
```rust
# use prudent::unsafe_val;
const B: bool = true;
const PT: *const bool = &B as *const bool;

const _: bool = unsafe_val!(PT);
let _ = unsafe_val!(PT);
```

## unsafe_val - two args, typed
```rust
# use prudent::unsafe_val;
const B: bool = true;
const PT: *const bool = &B as *const bool;

const _: bool = unsafe_val!(PT, bool);
let _ = unsafe_val!(PT, bool);
```

# unsafe_set
```rust
# use prudent::unsafe_set;
let mut b: bool = true;
let pt: *mut bool = &mut b as *mut bool;

unsafe_set!(pt, false);
unsafe_set!(pt, true);
```

```rust
# use prudent::unsafe_set;
struct SNonCopy {}
let mut s: SNonCopy = SNonCopy {};
let pt: *mut SNonCopy = &mut s as *mut SNonCopy;

let setFrom: SNonCopy = SNonCopy {};
unsafe_set!(pt, setFrom);
let setFrom: SNonCopy = SNonCopy {};
unsafe_set!(pt, setFrom);
```

# unsafe_static_set
```rust
# use prudent::unsafe_static_set;
static mut B: bool = true;

unsafe_static_set!(B, false);
```

# Details

`prudent` helps both authors, reviewers and all of us:

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
  calls/interacts with/is used by `unsafe` code or with data used by both. ["Unsafe Rust cannot
  trust Safe Rust without care."](https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html) and
  ["Unsafe code must trust some Safe code, but shouldn't trust generic Safe
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

`prudent` is planned to be always below version `1.0`. So stable (**even**-numbered) versions will
be forward compatible. (If a need ever arises for big incompatibility, that can go to a new crate.)

That allows you to specify `prudent` as a dependency with version `0.*`, which will match ANY
**major** versions (below `1.0`, of course). That will match the newest (**even**-numbered major)
stable version (available for your Rust) automatically.

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
     2. Relative links also work auto-magically on https://crates.io/crates/prudent.
     3. Keep them in the same order as used above.
-->
[`MIRI`]: https://github.com/rust-lang/miri
[GitHub Actions]: .github/workflows/main.yml
[core::marker::Copy]: https://doc.rust-lang.org/core/marker/trait.Copy.html
