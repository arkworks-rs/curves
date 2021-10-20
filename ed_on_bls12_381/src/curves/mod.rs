use crate::{Fq, Fr};
use ark_ec::{
    models::{ModelParameters, MontgomeryModelParameters, TEModelParameters},
    short_weierstrass_jacobian::{
        GroupAffine as SWGroupAffine, GroupProjective as SWGroupProjective,
    },
    twisted_edwards_extended::{GroupAffine, GroupProjective},
    SWModelParameters,
};
use ark_ff::field_new;

#[cfg(test)]
mod tests;

pub type EdwardsAffine = GroupAffine<JubjubParameters>;
pub type EdwardsProjective = GroupProjective<JubjubParameters>;
pub type SWAffine = SWGroupAffine<JubjubParameters>;
pub type SWProjective = SWGroupProjective<JubjubParameters>;

/// `JubJub` is a twisted Edwards curve. These curves have equations of the
/// form: ax² + y² = 1 - dx²y².
/// over some base finite field Fq.
///
/// JubJub's curve equation: -x² + y² = 1 - (10240/10241)x²y²
///
/// q = 52435875175126190479447740508185965837690552500527637822603658699938581184513.
///
/// a = -1.
/// d = (10240/10241) mod q
///   = 19257038036680949359750312669786877991949435402254120286184196891950884077233.
///
/// Sage script to calculate these:
///
/// ```text
/// q = 52435875175126190479447740508185965837690552500527637822603658699938581184513
/// Fq = GF(q)
/// d = -(Fq(10240)/Fq(10241))
/// ```
/// These parameters and the sage script obtained from:
/// <https://github.com/zcash/zcash/issues/2230#issuecomment-317182190>
///
///
/// `jubjub` also has a short Weierstrass curve form, following the
/// form: y² = x³ + A * x + B
/// where
///
/// A = 52296097456646850916096512823759002727550416093741407922227928430486925478210
/// B = 48351165704696163914533707656614864561753505123260775585269522553028192119009
///
/// We can use the script available
/// [here](https://github.com/zhenfeizhang/bandersnatch/blob/main/bandersnatch/script/jubjub.sage)
/// to convert between the different representations.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct JubjubParameters;
pub type EdwardsParameters = JubjubParameters;
pub type SWParameters = JubjubParameters;

impl ModelParameters for JubjubParameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

impl TEModelParameters for JubjubParameters {
    /// COEFF_A = -1
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "-1");

    /// COEFF_D = (10240/10241) mod q
    #[rustfmt::skip]
    const COEFF_D: Fq = field_new!(Fq, "19257038036680949359750312669786877991949435402254120286184196891950884077233");

    /// COFACTOR = 8
    const COFACTOR: &'static [u64] = &[8];

    /// COFACTOR^(-1) mod r =
    /// 819310549611346726241370945440405716213240158234039660170669895299022906775
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "819310549611346726241370945440405716213240158234039660170669895299022906775");

    /// AFFINE_GENERATOR_COEFFS = (GENERATOR_X, GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) = (GENERATOR_X, GENERATOR_Y);

    type MontgomeryModelParameters = JubjubParameters;

    /// Multiplication by `a` is simply negation here.
    #[inline(always)]
    fn mul_by_a(elem: &Self::BaseField) -> Self::BaseField {
        -(*elem)
    }
}

impl MontgomeryModelParameters for JubjubParameters {
    /// COEFF_A = 40962
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "40962");
    /// COEFF_B = -40964
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "-40964");

    type TEModelParameters = JubjubParameters;
}

#[rustfmt::skip]
const GENERATOR_X: Fq = field_new!(Fq, "8076246640662884909881801758704306714034609987455869804520522091855516602923");
#[rustfmt::skip]
const GENERATOR_Y: Fq = field_new!(Fq, "13262374693698910701929044844600465831413122818447359594527400194675274060458");

impl SWModelParameters for JubjubParameters {
    /// COEFF_A = 52296097456646850916096512823759002727550416093741407922227928430486925478210
    #[rustfmt::skip]
    const COEFF_A: Self::BaseField = field_new!(Fq, "52296097456646850916096512823759002727550416093741407922227928430486925478210");

    /// COEFF_B = 48351165704696163914533707656614864561753505123260775585269522553028192119009
    #[rustfmt::skip]
    const COEFF_B: Self::BaseField = field_new!(Fq, "48351165704696163914533707656614864561753505123260775585269522553028192119009");

    /// COFACTOR = 8
    const COFACTOR: &'static [u64] = &[8];

    /// COFACTOR^(-1) mod r =
    /// 819310549611346726241370945440405716213240158234039660170669895299022906775
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "819310549611346726241370945440405716213240158234039660170669895299022906775");

    /// generators
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (SW_GENERATOR_X, SW_GENERATOR_Y);
}

/// x coordinate for SW curve generator
#[rustfmt::skip]
const SW_GENERATOR_X: Fq = field_new!(Fq, "33835869156188682335217394949746694649676633840125476177319971163079011318731");
/// y coordinate for SW curve generator
#[rustfmt::skip]
const SW_GENERATOR_Y: Fq = field_new!(Fq, "43777270878440091394432848052353307184915192688165709016756678962558652055320");
