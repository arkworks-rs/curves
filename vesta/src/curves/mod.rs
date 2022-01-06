use crate::{fq::Fq, fr::Fr};
use ark_ec::{
    models::{
        ModelParameters, 
        MontgomeryModelParameters, 
        SWModelParameters, 
        TEModelParameters
    },
    twisted_edwards_extended::{
        GroupAffine as SWGroupAffine, GroupProjective as SWGroupProjective,
    },
    short_weierstrass_jacobian::{GroupAffine, GroupProjective},
};
use ark_ff::{field_new, Zero};

#[cfg(test)]
mod tests;


pub type EdwardsAffine = GroupAffine<VestaParameters>;
pub type EdwardsProjective = GroupProjective<VestaParameters>;

pub type SWAffine = SWGroupAffine<VestaParameters>;
pub type SWProjective = SWGroupProjective<VestaParameters>;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct VestaParameters;


pub type EdwardsParameters = VestaParameters;
pub type SWParameters = VestaParameters;

impl ModelParameters for VestaParameters {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[0x1];

    /// COFACTOR_INV = 1
    const COFACTOR_INV: Fr = field_new!(Fr, "1");
}

pub type Affine = GroupAffine<VestaParameters>;
pub type Projective = GroupProjective<VestaParameters>;

impl TEModelParameters for VestaParameters {
    // !
    // ! ONLY FOR TESTS!
    // ! VESTA DOES NOT HAVE A TEMODEL REPRESENTATION!
    // !

    /// COEFF_A = -1
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "0");

    /// COEFF_D = 3021
    #[rustfmt::skip]
    const COEFF_D: Fq = field_new!(Fq, "0");

    /// Generated randomly
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) = (G_GENERATOR_X, G_GENERATOR_Y);

    type MontgomeryModelParameters = VestaParameters;

    /// Multiplication by `a` is just negation.
    /// Is `a` 1 or -1?
    #[inline(always)]
    fn mul_by_a(elem: &Self::BaseField) -> Self::BaseField {
        -*elem
    }
}

impl MontgomeryModelParameters for VestaParameters {
    // !
    // !
    // ! ONLY FOR TESTS
    // !
    // !

    /// COEFF_A = 0x8D26E3FADA9010A26949031ECE3971B93952AD84D4753DDEDB748DA37E8F552
    ///         = 3990301581132929505568273333084066329187552697088022219156688740916631500114
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "3990301581132929505568273333084066329187552697088022219156688740916631500114");
    /// COEFF_B = 0x9D8F71EEC83A44C3A1FBCEC6F5418E5C6154C2682B8AC231C5A3725C8170AAD
    ///         = 4454160168295440918680551605697480202188346638066041608778544715000777738925
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "4454160168295440918680551605697480202188346638066041608778544715000777738925");

    type TEModelParameters = VestaParameters;
}

impl SWModelParameters for VestaParameters {
    /// COEFF_A = 0
    const COEFF_A: Fq = field_new!(Fq, "0");

    /// COEFF_B = 5
    const COEFF_B: Fq = field_new!(Fq, "5");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (SW_G_GENERATOR_X, SW_G_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}


/// G_GENERATOR_X = -1
/// Encoded in Montgomery form, so the value here is -R mod p.
pub const G_GENERATOR_X: Fq = field_new!(Fq, "0");

/// G_GENERATOR_Y = 2
/// Encoded in Montgomery form, so the value here is 2R mod p.
pub const G_GENERATOR_Y: Fq = field_new!(Fq, "1");

//

/// SW_G_GENERATOR_X = -1
/// Encoded in Montgomery form, so the value here is -R mod p.
pub const SW_G_GENERATOR_X: Fq = field_new!(Fq, "-1");

/// SW_G_GENERATOR_Y = 2
/// Encoded in Montgomery form, so the value here is 2R mod p.
pub const SW_G_GENERATOR_Y: Fq = field_new!(Fq, "2");
