use crate::{fq::Fq, fr::Fr};
use ark_ec::{
    models::{CurveConfig, SWCurveConfig},
    short_weierstrass::{Affine, Projective},
};
use ark_ff::{MontFp, Zero};

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct VestaParameters;

impl CurveConfig for VestaParameters {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[0x1];

    /// COFACTOR_INV = 1
    const COFACTOR_INV: Fr = MontFp!(Fr, "1");
}

pub type Affine = Affine<VestaParameters>;
pub type Projective = Projective<VestaParameters>;

impl SWCurveConfig for VestaParameters {
    /// COEFF_A = 0
    const COEFF_A: Fq = MontFp!(Fq, "0");

    /// COEFF_B = 5
    const COEFF_B: Fq = MontFp!(Fq, "5");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G_GENERATOR_X, G_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}

/// G_GENERATOR_X = -1
/// Encoded in Montgomery form, so the value here is -R mod p.
pub const G_GENERATOR_X: Fq = MontFp!(Fq, "-1");

/// G_GENERATOR_Y = 2
/// Encoded in Montgomery form, so the value here is 2R mod p.
pub const G_GENERATOR_Y: Fq = MontFp!(Fq, "2");
