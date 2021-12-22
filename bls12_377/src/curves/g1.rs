use ark_ec::models::{ModelParameters, SWModelParameters};
use ark_ff::{field_new, Zero};

use crate::{
    fields::{FQ_ONE, FQ_ZERO},
    Fq, Fr,
};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;
    type Affine = ark_ec::short_weierstrass_jacobian::GroupAffine<Self>;

    /// COFACTOR = (x - 1)^2 / 3  = 30631250834960419227450344600217059328
    const COFACTOR: &'static [u64] = &[0x0, 0x170b5d4430000000];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// = 5285428838741532253824584287042945485047145357130994810877
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "5285428838741532253824584287042945485047145357130994810877");
}

impl SWModelParameters for Parameters {
    /// COEFF_A = 0
    const COEFF_A: Fq = FQ_ZERO;

    /// COEFF_B = 1
    #[rustfmt::skip]
    const COEFF_B: Fq = FQ_ONE;

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G1_GENERATOR_X, G1_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}

/// G1_GENERATOR_X =
/// 81937999373150964239938255573465948239988671502647976594219695644855304257327692006745978603320413799295628339695
#[rustfmt::skip]
pub const G1_GENERATOR_X: Fq = field_new!(Fq, "81937999373150964239938255573465948239988671502647976594219695644855304257327692006745978603320413799295628339695");

/// G1_GENERATOR_Y =
/// 241266749859715473739788878240585681733927191168601896383759122102112907357779751001206799952863815012735208165030
#[rustfmt::skip]
pub const G1_GENERATOR_Y: Fq = field_new!(Fq, "241266749859715473739788878240585681733927191168601896383759122102112907357779751001206799952863815012735208165030");
