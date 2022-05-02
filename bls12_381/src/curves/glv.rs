use crate::{g1::Parameters, Fq, Fr};
use ark_ec::{glv::GLVParameters, msm::ScalarMul};
use ark_ff::MontFp;

impl ScalarMul for Parameters {
    type CurveAffine = crate::G1Affine;
}

impl GLVParameters for Parameters {
    type CurveProjective = crate::G1Projective;

    const COEFF_A1: Self::BaseField = MontFp!(Fq, "0");

    const COEFF_A2: Self::BaseField = MontFp!(Fq, "0");

    const COEFF_A3: Self::BaseField = MontFp!(Fq, "0");

    const COEFF_B1: Self::BaseField = MontFp!(Fq, "793479390729215512621379701633421447060886740281060493010456487427281649075476305620758731620350");

    const COEFF_B2: Self::BaseField = MontFp!(Fq, "0");

    const COEFF_B3: Self::BaseField = MontFp!(Fq, "0");

    const COEFF_C1: Self::BaseField = MontFp!(Fq, "0");

    const COEFF_C2: Self::BaseField = MontFp!(Fq, "0");

    const LAMBDA: Self::ScalarField = MontFp!(
        Fr,
        "52435875175126190479447740508185965837461563690374988244538805122978187051009"
    );

    const COEFF_N11: Self::ScalarField = MontFp!(Fr, "228988810152649578064853576960394133504");

    const COEFF_N12: Self::ScalarField = MontFp!(Fr, "1");

    const COEFF_N21: Self::ScalarField = MontFp!(Fr, "-1");

    const COEFF_N22: Self::ScalarField = MontFp!(Fr, "228988810152649578064853576960394133503");

    fn endomorphism(base: &Self::CurveAffine) -> Self::CurveAffine {
        Self::CurveAffine::new(Self::COEFF_B1 * base.x, base.y, false)
    }
}
