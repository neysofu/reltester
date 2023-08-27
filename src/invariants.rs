//! Granular checkers for specific trait invariants. Only use these if you
//! implement [`PartialEq`] and [`PartialOrd`] with a non-`Self` type parameter
//! and you can't satisfy the type bounds of the main helper functions.

use std::{
    cmp::{max_by, min_by, Ordering},
    hash::{Hash, Hasher},
    iter::FusedIterator,
};

use crate::error::*;

/// Checks that [`PartialEq::eq`] and [`PartialEq::ne`] are strict inverses.
///
/// This is guaranteed by default method implementations but may be broken
/// by non-default method implementations.
pub fn partial_eq_methods_consistency<A, B>(a: &A, b: &B) -> Result<(), PartialEqError>
where
    A: PartialEq<B>,
{
    if (a == b) != !(a != b) {
        return Err(PartialEqError::BadNe);
    }

    Ok(())
}

/// Checks that [`PartialEq`] is a
/// [symmetric relation](https://en.wikipedia.org/wiki/Symmetric_relation).
pub fn partial_eq_symmetry<A, B>(a: &A, b: &B) -> Result<(), PartialEqError>
where
    A: PartialEq<B>,
    B: PartialEq<A>,
{
    if (a == b) != (b == a) {
        return Err(PartialEqError::BrokeSymmetry);
    }

    Ok(())
}

/// Checks that [`PartialEq`] is a [transitive
/// relation](https://en.wikipedia.org/wiki/Transitive_relation).
pub fn partial_eq_transitivity<A, B, C>(a: &A, b: &B, c: &C) -> Result<(), PartialEqError>
where
    A: PartialEq<B> + PartialEq<C>,
    B: PartialEq<C>,
{
    if a == b && b == c && a != c {
        return Err(PartialEqError::BrokeTransitivity);
    }

    Ok(())
}

/// Checks that [`PartialEq`] is a [reflexive
/// relation](https://en.wikipedia.org/wiki/Reflexive_relation).
///
/// Note that [`PartialEq`] alone does **not** require reflexivity, [`Eq`]
/// does.
pub fn eq_reflexivity<A>(a: &A) -> Result<(), PartialEqError>
where
    A: PartialEq<A>,
{
    if a != a {
        return Err(PartialEqError::BrokeTransitivity);
    }

    Ok(())
}

/// Checks that [`PartialOrd`] methods are implemented consistently with
/// each other.
///
/// This is guaranteed by default method implementations but may be broken
/// by non-default method implementations.
pub fn partial_ord_methods_consistency<A, B>(a: &A, b: &B) -> Result<(), PartialOrdError>
where
    A: PartialOrd<B>,
{
    if (a == b) != (a.partial_cmp(b) == Some(Ordering::Equal)) {
        return Err(PartialOrdError::BadPartialCmp);
    }
    if (a < b) != (a.partial_cmp(b) == Some(Ordering::Less)) {
        return Err(PartialOrdError::BadLt);
    }
    if (a > b) != (a.partial_cmp(b) == Some(Ordering::Greater)) {
        return Err(PartialOrdError::BadGt);
    }
    if (a <= b) != ((a < b) || (a == b)) {
        return Err(PartialOrdError::BadLe);
    }
    if (a >= b) != ((a > b) || (a == b)) {
        return Err(PartialOrdError::BadGe);
    }

    Ok(())
}

/// Checks that [`PartialOrd`] respects
/// [duality](https://en.wikipedia.org/wiki/Duality_(order_theory)) (i.e. `a
/// > b` iff `b < a`).
pub fn partial_ord_duality<A, B>(a: &A, b: &B) -> Result<(), PartialOrdError>
where
    A: PartialOrd<B>,
    B: PartialOrd<A>,
{
    if ((a < b) != (b > a)) && ((a > b) != (b < a)) {
        return Err(PartialOrdError::BrokeDuality);
    }

    Ok(())
}

/// Checks that [`PartialOrd`] is a [transitive
/// relation](https://en.wikipedia.org/wiki/Transitive_relation).
pub fn partial_ord_transitivity<A, B, C>(a: &A, b: &B, c: &C) -> Result<(), PartialOrdError>
where
    A: PartialOrd<B> + PartialOrd<C>,
    B: PartialOrd<C>,
{
    if a < b && b < c && !(a < c) {
        return Err(PartialOrdError::BrokeTransitivity);
    }
    if a > b && b > c && !(a > c) {
        return Err(PartialOrdError::BrokeTransitivity);
    }

    Ok(())
}

/// Checks that [`Ord`] methods are implemented consistently with each other.
///
/// This is guaranteed by default method implementations but may be broken
/// by non-default method implementations.
pub fn ord_methods_consistency<T>(a: &T, b: &T, c: &T) -> Result<(), OrdError>
where
    T: Ord,
{
    if a.partial_cmp(b) != Some(a.cmp(b)) {
        return Err(OrdError::BadCmp);
    }
    if a.max(b) != max_by(a, b, |x, y| x.cmp(y)) {
        return Err(OrdError::BadMax);
    }
    if a.min(b) != min_by(a, b, |x, y| x.cmp(y)) {
        return Err(OrdError::BadMin);
    }

    // clamp
    let min = b.min(c);
    let max = b.max(c);
    let clamped = a.clamp(min, max);
    if clamped < min || clamped > max {
        return Err(OrdError::BadClamp);
    }

    Ok(())
}

/// Checks that the output of [`Hash`] is the same for equal values, and
/// different for different values.
///
/// See what the `std`
/// [docs](https://doc.rust-lang.org/std/hash/trait.Hash.html#hash-and-eq) have
/// to say about this invariant.
pub fn hash_consistency_with_eq<K>(a: &K, b: &K) -> Result<(), HashError>
where
    K: Hash + Eq + ?Sized,
{
    let hasher_output_equality = hasher_output(a) == hasher_output(b);
    let equality = a == b;

    if hasher_output_equality != equality {
        return Err(HashError::EqualButDifferentHashes);
    }

    Ok(())
}

/// Checks that neither of the outputs of [`Hash`] of two different values is a
/// prefix of the other.
///
/// See what the `std`
/// [docs](https://doc.rust-lang.org/std/hash/trait.Hash.html#prefix-collisions) have
/// to say about this invariant.
pub fn hash_prefix_collision<K>(a: &K, b: &K) -> Result<(), HashError>
where
    K: Hash + Eq + ?Sized,
{
    if a != b {
        let hasher_output_a = hasher_output(a);
        let hasher_output_b = hasher_output(b);

        if hasher_output_a.starts_with(&hasher_output_b)
            || hasher_output_b.starts_with(&hasher_output_a)
        {
            return Err(HashError::PrefixCollision);
        }
    }

    Ok(())
}

/// Checks that [`Iterator::size_hint`] provides correct lower and upper bounds
/// which are consistent with the true value of [`Iterator::count`].
pub fn iterator_size_hint<I>(iter: I) -> Result<(), IteratorError>
where
    I: Iterator,
{
    let size_hint = iter.size_hint();
    let count = iter.count();

    if size_hint.0 > count {
        return Err(IteratorError::BadSizeHint);
    } else if let Some(upper_bound) = size_hint.1 {
        if upper_bound < count {
            return Err(IteratorError::BadSizeHint);
        }
    }

    Ok(())
}

/// Checks that [`Iterator::count`] returns the same value as the length of the
/// [`Vec`] obtained from [`Iterator::collect`].
pub fn iterator_count<I>(iter: I) -> Result<(), IteratorError>
where
    I: Iterator + Clone,
{
    let count = iter.clone().count();
    let collected = iter.collect::<Vec<_>>();

    if count != collected.len() {
        return Err(IteratorError::BadCount);
    }

    Ok(())
}

/// Checks that [`Iterator::last`] returns the same value as the last element of
/// the [`Vec`] obtained from [`Iterator::collect`].
pub fn iterator_last<I>(iter: I) -> Result<(), IteratorError>
where
    I: Iterator + Clone,
    I::Item: PartialEq,
{
    let last = iter.clone().last();
    let collected = iter.collect::<Vec<_>>();

    if last.as_ref() != collected.last() {
        return Err(IteratorError::BadLast);
    }

    Ok(())
}

/// Checks that alternating random calls to [`Iterator::next`] and
/// [`DoubleEndedIterator::next_back`] results in the same sequence as the
/// [`Vec`] obtained from [`Iterator::collect`].
pub fn double_ended_iterator_next_back<I>(mut iter: I) -> Result<(), IteratorError>
where
    I: DoubleEndedIterator + Clone,
    I::Item: PartialEq,
{
    let collected = iter.clone().collect::<Vec<_>>();

    let mut from_start = vec![];
    let mut from_end = vec![];
    loop {
        if rand::random() {
            if let Some(item) = iter.next() {
                from_start.push(item);
            } else {
                break;
            }
        } else {
            if let Some(item) = iter.next_back() {
                from_end.push(item);
            } else {
                break;
            }
        }
    }

    let assembled = from_start
        .into_iter()
        .chain(from_end.into_iter().rev())
        .collect::<Vec<_>>();

    if assembled != collected {
        return Err(IteratorError::BadNextBack);
    }

    Ok(())
}

/// Checks that [`FusedIterator`] returns [`None`] for a large number of times after
/// returning [`None`] for the first time.
pub fn fused_iterator_none_forever<I>(mut iter: I) -> Result<(), IteratorError>
where
    I: FusedIterator + Clone,
{
    let mut count = 0;
    while iter.next().is_some() {
        count += 1;
    }

    // How many times does it make sense to keep going to have decent confidence
    // it will return `None` forever? Hard to say. I'm going with .count() + 1
    // in case the iterator "goes back" or something.
    for _ in 0..count + 1 {
        if iter.next().is_some() {
            return Err(IteratorError::FusedIteratorReturnedSomeAfterExhaustion);
        }
    }

    Ok(())
}

fn hasher_output<K>(item: &K) -> Vec<u8>
where
    K: Hash + ?Sized,
{
    struct NoHasher(Vec<u8>);

    impl Hasher for NoHasher {
        fn finish(&self) -> u64 {
            0
        }

        fn write(&mut self, bytes: &[u8]) {
            self.0.extend_from_slice(bytes);
        }
    }

    let mut hasher = NoHasher(vec![]);
    item.hash(&mut hasher);
    hasher.0
}
