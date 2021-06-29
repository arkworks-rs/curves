use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::{bytes::FromBytes, Zero};
use ark_std::rand::Rng;
use ark_std::str::FromStr;
use ark_std::test_rng;

use crate::*;

use ark_algebra_test_templates::{curves::*, groups::*};

#[test]
fn test_projective_curve() {
    curve_tests::<EdwardsProjective>();

    edwards_tests::<EdwardsParameters>();
}

#[test]
fn test_projective_group() {
    let mut rng = test_rng();
    let a = rng.gen();
    let b = rng.gen();
    for _i in 0..100 {
        group_test::<EdwardsProjective>(a, b);
    }
}

#[test]
fn test_affine_group() {
    let mut rng = test_rng();
    let a: EdwardsAffine = rng.gen();
    let b: EdwardsAffine = rng.gen();
    for _i in 0..100 {
        group_test::<EdwardsAffine>(a, b);
    }
}

#[test]
fn test_generator() {
    let generator = EdwardsAffine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_conversion() {
    let mut rng = test_rng();
    let a: EdwardsAffine = rng.gen();
    let b: EdwardsAffine = rng.gen();
    let a_b = {
        use ark_ec::group::Group;
        (a + &b).double().double()
    };
    let a_b2 = (a.into_projective() + &b.into_projective())
        .double()
        .double();
    assert_eq!(a_b, a_b2.into_affine());
    assert_eq!(a_b.into_projective(), a_b2);
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
fn test_bytes() {
    let g_from_repr = EdwardsAffine::from_str(
        "(29627151942733444043031429156003786749302466371339015363120350521834195802525, \
         27488387519748396681411951718153463804682561779047093991696427532072116857978)",
    )
    .unwrap();

    let g_bytes = ark_ff::to_bytes![g_from_repr].unwrap();
    let g = EdwardsAffine::read(g_bytes.as_slice()).unwrap();
    assert_eq!(g_from_repr, g);
}

#[test]
fn test_montgomery_conversion() {
    montgomery_conversion_test::<EdwardsParameters>();
}
