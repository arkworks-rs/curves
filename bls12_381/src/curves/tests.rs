use ark_algebra_test_templates::*;
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{fields::Field, One, UniformRand, Zero};
use ark_std::{rand::Rng, test_rng};

use crate::{Bls12_381, Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};

test_group!(g1; G1Projective; sw);
test_group!(g2; G2Projective; sw);
test_group!(pairing_output; ark_ec::pairing::PairingOutput<Bls12_381>; msm);
test_pairing!(pairing; crate::Bls12_381);

#[test]
fn test_g1_endomorphism_beta() {
    assert!(crate::g1::BETA.pow(&[3u64]).is_one());
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
            if !p.mul_bigint(Fr::characteristic()).is_zero() {
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
            if !p.mul_bigint(Fr::characteristic()).is_zero() {
                assert!(!p.is_in_correct_subgroup_assuming_on_curve());
                return;
            }
        }
    }
}
