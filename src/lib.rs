//! **Rel**ation **tester** is a small testing utility for automatically checking the correctness of
//! [`PartialEq`], [`Eq`],
//! [`PartialOrd`], and [`Ord`] implementations. It's most useful when used in
//! conjuction with [`quickcheck`](https://github.com/BurntSushi/quickcheck) or
//! some other property-based testing framework.
//!
//! # Rationale
//!
//! Imagine a scenario where you have a type `Foo` with a custom implementation of either [`PartialEq`], [`Eq`],
//! [`PartialOrd`], or [`Ord`]. By "custom" we mean hand-written as opposed to
//! derived. The Rust compiler alone cannot verify the correctness of these
//! implementations and thus it is up to you, the programmer, to uphold certain
//! invariants about the specific [binary
//! relation](https://en.wikipedia.org/wiki/Binary_relation) that you're
//! implementing. For example, if you implement [`PartialEq`] for `Foo`, you
//! must guarantee that `foo1 == foo2` implies `foo2 == foo1` (*symmetry*).
//!
//! This is what `reltester` is for. Rather than learning all subtle details of [`PartialEq`], [`Eq`],
//! [`PartialOrd`], and [`Ord`], you can write some tests that will
//! automatically check these invariants for you.
//!
//! # How to use
//!
//! 1. Write some tests that generate random values of the type you wish to
//! test. You can do this by hand or using crates such as
//! [`quickcheck`](https://github.com/BurntSushi/quickcheck) and
//! [`proptest`](https://github.com/proptest-rs/proptest).
//! 2. Based on the traits that your type implements, call the appropriate checker:
//!
//!    - [`reltester::eq`](eq) for [`Eq`];
//!    - [`reltester::ord`](ord) for [`Ord`];
//!    - [`reltester::partial_eq`](partial_eq) for [`PartialEq`];
//!    - [`reltester::partial_ord`](partial_ord) for [`PartialOrd`].
//!
//!    All of these functions take three arguments of the same type: `a`, `b`, and
//! `c`. This is because it takes up to three values to test some invariants.
//!
//! ## Multi-type relations: `Foo: PartialEq<Bar>` and `Foo: PartialOrd<Bar>`
//!
//! In some cases your [`PartialEq`] and [`PartialOrd`] implementations
//! may use a non-`Self` type parameter. (Note: [`Eq`] and [`Ord`] don't accept
//! type parameters and this use case doesn't apply to them.) `reltester`
//! supports this use case and exposes granular invariant checking functions in
//! the [`invariants`] module with more lax type constraints.
//!
//! # Examples
//!
//! ```rust
//! use reltester;
//! use quickcheck_macros::quickcheck;
//!
//! #[quickcheck]
//! fn test_f32(a: f32, b: f32, c: f32) -> bool {
//!     // Let's check if `f32` implements `PartialEq` and `PartialOrd` correctly
//!     // (spoiler: it does)
//!     reltester::partial_eq(&a, &b, &c).is_ok()
//!         && reltester::partial_ord(&a, &b, &c).is_ok()
//! }
//! ```
//!
//! # TL;DR invariants
//!
//! Chances are you don't need to concern yourself with the mathematical definitions of
//! comparison traits, as long as your implementations are sensible. They are
//! listed here only for the sake of completeness.
//!
//! - [`PartialEq`] requires **symmetry** and **transitivity** of `==` ([partial equivalence relation](https://en.wikipedia.org/wiki/Partial_equivalence_relation)).
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
//!   `==`; **transitivity** and **duality** of `>` and `<`; and must be **trichotomous**[^1]. Just like
//!   [`PartialOrd`], the mathematical definition of [`Ord`] is a bit open to
//!   interpretation, though it's generally understood to mean [total
//!   order](https://en.wikipedia.org/wiki/Total_order#Strict_and_non-strict_total_orders).
//!
//! [^1]: Trichotomy is a corollary that follows from the definitions of `>`,
//! `==`, and `<` based on [`Ordering`].

#![allow(clippy::eq_op, clippy::double_comparisons)]

use std::cmp::{max_by, min_by, Ordering};
use thiserror::Error;

/// Represents a broken invariant of a tested trait implementation.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// [`PartialEq::eq`] and [`PartialEq::ne`] are not consistent.
    #[error("`eq` and `ne` are not consistent")]
    NotConsistentEqNe,
    /// [`PartialOrd::partial_cmp`] and [`PartialEq::eq`] are not consistent.
    #[error("`partial_cmp` and `eq` are not consistent")]
    NotConsistentPartialCmpEq,
    /// [`PartialOrd::partial_cmp`] and [`PartialOrd::lt`] are not consistent.
    #[error("`partial_cmp` and `lt` are not consistent")]
    NotConsistentPartialCmpLt,
    /// [`PartialOrd::partial_cmp`] and [`PartialOrd::le`] are not consistent.
    #[error("`partial_cmp` and `le` are not consistent")]
    NotConsistentPartialCmpLe,
    /// [`PartialOrd::partial_cmp`] and [`PartialOrd::gt`] are not consistent.
    #[error("`partial_cmp` and `gt` are not consistent")]
    NotConsistentPartialCmpGt,
    /// [`PartialOrd::partial_cmp`] and [`PartialOrd::ge`] are not consistent.
    #[error("`partial_cmp` and `ge` are not consistent")]
    NotConsistentPartialCmpGe,
    /// [`Ord::cmp`] and [`PartialOrd::partial_cmp`] are not consistent.
    #[error("`cmp` and `partial_cmp` are not consistent")]
    NotConsistentCmpPartialCmp,
    /// [`Ord::cmp`] and [`Ord::max`] are not consistent.
    #[error("`cmp` and `max` are not consistent")]
    NotConsistentCmpMax,
    /// [`Ord::cmp`] and [`Ord::min`] are not consistent.
    #[error("`cmp` and `min` are not consistent")]
    NotConsistentCmpMin,
    /// [`Ord::cmp`] and [`Ord::clamp`] are not consistent.
    #[error("`cmp` and `clamp` are not consistent")]
    NotConsistentCmpClamp,
    /// Reflexivity is broken in this [`Eq`] implementation.
    #[error("`Eq` is not reflexive")]
    BrokeReflexivity,
    /// Symmetry is broken in this [`PartialEq`] implementation.
    #[error("`PartialEq` is not symmetric")]
    BrokeSymmetry,
    /// Transitivity is broken in this [`PartialEq`] or [`PartialOrd`]
    /// implementation.
    #[error("`PartialEq` or `PartialOrd` is not transitive")]
    BrokeTransitivity,
    /// Duality is broken in this [`PartialOrd`] implementation.
    #[error("`PartialOrd` is not dual")]
    BrokeDuality,
}

/// Checks the correctness of the [`Ord`] trait (and [`PartialOrd`] by extension)
/// for some values.
pub fn ord<T>(a: &T, b: &T, c: &T) -> Result<(), Error>
where
    T: Ord,
{
    eq(a, b, c)?;
    partial_ord(a, b, c)?;

    invariants::ord_methods_consistency(a, b, c)?;

    Ok(())
}

/// Checks the correctness of the [`PartialOrd`] trait for some values.
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
pub fn eq<T>(a: &T, b: &T, c: &T) -> Result<(), Error>
where
    T: Eq,
{
    partial_eq(a, b, c)?;

    // `Eq` is just like `PartialEq`, except it also requires reflexivity.
    invariants::eq_reflexivity(a)?;

    Ok(())
}

/// Checks the correctness of the [`PartialEq`] trait
/// for some values.
pub fn partial_eq<T>(a: &T, b: &T, c: &T) -> Result<(), Error>
where
    T: PartialEq,
{
    invariants::partial_eq_methods_consistency(a, b)?;
    invariants::partial_eq_symmetry(a, b)?;
    invariants::partial_eq_transitivity(a, b, c)?;

    Ok(())
}

/// Granular checkers for specific trait invariants. Only use these if you
/// implement [`PartialEq`] and [`PartialOrd`] with a non-`Self` type parameter.
pub mod invariants {
    use super::*;

    /// Checks that [`PartialEq::eq`] and [`PartialEq::ne`] are strict inverses.
    ///
    /// This is guaranteed by default method implementations but may be broken
    /// by non-default method implementations.
    pub fn partial_eq_methods_consistency<A, B>(a: &A, b: &B) -> Result<(), Error>
    where
        A: PartialEq<B>,
    {
        if (a == b) != !(a != b) {
            return Err(Error::NotConsistentEqNe);
        }

        Ok(())
    }

    /// Checks that [`PartialEq`] is a
    /// [symmetric relation](https://en.wikipedia.org/wiki/Symmetric_relation).
    pub fn partial_eq_symmetry<A, B>(a: &A, b: &B) -> Result<(), Error>
    where
        A: PartialEq<B>,
        B: PartialEq<A>,
    {
        if (a == b) != (b == a) {
            return Err(Error::BrokeSymmetry);
        }

        Ok(())
    }

    /// Checks that [`PartialEq`] is a [transitive
    /// relation](https://en.wikipedia.org/wiki/Transitive_relation).
    pub fn partial_eq_transitivity<A, B, C>(a: &A, b: &B, c: &C) -> Result<(), Error>
    where
        A: PartialEq<B> + PartialEq<C>,
        B: PartialEq<C>,
    {
        if a == b && b == c && a != c {
            return Err(Error::BrokeTransitivity);
        }

        Ok(())
    }

    /// Checks that [`PartialEq`] is a [reflexive
    /// relation](https://en.wikipedia.org/wiki/Reflexive_relation).
    ///
    /// Note that [`PartialEq`] alone does **not** require reflexivity, [`Eq`] does.
    pub fn eq_reflexivity<A>(a: &A) -> Result<(), Error>
    where
        A: PartialEq<A>,
    {
        if a != a {
            return Err(Error::BrokeReflexivity);
        }

        Ok(())
    }

    /// Checks that [`PartialOrd`] methods are implemented consistently with each other.
    ///
    /// This is guaranteed by default method implementations but may be broken
    /// by non-default method implementations.
    pub fn partial_ord_methods_consistency<A, B>(a: &A, b: &B) -> Result<(), Error>
    where
        A: PartialOrd<B>,
    {
        partial_eq_methods_consistency(a, b)?;

        if (a == b) != (a.partial_cmp(b) == Some(Ordering::Equal)) {
            return Err(Error::NotConsistentPartialCmpEq);
        }
        if (a < b) != (a.partial_cmp(b) == Some(Ordering::Less)) {
            return Err(Error::NotConsistentPartialCmpLt);
        }
        if (a > b) != (a.partial_cmp(b) == Some(Ordering::Greater)) {
            return Err(Error::NotConsistentPartialCmpGt);
        }
        if (a <= b) != ((a < b) || (a == b)) {
            return Err(Error::NotConsistentPartialCmpLe);
        }
        if (a >= b) != ((a > b) || (a == b)) {
            return Err(Error::NotConsistentPartialCmpGe);
        }

        Ok(())
    }

    /// TODO
    pub fn partial_ord_duality<A, B>(a: &A, b: &B) -> Result<(), Error>
    where
        A: PartialOrd<B>,
        B: PartialOrd<A>,
    {
        if ((a < b) != (b > a)) && ((a > b) != (b < a)) {
            return Err(Error::BrokeDuality);
        }

        Ok(())
    }

    /// Checks that [`PartialOrd`] is a [transitive
    /// relation](https://en.wikipedia.org/wiki/Transitive_relation).
    pub fn partial_ord_transitivity<A, B, C>(a: &A, b: &B, c: &C) -> Result<(), Error>
    where
        A: PartialOrd<B> + PartialOrd<C>,
        B: PartialOrd<C>,
    {
        partial_eq_transitivity(a, b, c)?;

        if a < b && b < c && !(a < c) {
            return Err(Error::BrokeTransitivity);
        }
        if a > b && b > c && !(a > c) {
            return Err(Error::BrokeTransitivity);
        }

        Ok(())
    }

    /// Checks that [`Ord`] methods are implemented consistently with each other.
    ///
    /// This is guaranteed by default method implementations but may be broken
    /// by non-default method implementations.
    pub fn ord_methods_consistency<T>(a: &T, b: &T, _c: &T) -> Result<(), Error>
    where
        T: Ord,
    {
        if a.partial_cmp(b) != Some(a.cmp(b)) {
            return Err(Error::NotConsistentCmpPartialCmp);
        }
        if a.max(b) != max_by(a, b, |x, y| x.cmp(y)) {
            return Err(Error::NotConsistentCmpMax);
        }
        if a.min(b) != min_by(a, b, |x, y| x.cmp(y)) {
            return Err(Error::NotConsistentCmpMin);
        }
        // TODO: clamp

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;
    use std::net::IpAddr;

    #[quickcheck]
    fn correctness_u32(a: u32, b: u32, c: u32) -> bool {
        eq(&a, &b, &c).is_ok() && ord(&a, &b, &c).is_ok()
    }

    #[quickcheck]
    fn correctness_f32(a: f32, b: f32, c: f32) -> bool {
        partial_eq(&a, &b, &c).is_ok() && partial_ord(&a, &b, &c).is_ok()
    }

    #[test]
    fn f64_not_eq() {
        assert!(invariants::eq_reflexivity(&f64::NAN).is_err());
    }

    #[quickcheck]
    fn correctness_ip_address(a: IpAddr, b: IpAddr, c: IpAddr) -> bool {
        eq(&a, &b, &c).is_ok() && ord(&a, &b, &c).is_ok()
    }

    #[derive(Clone, Debug)]
    pub enum ArweaveTrigger {
        Block(u32),
        Transaction(u32),
    }

    /// Dumb counter-example that I took from https://github.com/graphprotocol/graph-node.
    #[quickcheck]
    #[should_panic]
    fn arweave_is_incorrect(a: ArweaveTrigger, b: ArweaveTrigger, c: ArweaveTrigger) -> bool {
        impl Arbitrary for ArweaveTrigger {
            fn arbitrary(g: &mut Gen) -> Self {
                if bool::arbitrary(g) {
                    ArweaveTrigger::Block(u32::arbitrary(g))
                } else {
                    ArweaveTrigger::Transaction(u32::arbitrary(g))
                }
            }
        }

        impl PartialEq for ArweaveTrigger {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::Block(a_ptr), Self::Block(b_ptr)) => a_ptr == b_ptr,
                    (Self::Transaction(a_tx), Self::Transaction(b_tx)) => a_tx == b_tx,
                    _ => false,
                }
            }
        }

        impl Eq for ArweaveTrigger {}

        impl PartialOrd for ArweaveTrigger {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for ArweaveTrigger {
            fn cmp(&self, other: &Self) -> Ordering {
                match (self, other) {
                    // Keep the order when comparing two block triggers
                    (Self::Block(..), Self::Block(..)) => Ordering::Equal,

                    // Block triggers always come last
                    (Self::Block(..), _) => Ordering::Greater,
                    (_, Self::Block(..)) => Ordering::Less,

                    // Execution outcomes have no intrinsic ordering information so we keep the order in
                    // which they are included in the `txs` field of `Block`.
                    (Self::Transaction(..), Self::Transaction(..)) => Ordering::Equal,
                }
            }
        }

        ord(&a, &b, &c).is_ok()
    }
}
