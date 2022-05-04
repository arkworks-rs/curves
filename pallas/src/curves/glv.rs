use crate::{Fq, Fr, PallasParameters};
use ark_ec::{glv::GLVParameters, msm::ScalarMul, ModelParameters};
use ark_ff::MontFp;

impl ScalarMul for PallasParameters {
    type CurveAffine = crate::Affine;
}

impl GLVParameters for PallasParameters {
    type CurveProjective = crate::Projective;

    const COEFF_A1: Self::BaseField = MontFp!(Fq, "0");

    const COEFF_A2: Self::BaseField = MontFp!(Fq, "0");

    const COEFF_A3: Self::BaseField = MontFp!(Fq, "0");

    const COEFF_B1: Self::BaseField = MontFp!(
        Fq,
        "20444556541222657078399132219657928148671392403212669005631716460534733845831"
    );

    const COEFF_B2: Self::BaseField = MontFp!(Fq, "0");

    const COEFF_B3: Self::BaseField = MontFp!(Fq, "0");

    const COEFF_C1: Self::BaseField = MontFp!(Fq, "0");

    const COEFF_C2: Self::BaseField = MontFp!(Fq, "0");

    const LAMBDA: Self::ScalarField = MontFp!(
        Fr,
        "26005156700822196841419187675678338661165322343552424574062261873906994770353"
    );

    const COEFF_N: [<Self as ModelParameters>::ScalarField; 4] = [
        MontFp!(Fr, "98231058071100081932162823354453065728"),
        MontFp!(Fr, "98231058071186745657228807397848383489"),
        MontFp!(Fr, "196462116142286827589391630752301449217"),
        MontFp!(Fr, "98231058071100081932162823354453065728"),
    ];
    const SGN_N: [bool; 4] = [false, true, false, false];

    fn endomorphism(base: &Self::CurveAffine) -> Self::CurveAffine {
        Self::CurveAffine::new(Self::COEFF_B1 * base.x, base.y, false)
    }
}