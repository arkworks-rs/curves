use ark_algebra_test_templates::{
    curves::*, generate_bilinearity_test, generate_g1_generator_raw_test, generate_g1_test,
    generate_g2_test, msm::*,
};
use ark_ec::{
    models::short_weierstrass::SWCurveConfig, AffineCurve, PairingEngine, ProjectiveCurve,
};
use ark_ff::{
    fields::{Field, PrimeField},
    One, UniformRand, Zero,
};
use ark_std::{rand::Rng, test_rng};
use core::ops::{AddAssign, MulAssign};

use crate::{g1, g2, Bls12_381, Fq, Fq12, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};

generate_g1_test!(bls12_381; curve_tests; sw_tests;);
generate_g2_test!(bls12_381; curve_tests; sw_tests;);
generate_bilinearity_test!(Bls12_381, Fq12);
generate_g1_generator_raw_test!(bls12_381, 4);

#[test]
fn test_g1_endomorphism_beta() {
    assert!(g1::BETA.pow(&[3u64]).is_one());
}

#[test]
fn test_g1_subgroup_membership_via_endomorphism() {
    let mut rng = test_rng();
    let generator = G1Projective::rand(&mut rng).into_affine();
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_g1_subgroup_non_membership_via_endomorphism() {
    let mut rng = test_rng();
    loop {
        let x = Fq::rand(&mut rng);
        let greatest = rng.gen();

        if let Some(p) = G1Affine::get_point_from_x(x, greatest) {
            if !p
                .into_projective()
                .mul_bigint(Fr::characteristic())
                .is_zero()
            {
                assert!(!p.is_in_correct_subgroup_assuming_on_curve());
                return;
            }
        }
    }
}

#[test]
fn test_g2_subgroup_membership_via_endomorphism() {
    let mut rng = test_rng();
    let generator = G2Projective::rand(&mut rng).into_affine();
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_g2_subgroup_non_membership_via_endomorphism() {
    let mut rng = test_rng();
    loop {
        let x = Fq2::rand(&mut rng);
        let greatest = rng.gen();

        if let Some(p) = G2Affine::get_point_from_x(x, greatest) {
            if !p
                .into_projective()
                .mul_bigint(Fr::characteristic())
                .is_zero()
            {
                assert!(!p.is_in_correct_subgroup_assuming_on_curve());
                return;
            }
        }
    }
}
