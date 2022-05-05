use crate::{BandersnatchParameters, Fq, Fr};
use ark_ec::{glv::GLVParameters, msm::ScalarMul, ModelParameters, ProjectiveCurve};
use ark_ff::{MontFp, One};

impl ScalarMul for BandersnatchParameters {
    type CurveAffine = crate::EdwardsAffine;
}

impl GLVParameters for BandersnatchParameters {
    type CurveProjective = crate::EdwardsProjective;

    const COEFFS_ENDOMORPHISM: &'static [Self::BaseField] = &[
        MontFp!(
            Fq,
            "37446463827641770816307242315180085052603635617490163568005256780843403514036"
        ),
        MontFp!(
            Fq,
            "49199877423542878313146170939139662862850515542392585932876811575731455068989"
        ),
    ];

    const LAMBDA: Self::ScalarField = MontFp!(
        Fr,
        "8913659658109529928382530854484400854125314752504019737736543920008458395397"
    );

    const COEFF_N: [<Self as ModelParameters>::ScalarField; 4] = [
        MontFp!(Fr, "113482231691339203864511368254957623327"),
        MontFp!(Fr, "10741319382058138887739339959866629956"),
        MontFp!(Fr, "21482638764116277775478679919733259912"),
        MontFp!(Fr, "113482231691339203864511368254957623327"),
    ];
    const SGN_N: [bool; 4] = [true, true, false, true];

    fn endomorphism(base: &Self::CurveAffine) -> Self::CurveAffine {
        let x = base.x;
        let y = base.y;

        let xy = x * y;
        let y_square = y * y;
        let f_y = Self::COEFFS_ENDOMORPHISM[1] * (Fq::one() - y_square);
        let g_y = Self::COEFFS_ENDOMORPHISM[0] * (y_square + Self::COEFFS_ENDOMORPHISM[0]);
        let h_y = y_square - Self::COEFFS_ENDOMORPHISM[0];

        Self::CurveProjective::new(f_y * h_y, g_y * xy, Fq::one(), h_y * xy).into_affine()
    }
}
