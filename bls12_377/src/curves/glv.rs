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

    const COEFF_B1: Self::BaseField = MontFp!(Fq, "258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231");

    const COEFF_B2: Self::BaseField = MontFp!(Fq, "0");

    const COEFF_B3: Self::BaseField = MontFp!(Fq, "0");

    const COEFF_C1: Self::BaseField = MontFp!(Fq, "0");

    const COEFF_C2: Self::BaseField = MontFp!(Fq, "0");

    const LAMBDA: Self::ScalarField = MontFp!(Fr, "8444461749428370424248824938781546531284005582649182570233710176290576793600");

    // LLL Matrix
    const COEFF_N11: Self::ScalarField = MontFp!(Fr, "91893752504881257701523279626832445441");

    const COEFF_N12: Self::ScalarField = MontFp!(Fr, "1");

    const COEFF_N21: Self::ScalarField = MontFp!(Fr, "-1");

    const COEFF_N22: Self::ScalarField = MontFp!(Fr, "91893752504881257701523279626832445440");

    /// Mapping a point G to phi(G):= lambda G where phi is the endomorphism
    fn endomorphism(base: &Self::CurveAffine) -> Self::CurveAffine {
        Self::CurveAffine::new(Self::COEFF_B1 * base.x, base.y, false)
    }
}
