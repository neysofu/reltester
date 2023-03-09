# Reltester

[![Crates.io](https://img.shields.io/crates/l/reltester)](https://github.com/neysofu/reltester/blob/main/LICENSE.txt) [![docs.rs](https://img.shields.io/docsrs/reltester)](https://docs.rs/reltester/latest/reltester/) [![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/neysofu/reltester/ci.yml)](https://github.com/neysofu/reltester/actions) [![Crates.io](https://img.shields.io/crates/v/reltester)](https://crates.io/crates/reltester)

**Rel**ation **tester** is a small testing utility for automatically checking the correctness of `PartialEq`, `PartialOrd`, `Eq`, and `Ord` implementations. It's most useful when used in conjuction with [`quickcheck`](https://github.com/BurntSushi/quickcheck) or some other property-based testing framework.


*Go to the [docs](https://docs.rs/reltester/latest/reltester/)!*

## Rationale

Imagine a scenario where you have a type `Foo` with a custom implementation of either `PartialEq`, `Eq`, `PartialOrd`, or `Ord`. By "custom" we mean hand-written as opposed to derived. The Rust compiler alone cannot verify the correctness of these implementations and thus it is up to you, the programmer, to uphold certain invariants about the specific [binary relation](https://en.wikipedia.org/wiki/Binary_relation) that you're implementing. For example, if you implement `PartialEq` for `Foo`, you must guarantee that `foo1 == foo2` implies `foo2 == foo1` (*symmetry*).

This is what `reltester` is for. Rather than learning all subtle details of `PartialEq`, `Eq`, `PartialOrd`, and `Ord`, you can write some tests that will automatically check these invariants for you.

## How to use

1. Write some tests that generate random values of the type you wish to test. You can do this by hand or using crates such as [`quickcheck`](https://github.com/BurntSushi/quickcheck) and [`proptest`](https://github.com/proptest-rs/proptest).
2. Based on the traits that your type implements, call the appropriate checker:

   - `reltester::eq` for `Eq`;
   - `reltester::ord` for `Ord`;
   - `reltester::partial_eq` for `PartialEq`;
   - `reltester::partial_ord` for `PartialOrd`.

   All of these functions take three arguments of the same type: `a`, `b`, and `c`. This is because it takes up to three values are needed to test some invariants.

## Legal

Reltester is available under the terms of the MIT license.
