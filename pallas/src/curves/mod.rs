use ark_ec::{
    models::{ModelParameters, SWModelParameters},
    short_weierstrass_jacobian::{GroupAffine, GroupProjective},
};
use ark_ff::{MontFp, Zero};

use crate::{fq::Fq, fr::Fr};

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct PallasParameters;

impl ModelParameters for PallasParameters {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[0x1];

    /// COFACTOR_INV = 1
    const COFACTOR_INV: Fr = MontFp!(Fr, "1");
}

pub type Affine = GroupAffine<PallasParameters>;
pub type Projective = GroupProjective<PallasParameters>;

impl SWModelParameters for PallasParameters {
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
pub const G_GENERATOR_X: Fq = MontFp!(Fq, "-1");

/// G_GENERATOR_Y = 2
pub const G_GENERATOR_Y: Fq = MontFp!(Fq, "2");
