use ark_ff::{fields::*, CubicExt, MontFp, QuadExt};

use crate::*;

pub type Fq12 = Fp12<Fq12Config>;

#[derive(Clone, Copy)]
pub struct Fq12Config;

impl Fp12Parameters for Fq12Config {
    type Fp6Params = Fq6Config;

    const NONRESIDUE: Fq6 = CubicExt!(FQ2_ZERO, FQ2_ONE, FQ2_ZERO);

    const FROBENIUS_COEFF_FP12_C1: &'static [Fq2] = &[
        // Fp2::NONRESIDUE^(((q^0) - 1) / 6)
        QuadExt!(FQ_ONE, FQ_ZERO),
        // Fp2::NONRESIDUE^(((q^1) - 1) / 6)
        QuadExt!(
            MontFp!(Fq, "92949345220277864758624960506473182677953048909283248980960104381795901929519566951595905490535835115111760994353"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^2) - 1) / 6)
        QuadExt!(
            MontFp!(Fq, "80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410946"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^3) - 1) / 6)
        QuadExt!(
            MontFp!(Fq, "216465761340224619389371505802605247630151569547285782856803747159100223055385581585702401816380679166954762214499"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^4) - 1) / 6)
        QuadExt!(
            MontFp!(Fq, "80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410945"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^5) - 1) / 6)
        QuadExt!(
            MontFp!(Fq, "123516416119946754630746545296132064952198520638002533875843642777304321125866014634106496325844844051843001220146"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^6) - 1) / 6)
        QuadExt!(
            MontFp!(Fq, "-1"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^7) - 1) / 6)
        QuadExt!(
            MontFp!(Fq, "165715080792691229252027773188420350858440463845631411558924158284924566418821255823372982649037525009328560463824"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^8) - 1) / 6)
        QuadExt!(
            MontFp!(Fq, "258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^9) - 1) / 6)
        QuadExt!(
            MontFp!(Fq, "42198664672744474621281227892288285906241943207628877683080515507620245292955241189266486323192680957485559243678"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^10) - 1) / 6)
        QuadExt!(
            MontFp!(Fq, "258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047232"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^11) - 1) / 6)
        QuadExt!(
            MontFp!(Fq, "135148009893022339379906188398761468584194992116912126664040619889416147222474808140862391813728516072597320238031"),
            FQ_ZERO,
        ),
    ];
}
