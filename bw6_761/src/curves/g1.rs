use crate::{Fq, Fr};
use ark_ec::{
    models::{ModelParameters, SWModelParameters},
    short_weierstrass_jacobian::{GroupAffine, GroupProjective},
};
use ark_ff::field_new;

pub type G1Affine = GroupAffine<Parameters>;
pub type G1Projective = GroupProjective<Parameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;
    type Affine = G1Affine;

    /// COFACTOR =
    /// 26642435879335816683987677701488073867751118270052650655942102502312977592501693353047140953112195348280268661194876
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0x3de580000000007c,
        0x832ba4061000003b,
        0xc61c554757551c0c,
        0xc856a0853c9db94c,
        0x2c77d5ac34cb12ef,
        0xad1972339049ce76,
    ];

    /// COFACTOR^(-1) mod r =
    /// 91141326767669940707819291241958318717982251277713150053234367522357946997763584490607453720072232540829942217804
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "91141326767669940707819291241958318717982251277713150053234367522357946997763584490607453720072232540829942217804");
}

impl SWModelParameters for Parameters {
    /// COEFF_A = 0
    #[rustfmt::skip]

    const COEFF_A: Fq = field_new!(Fq, "0");

    /// COEFF_B = -1
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "-1");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G1_GENERATOR_X, G1_GENERATOR_Y);
    #[inline(always)]
    fn mul_by_a(_elem: &Self::BaseField) -> Self::BaseField {
        use ark_ff::Zero;
        Self::BaseField::zero()
    }
}

/// G1_GENERATOR_X =
/// 6238772257594679368032145693622812838779005809760824733138787810501188623461307351759238099287535516224314149266511977132140828635950940021790489507611754366317801811090811367945064510304504157188661901055903167026722666149426237
#[rustfmt::skip]
pub const G1_GENERATOR_X: Fq = field_new!(Fq, "6238772257594679368032145693622812838779005809760824733138787810501188623461307351759238099287535516224314149266511977132140828635950940021790489507611754366317801811090811367945064510304504157188661901055903167026722666149426237");

/// G1_GENERATOR_Y =
/// 2101735126520897423911504562215834951148127555913367997162789335052900271653517958562461315794228241561913734371411178226936527683203879553093934185950470971848972085321797958124416462268292467002957525517188485984766314758624099
#[rustfmt::skip]
pub const G1_GENERATOR_Y: Fq = field_new!(Fq, "2101735126520897423911504562215834951148127555913367997162789335052900271653517958562461315794228241561913734371411178226936527683203879553093934185950470971848972085321797958124416462268292467002957525517188485984766314758624099");
