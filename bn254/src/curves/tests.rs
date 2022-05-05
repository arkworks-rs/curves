use ark_algebra_test_templates::{
    curves::*, generate_bilinearity_test, generate_g1_test, generate_g2_test, generate_glv_test,
    groups::*, msm::*,
};
use ark_ec::{AffineCurve, PairingEngine};
use ark_ff::{
    fields::{Field, PrimeField},
    One,
};
use ark_std::{rand::Rng, test_rng};
use core::ops::MulAssign;

use crate::{
    g1, g1::Parameters as G1Parameters, g2, g2::Parameters as G2Parameters, Bn254, Fq12, Fr,
    G1Affine, G1Projective, G2Affine, G2Projective,
};

generate_g1_test!(bn254; curve_tests; sw_tests;);
generate_g2_test!(bn254; curve_tests; sw_tests;);
generate_bilinearity_test!(Bn254, Fq12);
generate_glv_test!(G1Parameters, G2Parameters);
