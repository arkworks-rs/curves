use crate::*;
use ark_ec::{
    AffineCurve,
    bls12,
    models::{ModelParameters, SWModelParameters},
    short_weierstrass_jacobian::GroupAffine,
};
use ark_ff::{field_new, Zero};
use ark_ec::bls12::Bls12Parameters;

pub type G1Affine = bls12::G1Affine<crate::Parameters0>;
pub type G1Projective = bls12::G1Projective<crate::Parameters0>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

impl SWModelParameters for Parameters {
    /// COEFF_A = 0
    const COEFF_A: Fq = field_new!(Fq, "0");

    /// COEFF_B = 4
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "4");

    /// COFACTOR = (x - 1)^2 / 3  = 76329603384216526031706109802092473003
    const COFACTOR: &'static [u64] = &[0x8c00aaab0000aaab, 0x396c8c005555e156];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// = 52435875175126190458656871551744051925719901746859129887267498875565241663483
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "52435875175126190458656871551744051925719901746859129887267498875565241663483");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G1_GENERATOR_X, G1_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }

    fn is_in_correct_subgroup_assuming_on_curve_fast(
	p : &GroupAffine<Self>
    ) -> Option<bool> {
	// r = (u⁴ - u² + 1) = u² * (u²-1) + 1 = u² * lambda + 1
        // [r]P = 0 iff [u²] sigma(P) + P = 0
        let sigma_p = sigma(p);
        let mut mul_sigma_p:GroupAffine<_> =
            sigma_p.mul(Parameters0::X[0]).into();
	mul_sigma_p = mul_sigma_p.mul(Parameters0::X[0]).into();
        Some((mul_sigma_p+*p).is_zero())
    }
}

/// G1_GENERATOR_X =
/// 3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507
#[rustfmt::skip]
pub const G1_GENERATOR_X: Fq = field_new!(Fq, "3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507");

/// G1_GENERATOR_Y =
/// 1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569
#[rustfmt::skip]
pub const G1_GENERATOR_Y: Fq = field_new!(Fq, "1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569");

pub const BETA: Fq = field_new!(Fq,"4002409555221667392624310435006688643935503118305586438271171395842971157480381377015405980053539358417135540939436");	

pub fn sigma(p: &GroupAffine<Parameters>)-> GroupAffine<Parameters> {
    let mut sigma_p = *p;
    sigma_p.x *= BETA;
    sigma_p
}
    
