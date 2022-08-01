use ark_algebra_test_templates::{
    fields::*, generate_field_serialization_test, generate_field_test,
};
use ark_ff::{Field, One, PrimeField, UniformRand, Zero};
use ark_serialize::{buffer_bit_byte_size, CanonicalSerialize};
use ark_std::{rand::Rng, test_rng};
use core::ops::{AddAssign, MulAssign, SubAssign};

use crate::*;

generate_field_test!(mnt4_298; fq2; fq4; mont(5, 5); );
generate_field_serialization_test!(mnt4_298;);
