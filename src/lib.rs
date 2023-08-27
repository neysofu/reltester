//! **Rel**ation **tester** is a small testing utility for automatically
//! checking the correctness of `[Partial]Eq`, `[Partial]Ord`, `Hash`, and
//! `[DoubleEnded|Fused]Iterator` trait implementations. It's most useful when
//! used in conjuction with
//! [`quickcheck`](https://github.com/BurntSushi/quickcheck) or some other
//! property-based testing framework.
//!
//! # Rationale
//!
//! Imagine a scenario where you have a type `Foo` with a custom implementation
//! of either [`PartialEq`], [`Eq`], [`PartialOrd`], or [`Ord`]. By "custom" we mean
//! hand-written as opposed to derived. The Rust compiler alone cannot verify
//! the correctness of these implementations and thus it is up to you, the
//! programmer, to uphold certain invariants about the specific [binary
//! relation](https://en.wikipedia.org/wiki/Binary_relation) that you're
//! implementing. For example, if you implement [`PartialEq`] for `Foo`, you must
//! guarantee that `foo1 == foo2` implies `foo2 == foo1` (*symmetry*).
//!
//! Other traits such as [`Hash`] and [`Iterator`] mandate several invariants as
//! well – some of which are very intuitive, and
//! [others](https://doc.rust-lang.org/std/hash/trait.Hash.html#prefix-collisions)
//! which are not. It's especially common for less-than-perfect implementations
//! of the [`std::iter`] family of traits to introduce off-by-one
//! bugs[^1] [^2] [^3] [^4] among others.
//!
//! The idea is, instead of keeping these invariants in your head whenever you
//! go about manually implementing one of these traits in your codebase, you can
//! add a Reltester check to your test suite and have a higher degree of
//! confidence that your implementation is correct.
//!
//! # How to use
//!
//! 1. Write some tests that generate random values of the type you wish to
//!    test. You can do this by hand or using crates such as
//!    [`quickcheck`](https://github.com/BurntSushi/quickcheck) and
//!    [`proptest`](https://github.com/proptest-rs/proptest). Calling the checkers
//!    on static, non-randomized values is possible but is less effective in
//!    catching bugs.
//! 2. Based on the traits that your type implements, call the appropriate checker(s):
//!
//!    - [`reltester::eq`](eq) for [`Eq`];
//!    - [`reltester::ord`](ord) for [`Ord`];
//!    - [`reltester::partial_eq`](partial_eq) for [`PartialEq`];
//!    - [`reltester::partial_ord`](partial_ord) for [`PartialOrd`];
//!    - [`reltester::hash`](hash) for [`Hash`];
//!    - [`reltester::iterator`](iterator) for [`Iterator`];
//!    - [`reltester::fused_iterator`](fused_iterator) for [`FusedIterator`];
//!    - [`reltester::double_ended_iterator`](double_ended_iterator) for [`DoubleEndedIterator`];
//!
//!    Some of these functions take multiple (two or three) values of the same
//!    type. This is because it takes up to three values to test some
//!    invariants.
//!
//! ## Multi-type relations: `Foo: PartialEq<Bar>` and `Foo: PartialOrd<Bar>`
//!
//! In some cases your [`PartialEq`] and [`PartialOrd`] implementations
//! may use a non-`Self` type parameter. (Note: [`Eq`] and [`Ord`] don't accept
//! type parameters and this use case doesn't apply to them.) Reltester
//! supports this use case and exposes granular invariant checking functions in
//! the [`invariants`] module with more lax type constraints.
//!
//! ## Examples
//!
//! ### `f32` (`PartialEq`, `PartialOrd`)
//!
//! ```rust
//! use reltester;
//! use quickcheck_macros::quickcheck;
//!
//! #[quickcheck]
//! fn test_f32(a: f32, b: f32, c: f32) -> bool {
//!     // Let's check if `f32` implements `PartialEq` and `PartialOrd` correctly
//!     // (spoiler: it does).
//!     reltester::partial_eq(&a, &b, &c).is_ok()
//!         && reltester::partial_ord(&a, &b, &c).is_ok()
//! }
//! ```
//!
//! ### `u32` (`Hash`)
//!
//! ```rust
//! use reltester;
//! use quickcheck_macros::quickcheck;
//!
//! #[quickcheck]
//! fn test_u32(a: u32, b: u32) -> bool {
//!     // Unlike `f32`, `u32` implements both `Eq` and `Hash`, which allows us to
//!     // test `Hash` invariants.
//!     reltester::hash(&a, &b).is_ok()
//! }
//! ```
//!
//! ### `Vec<u32>` (`DoubleEndedIterator`, `FusedIterator`, `Iterator`)
//!
//! ```rust
//! use reltester;
//! use quickcheck_macros::quickcheck;
//!
//! #[quickcheck]
//! fn test_vec_u32(nums: Vec<u32>) -> bool {
//!     // `Iterator` is implied and checked by both `DoubleEndedIterator` and
//!     // `FusedIterator`.
//!     reltester::double_ended_iterator(nums.iter()).is_ok()
//!         && reltester::fused_iterator(nums.iter()).is_ok()
//! }
//! ```
//!
//! # TL;DR invariants of the comparison traits
//!
//! Chances are you don't need to concern yourself with the mathematical definitions of
//! comparison traits; as long as your implementations are sensible and your
//! `reltester` tests pass, you can move on and assume your implementations are
//! correct. The required invariants are listed here only for the sake of
//! completeness.
//!
//! - [`PartialEq`] requires **symmetry** and **transitivity** of `==` whenever applicable ([partial
//!   equivalence
//!   relation](https://en.wikipedia.org/wiki/Partial_equivalence_relation) in the
//!   case of `Rhs == Self`).
//! - [`Eq`] requires **symmetry**, **transitivity**, and **reflexivity** of `==` ([equivalence relation](https://en.wikipedia.org/wiki/Equivalence_relation)).
//! - [`PartialOrd`] requires **symmetry** of `==`, **transitivity** of `>`,
//!   `==`, and `<`; and **duality** of `>` and `<`. Note that duality is not
//!   common mathematical
//!   terminology, it's just what the Rust [`std`] uses to describe `a > b iff b < a`.
//!   Thus the exact mathematical definition of [`PartialOrd`] seems [open to
//!   debate](https://users.rust-lang.org/t/traits-in-std-cmp-and-mathematical-terminology/69887),
//!   though it's generally understood to mean [strict partial
//!   order](https://en.wikipedia.org/wiki/Partially_ordered_set#Strict_partial_orders).
//! - [`Ord`] requires **symmetry** and **reflexivity** of `==`; **transitivity** of `>`, `==`, and `<`; and **duality** of `>` and `<`.
//!   `==`; **transitivity** and **duality** of `>` and `<`; and must be **trichotomous**[^5]. Just like
//!   [`PartialOrd`], the mathematical definition of [`Ord`] is a bit open to
//!   interpretation, though it's generally understood to mean [total
//!   order](https://en.wikipedia.org/wiki/Total_order#Strict_and_non-strict_total_orders).
//!
//! In addition to the above, trait method default implementation overrides (for e.g.
//! [`PartialOrd::lt`] or [`Ord::max`]) must have the same behavior as the
//! default implementations. `reltester` always checks these for you.
//!
//!
//! [^1]: <https://github.com/rust-lang/rust/issues/41964>
//!
//! [^2]: <https://github.com/bevyengine/bevy/pull/7469>
//!
//! [^3]: <https://github.com/bluejekyll/trust-dns/issues/1638>
//!
//! [^4]: <https://github.com/sparsemat/sprs/issues/261>
//!
//! [^5]: Trichotomy is a corollary that follows from the definitions of `>`,
//! `==`, and `<` based on [`Ordering`](std::cmp::Ordering).

#![allow(clippy::eq_op, clippy::double_comparisons)]

pub mod error;
pub mod invariants;

use error::*;
use std::{hash::Hash, iter::FusedIterator};

/// Checks the correctness of the [`Ord`] trait (and [`Eq`] and [`PartialOrd`]
/// by extension) for some values.
pub fn ord<T>(a: &T, b: &T, c: &T) -> Result<(), Error>
where
    T: Ord,
{
    eq(a, b, c)?;
    partial_ord(a, b, c)?;

    invariants::ord_methods_consistency(a, b, c)?;

    Ok(())
}

/// Checks the correctness of the [`PartialOrd`] trait (and [`PartialEq`] by
/// extension) for some values.
pub fn partial_ord<T>(a: &T, b: &T, c: &T) -> Result<(), Error>
where
    T: PartialOrd,
{
    partial_eq(a, b, c)?;

    invariants::partial_ord_methods_consistency(a, b)?;
    invariants::partial_ord_duality(a, b)?;
    invariants::partial_ord_transitivity(a, b, c)?;

    Ok(())
}

/// Checks the correctness of the [`Eq`] trait (and [`PartialEq`] by extension)
/// for some values.
///
/// The type bound is intentionally [`PartialEq`] instead of [`Eq`] to allow
/// for negative testing, i.e. ensuring that your [`PartialEq`] implementor
/// *doesn't* implement [`Eq`] when it shouldn't.
pub fn eq<T>(a: &T, b: &T, c: &T) -> Result<(), Error>
where
    T: PartialEq<T>,
{
    partial_eq(a, b, c)?;

    // Checking `Eq` is the same as checking `PartialEq`, except it also
    // requires reflexivity.
    invariants::eq_reflexivity(a)?;

    Ok(())
}

/// Checks the correctness of the [`PartialEq`] trait
/// for some values.
pub fn partial_eq<T>(a: &T, b: &T, c: &T) -> Result<(), PartialEqError>
where
    T: PartialEq,
{
    invariants::partial_eq_methods_consistency(a, b)?;
    invariants::partial_eq_symmetry(a, b)?;
    invariants::partial_eq_transitivity(a, b, c)?;

    Ok(())
}

/// Checks the correctness of the [`Hash`] trait in relation to [`Eq`] for some
/// values.
pub fn hash<K>(a: &K, b: &K) -> Result<(), HashError>
where
    K: Hash + Eq + ?Sized,
{
    invariants::hash_consistency_with_eq(a, b)?;
    invariants::hash_prefix_collision(a, b)?;

    Ok(())
}

/// Checks the correctness of the [`Iterator`] trait for some value `iter`.
///
/// Note that `iter` must be a finite iterator.
pub fn iterator<I>(iter: I) -> Result<(), IteratorError>
where
    I: Iterator + Clone,
    I::Item: PartialEq,
{
    invariants::iterator_size_hint(iter.clone())?;
    invariants::iterator_count(iter.clone())?;
    invariants::iterator_last(iter)?;

    Ok(())
}

/// Checks the correctness of the [`DoubleEndedIterator`] trait (and
/// [`Iterator`] by extension) for some value `iter`.
///
/// Note that `iter` must be a finite iterator.
pub fn double_ended_iterator<I>(iter: I) -> Result<(), IteratorError>
where
    I: DoubleEndedIterator + Clone,
    I::Item: PartialEq,
{
    iterator(iter.clone())?;

    invariants::double_ended_iterator_next_back(iter)?;

    Ok(())
}

/// Checks the correctness of the [`FusedIterator`] trait (and
/// [`Iterator`] by extension) for some value `iter`.
///
/// Note that `iter` must be a finite iterator.
pub fn fused_iterator<I>(iter: I) -> Result<(), IteratorError>
where
    I: FusedIterator + Clone,
    I::Item: PartialEq,
{
    iterator(iter.clone())?;

    invariants::fused_iterator_none_forever(iter)?;

    Ok(())
}

#[allow(dead_code)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctest;
