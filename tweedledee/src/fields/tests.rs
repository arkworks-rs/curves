use ark_ff::test_rng;
use rand::Rng;

use crate::*;

use ark_curve_tests::fields::*;

#[test]
fn test_fp() {
    let mut rng = test_rng();
    let a: Fp = rng.gen();
    let b: Fp = rng.gen();
    field_test(a, b);
    sqrt_field_test(a);
    primefield_test::<Fp>();
}

#[test]
fn test_fq() {
    let mut rng = test_rng();
    let a: Fq = rng.gen();
    let b: Fq = rng.gen();
    field_test(a, b);
    sqrt_field_test(a);
    primefield_test::<Fq>();
}
