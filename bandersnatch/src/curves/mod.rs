use crate::{Fq, Fr};
use ark_ec::{
    models::{ModelParameters, MontgomeryModelParameters, TEModelParameters},
    twisted_edwards_extended::{GroupAffine, GroupProjective},
};
use ark_ff::field_new;
// use std::ops::Mul;
use ark_ff::Field;
#[cfg(test)]
mod tests;

pub type EdwardsAffine = GroupAffine<EdwardsParameters>;
pub type EdwardsProjective = GroupProjective<EdwardsParameters>;

/// `banersnatch` is a twisted Edwards curve. These curves have equations of the
/// form: ax² + y² = 1 - dx²y².
/// over some base finite field Fq.
///
/// banersnatch's curve equation: -5x² + y² = 1 - dx²y²
///
/// q = 52435875175126190479447740508185965837690552500527637822603658699938581184513.
///
/// a = 52435875175126190479447740508185965837690552500527637822603658699938581184508.
/// d = (138827208126141220649022263972958607803/171449701953573178309673572579671231137) mod q
///   = 45022363124591815672509500913686876175488063829319466900776701791074614335719.
///
/// Sage script to calculate these:
///
/// ```text
/// q = 52435875175126190479447740508185965837690552500527637822603658699938581184513
/// Fq = GF(q)
/// d = (Fq(138827208126141220649022263972958607803)/Fq(171449701953573178309673572579671231137))
/// ```
/// These parameters and the sage script obtained from:
/// <https://github.com/zcash/zcash/issues/2230#issuecomment-317182190>
#[derive(Clone, Default, PartialEq, Eq)]
pub struct EdwardsParameters;

impl ModelParameters for EdwardsParameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

impl TEModelParameters for EdwardsParameters {
    /// COEFF_A = -1
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "-5");

    /// COEFF_D = (138827208126141220649022263972958607803/171449701953573178309673572579671231137) mod q
    #[rustfmt::skip]
    const COEFF_D: Fq = field_new!(Fq, "45022363124591815672509500913686876175488063829319466900776701791074614335719");

    /// COFACTOR = 4
    const COFACTOR: &'static [u64] = &[4];

    /// COFACTOR^(-1) mod r =
    /// 9831726595336160714896451345284868594481866920080427688839802480047265754601
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "9831726595336160714896451345284868594481866920080427688839802480047265754601");

    /// AFFINE_GENERATOR_COEFFS = (GENERATOR_X, GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) = (GENERATOR_X, GENERATOR_Y);

    type MontgomeryModelParameters = EdwardsParameters;

    /// Multiplication by `a` is multiply by `-5`.
    #[inline(always)]
    fn mul_by_a(elem: &Self::BaseField) -> Self::BaseField {
        let t =(*elem).double().double();
        -(t+*elem)
    }
}

impl MontgomeryModelParameters for EdwardsParameters {
    /// COEFF_A = 29978822694968839326280996386011761570173833766074948509196803838190355340952
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "29978822694968839326280996386011761570173833766074948509196803838190355340952");
    /// COEFF_B = 25465760566081946422412445027709227188579564747101592991722834452325077642517
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "25465760566081946422412445027709227188579564747101592991722834452325077642517");

    type TEModelParameters = EdwardsParameters;
}

// using the generator from bench.py (in affine form)
// P = BandersnatchPoint(
//      13738737789055671334382939318077718462576533426798874551591468520593954805549,
//      11575885077368931610486103676191793534029821920164915325066801506752632626968,
//      14458123306641001284399433086015669988340559992755622870694102351476334505845,
//      C)
#[rustfmt::skip]
const GENERATOR_X: Fq = field_new!(Fq, "29627151942733444043031429156003786749302466371339015363120350521834195802525");
#[rustfmt::skip]
const GENERATOR_Y: Fq = field_new!(Fq, "27488387519748396681411951718153463804682561779047093991696427532072116857978");
