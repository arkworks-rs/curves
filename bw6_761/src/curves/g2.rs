use ark_ec::{
    models::{short_weierstrass::SWCurveConfig, CurveConfig},
    scalar_mul::glv::GLVConfig,
    short_weierstrass::{Affine, Projective},
};
use ark_ff::{Field, MontFp};

use crate::{Fq, Fr};

pub type G2Affine = Affine<Config>;
pub type G2Projective = Projective<Config>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config;

impl CurveConfig for Config {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR =
    /// 26642435879335816683987677701488073867751118270052650655942102502312977592501693353047140953112195348280268661194869
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0x3de5800000000075,
        0x832ba4061000003b,
        0xc61c554757551c0c,
        0xc856a0853c9db94c,
        0x2c77d5ac34cb12ef,
        0xad1972339049ce76,
    ];

    /// COFACTOR^(-1) mod r =
    /// 214911522365886453591244899095480747723790054550866810551297776298664428889000553861210287833206024638187939842124
    const COFACTOR_INV: Fr = MontFp!("214911522365886453591244899095480747723790054550866810551297776298664428889000553861210287833206024638187939842124");
}

impl SWCurveConfig for Config {
    /// COEFF_A = 0
    const COEFF_A: Fq = Fq::ZERO;

    /// COEFF_B = 4
    const COEFF_B: Fq = MontFp!("4");

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const GENERATOR: G2Affine = G2Affine::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_elem: Self::BaseField) -> Self::BaseField {
        use ark_ff::Zero;
        Self::BaseField::zero()
    }
}

impl GLVConfig for Config {
    const ENDO_COEFFS: &'static [Self::BaseField] = &[
        MontFp!("4922464560225523242118178942575080391082002530232324381063048548642823052024664478336818169867474395270858391911405337707247735739826664939444490469542109391530482826728203582549674992333383150446779312029624171857054392282775648"),
    ];

    const LAMBDA: Self::ScalarField =
        MontFp!("80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410945");

    const SCALAR_DECOMP_COEFFS: [[<Self as CurveConfig>::ScalarField; 2]; 2] = [
        [
            MontFp!("293634935485640680722085584138834120315328839056164388863"),
            MontFp!("293634935485640680722085584138834120324914961969255022593"),
        ],
        [
            MontFp!("293634935485640680722085584138834120324914961969255022593"),
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

/// G2_GENERATOR_X =
///  6445332910596979336035888152774071626898886139774101364933948236926875073754470830732273879639675437155036544153105017729592600560631678554299562762294743927912429096636156401171909259073181112518725201388196280039960074422214428
pub const G2_GENERATOR_X: Fq = MontFp!("6445332910596979336035888152774071626898886139774101364933948236926875073754470830732273879639675437155036544153105017729592600560631678554299562762294743927912429096636156401171909259073181112518725201388196280039960074422214428");

/// G2_GENERATOR_Y =
/// 562923658089539719386922163444547387757586534741080263946953401595155211934630598999300396317104182598044793758153214972605680357108252243146746187917218885078195819486220416605630144001533548163105316661692978285266378674355041
pub const G2_GENERATOR_Y: Fq = MontFp!("562923658089539719386922163444547387757586534741080263946953401595155211934630598999300396317104182598044793758153214972605680357108252243146746187917218885078195819486220416605630144001533548163105316661692978285266378674355041");

#[cfg(test)]
mod test {

    use std::time::Instant;

    use crate::g2;

    use super::*;
    use ark_std::{test_rng, UniformRand};

    #[test]
    fn bench_glv() {
        let mut rng = test_rng();
        let p = Projective::<g2::Config>::rand(&mut rng);
        let s = Fr::rand(&mut rng);
        // bench
        let now = Instant::now();
        for _ in 1..100 {
            let _ = p * s;
        }
        println!("SM: {:?}", now.elapsed());
        let now = Instant::now();
        for _ in 1..100 {
            let _ = g2::Config::glv_mul_projective(p, s);
        }
        println!("GLV: {:?}", now.elapsed());
    }
}
