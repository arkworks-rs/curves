use ark_ec::{AffineCurve, PairingEngine};
use ark_ff::{Field, One, PrimeField};
use ark_std::{rand::Rng, test_rng};

use crate::*;

use ark_algebra_test_templates::{
    curves::*, generate_bilinearity_test, generate_g1_test, generate_g2_test, groups::*, msm::*,
};

use core::ops::MulAssign;

generate_g1_test!(bw6_761; curve_tests; sw_tests;);
generate_g2_test!(bw6_761; curve_tests; sw_tests;);
generate_bilinearity_test!(BW6_761, Fq6);
