use ark_algebra_test_templates::curves::*;
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::Zero;
use ark_std::{ops::Mul, str::FromStr};

use crate::*;

#[test]
fn test_projective_curve() {
    curve_tests::<EdwardsProjective>();

    edwards_tests::<BandersnatchParameters>();
    montgomery_conversion_test::<BandersnatchParameters>();
    sw_tests::<BandersnatchParameters>();
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
        "4257185345094557079734489188109952172285839137338142340240392707284963971010",
    )
    .unwrap();
    let f2 = Fr::from_str(
        "1617998875791656082457755819308421023664764572929977389209373068350490665160",
    )
    .unwrap();

    let g = EdwardsAffine::from_str(
        "(29627151942733444043031429156003786749302466371339015363120350521834195802525, \
         27488387519748396681411951718153463804682561779047093991696427532072116857978)",
    )
    .unwrap();
    let f1f2g = EdwardsAffine::from_str(
        "(16530491029447613915334753043669938793793987372416328257719459807614119987301, \
         42481140308370805476764840229335460092474682686441442216596889726548353970772)",
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
    montgomery_conversion_test::<BandersnatchParameters>();
}
