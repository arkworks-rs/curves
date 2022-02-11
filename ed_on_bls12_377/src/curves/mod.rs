use crate::{fq::Fq, fr::Fr};
use ark_ec::{
    models::{ModelParameters, MontgomeryModelParameters, TEModelParameters},
    twisted_edwards_extended::{GroupAffine, GroupProjective},
};
use ark_ff::field_new;

#[cfg(test)]
mod tests;

pub type EdwardsAffine = GroupAffine<EdwardsParameters>;
pub type EdwardsProjective = GroupProjective<EdwardsParameters>;
pub type SWAffine = SWGroupAffine<JubjubParameters>;
pub type SWProjective = SWGroupProjective<JubjubParameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct JubjubParameters;
pub type EdwardsParameters = JubjubParameters;
pub type SWParameters = JubjubParameters;

impl ModelParameters for JubjubParameters {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 4
    const COFACTOR: &'static [u64] = &[4];

    /// COFACTOR_INV =
    /// 527778859339273151515551558673846658209717731602102048798421311598680340096
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "527778859339273151515551558673846658209717731602102048798421311598680340096");
}

impl TEModelParameters for JubjubParameters {
    /// COEFF_A = -1
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "-1");

    /// COEFF_D = 3021
    #[rustfmt::skip]
    const COEFF_D: Fq = field_new!(Fq, "3021");

    /// Generated randomly
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) = (GENERATOR_X, GENERATOR_Y);

    type MontgomeryModelParameters = JubjubParameters;

    /// Multiplication by `a` is just negation.
    #[inline(always)]
    fn mul_by_a(elem: &Self::BaseField) -> Self::BaseField {
        -*elem
    }
}

impl MontgomeryModelParameters for JubjubParameters {
    /// COEFF_A = 0x8D26E3FADA9010A26949031ECE3971B93952AD84D4753DDEDB748DA37E8F552
    ///         = 3990301581132929505568273333084066329187552697088022219156688740916631500114
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "3990301581132929505568273333084066329187552697088022219156688740916631500114");
    /// COEFF_B = 0x9D8F71EEC83A44C3A1FBCEC6F5418E5C6154C2682B8AC231C5A3725C8170AAD
    ///         = 4454160168295440918680551605697480202188346638066041608778544715000777738925
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "4454160168295440918680551605697480202188346638066041608778544715000777738925");

    type TEModelParameters = JubjubParameters;
}

/// GENERATOR_X =
/// 4497879464030519973909970603271755437257548612157028181994697785683032656389,
#[rustfmt::skip]
const GENERATOR_X: Fq = field_new!(Fq, "4497879464030519973909970603271755437257548612157028181994697785683032656389");

/// GENERATOR_Y =
/// 4357141146396347889246900916607623952598927460421559113092863576544024487809
#[rustfmt::skip]
const GENERATOR_Y: Fq = field_new!(Fq, "4357141146396347889246900916607623952598927460421559113092863576544024487809");

impl SWModelParameters for JubjubParameters {
    /// COEFF_A = 703705145785697535354068744898462210947991611262838652327936121326450580667
    #[rustfmt::skip]
    const COEFF_A: Self::BaseField = field_new!(Fq, "703705145785697535354068744898462210947991611262838652327936121326450580667");

    /// COEFF_B = 4534988717285606338948443022678978692775945939249404648335588337437094924611
    #[rustfmt::skip]
    const COEFF_B: Self::BaseField = field_new!(Fq, "4534988717285606338948443022678978692775945939249404648335588337437094924611");

    /// generators
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (SW_GENERATOR_X, SW_GENERATOR_Y);
}

/// x coordinate for SW curve generator
#[rustfmt::skip]
const SW_GENERATOR_X: Fq = field_new!(Fq, "3421912078908394748282540611691040025770525566253155016768561915202023608319");
/// y coordinate for SW curve generator
#[rustfmt::skip]
const SW_GENERATOR_Y: Fq = field_new!(Fq, "943050486128738681953305652984679658685617558655311906095277947815247138018");
