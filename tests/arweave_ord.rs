//! Dumb counter-example that I took from https://github.com/graphprotocol/graph-node.

use std::cmp::Ordering;

use quickcheck::{Arbitrary, Gen};
use quickcheck_macros::quickcheck;

#[derive(Clone, Debug)]
pub enum ArweaveTrigger {
    Block(u32),
    Transaction(u32),
}

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

    reltester::ord(&a, &b, &c).is_ok()
}
