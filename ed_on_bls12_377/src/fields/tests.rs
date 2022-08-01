use ark_algebra_test_templates::{
    fields::*, generate_field_serialization_test, generate_field_test,
};
use ark_ff::{
    fields::{Field, PrimeField},
    One, UniformRand, Zero,
};
use ark_serialize::{buffer_bit_byte_size, CanonicalSerialize};
use ark_std::{rand::Rng, test_rng};
use core::ops::{AddAssign, MulAssign, SubAssign};

use crate::{Fq, FqConfig, Fr, FrConfig};

generate_field_test!(ed_on_bls12_377; mont(4, 4); );
generate_field_serialization_test!(ed_on_bls12_377;);
