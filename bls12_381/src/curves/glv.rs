use crate::{g1::Parameters, Fq, Fr};
use ark_ec::{glv::GLVParameters, msm::ScalarMul, ModelParameters};
use ark_ff::MontFp;

impl ScalarMul for Parameters {
    type CurveAffine = crate::G1Affine;
}

impl GLVParameters for Parameters {
    type CurveProjective = crate::G1Projective;

    const COEFFS_ENDOMORPHISM: &'static[Self::BaseField] = &[
        MontFp!(Fq, "793479390729215512621379701633421447060886740281060493010456487427281649075476305620758731620350")
    ];

    const LAMBDA: Self::ScalarField = MontFp!(
        Fr,
        "52435875175126190479447740508185965837461563690374988244538805122978187051009"
    );

    const COEFF_N: [<Self as ModelParameters>::ScalarField; 4] = [
        MontFp!(Fr, "228988810152649578064853576960394133504"),
        MontFp!(Fr, "1"),
        MontFp!(Fr, "1"),
        MontFp!(Fr, "228988810152649578064853576960394133503"),
    ];
    const SGN_N: [bool; 4] = [true, true, false, true];

    fn endomorphism(base: &Self::CurveAffine) -> Self::CurveAffine {
        Self::CurveAffine::new(Self::COEFFS_ENDOMORPHISM[0] * base.x, base.y, false)
    }
}
