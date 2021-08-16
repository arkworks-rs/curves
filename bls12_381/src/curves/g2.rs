use crate::*;
use ark_ec::{
    AffineCurve,
    bls12,
    models::{ModelParameters, SWModelParameters},
    short_weierstrass_jacobian::GroupAffine,
};
use ark_ff::{field_new, Zero, Field};

pub type G2Affine = bls12::G2Affine<crate::Parameters>;
pub type G2Projective = bls12::G2Projective<crate::Parameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq2;
    type ScalarField = Fr;
}

impl SWModelParameters for Parameters {
    /// COEFF_A = [0, 0]
    const COEFF_A: Fq2 = field_new!(Fq2, g1::Parameters::COEFF_A, g1::Parameters::COEFF_A,);

    /// COEFF_B = [4, 4]
    const COEFF_B: Fq2 = field_new!(Fq2, g1::Parameters::COEFF_B, g1::Parameters::COEFF_B,);

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
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "26652489039290660355457965112010883481355318854675681319708643586776743290055");

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G2_GENERATOR_X, G2_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }

    fn is_in_correct_subgroup_assuming_on_curve_fast(
	p : &GroupAffine<Self>
    ) -> Option<bool> {
	let psi2_p = psi2(p);
	let psi3_p = psi3(p);
        let mul_psi3_p: GroupAffine<_> = (-psi3_p).mul(MULTIPLIER_G2).into();
	Some((mul_psi3_p + (-psi2_p) + *p).is_zero())
    }

}

pub const G2_GENERATOR_X: Fq2 = field_new!(Fq2, G2_GENERATOR_X_C0, G2_GENERATOR_X_C1);
pub const G2_GENERATOR_Y: Fq2 = field_new!(Fq2, G2_GENERATOR_Y_C0, G2_GENERATOR_Y_C1);

/// G2_GENERATOR_X_C0 =
/// 352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160
#[rustfmt::skip]
pub const G2_GENERATOR_X_C0: Fq = field_new!(Fq, "352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160");

/// G2_GENERATOR_X_C1 =
/// 3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758
#[rustfmt::skip]
pub const G2_GENERATOR_X_C1: Fq = field_new!(Fq, "3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758");

/// G2_GENERATOR_Y_C0 =
/// 1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905
#[rustfmt::skip]
pub const G2_GENERATOR_Y_C0: Fq = field_new!(Fq, "1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905");

/// G2_GENERATOR_Y_C1 =
/// 927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582
#[rustfmt::skip]
pub const G2_GENERATOR_Y_C1: Fq = field_new!(Fq, "927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582");


// psi(x,y) = (x^p * PSI_X, y^p * PSI_Y) is the Frobenius composed
// with the quadratic twist and its inverse
pub const PSI_X:Fq2 = field_new!(
    Fq2,
    FQ_ZERO,
    field_new!(
	Fq,
	"4002409555221667392624310435006688643935503118305586438271171395842971157480381377015405980053539358417135540939437"
    )
);
pub const PSI_Y: Fq2 = field_new!(
    Fq2,
    field_new!(
	Fq,
	"2973677408986561043442465346520108879172042883009249989176415018091420807192182638567116318576472649347015917690530"),
    field_new!(
	Fq,
	"1028732146235106349975324479215795277384839936929757896155643118032610843298655225875571310552543014690878354869257")
);
pub fn psi(p: &GroupAffine<Parameters>) -> GroupAffine<Parameters> {
    let mut psi_p = *p;
    psi_p.x.frobenius_map(1);
    psi_p.y.frobenius_map(1);
    // psi_p.x *= PSI_X but PSI_X.c0 = 0
    let tmp = psi_p.x;
    psi_p.x.c0 = -tmp.c1 * PSI_X.c1;
    psi_p.x.c1 = tmp.c0 * PSI_X.c1;
    psi_p.y *= PSI_Y;
    psi_p
}

// psi2(x,y) = psi²(x,y) = (x*PSI2_X, -y) where PSI2_X = PSI_X^{p+1}
// is in Fq and PSI_Y^{p+1} = -1.
pub const PSI2_X:Fq =
    field_new!(
	Fq,
	"4002409555221667392624310435006688643935503118305586438271171395842971157480381377015405980053539358417135540939436"
    );
pub fn psi2(p: &GroupAffine<Parameters>) -> GroupAffine<Parameters> {
    let mut psi2_p = -*p;
    psi2_p.x.mul_assign_by_basefield(&PSI2_X);
    psi2_p
}

// psi3(x,y) = psi³(x,y) = (x^p*(-u), -y^p * PSI_Y)
// (PSI2_X^p * PSI_X = -u and (-1)^p = -1)
pub fn psi3(p: &GroupAffine<Parameters>) -> GroupAffine<Parameters> {
    let mut psi3_p = p.clone();
    psi3_p.x.frobenius_map(1);
    psi3_p.y.frobenius_map(1);
    // psi3_p.x *= (-u)
    let tmp = psi3_p.x;
    psi3_p.x.c1 = -tmp.c0;
    psi3_p.x.c0 = tmp.c1; // * (-u²) but u² = -1
    psi3_p.y *= PSI_Y;
    -psi3_p
}

pub const MULTIPLIER_G2:Fr = field_new!(Fr, "15132376222941642752");
	
