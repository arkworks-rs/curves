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

    // LLL Matrix
    const COEFF_N11: Self::ScalarField = MontFp!(Fr, "228988810152649578064853576960394133504");

    const COEFF_N12: Self::ScalarField = MontFp!(Fr, "1");

    const COEFF_N21: Self::ScalarField = MontFp!(Fr, "-1");

    const COEFF_N22: Self::ScalarField = MontFp!(Fr, "228988810152649578064853576960394133503");

    /// Mapping a point G to phi(G):= lambda G where phi is the endomorphism
    fn endomorphism(base: &Self::CurveAffine) -> Self::CurveAffine {
        Self::CurveAffine::new(Self::COEFF_B1 * base.x, base.y, false)
    }
}

#[test]
fn test_scalar_decomposition() {
    use ark_ec::ModelParameters;
    use ark_ff::One;
    use ark_std::UniformRand;

    let lambda = MontFp!(
        Fr,
        "52435875175126190479447740508185965837461563690374988244538805122978187051009"
    );
    assert_eq!(lambda * lambda * lambda, Fr::one());

    let mut rng = ark_std::test_rng();
    for _i in 0..100 {
        let k = <Parameters as ModelParameters>::ScalarField::rand(&mut rng);
        let (k1, is_k1_positive, k2, is_k2_positive) =
            <Parameters as GLVParameters>::scalar_decomposition(&k);
        if is_k1_positive && is_k2_positive {
            assert_eq!(k1 + k2 * lambda, k);
        }
        if is_k1_positive && !is_k2_positive {
            assert_eq!(k1 - k2 * lambda, k);
        }
        if !is_k1_positive && is_k2_positive {
            assert_eq!(-k1 + k2 * lambda, k);
        }
        if !is_k1_positive && !is_k2_positive {
            assert_eq!(-k1 - k2 * lambda, k);
        }
        // could be nice to check if k1 and k2 are indeed small.
    }
}

#[test]
fn test_beta() {
    // curve of j-invariant 0 so the endomorphism is (x,y) -> (βx, y) where β is a
    // 3rd root of unity.
    use ark_ec::ModelParameters;
    use ark_ff::{Field, One};
    assert_eq!(
        Parameters::COEFF_B1.pow(&[3u64]),
        <Parameters as ModelParameters>::BaseField::one()
    );
    assert_ne!(
        Parameters::COEFF_B1,
        <Parameters as ModelParameters>::BaseField::one()
    );
}

#[test]
fn test_endomorphism() {
    // check that `endomorphism³(P)` is zero
    use ark_ec::{short_weierstrass_jacobian::GroupAffine, AffineCurve, ProjectiveCurve};
    let g = GroupAffine::<Parameters>::prime_subgroup_generator();
    let psi_g = <Parameters as GLVParameters>::endomorphism(&g);
    let psi2_g = <Parameters as GLVParameters>::endomorphism(&psi_g);
    let psi3_g = <Parameters as GLVParameters>::endomorphism(&psi2_g);
    assert_eq!(psi3_g, g);
    let lambda: Fr = MontFp!(
        Fr,
        "52435875175126190479447740508185965837461563690374988244538805122978187051009"
    );
    assert_eq!(psi_g, g.mul(lambda).into_affine());
}

#[test]
fn test_glv() {
    // check that glv_mul indeed computes the scalar multiplication
    use ark_ec::{
        short_weierstrass_jacobian::GroupAffine, AffineCurve, ModelParameters, ProjectiveCurve,
    };
    use ark_std::UniformRand;
    let g = GroupAffine::<Parameters>::prime_subgroup_generator();
    let mut rng = ark_std::test_rng();
    for _i in 0..100 {
        let k = <Parameters as ModelParameters>::ScalarField::rand(&mut rng);
        let k_g = <Parameters as GLVParameters>::glv_mul(&g, &k).into_affine();
        assert_eq!(k_g, g.mul(k).into_affine());
    }
}
