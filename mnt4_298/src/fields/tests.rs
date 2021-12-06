use ark_ff::{Field, One, SquareRootField, UniformRand, Zero};
use ark_std::test_rng;

use crate::*;
use ark_algebra_test_templates::{fields::*, generate_field_test};

use core::ops::{AddAssign, MulAssign, SubAssign};

generate_field_test!(mnt4_298; fq2; fq4;);
