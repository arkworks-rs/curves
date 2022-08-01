use ark_algebra_test_templates::{
    curves::*, generate_bilinearity_test, generate_g1_test, generate_g2_test, msm::*,
};
use ark_ec::{AffineCurve, PairingEngine};
use ark_ff::{
    fields::{Field, PrimeField},
    One,
};
use ark_std::{rand::Rng, test_rng};
use core::ops::MulAssign;

use crate::{g1, g2, Bn254, Fq12, Fr, G1Affine, G1Projective, G2Affine, G2Projective};

generate_g1_test!(bn254; curve_tests; sw_tests;);
generate_g2_test!(bn254; curve_tests; sw_tests;);
generate_bilinearity_test!(Bn254, Fq12);
