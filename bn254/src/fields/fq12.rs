use ark_ff::{fields::*, MontFp};

use crate::*;

pub type Fq12 = Fp12<Fq12Config>;

#[derive(Clone, Copy)]
pub struct Fq12Config;

impl Fp12Config for Fq12Config {
    type Fp6Config = Fq6Config;

    const NONRESIDUE: Fq6 = Fq6::new(FQ2_ZERO, FQ2_ONE, FQ2_ZERO);

    const FROBENIUS_COEFF_FP12_C1: &'static [Fq2] = &[
        // Fp2::NONRESIDUE^(((q^0) - 1) / 6)
        Fq2::new(MontFp!("1"), MontFp!("0")),
        // Fp2::NONRESIDUE^(((q^1) - 1) / 6)
        Fq2::new(
            MontFp!("8376118865763821496583973867626364092589906065868298776909617916018768340080"),
            MontFp!(
                "16469823323077808223889137241176536799009286646108169935659301613961712198316"
            ),
        ),
        // Fp2::NONRESIDUE^(((q^2) - 1) / 6)
        Fq2::new(
            MontFp!(
                "21888242871839275220042445260109153167277707414472061641714758635765020556617"
            ),
            MontFp!("0"),
        ),
        // Fp2::NONRESIDUE^(((q^3) - 1) / 6)
        Fq2::new(
            MontFp!(
                "11697423496358154304825782922584725312912383441159505038794027105778954184319"
            ),
            MontFp!("303847389135065887422783454877609941456349188919719272345083954437860409601"),
        ),
        // Fp2::NONRESIDUE^(((q^4) - 1) / 6)
        Fq2::new(
            MontFp!(
                "21888242871839275220042445260109153167277707414472061641714758635765020556616"
            ),
            MontFp!("0"),
        ),
        // Fp2::NONRESIDUE^(((q^5) - 1) / 6)
        Fq2::new(
            MontFp!("3321304630594332808241809054958361220322477375291206261884409189760185844239"),
            MontFp!("5722266937896532885780051958958348231143373700109372999374820235121374419868"),
        ),
        // Fp2::NONRESIDUE^(((q^6) - 1) / 6)
        Fq2::new(MontFp!("-1"), MontFp!("0")),
        // Fp2::NONRESIDUE^(((q^7) - 1) / 6)
        Fq2::new(
            MontFp!(
                "13512124006075453725662431877630910996106405091429524885779419978626457868503"
            ),
            MontFp!("5418419548761466998357268504080738289687024511189653727029736280683514010267"),
        ),
        // Fp2::NONRESIDUE^(((q^8) - 1) / 6)
        Fq2::new(
            MontFp!("2203960485148121921418603742825762020974279258880205651966"),
            MontFp!("0"),
        ),
        // Fp2::NONRESIDUE^(((q^9) - 1) / 6)
        Fq2::new(
            MontFp!(
                "10190819375481120917420622822672549775783927716138318623895010788866272024264"
            ),
            MontFp!(
                "21584395482704209334823622290379665147239961968378104390343953940207365798982"
            ),
        ),
        // Fp2::NONRESIDUE^(((q^10) - 1) / 6)
        Fq2::new(
            MontFp!("2203960485148121921418603742825762020974279258880205651967"),
            MontFp!("0"),
        ),
        // Fp2::NONRESIDUE^(((q^11) - 1) / 6)
        Fq2::new(
            MontFp!(
                "18566938241244942414004596690298913868373833782006617400804628704885040364344"
            ),
            MontFp!(
                "16165975933942742336466353786298926857552937457188450663314217659523851788715"
            ),
        ),
    ];
}
