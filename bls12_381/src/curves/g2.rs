use core::ops::Neg;

use ark_ec::{
    bls12,
    bls12::Bls12Parameters,
    models::{ModelParameters, SWModelParameters},
    short_weierstrass_jacobian::{GroupAffine, GroupProjective},
    AffineCurve, ProjectiveCurve,
};
use ark_ff::{BigInt, Field, MontFp, QuadExt, Zero};

use crate::*;

pub type G2Affine = bls12::G2Affine<crate::Parameters>;
pub type G2Projective = bls12::G2Projective<crate::Parameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq2;
    type ScalarField = Fr;

    /// COFACTOR = (x^8 - 4 x^7 + 5 x^6) - (4 x^4 + 6 x^3 - 4 x^2 - 4 x + 13) //
    /// 9
    /// = 305502333931268344200999753193121504214466019254188142667664032982267604182971884026507427359259977847832272839041616661285803823378372096355777062779109
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0xcf1c38e31c7238e5,
        0x1616ec6e786f0c70,
        0x21537e293a6691ae,
        0xa628f1cb4d9e82ef,
        0xa68a205b2e5a7ddf,
        0xcd91de4547085aba,
        0x91d50792876a202,
        0x5d543a95414e7f1,
    ];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// 26652489039290660355457965112010883481355318854675681319708643586776743290055
    const COFACTOR_INV: Fr = MontFp!(
        Fr,
        "26652489039290660355457965112010883481355318854675681319708643586776743290055"
    );
}

impl SWModelParameters for Parameters {
    /// COEFF_A = [0, 0]
    const COEFF_A: Fq2 = QuadExt!(g1::Parameters::COEFF_A, g1::Parameters::COEFF_A,);

    /// COEFF_B = [4, 4]
    const COEFF_B: Fq2 = QuadExt!(g1::Parameters::COEFF_B, g1::Parameters::COEFF_B,);

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G2_GENERATOR_X, G2_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }

    fn is_in_correct_subgroup_assuming_on_curve(point: &G2Affine) -> bool {
        // Algorithm from Section 4 of https://eprint.iacr.org/2021/1130.
        //
        // Checks that [p]P = [X]P

        let mut x_times_point = point.mul(BigInt::new([crate::Parameters::X[0], 0, 0, 0]));
        if crate::Parameters::X_IS_NEGATIVE {
            x_times_point = -x_times_point;
        }

        let p_times_point = p_power_endomorphism(point);

        x_times_point.eq(&p_times_point)
    }

    #[inline]
    fn clear_cofactor(p: &G2Affine) -> G2Affine {
        // Based on Section 4.1 of https://eprint.iacr.org/2017/419.pdf
        // [h(ψ)]P = [x^2 − x − 1]P + [x − 1]ψ(P) + (ψ^2)(2P)

        // x = -15132376222941642752
        // When multiplying, use -c1 instead, and then negate the result. That's much
        // more efficient, since the scalar -c1 has less limbs and a much lower Hamming
        // weight.
        let x: &'static [u64] = crate::Parameters::X;
        let p_projective = p.into_projective();

        // [x]P
        let x_p = Parameters::mul_affine(p, &x).neg();
        // ψ(P) 
        let psi_p = p_power_endomorphism(&p);
        // (ψ^2)(2P)
        let mut psi2_p2 = double_p_power_endomorphism(&p_projective.double());

        // tmp = [x]P + ψ(P)
        let mut tmp = x_p.clone();
        tmp.add_assign_mixed(&psi_p);

        // tmp2 = [x^2]P + [x]ψ(P)
        let mut tmp2: GroupProjective<Parameters> = tmp;
        tmp2 = tmp2.mul(x).neg();

        // add up all the terms
        psi2_p2 += tmp2;
        psi2_p2 -= x_p;
        psi2_p2.add_assign_mixed(&-psi_p);
        (psi2_p2 - p_projective).into_affine()
    }
}

pub const G2_GENERATOR_X: Fq2 = QuadExt!(G2_GENERATOR_X_C0, G2_GENERATOR_X_C1);
pub const G2_GENERATOR_Y: Fq2 = QuadExt!(G2_GENERATOR_Y_C0, G2_GENERATOR_Y_C1);

/// G2_GENERATOR_X_C0 =
/// 352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160
pub const G2_GENERATOR_X_C0: Fq = MontFp!(Fq, "352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160");

/// G2_GENERATOR_X_C1 =
/// 3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758
pub const G2_GENERATOR_X_C1: Fq = MontFp!(Fq, "3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758");

/// G2_GENERATOR_Y_C0 =
/// 1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905
pub const G2_GENERATOR_Y_C0: Fq = MontFp!(Fq, "1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905");

/// G2_GENERATOR_Y_C1 =
/// 927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582
pub const G2_GENERATOR_Y_C1: Fq = MontFp!(Fq, "927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582");

// psi(x,y) = (x**p * PSI_X, y**p * PSI_Y) is the Frobenius composed
// with the quadratic twist and its inverse

// PSI_X = 1/(u+1)^((p-1)/3)
pub const P_POWER_ENDOMORPHISM_COEFF_0 : Fq2 = QuadExt!(
    FQ_ZERO,
    MontFp!(
        Fq,
        "4002409555221667392624310435006688643935503118305586438271171395842971157480381377015405980053539358417135540939437"
    )
);

// PSI_Y = 1/(u+1)^((p-1)/2)
pub const P_POWER_ENDOMORPHISM_COEFF_1: Fq2 = QuadExt!(
    MontFp!(
        Fq,
        "2973677408986561043442465346520108879172042883009249989176415018091420807192182638567116318576472649347015917690530"),
    MontFp!(
        Fq,
        "1028732146235106349975324479215795277384839936929757896155643118032610843298655225875571310552543014690878354869257")
);

pub const DOUBLE_P_POWER_ENDOMORPHISM: Fq2 = QuadExt!(
    MontFp!(Fq, "4002409555221667392624310435006688643935503118305586438271171395842971157480381377015405980053539358417135540939436"),
    FQ_ZERO
);

pub fn p_power_endomorphism(p: &GroupAffine<Parameters>) -> GroupAffine<Parameters> {
    // The p-power endomorphism for G2 is defined as follows:
    // 1. Note that G2 is defined on curve E': y^2 = x^3 + 4(u+1).
    //    To map a point (x, y) in E' to (s, t) in E,
    //    one set s = x / ((u+1) ^ (1/3)), t = y / ((u+1) ^ (1/2)),
    //    because E: y^2 = x^3 + 4.
    // 2. Apply the Frobenius endomorphism (s, t) => (s', t'),
    //    another point on curve E, where s' = s^p, t' = t^p.
    // 3. Map the point from E back to E'; that is,
    //    one set x' = s' * ((u+1) ^ (1/3)), y' = t' * ((u+1) ^ (1/2)).
    //
    // To sum up, it maps
    // (x,y) -> (x^p / ((u+1)^((p-1)/3)), y^p / ((u+1)^((p-1)/2)))
    // as implemented in the code as follows.

    let mut res = *p;
    res.x.frobenius_map(1);
    res.y.frobenius_map(1);

    let tmp_x = res.x.clone();
    res.x.c0 = -P_POWER_ENDOMORPHISM_COEFF_0.c1 * &tmp_x.c1;
    res.x.c1 = P_POWER_ENDOMORPHISM_COEFF_0.c1 * &tmp_x.c0;
    res.y *= P_POWER_ENDOMORPHISM_COEFF_1;

    res
}

/// For a p-power endomorphism psi(P), compute psi(psi(P))
pub fn double_p_power_endomorphism(p: &GroupProjective<Parameters>) -> GroupProjective<Parameters> {
    let mut res = *p;

    res.x *= DOUBLE_P_POWER_ENDOMORPHISM;
    res.y = res.y.neg();

    res
}

#[cfg(test)]
mod test {

    use super::*;
    use ark_std::UniformRand;

    #[test]
    fn test_cofactor_clearing() {
        // multiplying by h_eff and clearing the cofactor by the efficient
        // endomorphism-based method should yield the same result.
        let h_eff: &'static [u64] = &[
            0xe8020005aaa95551,
            0x59894c0adebbf6b4,
            0xe954cbc06689f6a3,
            0x2ec0ec69d7477c1a,
            0x6d82bf015d1212b0,
            0x329c2f178731db95,
            0x9986ff031508ffe1,
            0x88e2a8e9145ad768,
            0x584c6a0ea91b3528,
            0xbc69f08f2ee75b3,
        ];

        let mut rng = ark_std::test_rng();
        const SAMPLES: usize = 10;
        for _ in 0..SAMPLES {
            let p = GroupAffine::<g2::Parameters>::rand(&mut rng);
            let optimised = p.clear_cofactor().into_projective();
            let naive = g2::Parameters::mul_affine(&p, h_eff);
            assert_eq!(optimised, naive);
        }
    }
}
