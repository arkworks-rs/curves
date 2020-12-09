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

// Many parameters need to be written down in the Montgomery residue form,
// discussed below. Some useful numbers:
// R for Fq: 223364648326281414938801705359223029554923725549792420683051274872200260503540791531766876
// R for Fr: 104384076783966083500464392945960916666734135485183910065100558776489954102951241798239545

impl TEModelParameters for EdwardsParameters {
    /// COEFF_A = -1
    /// Needs to be in the Montgomery residue form in Fq
    /// I.e., -1 * R for Fq
    ///     = 252557637842979910814547544293825421990201153003031094870216460866964386803867699028196261
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "-1");

    /// COEFF_D = 4212
    /// Needs to be in the Montgomery residue form in Fq
    /// I.e., 4212 * R for Fq
    ///     = 389461279836940033614665658623660232171971995346409183754923941118154161474636585314923000
    #[rustfmt::skip]
    const COEFF_D: Fq = field_new!(Fq, "4212");

    /// COFACTOR = 4
    const COFACTOR: &'static [u64] = &[4];

    /// COFACTOR_INV (mod r) =
    /// 29745142885578832859584328103315528221570304936126890280067991221921526670592508030983158
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "29745142885578832859584328103315528221570304936126890280067991221921526670592508030983158");

    /// Generated randomly
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) = (GENERATOR_X, GENERATOR_Y);

    type MontgomeryModelParameters = EdwardsParameters;

    /// Multiplication by `a` is just negation.
    #[inline(always)]
    fn mul_by_a(elem: &Self::BaseField) -> Self::BaseField {
        -*elem
    }
}

impl MontgomeryModelParameters for EdwardsParameters {
    /// COEFF_A = 203563247015667910991582090642011229452721346107806307863040223071914240315202967004285204
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "203563247015667910991582090642011229452721346107806307863040223071914240315202967004285204");
    /// COEFF_B = 272359039153593414761767159011037222092403532445017207690227512667250406992205523555677931
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "272359039153593414761767159011037222092403532445017207690227512667250406992205523555677931");

    type TEModelParameters = EdwardsParameters;
}

/// GENERATOR_X =
/// 282406820114868156776872298252698015906762052916420164316497572033519876761239463633892227
#[rustfmt::skip]
const GENERATOR_X: Fq = field_new!(Fq, "282406820114868156776872298252698015906762052916420164316497572033519876761239463633892227");

/// GENERATOR_Y =
/// 452667754940241021433619311795265643711152068500301853535337412655162600774122192283142703
#[rustfmt::skip]
const GENERATOR_Y: Fq = field_new!(Fq, "452667754940241021433619311795265643711152068500301853535337412655162600774122192283142703");
