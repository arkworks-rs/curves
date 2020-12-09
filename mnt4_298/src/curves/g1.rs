use crate::{Fq, Fr, FR_ONE};
use ark_ec::{
    mnt4,
    models::{ModelParameters, SWModelParameters},
};
use ark_ff::field_new;

pub type G1Affine = mnt4::G1Affine<crate::Parameters>;
pub type G1Projective = mnt4::G1Projective<crate::Parameters>;
pub type G1Prepared = mnt4::G1Prepared<crate::Parameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

impl SWModelParameters for Parameters {
    /// COEFF_A = 2
    /// Reference: https://github.com/scipr-lab/libff/blob/c927821ebe02e0a24b5e0f9170cec5e211a35f08/libff/algebra/curves/mnt/mnt4/mnt4_init.cpp#L116
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "2");

    /// COEFF_B = 423894536526684178289416011533888240029318103673896002803341544124054745019340795360841685
    /// Reference: https://github.com/scipr-lab/libff/blob/c927821ebe02e0a24b5e0f9170cec5e211a35f08/libff/algebra/curves/mnt/mnt4/mnt4_init.cpp#L117
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "423894536526684178289416011533888240029318103673896002803341544124054745019340795360841685");

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[1];

    /// COFACTOR^(-1) mod r =
    /// 1
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = FR_ONE;

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G1_GENERATOR_X, G1_GENERATOR_Y);
}

// Generator of G1
// X = 60760244141852568949126569781626075788424196370144486719385562369396875346601926534016838,
// Y = 363732850702582978263902770815145784459747722357071843971107674179038674942891694705904306,
/// G1_GENERATOR_X
/// Reference: https://github.com/scipr-lab/libff/blob/c927821ebe02e0a24b5e0f9170cec5e211a35f08/libff/algebra/curves/mnt/mnt4/mnt4_init.cpp#L137
#[rustfmt::skip]
pub const G1_GENERATOR_X: Fq = field_new!(Fq, "60760244141852568949126569781626075788424196370144486719385562369396875346601926534016838");

/// G1_GENERATOR_Y
/// Reference: https://github.com/scipr-lab/libff/blob/c927821ebe02e0a24b5e0f9170cec5e211a35f08/libff/algebra/curves/mnt/mnt4/mnt4_init.cpp#L138
#[rustfmt::skip]
pub const G1_GENERATOR_Y: Fq = field_new!(Fq, "363732850702582978263902770815145784459747722357071843971107674179038674942891694705904306");
