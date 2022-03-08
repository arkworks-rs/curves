use ark_algebra_test_templates::{curves::*, groups::*};
use ark_ec::{glv::GLVParameters, AffineCurve, ProjectiveCurve};
use ark_ff::{bytes::FromBytes, MontFp, Zero};
use ark_std::{rand::Rng, str::FromStr, test_rng};

use crate::*;

#[test]
fn test_projective_curve() {
    curve_tests::<EdwardsProjective>();

    edwards_tests::<BandersnatchParameters>();
    montgomery_conversion_test::<BandersnatchParameters>();
    sw_tests::<BandersnatchParameters>();
}

#[test]
fn test_projective_group() {
    let mut rng = test_rng();
    let a = rng.gen();
    let b = rng.gen();

    let c = rng.gen();
    let d = rng.gen();

    for _i in 0..100 {
        group_test::<EdwardsProjective>(a, b);
        group_test::<SWProjective>(c, d);
    }
}

#[test]
fn test_affine_group() {
    let mut rng = test_rng();
    let a: EdwardsAffine = rng.gen();
    let b: EdwardsAffine = rng.gen();
    for _i in 0..100 {
        group_test::<EdwardsAffine>(a, b);
    }
}

#[test]
fn test_generator() {
    // edward curve
    let generator = EdwardsAffine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());

    // weierstrass curve
    let generator = SWAffine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_conversion() {
    // edward curve
    let mut rng = test_rng();
    let a: EdwardsAffine = rng.gen();
    let b: EdwardsAffine = rng.gen();
    let a_b = {
        use ark_ec::group::Group;
        (a + &b).double().double()
    };
    let a_b2 = (a.into_projective() + &b.into_projective())
        .double()
        .double();
    assert_eq!(a_b, a_b2.into_affine());
    assert_eq!(a_b.into_projective(), a_b2);

    // weierstrass curve
    let mut rng = test_rng();
    let a: SWProjective = rng.gen();
    let b: SWProjective = rng.gen();
    let a_b = { (a + &b).double().double() };
    let a_b2 = (a + &b).double().double();
    assert_eq!(a_b.into_affine(), a_b2.into_affine());
}

#[test]
fn test_scalar_multiplication() {
    let f1 = Fr::from_str(
        "4257185345094557079734489188109952172285839137338142340240392707284963971010",
    )
    .unwrap();
    let f2 = Fr::from_str(
        "1617998875791656082457755819308421023664764572929977389209373068350490665160",
    )
    .unwrap();

    let g = EdwardsAffine::from_str(
        "(29627151942733444043031429156003786749302466371339015363120350521834195802525, \
         27488387519748396681411951718153463804682561779047093991696427532072116857978)",
    )
    .unwrap();
    let f1f2g = EdwardsAffine::from_str(
        "(16530491029447613915334753043669938793793987372416328257719459807614119987301, \
         42481140308370805476764840229335460092474682686441442216596889726548353970772)",
    )
    .unwrap();

    assert!(!g.is_zero());
    assert!(!f1f2g.is_zero());

    let f1g = g.mul(f1).into_affine();
    assert_eq!(g.mul(f1 * &f2).into_affine(), f1f2g);
    assert_eq!(f1g.mul(f2).into_affine(), f1f2g);
}

#[test]
fn test_bytes() {
    let g_from_repr = EdwardsAffine::from_str(
        "(29627151942733444043031429156003786749302466371339015363120350521834195802525, \
         27488387519748396681411951718153463804682561779047093991696427532072116857978)",
    )
    .unwrap();

    let g_bytes = ark_ff::to_bytes![g_from_repr].unwrap();
    let g = EdwardsAffine::read(g_bytes.as_slice()).unwrap();
    assert_eq!(g_from_repr, g);
}

#[test]
fn test_montgomery_conversion() {
    montgomery_conversion_test::<BandersnatchParameters>();
}

#[test]
fn test_psi() {
    let base_point = EdwardsAffine::from_str(
        "(29627151942733444043031429156003786749302466371339015363120350521834195802525, \
        27488387519748396681411951718153463804682561779047093991696427532072116857978)",
    )
    .unwrap();
    let psi_point = EdwardsAffine::from_str(
        "(3995099504672814451457646880854530097687530507181962222512229786736061793535, \
         33370049900732270411777328808452912493896532385897059012214433666611661340894)",
    )
    .unwrap();

    let t = BandersnatchParameters::endomorphism(&base_point);
    assert_eq!(t, psi_point);
}

#[test]
fn test_decomp() {
    let scalar: Fr = MontFp!(
        Fr,
        "4257185345094557079734489188109952172285839137338142340240392707284963971010"
    );
    let k1: Fr = MontFp!(Fr, "30417741863887432744214758610616508258");
    let k2: Fr = MontFp!(Fr, "-6406990765953933188067911864924578940");
    assert_eq!(
        BandersnatchParameters::scalar_decomposition(&scalar),
        (k1, k2)
    )
}

#[test]
fn test_msm() {
    let base_point = EdwardsAffine::from_str(
        "(29627151942733444043031429156003786749302466371339015363120350521834195802525, \
        27488387519748396681411951718153463804682561779047093991696427532072116857978)",
    )
    .unwrap();
    let psi_point = EdwardsAffine::from_str(
        "(3995099504672814451457646880854530097687530507181962222512229786736061793535, \
         33370049900732270411777328808452912493896532385897059012214433666611661340894)",
    )
    .unwrap();
    let t = BandersnatchParameters::endomorphism(&base_point);
    assert_eq!(t, psi_point);

    let scalar: Fr = MontFp!(
        Fr,
        "4257185345094557079734489188109952172285839137338142340240392707284963971010"
    );
    let k1: Fr = MontFp!(Fr, "30417741863887432744214758610616508258");
    let k2: Fr = MontFp!(Fr, "-6406990765953933188067911864924578940");
    assert_eq!(
        BandersnatchParameters::scalar_decomposition(&scalar),
        (k1, k2)
    );

    let res = EdwardsAffine::from_str(
        "(6018810645516749504657411940673266094850700554607419759628157493373766067122, \
         13929928331741974885869757126422340790588975043986274897468601817898742989376)",
    )
    .unwrap();

    let tmp = base_point.mul(scalar);
    let res2 = super::glv::multi_scalar_mul(&base_point, &k1, &psi_point, &k2).into_affine();

    assert_eq!(tmp.into_affine(), res);
    assert_eq!(res, res2);
}

#[test]
fn test_gen_mul() {
    let a = EdwardsAffine::prime_subgroup_generator();
    let r: Fr = MontFp!(
        Fr,
        "4257185345094557079734489188109952172285839137338142340240392707284963971010"
    );

    let b = a.mul(r);
    let c = BandersnatchParameters::glv_mul(&a, &r);

    assert_eq!(b.into_affine(), c.into_affine())
}

#[test]
fn test_rnd_mul() {
    use ark_std::{rand::Rng, test_rng};

    let mut rng = test_rng();
    for _ in 0..100 {
        let a: EdwardsAffine = rng.gen();
        let r: Fr = rng.gen();

        let b = a.mul(r);
        let c = BandersnatchParameters::glv_mul(&a, &r);

        assert_eq!(b.into_affine(), c.into_affine())
    }
}
