use crate::{g1::Parameters, Fq, Fr};
use ark_ec::{glv::GLVParameters, msm::ScalarMul, ModelParameters};
use ark_ff::MontFp;

impl ScalarMul for Parameters {
    type CurveAffine = crate::G1Affine;
}

impl GLVParameters for Parameters {
    type CurveProjective = crate::G1Projective;

    const COEFFS_ENDOMORPHISM: &'static[Self::BaseField] = &[
        MontFp!(Fq, "4922464560225523242118178942575080391082002530232324381063048548642823052024664478336818169867474395270858391911405337707247735739826664939444490469542109391530482826728203582549674992333383150446779312029624171857054392282775648")
    ];

    const LAMBDA: Self::ScalarField = MontFp!(
        Fr,
        "258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231"
    );

    const COEFF_N: [<Self as ModelParameters>::ScalarField; 4] = [
        MontFp!(
            Fr,
            "293634935485640680722085584138834120324914961969255022593"
        ),
        MontFp!(
            Fr,
            "293634935485640680722085584138834120315328839056164388863"
        ),
        MontFp!(
            Fr,
            "293634935485640680722085584138834120315328839056164388863"
        ),
        MontFp!(
            Fr,
            "587269870971281361444171168277668240640243801025419411456"
        ),
    ];
    const SGN_N: [bool; 4] = [true, false, true, true];

    fn endomorphism(base: &Self::CurveAffine) -> Self::CurveAffine {
        Self::CurveAffine::new(Self::COEFFS_ENDOMORPHISM[0] * base.x, base.y, false)
    }
}
