use ark_ec::{
    mnt6,
    models::{ModelParameters, SWModelParameters},
};
use ark_ff::field_new;

use crate::{Fq, Fr};

pub type G1Affine = mnt6::G1Affine<crate::Parameters>;
pub type G1Projective = mnt6::G1Projective<crate::Parameters>;
pub type G1Prepared = mnt6::G1Prepared<crate::Parameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

impl SWModelParameters for Parameters {
    /// COEFF_A = 11
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "11");

    /// COEFF_B = 106700080510851735677967319632585352256454251201367587890185989362936000262606668469523074
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "106700080510851735677967319632585352256454251201367587890185989362936000262606668469523074");

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[1];

    /// COFACTOR^(-1) mod r =
    /// 1
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "1");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G1_GENERATOR_X, G1_GENERATOR_Y);
}

/// G1_GENERATOR_X =
#[rustfmt::skip]
pub const G1_GENERATOR_X: Fq = field_new!(Fq, "336685752883082228109289846353937104185698209371404178342968838739115829740084426881123453");

/// G1_GENERATOR_Y =
#[rustfmt::skip]
pub const G1_GENERATOR_Y: Fq = field_new!(Fq, "402596290139780989709332707716568920777622032073762749862342374583908837063963736098549800");
