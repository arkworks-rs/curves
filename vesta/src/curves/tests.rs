use ark_algebra_test_templates::{
    curves::{curve_tests, sw_tests},
    groups::group_test,
};
use ark_ec::AffineCurve;
use ark_std::{rand::Rng, test_rng};

use crate::{Affine, Projective, VestaParameters};

#[test]
fn test_projective_curve() {
    curve_tests::<Projective>();
    sw_tests::<VestaParameters>();
}

#[test]
fn test_projective_group() {
    let mut rng = test_rng();
    let a: Projective = rng.gen();
    let b: Projective = rng.gen();
    group_test(a, b);
}

#[test]
fn test_generator() {
    let generator = Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}
