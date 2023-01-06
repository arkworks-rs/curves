use ark_ec::{
    models::CurveConfig,
    scalar_mul::glv::GLVConfig,
    short_weierstrass::{self, SWCurveConfig},
    twisted_edwards::{Affine, MontCurveConfig, Projective, TECurveConfig},
};
use ark_ff::{Field, MontFp};

use crate::{Fq, Fr};

#[cfg(test)]
mod tests;

pub type EdwardsAffine = Affine<BandersnatchConfig>;
pub type EdwardsProjective = Projective<BandersnatchConfig>;

pub type SWAffine = short_weierstrass::Affine<BandersnatchConfig>;
pub type SWProjective = short_weierstrass::Projective<BandersnatchConfig>;

/// `bandersnatch` is an incomplete twisted Edwards curve. These curves have
/// equations of the form: ax² + y² = 1 + dx²y².
/// over some base finite field Fq.
///
/// bandersnatch's curve equation: -5x² + y² = 1 + dx²y²
///
/// q = 52435875175126190479447740508185965837690552500527637822603658699938581184513.
///
/// a = -5.
/// d = (138827208126141220649022263972958607803/
///     171449701953573178309673572579671231137) mod q
///   = 45022363124591815672509500913686876175488063829319466900776701791074614335719.
///
/// Sage script to calculate these:
///
/// ```text
/// q = 52435875175126190479447740508185965837690552500527637822603658699938581184513
/// Fq = GF(q)
/// d = (Fq(138827208126141220649022263972958607803)/Fq(171449701953573178309673572579671231137))
/// ```
/// These parameters and the sage script obtained from:
/// <https://github.com/asanso/Bandersnatch/>
///
/// bandersnatch also has a short Weierstrass curve form, following the
/// form: y² = x³ + A * x + B
/// where
///
/// A = 10773120815616481058602537765553212789256758185246796157495669123169359657269
/// B = 29569587568322301171008055308580903175558631321415017492731745847794083609535
///
/// Script to transfer between different curves are available
/// <https://github.com/zhenfeizhang/bandersnatch/blob/main/bandersnatch/script/bandersnatch.sage>
#[derive(Clone, Default, PartialEq, Eq)]
pub struct BandersnatchConfig;

pub type EdwardsConfig = BandersnatchConfig;
pub type SWConfig = BandersnatchConfig;

impl CurveConfig for BandersnatchConfig {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 4
    const COFACTOR: &'static [u64] = &[4];

    /// COFACTOR^(-1) mod r =
    /// 9831726595336160714896451345284868594481866920080427688839802480047265754601
    const COFACTOR_INV: Fr =
        MontFp!("9831726595336160714896451345284868594481866920080427688839802480047265754601");
}

impl TECurveConfig for BandersnatchConfig {
    /// COEFF_A = -5
    const COEFF_A: Fq = MontFp!("-5");

    /// COEFF_D = (138827208126141220649022263972958607803/
    /// 171449701953573178309673572579671231137) mod q
    const COEFF_D: Fq =
        MontFp!("45022363124591815672509500913686876175488063829319466900776701791074614335719");

    /// AFFINE_GENERATOR_COEFFS = (GENERATOR_X, GENERATOR_Y)
    const GENERATOR: EdwardsAffine = EdwardsAffine::new_unchecked(TE_GENERATOR_X, TE_GENERATOR_Y);

    type MontCurveConfig = BandersnatchConfig;

    /// Multiplication by `a` is multiply by `-5`.
    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        -(elem.double().double() + elem)
    }
}

impl MontCurveConfig for BandersnatchConfig {
    /// COEFF_A = 29978822694968839326280996386011761570173833766074948509196803838190355340952
    const COEFF_A: Fq =
        MontFp!("29978822694968839326280996386011761570173833766074948509196803838190355340952");

    /// COEFF_B = 25465760566081946422412445027709227188579564747101592991722834452325077642517
    const COEFF_B: Fq =
        MontFp!("25465760566081946422412445027709227188579564747101592991722834452325077642517");

    type TECurveConfig = BandersnatchConfig;
}

// The TE form generator is generated following Zcash's fashion:
//  "The generators of G1 and G2 are computed by finding the lexicographically
//   smallest valid x-coordinate, and its lexicographically smallest
//   y-coordinate and scaling it by the cofactor such that the result is not
//   the point at infinity."
// The SW form generator is the same TE generator converted into SW form,
// obtained from the scripts:
//   <https://github.com/zhenfeizhang/bandersnatch/blob/main/bandersnatch/script/bandersnatch.sage>

/// x coordinate for TE curve generator
const TE_GENERATOR_X: Fq =
    MontFp!("18886178867200960497001835917649091219057080094937609519140440539760939937304");

/// y coordinate for TE curve generator
const TE_GENERATOR_Y: Fq =
    MontFp!("19188667384257783945677642223292697773471335439753913231509108946878080696678");

/// x coordinate for SW curve generator
const SW_GENERATOR_X: Fq =
    MontFp!("4732093294267640299242820317528400560681136891967543338160850811774078125840");

/// y coordinate for SW curve generator
const SW_GENERATOR_Y: Fq =
    MontFp!("31127102290931869693084292284935581507759552409643462510093198106308390504714");

// We define the WS equation given in eprint/2021/1152
impl SWCurveConfig for BandersnatchConfig {
    /// COEFF_A = 52435875175126190479447740508185965837690552500527637822603658699934817984513
    const COEFF_A: Self::BaseField =
        MontFp!("52435875175126190479447740508185965837690552500527637822603658699934817984513");

    /// COEFF_B = 52435875175126190479447740508185965837690552500527637822603658621262613184513
    const COEFF_B: Self::BaseField =
        MontFp!("52435875175126190479447740508185965837690552500527637822603658621262613184513");

    /// generators
    const GENERATOR: SWAffine = SWAffine::new_unchecked(SW_GENERATOR_X, SW_GENERATOR_Y);
}

impl GLVConfig for BandersnatchConfig {
    const COEFFS_ENDOMORPHISM: &'static [Self::BaseField] = &[
        MontFp!("44800"),
        MontFp!("2257920000"),
        MontFp!("26217937587563095239723870254092982918845276250263818911301829349969290592256"), // u² in eprint/2021/1152 page 5
        MontFp!("8089994378958280414021351509578931277711957629993758335181991576135134307306"), // u³ in eprint/2021/1152 page 5
        MontFp!("52435875175126190479447740508185965837690552500527637822603658699938330304513"), // t0 in eprint/2021/1152 page 5
    ];

    const LAMBDA: Self::ScalarField =
        MontFp!("8913659658109529928382530854484400854125314752504019737736543920008458395397");

    const COEFF_N: [<Self as CurveConfig>::ScalarField; 4] = [
        MontFp!("113482231691339203864511368254957623327"),
        MontFp!("10741319382058138887739339959866629956"),
        MontFp!("21482638764116277775478679919733259912"),
        MontFp!("113482231691339203864511368254957623327"),
    ];
    const SGN_N: [bool; 4] = [true, true, false, true];

    fn endomorphism(base: &SWProjective) -> SWProjective {
        /*
        From the eprint, (x,y) \mapsto (u² (x² + 44800x + 2257920000)/(x+44800), u³ y (x² + 2*44800 x + t0) / (x+44800)²)
        In Jacobian projective coordinates:
        (x,y,z) \mapsto (u² (x²/z⁴ + 44800x/z² + 2257920000)/(x/z²+44800), u³ y/z³ (x²/z⁴ + 2*44800 x/z² + t0) / (x/²+44800)²)
                      = (u² (x² + 44800x*z² + 2257920000*z⁴)/(x*z²+44800*z⁴), u³ y/z³ (x² + 2*44800 x*z² + t0*z⁴) / (x+44800*z²)²)
                      = (u² (x² + 44800x*z² + 2257920000*z⁴)*(x+44800*z²), u³ y (x² + 2*44800 x*z² + t0*z⁴) *(x+44800*z²), z*(x+44800*z²))
                      = (u² pol21 * pol1, u³ y pol22 * pol1, z * pol1)
        */
        let x = base.x;
        let y = base.y;
        let z = base.z;
        let x2 = x * x;
        let z2 = z * z;
        let z3 = z2 * z;
        let z4 = z3 * z;

        let u2 = Self::COEFFS_ENDOMORPHISM[2];
        let u3 = Self::COEFFS_ENDOMORPHISM[3];
        let t0 = Self::COEFFS_ENDOMORPHISM[4];

        // 44800xz²
        let tmp1 = x * z2 * Self::COEFFS_ENDOMORPHISM[0];
        // x²
        let tmp2 = x2;
        // (x² + 44800xz²)
        let tmp3 = tmp2 + tmp1;
        // (x² + 2 * 44800xz²)
        let tmp4 = tmp3 + tmp1;

        // num_x = u² (x² + 44800xz² + t0z⁴)
        let num_x = u2 * (tmp3 + Self::COEFFS_ENDOMORPHISM[1] * z4);
        // num_y = u³ y * (x² + 2 * 44800xz² + t0 z⁴)
        let num_y = u3 * y * (tmp4 + t0 * z4);
        let den = x + Self::COEFFS_ENDOMORPHISM[0] * z2;

        SWProjective::new(num_x * den, num_y * den, den * z)
    }
}

#[cfg(test)]
mod test {

    use std::time::Instant;

    use super::*;
    use ark_std::{test_rng, UniformRand};

    #[test]
    fn test_bench_glv() {
        // TODO THIS IS STILL SLOW. Look at the scalar decomposition?
        println!("STILL SLOW HERE");

        let mut rng = test_rng();
        let p = SWProjective::rand(&mut rng);
        println!("{:?}", p);
        let s = Fr::rand(&mut rng);
        // test
        let q = p * s;
        let r = BandersnatchConfig::glv_mul(p, s);
        assert_eq!(q, r);
        // bench
        let now = Instant::now();
        for _ in 1..100 {
            let _ = p * s;
        }
        println!("SM: {:?}", now.elapsed());
        let now = Instant::now();
        for _ in 1..100 {
            let _ = BandersnatchConfig::glv_mul(p, s);
        }
        println!("GLV: {:?}", now.elapsed());
    }
}
