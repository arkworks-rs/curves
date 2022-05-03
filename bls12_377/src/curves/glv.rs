use crate::{g1::Parameters, Fq, Fr};
use ark_ec::{glv::GLVParameters, msm::ScalarMul};
use ark_ff::MontFp;
use num_bigint::{Sign::{*, self}};

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

    const LAMBDA: Self::ScalarField = MontFp!(
        Fr,
        "8444461749428370424248824938781546531284005582649182570233710176290576793600"
    );

    // LLL Matrix
    const COEFF_N: [[u32;4];4] = [
        [1,168919040,2415919105,1159862220], // 91893752504881257701523279626832445441
        [1,0,0,0], // 1
        [1,0,0,0], // -1
        [0,168919040,2415919105,1159862220], // 91893752504881257701523279626832445440
    ];
    const SGN_N: [Sign;4] = [Plus, Plus, Minus, Plus];

    /// Mapping a point G to phi(G):= lambda G where phi is the endomorphism
    fn endomorphism(base: &Self::CurveAffine) -> Self::CurveAffine {
        Self::CurveAffine::new(Self::COEFF_B1 * base.x, base.y, false)
    }
}
