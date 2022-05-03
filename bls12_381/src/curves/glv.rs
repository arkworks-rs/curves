use crate::{g1::Parameters, Fq, Fr};
use ark_ec::{glv::GLVParameters, msm::ScalarMul};
use ark_ff::MontFp;
use num_bigint::Sign::{self, Plus, Minus};

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

    const COEFF_N: [[u32;4]; 4] = [
        [0,1,107522,2890245121], // 228988810152649578064853576960394133504
        [1,0,0,0], // 1
        [1,0,0,0], // -1
        [4294967295,0,107522,2890245121], // 228988810152649578064853576960394133503
    ];
    const SGN_N: [Sign;4] = [Plus, Plus, Minus, Plus];

    fn endomorphism(base: &Self::CurveAffine) -> Self::CurveAffine {
        Self::CurveAffine::new(Self::COEFF_B1 * base.x, base.y, false)
    }
}
