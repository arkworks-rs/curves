use crate::{fq::Fq, fr::Fr};
use ark_ec::{
    models::CurveConfig,
    scalar_mul::glv::GLVConfig,
    short_weierstrass::{self as sw, SWCurveConfig},
};
use ark_ff::{Field, MontFp, Zero};

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct VestaConfig;

impl CurveConfig for VestaConfig {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[0x1];

    /// COFACTOR_INV = 1
    const COFACTOR_INV: Fr = Fr::ONE;
}

pub type Affine = sw::Affine<VestaConfig>;
pub type Projective = sw::Projective<VestaConfig>;

impl SWCurveConfig for VestaConfig {
    /// COEFF_A = 0
    const COEFF_A: Fq = Fq::ZERO;

    /// COEFF_B = 5
    const COEFF_B: Fq = MontFp!("5");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const GENERATOR: Affine = Affine::new_unchecked(G_GENERATOR_X, G_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}

impl GLVConfig for VestaConfig {
    const COEFFS_ENDOMORPHISM: &'static [Self::BaseField] = &[MontFp!(
        "26005156700822196841419187675678338661165322343552424574062261873906994770353"
    )];

    const LAMBDA: Self::ScalarField =
        MontFp!("20444556541222657078399132219657928148671392403212669005631716460534733845831");

    const COEFF_N: [<Self as CurveConfig>::ScalarField; 4] = [
        MontFp!("98231058071100081932162823354453065729"),
        MontFp!("98231058071186745657228807397848383488"),
        MontFp!("196462116142286827589391630752301449217"),
        MontFp!("98231058071100081932162823354453065729"),
    ];
    const SGN_N: [bool; 4] = [false, true, false, false];

    fn endomorphism(p: &Projective) -> Projective {
        // Endomorphism of the points on the curve.
        // endomorphism_p(x,y) = (BETA * x, y)
        // where BETA is a non-trivial cubic root of unity in Fq.
        let mut res = (*p).clone();
        res.x *= Self::COEFFS_ENDOMORPHISM[0];
        res
    }
}

/// G_GENERATOR_X = -1
/// Encoded in Montgomery form, so the value here is -R mod p.
pub const G_GENERATOR_X: Fq = MontFp!("-1");

/// G_GENERATOR_Y = 2
/// Encoded in Montgomery form, so the value here is 2R mod p.
pub const G_GENERATOR_Y: Fq = MontFp!("2");

#[cfg(test)]
mod test {

    use std::time::Instant;

    use super::*;
    use ark_std::{test_rng, UniformRand};

    #[test]
    fn bench_glv() {
        let mut rng = test_rng();
        let p = Projective::rand(&mut rng);
        let s = Fr::rand(&mut rng);
        // bench
        let now = Instant::now();
        for _ in 1..100 {
            let _ = p * s;
        }
        println!("SM: {:?}", now.elapsed());
        let now = Instant::now();
        for _ in 1..100 {
            let _ = VestaConfig::glv_mul(p, s);
        }
        println!("GLV: {:?}", now.elapsed());
    }
}
