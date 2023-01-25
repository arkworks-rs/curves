use ark_ec::{
    models::{short_weierstrass::SWCurveConfig, CurveConfig},
    scalar_mul::glv::GLVConfig,
    short_weierstrass::{Affine, Projective},
};
use ark_ff::{Field, MontFp, Zero};

use crate::{Fq, Fq2, Fr};

pub type G2Affine = Affine<Config>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config;

impl CurveConfig for Config {
    type BaseField = Fq2;
    type ScalarField = Fr;

    /// COFACTOR = (36 * X^4) + (36 * X^3) + (30 * X^2) + 6*X + 1
    /// 21888242871839275222246405745257275088844257914179612981679871602714643921549
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0x345f2299c0f9fa8d,
        0x06ceecda572a2489,
        0xb85045b68181585e,
        0x30644e72e131a029,
    ];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    const COFACTOR_INV: Fr =
        MontFp!("10944121435919637613327163357776759465618812564592884533313067514031822496649");
}

impl SWCurveConfig for Config {
    /// COEFF_A = [0, 0]
    const COEFF_A: Fq2 = Fq2::ZERO;

    /// COEFF_B = 3/(u+9)
    /// (19485874751759354771024239261021720505790618469301721065564631296452457478373, 266929791119991161246907387137283842545076965332900288569378510910307636690)
    const COEFF_B: Fq2 = Fq2::new(
        MontFp!("19485874751759354771024239261021720505790618469301721065564631296452457478373"),
        MontFp!("266929791119991161246907387137283842545076965332900288569378510910307636690"),
    );

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const GENERATOR: G2Affine = G2Affine::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}

impl GLVConfig for Config {
    const ENDO_COEFFS: &'static [Self::BaseField] = &[Fq2::new(
        MontFp!("21888242871839275220042445260109153167277707414472061641714758635765020556616"),
        Fq::ZERO,
    )];

    const LAMBDA: Self::ScalarField =
        MontFp!("4407920970296243842393367215006156084916469457145843978461");

    const SCALAR_DECOMP_COEFFS: [[<Self as CurveConfig>::ScalarField; 2]; 2] = [
        [
            MontFp!("147946756881789319010696353538189108491"),
            MontFp!("9931322734385697763"),
        ],
        [
            MontFp!("9931322734385697763"),
            MontFp!("147946756881789319000765030803803410728"),
        ],
    ];
    const SGN_N: [bool; 4] = [false, false, true, false];

    fn endomorphism(p: &Projective<Self>) -> Projective<Self> {
        let mut res = (*p).clone();
        res.x *= Self::ENDO_COEFFS[0];
        res
    }

    fn endomorphism_affine(p: &Affine<Self>) -> Affine<Self> {
        let mut res = (*p).clone();
        res.x *= Self::ENDO_COEFFS[0];
        res
    }
}

pub const G2_GENERATOR_X: Fq2 = Fq2::new(G2_GENERATOR_X_C0, G2_GENERATOR_X_C1);
pub const G2_GENERATOR_Y: Fq2 = Fq2::new(G2_GENERATOR_Y_C0, G2_GENERATOR_Y_C1);

/// G2_GENERATOR_X_C0 =
/// 10857046999023057135944570762232829481370756359578518086990519993285655852781
pub const G2_GENERATOR_X_C0: Fq =
    MontFp!("10857046999023057135944570762232829481370756359578518086990519993285655852781");

/// G2_GENERATOR_X_C1 =
/// 11559732032986387107991004021392285783925812861821192530917403151452391805634
pub const G2_GENERATOR_X_C1: Fq =
    MontFp!("11559732032986387107991004021392285783925812861821192530917403151452391805634");

/// G2_GENERATOR_Y_C0 =
/// 8495653923123431417604973247489272438418190587263600148770280649306958101930
pub const G2_GENERATOR_Y_C0: Fq =
    MontFp!("8495653923123431417604973247489272438418190587263600148770280649306958101930");

/// G2_GENERATOR_Y_C1 =
/// 4082367875863433681332203403145435568316851327593401208105741076214120093531
pub const G2_GENERATOR_Y_C1: Fq =
    MontFp!("4082367875863433681332203403145435568316851327593401208105741076214120093531");

#[cfg(test)]
mod test {

    use std::time::Instant;

    use crate::g2;

    use super::*;
    use ark_std::{test_rng, UniformRand};

    #[test]
    fn bench_glv() {
        let mut rng = test_rng();
        let p = Projective::<g2::Config>::rand(&mut rng);
        let s = Fr::rand(&mut rng);
        // bench
        let now = Instant::now();
        for _ in 1..100 {
            let _ = p * s;
        }
        println!("SM: {:?}", now.elapsed());
        let now = Instant::now();
        for _ in 1..100 {
            let _ = g2::Config::glv_mul_projective(p, s);
        }
        println!("GLV: {:?}", now.elapsed());
    }
}
