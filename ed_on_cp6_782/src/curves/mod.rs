use ark_ec::{
    models::{ModelParameters, MontgomeryModelParameters, TEModelParameters},
    twisted_edwards_extended::{GroupAffine, GroupProjective},
};
use ark_ff::field_new;

use crate::{fq::Fq, fr::Fr};

#[cfg(test)]
mod tests;

pub type EdwardsAffine = GroupAffine<EdwardsParameters>;
pub type EdwardsProjective = GroupProjective<EdwardsParameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct EdwardsParameters;

impl ModelParameters for EdwardsParameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

impl TEModelParameters for EdwardsParameters {
    /// COEFF_A = -1 =
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "-1");

    /// COEFF_D = 79743
    #[rustfmt::skip]
    const COEFF_D: Fq = field_new!(Fq, "79743");

    /// COFACTOR = 8
    const COFACTOR: &'static [u64] = &[8];

    /// COFACTOR^(-1) mod r =
    /// 12124894969357926281749346891948134384518445910386624712788431705725441736421489799867521238554906438478484045560
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "12124894969357926281749346891948134384518445910386624712788431705725441736421489799867521238554906438478484045560");

    /// AFFINE_GENERATOR_COEFFS = (GENERATOR_X, GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) = (GENERATOR_X, GENERATOR_Y);

    type MontgomeryModelParameters = EdwardsParameters;

    /// Multiplication by `a` is just negation.
    #[inline(always)]
    fn mul_by_a(elem: &Self::BaseField) -> Self::BaseField {
        -*elem
    }
}

impl MontgomeryModelParameters for EdwardsParameters {
    /// COEFF_A = 0x95D53EB3F6AC3F7A53C26020144439DC6073BCAE513E03FD06B6B3BAA390F25E51534B26719E33F4CD906D4DA9B535
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "90083623084271891037116870487743067984710080209539149685414147055329063590616489392386084989619674926965747987765");
    /// COEFF_B = 0x118650763CE64AB4BE743604C8D05013DC2663652A3D58B21ECAB7BFF65B70DB8BA09F9098E61CC903B2F92B2564ACA
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "168580802928697202973535863207150465551683432545375510854470115611391404757724333382582803149953685197474573470410");

    type TEModelParameters = EdwardsParameters;
}

/// GENERATOR_X =
/// 174701772324485506941690903512423551998294352968833659960042362742684869862495746426366187462669992073196420267127
#[rustfmt::skip]
const GENERATOR_X: Fq = field_new!(Fq, "174701772324485506941690903512423551998294352968833659960042362742684869862495746426366187462669992073196420267127");

/// GENERATOR_Y =
/// 208487200052258845495340374451540775445408439654930191324011635560142523886549663106522691296420655144190624954833
#[rustfmt::skip]
const GENERATOR_Y: Fq = field_new!(Fq, "208487200052258845495340374451540775445408439654930191324011635560142523886549663106522691296420655144190624954833");
