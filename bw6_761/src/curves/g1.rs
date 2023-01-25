use ark_ec::{
    models::{short_weierstrass::SWCurveConfig, CurveConfig},
    scalar_mul::glv::GLVConfig,
    short_weierstrass::{Affine, Projective},
};
use ark_ff::{Field, MontFp};

use crate::{Fq, Fr};

pub type G1Affine = Affine<Config>;
pub type G1Projective = Projective<Config>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config;

impl CurveConfig for Config {
    type BaseField = Fq;
    type ScalarField = Fr;

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
    const COFACTOR_INV: Fr = MontFp!("91141326767669940707819291241958318717982251277713150053234367522357946997763584490607453720072232540829942217804");
}

impl SWCurveConfig for Config {
    /// COEFF_A = 0
    const COEFF_A: Fq = Fq::ZERO;

    /// COEFF_B = -1
    const COEFF_B: Fq = MontFp!("-1");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const GENERATOR: G1Affine = G1Affine::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);
    #[inline(always)]
    fn mul_by_a(_elem: Self::BaseField) -> Self::BaseField {
        use ark_ff::Zero;
        Self::BaseField::zero()
    }
}

impl GLVConfig for Config {
    const ENDO_COEFFS: &'static [Self::BaseField] = &[MontFp!(
        "4922464560225523242118178942575080391082002530232324381063048548642823052024664478336818169867474395270858391911405337707247735739826664939444490469542109391530482826728203582549674992333383150446779312029624171857054392282775648"
    )];

    const LAMBDA: Self::ScalarField =
        MontFp!("258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231");

    const SCALAR_DECOMP_COEFFS: [[<Self as CurveConfig>::ScalarField; 2]; 2] = [
        [
            MontFp!("293634935485640680722085584138834120324914961969255022593"),
            MontFp!("293634935485640680722085584138834120315328839056164388863"),
        ],
        [
            MontFp!("293634935485640680722085584138834120315328839056164388863"),
            MontFp!("587269870971281361444171168277668240640243801025419411456"),
        ],
    ];
    const SGN_N: [bool; 4] = [true, false, true, true];

    fn endomorphism(p: &Projective<Self>) -> Projective<Self> {
        let mut res = (*p).clone();
        res.x *= Self::ENDO_COEFFS[0];
        res
    }

    fn endomorphism_affine(p: &Affine<Self>) -> Affine<Self> {
        let mut res = (*p).clone();
        res.x *= Self::ENDO_COEFFS[0];
        res
    }
}

/// G1_GENERATOR_X =
/// 6238772257594679368032145693622812838779005809760824733138787810501188623461307351759238099287535516224314149266511977132140828635950940021790489507611754366317801811090811367945064510304504157188661901055903167026722666149426237
pub const G1_GENERATOR_X: Fq = MontFp!("6238772257594679368032145693622812838779005809760824733138787810501188623461307351759238099287535516224314149266511977132140828635950940021790489507611754366317801811090811367945064510304504157188661901055903167026722666149426237");

/// G1_GENERATOR_Y =
/// 2101735126520897423911504562215834951148127555913367997162789335052900271653517958562461315794228241561913734371411178226936527683203879553093934185950470971848972085321797958124416462268292467002957525517188485984766314758624099
pub const G1_GENERATOR_Y: Fq = MontFp!("2101735126520897423911504562215834951148127555913367997162789335052900271653517958562461315794228241561913734371411178226936527683203879553093934185950470971848972085321797958124416462268292467002957525517188485984766314758624099");

#[cfg(test)]
mod test {

    use std::time::Instant;

    use crate::g1;

    use super::*;
    use ark_std::{test_rng, UniformRand};

    #[test]
    fn bench_glv() {
        let mut rng = test_rng();
        let p = Projective::<g1::Config>::rand(&mut rng);
        let s = Fr::rand(&mut rng);
        // bench
        let now = Instant::now();
        for _ in 1..100 {
            let _ = p * s;
        }
        println!("SM: {:?}", now.elapsed());
        let now = Instant::now();
        for _ in 1..100 {
            let _ = g1::Config::glv_mul_projective(p, s);
        }
        println!("GLV: {:?}", now.elapsed());
    }
}
