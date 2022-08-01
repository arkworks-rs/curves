use ark_algebra_test_templates::{
    fields::*, generate_field_serialization_test, generate_field_test,
};
use ark_ff::{
    fields::{models::fp6_2over3::*, quadratic_extension::QuadExtConfig},
    Field, PrimeField,
};
use ark_serialize::{buffer_bit_byte_size, CanonicalSerialize};
use ark_std::{rand::Rng, test_rng, One, UniformRand, Zero};
use core::ops::{AddAssign, MulAssign, SubAssign};

use crate::*;

generate_field_test!(mnt6_753; fq3; fq6_2_on_3; mont(12, 12); );
generate_field_serialization_test!(mnt6_753;);

#[test]
fn test_fq3_more() {
    let mut rng = test_rng();
    let a: Fq3 = rng.gen();
    let b: Fq3 = rng.gen();
    field_test(a, b);
    sqrt_field_test(a);
    frobenius_test::<Fq3, _>(Fq::characteristic(), 13);
    assert_eq!(
        a * Fq6Config::NONRESIDUE,
        <Fp6ConfigWrapper<Fq6Config>>::mul_base_field_by_nonresidue(&a)
    );
}
