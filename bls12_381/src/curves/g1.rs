use crate::*;
use ark_ec::bls12::Bls12Parameters;
use ark_ec::{
    bls12,
    models::{ModelParameters, SWModelParameters},
    short_weierstrass_jacobian::GroupAffine,
    AffineCurve, ProjectiveCurve,
};
use ark_ff::{biginteger::BigInteger256 as BigInteger, field_new, Zero};
pub type G1Affine = bls12::G1Affine<crate::Parameters>;
pub type G1Projective = bls12::G1Projective<crate::Parameters>;

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

    fn is_in_correct_subgroup_assuming_on_curve(p: &GroupAffine<Parameters>) -> bool {
        // Algorithm from https://eprint.iacr.org/2021/1130,
        // see Section 6.
        // Check that sigma(P) == -[X²]P
        // if [X]P = P during the computation, return False
        if p.is_zero() {
            true
        } else {
            // TODO
            let mut x_times_p = p.mul(BigInteger([crate::Parameters::X[0], 0, 0, 0]));
            if crate::Parameters::X_IS_NEGATIVE {
                x_times_p = -x_times_p;
            }
            if x_times_p.add_mixed(p).is_zero() {
                false
            } else {
                let sigma_p = sigma(p);
                // TODO
                let mut x2_times_p = x_times_p.mul(BigInteger([crate::Parameters::X[0], 0, 0, 0]));
                if crate::Parameters::X_IS_NEGATIVE {
                    x2_times_p = -x2_times_p;
                }
                x2_times_p.add_mixed(&sigma_p).is_zero()
            }
        }
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

// BETA is a third root of unity, meaning that BETA²+BETA+1 = 0
pub const BETA: Fq = field_new!(Fq,"793479390729215512621379701633421447060886740281060493010456487427281649075476305620758731620350");

pub fn sigma(p: &GroupAffine<Parameters>) -> GroupAffine<Parameters> {
    // Endomorphism of the curve
    // sigma(x,y) = (BETA*x, y) where BETA is a third root of unity of
    // Fq
    let mut sigma_p = *p;
    sigma_p.x *= BETA;
    sigma_p
}
