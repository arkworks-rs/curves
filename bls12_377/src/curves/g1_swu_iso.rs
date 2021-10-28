use ark_ec::models::{ModelParameters, SWModelParameters};
use ark_ec::hashing::curve_maps::swu::{SWUParams};

use ark_ff::{field_new};

use crate::{
    Fq, Fr,
};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

//sage: iso_G1
//Isogeny of degree 2 from Elliptic Curve defined by y^2 = x^3 + 258664426012969092796408009721202742408018065645352501567204841856062976176281513834280849065051431927238430294002*x + 22 over Finite Field of size 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177 to Elliptic Curve defined by y^2 = x^3 + 1 over Finite Field of size 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177

impl SWModelParameters for Parameters {
    /// COEFF_A 
    const COEFF_A: Fq = field_new!(Fq, "258664426012969092796408009721202742408018065645352501567204841856062976176281513834280849065051431927238430294002");

    /// COEFF_B
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "22");

    /// COFACTOR = (x - 1)^2 / 3  = iso_G1.domain().order() / 8444461749428370424248824938781546531375899335154063827935233455917409239041
    //  30631250834960419227450344600217059328
    const COFACTOR: &'static [u64] = &[0x0, 0x170b5d4430000000];


    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// = 5285428838741532253824584287042945485047145357130994810877
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "5285428838741532253824584287042945485047145357130994810877");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G1_GENERATOR_X, G1_GENERATOR_Y);
}

// sage: G1_gen  = iso_G1.domain().random_point()
// sage: G1_gen = 30631250834960419227450344600217059328* G1_gen
// sage: G1_gen.order() == 8444461749428370424248824938781546531375899335154063827935233455917409239041
// True
// sage: G1_gen
// (183898640136580512316530045470998831691790391453237259434516336279447756609241220664846162561503820562316877867830 : 69018534046895515891776145953191511526693172354818719412306559690461416836925400134233128432719372819569406562974 : 1)
/// G1_GENERATOR_X =
/// 183898640136580512316530045470998831691790391453237259434516336279447756609241220664846162561503820562316877867830
#[rustfmt::skip]
pub const G1_GENERATOR_X: Fq = field_new!(Fq, "183898640136580512316530045470998831691790391453237259434516336279447756609241220664846162561503820562316877867830");

/// G1_GENERATOR_Y =
/// 69018534046895515891776145953191511526693172354818719412306559690461416836925400134233128432719372819569406562974
#[rustfmt::skip]
pub const G1_GENERATOR_Y: Fq = field_new!(Fq, "183898640136580512316530045470998831691790391453237259434516336279447756609241220664846162561503820562316877867830");

impl SWUParams for Parameters {

    const XI : Fq = field_new!(Fq, "5"); //a nonsquare in Fq
    const ZETA: Fq = field_new!(Fq, "15"); //arbitatry primitive root of unity (element)
    const XI_ON_ZETA_SQRT: Fq = field_new!(Fq, "10189023633222963290707194929886294091415157242906428298294512798502806398782149227503530278436336312243746741931"); ////square root of THETA=Xi/Zeta

}
