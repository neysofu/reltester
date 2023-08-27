#[test]
fn f64_not_eq() {
    assert!(reltester::invariants::eq_reflexivity(&f64::NAN).is_err());
}
