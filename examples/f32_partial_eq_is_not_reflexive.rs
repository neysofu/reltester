//! Why can't `f32` be `Eq`? Here's a counterexample to show why:

fn main() {}

#[test]
fn f64_partial_eq_is_not_reflexive() {
    assert!(reltester::invariants::eq_reflexivity(&f64::NAN).is_err());
}
