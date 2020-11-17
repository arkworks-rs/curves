#![allow(unused_imports)]
use ark_ff::{
    fields::{Field, FpParameters, PrimeField, SquareRootField},
    test_rng, One, Zero,
};
use ark_serialize::CanonicalSerialize;

use ark_ec::{models::SWModelParameters, AffineCurve, PairingEngine, ProjectiveCurve};
use core::ops::{AddAssign, MulAssign};
use rand::Rng;

use crate::{Affine, Projective, TweedledumParameters};

use ark_curve_tests::{
    curves::{curve_tests, sw_tests},
    groups::group_test,
};

#[test]
fn test_projective_curve() {
    curve_tests::<Projective>();

    // sw_tests::<TweedledumParameters>();
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
