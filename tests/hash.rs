use std::{collections::BTreeSet, marker::PhantomData, path::PathBuf, rc::Rc};

use quickcheck_macros::quickcheck;

#[quickcheck]
fn hash_string(x1: String, x2: String) -> bool {
    reltester::hash(&x1, &x2).is_ok()
}

#[quickcheck]
fn hash_u32(x1: u32, x2: u32) -> bool {
    reltester::hash(&x1, &x2).is_ok()
}

#[quickcheck]
fn hash_string_tuples(x1: (String, String), x2: (String, String)) -> bool {
    reltester::hash(&x1, &x2).is_ok()
}

#[quickcheck]
fn hash_btreeset_of_units(x1: BTreeSet<()>, x2: BTreeSet<()>) -> bool {
    reltester::hash(&x1, &x2).is_ok()
}

#[quickcheck]
fn hash_path(x1: PathBuf, x2: PathBuf) -> bool {
    reltester::hash(x1.as_path(), x2.as_path()).is_ok()
}

#[test]
fn hash_array_tuples() {
    let x1 = ([1, 2, 3, 4], [5, 6, 7, 8]);
    let x2 = ([0, 0, 0, 0], [5, 6, 7, 8]);
    assert!(reltester::hash(&x1, &x2).is_ok());
}

#[test]
fn hash_phantomdata() {
    let phantom = PhantomData::<u32>::default();
    assert!(reltester::hash(&phantom, &phantom).is_ok());
}

#[test]
fn hash_rc() {
    let rc1 = Rc::new(1337);
    let rc2 = Rc::new(1337);
    let _rc2_cloned = rc2.clone();
    assert!(reltester::hash(&rc1, &rc2).is_ok());
}
