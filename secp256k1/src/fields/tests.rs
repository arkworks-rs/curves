use std::str::FromStr;
use crate::{Fq, Fr};
use ark_algebra_test_templates::*;
use ark_algebra_test_templates::num_bigint::BigUint;
use ark_ff::{MontFp, PrimeField};

test_field!(fr; Fr; mont_prime_field);
test_field!(fq; Fq; mont_prime_field);