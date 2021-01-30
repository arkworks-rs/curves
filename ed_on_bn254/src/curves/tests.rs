use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::{bytes::FromBytes, Zero};
use ark_std::test_rng;
use core::str::FromStr;
use rand::Rng;

use crate::*;

use ark_test_templates::{curves::*, groups::*};

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
        "4691331900926794624732159288782398864809513177368446695323460897088210774597",
    )
    .unwrap();
    let f2 = Fr::from_str(
        "1305028103380024953477151132159456965337646722479526711736847301646466538045",
    )
    .unwrap();

    let g = EdwardsAffine::from_str(
        "(15863623088992515880085393097393553694825975317405843389771115419751650972659, \
         16950150798460657717958625567821834550301663161624707787222815936182638968203)",
    )
    .unwrap();
    let f1f2g = EdwardsAffine::from_str(
        "(20773645713088336957786354488799297695596635653208610804806657050882264237947, \
         19987327827845206670850937090314462639017692512983955920885166014935289314257)",
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
        "(15863623088992515880085393097393553694825975317405843389771115419751650972659, \
         16950150798460657717958625567821834550301663161624707787222815936182638968203)",
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
