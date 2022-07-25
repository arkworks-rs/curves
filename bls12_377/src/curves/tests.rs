use ark_algebra_test_templates::{
    curves::{curve_tests, edwards_tests, sw_tests},
    generate_bilinearity_test, generate_g1_generator_raw_test, generate_g1_test, generate_g2_test,
    msm::test_var_base_msm,
};
use ark_ec::{models::short_weierstrass::SWCurveConfig, AffineCurve, PairingEngine};
use ark_ff::{
    fields::{Field, PrimeField},
    One, Zero,
};
use ark_std::{rand::Rng, test_rng};
use core::ops::{AddAssign, MulAssign};

use crate::{g1, g2, Bls12_377, Fq, Fq12, Fr, G1Affine, G1Projective, G2Affine, G2Projective};

generate_g1_test!(bls12_377; curve_tests; sw_tests; edwards_tests;);
generate_g2_test!(bls12_377; curve_tests; sw_tests;);
generate_bilinearity_test!(Bls12_377, Fq12);
generate_g1_generator_raw_test!(bls12_377, 1);
