# Reltester

[![Crates.io](https://img.shields.io/crates/l/reltester)](https://github.com/neysofu/reltester/blob/main/LICENSE.txt) [![docs.rs](https://img.shields.io/docsrs/reltester)](https://docs.rs/reltester/latest/reltester/) [![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/neysofu/reltester/ci.yml)](https://github.com/neysofu/reltester/actions) [![Crates.io](https://img.shields.io/crates/v/reltester)](https://crates.io/crates/reltester) [![min-rustc](https://img.shields.io/badge/min--rustc-1.56-blue)](https://github.com/neysofu/reltester/blob/main/rust-toolchain.toml)

**Rel**ation **tester** is a small testing utility for automatically checking the correctness of `[Partial]Eq`, `[Partial]Ord`, `Hash`, and `[DoubleEnded|Fused]Iterator` trait implementations. It's most useful when used in conjuction with [`quickcheck`](https://github.com/BurntSushi/quickcheck) or some other property-based testing framework.


*Go to the [docs](https://docs.rs/reltester/latest/reltester/)!*

## Rationale

Imagine a scenario where you have a type `Foo` with a custom implementation of either `PartialEq`, `Eq`, `PartialOrd`, or `Ord`. By "custom" we mean hand-written as opposed to derived. The Rust compiler alone cannot verify the correctness of these implementations and thus it is up to you, the programmer, to uphold certain invariants about the specific [binary relation](https://en.wikipedia.org/wiki/Binary_relation) that you're implementing. For example, if you implement `PartialEq` for `Foo`, you must guarantee that `foo1 == foo2` implies `foo2 == foo1` (*symmetry*).

Other traits such as `Hash` and `Iterator` mandate several invariants as well – some of which are very intuitive, and [others](https://doc.rust-lang.org/std/hash/trait.Hash.html#prefix-collisions) which are not. It's especially common for less-than-perfect implementations of the `std::iter` family of traits to introduce off-by-one bugs[^1][^2][^3][^4] among others.

The idea is, instead of keeping these invariants in your head whenever you go about manually implementing one of these traits in your codebase, you can add a Reltester check to your test suite and have a higher degree of confidence that your implementation is correct.


## How to use

1. Write some tests that generate random values of the type you wish to test. You can do this by hand or using crates such as [`quickcheck`](https://github.com/BurntSushi/quickcheck) and [`proptest`](https://github.com/proptest-rs/proptest). Calling the checkers on static, non-randomized values is possible but is less effective in catching bugs.
2. Based on the traits that your type implements, call the appropriate checker(s):

   - `reltester::eq` for `Eq`;
   - `reltester::ord` for `Ord`;
   - `reltester::partial_eq` for `PartialEq`;
   - `reltester::partial_ord` for `PartialOrd`;
   - `reltester::hash` for `Hash`;
   - `reltester::iterator` for `Iterator`;
   - `reltester::fused_iterator` for `FusedIterator`;
   - `reltester::double_ended_iterator` for `DoubleEndedIterator`;

   Some of these functions take multiple (two or three) values of the same type. This is because it takes up to three values to test some invariants.

Please refer to the documentation for more information. The `reltester::invariants` module is available for more granular checks if you can't satisfy the type bounds of the main functions.

## Examples

### `f32` (`PartialEq`, `PartialOrd`)

```rust
use reltester;
use quickcheck_macros::quickcheck;

#[quickcheck]
fn test_f32(a: f32, b: f32, c: f32) -> bool {
    // Let's check if `f32` implements `PartialEq` and `PartialOrd` correctly
    // (spoiler: it does).
    reltester::partial_eq(&a, &b, &c).is_ok()
        && reltester::partial_ord(&a, &b, &c).is_ok()
}
```

### `u32` (`Hash`)

```rust
use reltester;
use quickcheck_macros::quickcheck;

#[quickcheck]
fn test_u32(a: u32, b: u32) -> bool {
    // Unlike `f32`, `u32` implements both `Eq` and `Hash`, which allows us to
    // test `Hash` invariants.
    reltester::hash(&a, &b).is_ok()
}
```

### `Vec<u32>` (`DoubleEndedIterator`, `FusedIterator`, `Iterator`)

```rust
use reltester;
use quickcheck_macros::quickcheck;

#[quickcheck]
fn test_vec_u32(nums: Vec<u32>) -> bool {
    // `Iterator` is implied and checked by both `DoubleEndedIterator` and
    // `FusedIterator`.
    reltester::double_ended_iterator(nums.iter()).is_ok()
        && reltester::fused_iterator(nums.iter()).is_ok()
}
```

## Legal

Reltester is available under the terms of the MIT license.

## External references and footnotes

[^1]: https://github.com/rust-lang/rust/issues/41964
[^2]: https://github.com/bevyengine/bevy/pull/7469
[^3]: https://github.com/bluejekyll/trust-dns/issues/1638
[^4]: https://github.com/sparsemat/sprs/issues/261
