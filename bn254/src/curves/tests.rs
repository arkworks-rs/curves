#![allow(unused_imports)]
use ark_ec::{models::SWModelParameters, AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::{
    fields::{Field, FpParameters, PrimeField, SquareRootField},
    One, Zero,
};
use ark_serialize::CanonicalSerialize;
use ark_std::{rand::Rng, test_rng};
use core::ops::{AddAssign, MulAssign};

use crate::{g1, g2, Bn254, Fq, Fq12, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};

use ark_algebra_test_templates::{
    curves::*, generate_bilinearity_test, generate_g1_test, generate_g2_test, groups::*, msm::*,
};

generate_g1_test!(bn254; curve_tests; sw_tests;);
generate_g2_test!(bn254; curve_tests; sw_tests;);
generate_bilinearity_test!(Bn254, Fq12);
