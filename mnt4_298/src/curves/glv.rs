use crate::{g1::Parameters, Fq, Fr};
use ark_ec::{glv::GLVParameters, msm::ScalarMul, ModelParameters};
use ark_ff::MontFp;

impl ScalarMul for Parameters {
    type CurveAffine = crate::G1Affine;
}

impl GLVParameters for Parameters {
    type CurveProjective = crate::G1Projective;

    const COEFFS_ENDOMORPHISM: &'static [Self::BaseField] = &[MontFp!(
        Fq,
        ""
    )];

    const LAMBDA: Self::ScalarField = MontFp!(
        Fr,
        ""
    );

    const COEFF_N: [<Self as ModelParameters>::ScalarField; 4] = [
        MontFp!(Fr, ""),
        MontFp!(Fr, ""),
        MontFp!(Fr, ""),
        MontFp!(Fr, ""),
    ];
    const SGN_N: [bool; 4] = [???false, true, false, false];

    fn endomorphism(base: &Self::CurveAffine) -> Self::CurveAffine {
        ??
        Self::CurveAffine::new(Self::COEFFS_ENDOMORPHISM[0] * base.x, base.y, false)
    }
}
