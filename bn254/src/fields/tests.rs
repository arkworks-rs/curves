use ark_ff::{
    biginteger::{BigInteger, BigInteger256},
    fields::{
        fp6_3over2::Fp6Parameters, FftField, FftParameters, Field, FpParameters, PrimeField,
        SquareRootField,
    },
    One, UniformRand, Zero,
};
use ark_serialize::{buffer_bit_byte_size, CanonicalSerialize};
use ark_std::{rand::Rng, test_rng};
use core::{
    cmp::Ordering,
    ops::{AddAssign, MulAssign, SubAssign},
};

use crate::{Fq, Fq12, Fq2, Fq6, Fq6Parameters, FqParameters, Fr};
use ark_algebra_test_templates::{
    fields::*, generate_field_serialization_test, generate_field_test,
};

generate_field_test!(bn254; fq2; fq6; fq12;);
generate_field_serialization_test!(bn254; fq2; fq6; fq12;);

#[test]
fn test_fq_repr_from() {
    assert_eq!(BigInteger256::from(100), BigInteger256([100, 0, 0, 0]));
}

#[test]
fn test_fq_repr_is_odd() {
    assert!(!BigInteger256::from(0).is_odd());
    assert!(BigInteger256::from(0).is_even());
    assert!(BigInteger256::from(1).is_odd());
    assert!(!BigInteger256::from(1).is_even());
    assert!(!BigInteger256::from(324834872).is_odd());
    assert!(BigInteger256::from(324834872).is_even());
    assert!(BigInteger256::from(324834873).is_odd());
    assert!(!BigInteger256::from(324834873).is_even());
}

#[test]
fn test_fq_repr_is_zero() {
    assert!(BigInteger256::from(0).is_zero());
    assert!(!BigInteger256::from(1).is_zero());
    assert!(!BigInteger256([0, 0, 1, 0]).is_zero());
}

#[test]
fn test_fq_repr_num_bits() {
    let mut a = BigInteger256::from(0);
    assert_eq!(0, a.num_bits());
    a = BigInteger256::from(1);
    for i in 1..257 {
        assert_eq!(i, a.num_bits());
        a.mul2();
    }
    assert_eq!(0, a.num_bits());
}

#[test]
fn test_fq_num_bits() {
    assert_eq!(FqParameters::MODULUS_BITS, 254);
    assert_eq!(FqParameters::CAPACITY, 253);
}

#[test]
fn test_fq_root_of_unity() {
    assert_eq!(FqParameters::TWO_ADICITY, 1);
    assert_eq!(
        Fq::multiplicative_generator().pow([
            0x9e10460b6c3e7ea3,
            0xcbc0b548b438e546,
            0xdc2822db40c0ac2e,
            0x183227397098d014,
        ]),
        Fq::two_adic_root_of_unity()
    );
    assert_eq!(
        Fq::two_adic_root_of_unity().pow([1 << FqParameters::TWO_ADICITY]),
        Fq::one()
    );
    assert!(Fq::multiplicative_generator().sqrt().is_none());
}

#[test]
fn test_fq_ordering() {
    // BigInteger256's ordering is well-tested, but we still need to make sure the
    // Fq elements aren't being compared in Montgomery form.
    for i in 0..100 {
        assert!(Fq::from(BigInteger256::from(i + 1)) > Fq::from(BigInteger256::from(i)));
    }
}

#[test]
fn test_fq_legendre() {
    use ark_ff::fields::LegendreSymbol::*;

    assert_eq!(QuadraticResidue, Fq::one().legendre());
    assert_eq!(Zero, Fq::zero().legendre());
    assert_eq!(
        QuadraticResidue,
        Fq::from(BigInteger256::from(4)).legendre()
    );
    assert_eq!(
        QuadraticNonResidue,
        Fq::from(BigInteger256::from(5)).legendre()
    );
}

#[test]
fn test_fq2_ordering() {
    let mut a = Fq2::new(Fq::zero(), Fq::zero());
    let mut b = a.clone();

    assert!(a.cmp(&b) == Ordering::Equal);
    b.c0.add_assign(&Fq::one());
    assert!(a.cmp(&b) == Ordering::Less);
    a.c0.add_assign(&Fq::one());
    assert!(a.cmp(&b) == Ordering::Equal);
    b.c1.add_assign(&Fq::one());
    assert!(a.cmp(&b) == Ordering::Less);
    a.c0.add_assign(&Fq::one());
    assert!(a.cmp(&b) == Ordering::Less);
    a.c1.add_assign(&Fq::one());
    assert!(a.cmp(&b) == Ordering::Greater);
    b.c0.add_assign(&Fq::one());
    assert!(a.cmp(&b) == Ordering::Equal);
}

#[test]
fn test_fq2_basics() {
    assert_eq!(Fq2::new(Fq::zero(), Fq::zero(),), Fq2::zero());
    assert_eq!(Fq2::new(Fq::one(), Fq::zero(),), Fq2::one());
    assert!(Fq2::zero().is_zero());
    assert!(!Fq2::one().is_zero());
    assert!(!Fq2::new(Fq::zero(), Fq::one(),).is_zero());
}

#[test]
fn test_fq2_legendre() {
    use ark_ff::fields::LegendreSymbol::*;

    assert_eq!(Zero, Fq2::zero().legendre());
    // i^2 = -1
    let mut m1 = -Fq2::one();
    assert_eq!(QuadraticResidue, m1.legendre());
    m1 = Fq6Parameters::mul_fp2_by_nonresidue(&m1);
    assert_eq!(QuadraticNonResidue, m1.legendre());
}

#[test]
fn test_fq6_mul_by_1() {
    let mut rng = ark_std::test_rng();

    for _ in 0..1000 {
        let c1 = Fq2::rand(&mut rng);
        let mut a = Fq6::rand(&mut rng);
        let mut b = a;

        a.mul_by_1(&c1);
        b.mul_assign(&Fq6::new(Fq2::zero(), c1, Fq2::zero()));

        assert_eq!(a, b);
    }
}

#[test]
fn test_fq6_mul_by_01() {
    let mut rng = ark_std::test_rng();

    for _ in 0..1000 {
        let c0 = Fq2::rand(&mut rng);
        let c1 = Fq2::rand(&mut rng);
        let mut a = Fq6::rand(&mut rng);
        let mut b = a;

        a.mul_by_01(&c0, &c1);
        b.mul_assign(&Fq6::new(c0, c1, Fq2::zero()));

        assert_eq!(a, b);
    }
}

#[test]
fn test_fq12_mul_by_014() {
    let mut rng = ark_std::test_rng();

    for _ in 0..1000 {
        let c0 = Fq2::rand(&mut rng);
        let c1 = Fq2::rand(&mut rng);
        let c5 = Fq2::rand(&mut rng);
        let mut a = Fq12::rand(&mut rng);
        let mut b = a;

        a.mul_by_014(&c0, &c1, &c5);
        b.mul_assign(&Fq12::new(
            Fq6::new(c0, c1, Fq2::zero()),
            Fq6::new(Fq2::zero(), c5, Fq2::zero()),
        ));

        assert_eq!(a, b);
    }
}

#[test]
fn test_fq12_mul_by_034() {
    let mut rng = ark_std::test_rng();

    for _ in 0..1000 {
        let c0 = Fq2::rand(&mut rng);
        let c3 = Fq2::rand(&mut rng);
        let c4 = Fq2::rand(&mut rng);
        let mut a = Fq12::rand(&mut rng);
        let mut b = a;

        a.mul_by_034(&c0, &c3, &c4);
        b.mul_assign(&Fq12::new(
            Fq6::new(c0, Fq2::zero(), Fq2::zero()),
            Fq6::new(c3, c4, Fq2::zero()),
        ));

        assert_eq!(a, b);
    }
}
