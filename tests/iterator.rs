use std::collections::BTreeSet;

use quickcheck_macros::quickcheck;

#[quickcheck]
fn iterator_chars(x: String) -> bool {
    reltester::iterator(x.char_indices()).is_ok()
}

#[quickcheck]
fn iterator_vec_of_strings(x: Vec<String>) -> bool {
    reltester::double_ended_iterator(x.iter()).is_ok()
        && reltester::fused_iterator(x.iter()).is_ok()
}

#[quickcheck]
fn iterator_btreeset_of_u32(x: BTreeSet<u32>) -> bool {
    reltester::double_ended_iterator(x.iter()).is_ok()
        && reltester::fused_iterator(x.iter()).is_ok()
}
