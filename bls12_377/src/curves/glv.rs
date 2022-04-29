use crate::{Fq, Fr};
use ark_ec::{glv::GLVParameters, msm::ScalarMul};
use ark_ff::{MontFp};
use crate::g1::Parameters;

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

#[test]
fn test_scalar_decomposition() {
    use ark_ec::ModelParameters;
    use ark_std::UniformRand;
    use ark_ff::One;
    use rand::thread_rng;
    
    let lambda = MontFp!(Fr, "8444461749428370424248824938781546531284005582649182570233710176290576793600");
    assert_eq!(lambda*lambda*lambda, Fr::one());
    
    let mut rng = thread_rng();
    let k = <Parameters as ModelParameters>::ScalarField::rand(&mut rng);
    let (k1, k2, is_k2_positive) = <Parameters as GLVParameters>::scalar_decomposition(&k);
    // this is not working yet.
    if is_k2_positive {
        assert_eq!(k1 + k2 * lambda, k);
    }
    else {
        assert_eq!(k1-k2*lambda, k);
    }
}

 #[test]
 fn test_beta() {
     // curve of j-invariant 0 so the endomorphism is (x,y) -> (βx, y) where β is a 3rd root of unity.
     use ark_ff::{Field, One};
     use ark_ec::ModelParameters;
     assert_eq!(Parameters::COEFF_B1.pow(&[3u64]), <Parameters as ModelParameters>::BaseField::one());
     assert_ne!(Parameters::COEFF_B1, <Parameters as ModelParameters>::BaseField::one());
 }

 #[test]
 fn test_endomorphism() {
    // check that `endomorphism³(P)` is zero
    use ark_ec::{short_weierstrass_jacobian::GroupAffine, AffineCurve};
    let g = GroupAffine::<Parameters>::prime_subgroup_generator();
    let psi_g = <Parameters as GLVParameters>::endomorphism(&g);
    let psi2_g = <Parameters as GLVParameters>::endomorphism(&psi_g);
    let psi3_g = <Parameters as GLVParameters>::endomorphism(&psi2_g);
    assert_eq!(psi3_g, g);
 }

#[test]
fn test_glv() {
    // check that glv_mul indeed computes the scalar multiplication
    use ark_ec::{short_weierstrass_jacobian::GroupAffine, AffineCurve, ProjectiveCurve, ModelParameters};
    let g = GroupAffine::<Parameters>::prime_subgroup_generator();
    let k = <Parameters as ModelParameters>::ScalarField::from(12345u64);
    let k_g =  <Parameters as GLVParameters>::glv_mul(&g, &k).into_affine();
    assert_eq!(k_g, g.mul(k));
}