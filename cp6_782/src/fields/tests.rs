use ark_ff::{Field, One, PrimeField, SquareRootField, UniformRand, Zero};
use ark_serialize::{buffer_bit_byte_size, CanonicalSerialize};
use ark_std::{rand::Rng, test_rng};

use crate::*;

use ark_algebra_test_templates::{
    fields::*, generate_field_serialization_test, generate_field_test,
};

use core::ops::{AddAssign, MulAssign, SubAssign};

generate_field_test!(cp6_782; fq3; fq6;);
generate_field_serialization_test!(cp6_782;);
