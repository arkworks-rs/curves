use ark_ec::models::{ModelParameters, SWModelParameters};
use ark_ff::{MontFp, Zero};

use crate::{Fq, Fr};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[0x1];

    /// COFACTOR_INV = COFACTOR^{-1} mod r = 1
    const COFACTOR_INV: Fr = MontFp!(Fr, "1");
}

impl SWModelParameters for Parameters {
    /// COEFF_A = 0
    const COEFF_A: Fq = MontFp!(Fq, "0");

    /// COEFF_B = 3
    const COEFF_B: Fq = MontFp!(Fq, "3");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G1_GENERATOR_X, G1_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}

/// G1_GENERATOR_X = 1
pub const G1_GENERATOR_X: Fq = MontFp!(Fq, "1");

/// G1_GENERATOR_Y = 2
pub const G1_GENERATOR_Y: Fq = MontFp!(Fq, "2");
