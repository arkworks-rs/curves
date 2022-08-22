use ark_algebra_test_templates::curves::*;
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::Zero;
use ark_std::{ops::Mul, str::FromStr};

use crate::*;

#[test]
fn test_projective_curve() {
    curve_tests::<EdwardsProjective>();

    edwards_tests::<JubjubParameters>();
    montgomery_conversion_test::<JubjubParameters>();
    sw_tests::<JubjubParameters>();
}

#[test]
fn test_generator() {
    // edward curve
    let generator = EdwardsAffine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());

    // weierstrass curve
    let generator = SWAffine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_scalar_multiplication() {
    let f1 = Fr::from_str(
        "4691331900926794624732159288782398864809513177368446695323460897088210774597",
    )
    .unwrap();
    let f2 = Fr::from_str(
        "1305028103380024953477151132159456965337646722479526711736847301646466538045",
    )
    .unwrap();

    let g = EdwardsAffine::from_str(
        "(1158870117176967269192899343636553522971009777237254192973081388797299308391, \
         36933624999642413792569726058244472742169727126562409632889593958355839948294)",
    )
    .unwrap();
    let f1f2g = EdwardsAffine::from_str(
        "(12638652891150111215300246576936483137884466359309882317048163368620501191944, \
         38385045634663742820428406709832518145724237919360177362175527604556651918148)",
    )
    .unwrap();

    assert!(!g.is_zero());
    assert!(!f1f2g.is_zero());

    let f1g = g.mul(f1).into_affine();
    assert_eq!(g.mul(f1 * &f2).into_affine(), f1f2g);
    assert_eq!(f1g.mul(f2).into_affine(), f1f2g);
}

#[test]
fn test_montgomery_conversion() {
    montgomery_conversion_test::<JubjubParameters>();
}
