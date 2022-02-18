use ark_ff::{
    fields::{models::fp6_2over3::*, quadratic_extension::QuadExtConfig, SquareRootField},
    Field,
};
use ark_std::{rand::Rng, test_rng, One, UniformRand, Zero};

use crate::*;
use ark_algebra_test_templates::{fields::*, generate_field_test};

use core::ops::{AddAssign, MulAssign, SubAssign};

generate_field_test!(mnt6_298;);

#[test]
fn test_fq3() {
    let mut rng = test_rng();
    let a: Fq3 = rng.gen();
    let b: Fq3 = rng.gen();
    field_test(a, b);
    sqrt_field_test(a);
    frobenius_test::<Fq3, _>(Fq::characteristic(), 13);
    assert_eq!(
        a * Fq6Config::NONRESIDUE,
        <Fp6ParamsWrapper<Fq6Config>>::mul_base_field_by_nonresidue(&a)
    );
}

#[test]
fn test_fq6() {
    let mut rng = test_rng();
    let a: Fq6 = rng.gen();
    let b: Fq6 = rng.gen();
    field_test(a, b);
    frobenius_test::<Fq6, _>(Fq::characteristic(), 13);
}
