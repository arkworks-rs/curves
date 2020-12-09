use crate::{Fq, Fr};
use ark_ec::{
    models::{ModelParameters, MontgomeryModelParameters, TEModelParameters},
    twisted_edwards_extended::{GroupAffine, GroupProjective},
};
use ark_ff::field_new;

#[cfg(test)]
mod tests;

pub type EdwardsAffine = GroupAffine<EdwardsParameters>;
pub type EdwardsProjective = GroupProjective<EdwardsParameters>;

/// `Baby-JubJub` is a twisted Edwards curve. These curves have equations of the
/// form: ax² + y² = 1 + dx²y².
/// over some base finite field Fq.
///
/// Baby-JubJub's curve equation: x² + y² = 1 + (168696/168700)x²y²
///
/// q = 21888242871839275222246405745257275088548364400416034343698204186575808495617
///
#[derive(Clone, Default, PartialEq, Eq)]
pub struct EdwardsParameters;

impl ModelParameters for EdwardsParameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

impl TEModelParameters for EdwardsParameters {
    /// COEFF_A = 1
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "1");

    #[inline(always)]
    fn mul_by_a(elem: &Self::BaseField) -> Self::BaseField {
        *elem
    }

    /// COEFF_D = 168696/168700 mod q
    ///         = 9706598848417545097372247223557719406784115219466060233080913168975159366771
    #[rustfmt::skip]
    const COEFF_D: Fq = field_new!(Fq, "9706598848417545097372247223557719406784115219466060233080913168975159366771");

    /// COFACTOR = 8
    const COFACTOR: &'static [u64] = &[8];

    /// COFACTOR^(-1) mod r =
    /// 2394026564107420727433200628387514462817212225638746351800188703329891451411
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "2394026564107420727433200628387514462817212225638746351800188703329891451411");

    /// AFFINE_GENERATOR_COEFFS = (GENERATOR_X, GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) = (GENERATOR_X, GENERATOR_Y);

    type MontgomeryModelParameters = EdwardsParameters;
}

impl MontgomeryModelParameters for EdwardsParameters {
    /// COEFF_A = 168698
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "168698");
    /// COEFF_B = 168700
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "168700");

    type TEModelParameters = EdwardsParameters;
}

#[rustfmt::skip]
const GENERATOR_X: Fq = field_new!(Fq, "19698561148652590122159747500897617769866003486955115824547446575314762165298");
#[rustfmt::skip]
const GENERATOR_Y: Fq = field_new!(Fq, "19298250018296453272277890825869354524455968081175474282777126169995084727839");
