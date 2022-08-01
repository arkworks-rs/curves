use ark_algebra_test_templates::{
    curves::*, generate_bilinearity_test, generate_g1_test, generate_g2_test, msm::*,
};
use ark_ec::{AffineCurve, PairingEngine};
use ark_ff::{Field, One, PrimeField};
use ark_std::{rand::Rng, test_rng};
use core::ops::MulAssign;

use crate::*;

generate_g1_test!(cp6_782; curve_tests; sw_tests;);
generate_g2_test!(cp6_782; curve_tests; sw_tests;);
generate_bilinearity_test!(CP6_782, Fq6);
