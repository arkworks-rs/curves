use crate::{g1::Parameters as G1Parameters, g2::Parameters as G2Parameters, Fq, Fr, FQ_ZERO};
use ark_ec::{glv::GLVParameters, msm::ScalarMul, ModelParameters};
use ark_ff::{MontFp, QuadExt};

impl ScalarMul for G1Parameters {
    type CurveAffine = crate::G1Affine;
}

impl GLVParameters for G1Parameters {
    type CurveProjective = crate::G1Projective;

    const COEFFS_ENDOMORPHISM: &'static [Self::BaseField] = &[MontFp!(
        Fq,
        "21888242871839275220042445260109153167277707414472061641714758635765020556616"
    )];

    const LAMBDA: Self::ScalarField = MontFp!(
        Fr,
        "21888242871839275217838484774961031246154997185409878258781734729429964517155"
    );

    const COEFF_N: [<Self as ModelParameters>::ScalarField; 4] = [
        MontFp!(Fr, "147946756881789319000765030803803410728"),
        MontFp!(Fr, "9931322734385697763"),
        MontFp!(Fr, "9931322734385697763"),
        MontFp!(Fr, "147946756881789319010696353538189108491"),
    ];
    const SGN_N: [bool; 4] = [false, true, false, false];

    fn endomorphism(base: &Self::CurveAffine) -> Self::CurveAffine {
        Self::CurveAffine::new(Self::COEFFS_ENDOMORPHISM[0] * base.x, base.y, false)
    }
}

impl ScalarMul for G2Parameters {
    type CurveAffine = crate::G2Affine;
}

impl GLVParameters for G2Parameters {
    type CurveProjective = crate::G2Projective;

    const COEFFS_ENDOMORPHISM: &'static [Self::BaseField] = &[QuadExt!(
        MontFp!(
            Fq,
            "21888242871839275220042445260109153167277707414472061641714758635765020556616"
        ),
        FQ_ZERO
    )];

    const LAMBDA: Self::ScalarField = MontFp!(
        Fr,
        "21888242871839275217838484774961031246154997185409878258781734729429964517155"
    );

    const COEFF_N: [<Self as ModelParameters>::ScalarField; 4] = [
        MontFp!(Fr, "147946756881789319000765030803803410728"),
        MontFp!(Fr, "9931322734385697763"),
        MontFp!(Fr, "9931322734385697763"),
        MontFp!(Fr, "147946756881789319010696353538189108491"),
    ];
    const SGN_N: [bool; 4] = [false, true, false, false];

    fn endomorphism(base: &Self::CurveAffine) -> Self::CurveAffine {
        Self::CurveAffine::new(Self::COEFFS_ENDOMORPHISM[0] * base.x, base.y, false)
    }
}
