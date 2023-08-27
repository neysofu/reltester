//! Crate error types.

use thiserror::Error;

/// Represents a broken invariant of [`PartialEq`].
#[derive(Error, Debug, Clone)]
#[non_exhaustive]
pub enum PartialEqError {
    /// [`PartialEq::ne`] *MUST* always return the negation of [`PartialEq::eq`].
    #[error("PartialEq::ne MUST always return the negation of PartialEq::eq")]
    BadNe,
    /// If `A: PartialEq<B>` and `B: PartialEq<A>`, then `a == b` *MUST* imply `b == a`.
    #[error("a == b MUST imply b == a")]
    BrokeSymmetry,
    /// If `A: PartialEq<B>` and `B: PartialEq<C>` and `A: PartialEq<C>`, then
    /// `a == b && b == c` *MUST* imply `a == c`.
    #[error("a == b && b == c MUST imply a == c")]
    BrokeTransitivity,
}

/// Represents a broken invariant of [`Eq`].
///
/// Note that [`Eq`] also mandates all invariants of [`PartialEq`].
#[derive(Error, Debug, Clone)]
#[non_exhaustive]
pub enum EqError {
    /// All values must be equal to themselves.
    #[error("a == a MUST be true")]
    BrokeReflexivity,
}

/// Represents a broken invariant of [`PartialOrd`].
///
/// Note that [`PartialOrd`] also mandates all invariants of [`PartialEq`].
#[derive(Error, Debug, Clone)]
#[non_exhaustive]
pub enum PartialOrdError {
    /// [`PartialOrd::partial_cmp`] *MUST* return `Some(Ordering::Equal)` if
    /// and only if [`PartialEq::eq`] returns [`true`].
    #[error("PartialOrd::partial_cmp MUST return Some(Ordering::Equal) if and only if PartialEq::eq returns true")]
    BadPartialCmp,
    /// [`PartialOrd::lt`] *MUST* return [`true`]
    /// if and only if [`PartialOrd::partial_cmp`] returns `Some(Ordering::Less)`.
    #[error("PartialOrd::lt MUST return true if and only if PartialOrd::partial_cmp returns Some(Ordering::Less)")]
    BadLt,
    /// [`PartialOrd::le`] *MUST* return [`true`] if and only if
    /// [`PartialOrd::partial_cmp`] returns `Some(Ordering::Less)` or
    /// [`Some(Ordering::Equal)`].
    #[error("PartialOrd::le MUST return true if and only if PartialOrd::partial_cmp returns Some(Ordering::Less) or Some(Ordering::Equal)")]
    BadLe,
    /// [`PartialOrd::gt`] *MUST* return [`true`] if and only if
    /// [`PartialOrd::partial_cmp`] returns `Some(Ordering::Greater)`.
    #[error("PartialOrd::gt MUST return true if and only if PartialOrd::partial_cmp returns Some(Ordering::Greater)")]
    BadGt,
    /// [`PartialOrd::ge`] *MUST* return [`true`] if and only if
    /// [`PartialOrd::partial_cmp`] returns `Some(Ordering::Greater)` or
    /// `Some(Ordering::Equal)`.
    #[error("PartialOrd::ge MUST return true if and only if PartialOrd::partial_cmp returns Some(Ordering::Greater) or Some(Ordering::Equal)")]
    BadGe,
    /// If `a > b`, then `b < a` *MUST* be true.
    #[error("If a > b, then b < a MUST be true")]
    BrokeDuality,
    /// If `a > b` and `b > c`, then `a > c` *MUST* be true. The same must hold true for `<`.
    #[error("If a > b and b > c, then a > c MUST be true. The same must hold true for <")]
    BrokeTransitivity,
}

/// Represents a broken invariant of [`Ord`].
///
/// Note that [`Ord`] also mandates all invariants of [`PartialOrd`] and [`Eq`].
#[derive(Error, Debug, Clone)]
#[non_exhaustive]
pub enum OrdError {
    /// [`Ord::cmp`] *MUST* always return `Some(PartialOrd::partial_cmp())`.
    #[error("`cmp` and `partial_cmp` are not consistent")]
    BadCmp,
    /// [`Ord::cmp`] and [`Ord::max`] are not consistent.
    #[error("`cmp` and `max` are not consistent")]
    BadMax,
    /// [`Ord::cmp`] and [`Ord::min`] are not consistent.
    #[error("`cmp` and `min` are not consistent")]
    BadMin,
    /// [`Ord::cmp`] and [`Ord::clamp`] are not consistent.
    #[error("`cmp` and `clamp` are not consistent")]
    BadClamp,
}

/// Represents a broken invariant of [`Hash`].
#[derive(Error, Debug, Clone)]
#[non_exhaustive]
pub enum HashError {
    /// Equal values *MUST* have equal hash values.
    #[error("Equal values MUST have equal hash values")]
    EqualButDifferentHashes,
    /// When two values are different (as defined by [`PartialEq::ne`]), neither
    /// of the two hash outputs can be a prefix of the other. See
    /// <https://doc.rust-lang.org/std/hash/trait.Hash.html#prefix-collisions>
    /// for more information.
    #[error("When two values are different, one of the two hash outputs CAN NOT be a prefix of the other")]
    PrefixCollision,
}

/// Represents a broken invariant of [`Iterator`].
#[derive(Error, Debug, Clone)]
#[non_exhaustive]
pub enum IteratorError {
    /// [`Iterator::size_hint`] *MUST* always provide correct lower and upper
    /// bounds.
    #[error("Iterator::size_hint MUST always provide correct lower and upper bounds")]
    BadSizeHint,
    /// [`Iterator::count`] *MUST* be consistent with the actual number of
    /// elements returned by [`Iterator::next`].
    #[error(
        "Iterator::count MUST be consistent with the actual number of elements returned by .next()"
    )]
    BadCount,
    /// [`Iterator::last`] *MUST* be equal to the last element of the
    /// [`Vec`] resulting from [`Iterator::collect`].
    #[error(".last() MUST be equal to the last element of the Vec<_> resulting from .collect()")]
    BadLast,
    /// [`DoubleEndedIterator::next_back`] *MUST* return the same values as
    /// [`Iterator::next`], just in reverse order, and it MUST NOT return
    /// different values.
    #[error("DoubleEndedIterator::next_back() MUST return the same values as .next(), but in reverse order")]
    BadNextBack,
    /// [`FusedIterator`](core::iter::FusedIterator) *MUST* return [`None`]
    /// indefinitely after exhaustion.
    #[error("FusedIterator MUST return None indefinitely after exhaustion")]
    FusedIteratorReturnedSomeAfterExhaustion,
}

/// Represents a broken invariant of a tested trait implementation.
#[derive(Error, Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    PartialEq(#[from] PartialEqError),
    #[error(transparent)]
    Eq(#[from] EqError),
    #[error(transparent)]
    PartiaOrd(#[from] PartialOrdError),
    #[error(transparent)]
    Ord(#[from] OrdError),
    #[error(transparent)]
    Hash(#[from] HashError),
    #[error(transparent)]
    Iterator(#[from] IteratorError),
}
