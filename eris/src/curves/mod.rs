use crate::{fq::Fq, fr::Fr};
use ark_ec::{
    models::{ModelParameters, SWModelParameters},
    short_weierstrass_jacobian::{GroupAffine, GroupProjective},
};
use ark_ff::{field_new, Zero};

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct ErisParameters;

impl ModelParameters for ErisParameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

pub type Affine = GroupAffine<ErisParameters>;
pub type Projective = GroupProjective<ErisParameters>;

impl SWModelParameters for ErisParameters {
    /// COEFF_A = 0
    const COEFF_A: Fq = field_new!(Fq, "0");

    /// COEFF_B = 57
    const COEFF_B: Fq = field_new!(Fq, "57");

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[0x1];

    /// COFACTOR_INV = 1
    const COFACTOR_INV: Fr = field_new!(Fr, "1");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G_GENERATOR_X, G_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}

/// G_GENERATOR_X = -2
/// Encoded in Montgomery form, so the value here is -R mod p.
pub const G_GENERATOR_X: Fq = field_new!(Fq, "-2");

/// G_GENERATOR_Y = 7
/// Encoded in Montgomery form, so the value here is 2R mod p.
pub const G_GENERATOR_Y: Fq = field_new!(Fq, "7");
