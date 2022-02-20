use ark_ff::{
    fields::fp6_2over3::{Fp6, Fp6Config},
    CubicExt, MontFp,
};

use crate::{Fq, Fq3, Fq3Config, FQ_ONE, FQ_ZERO};

pub type Fq6 = Fp6<Fq6Config>;

pub struct Fq6Config;

impl Fp6Config for Fq6Config {
    type Fp3Config = Fq3Config;

    const NONRESIDUE: Fq3 = CubicExt!(FQ_ZERO, FQ_ONE, FQ_ZERO);

    const FROBENIUS_COEFF_FP6_C1: &'static [Fq] = &[
        MontFp!(Fq, "1"),
        MontFp!(Fq, "471738898967521029133040851318449165997304108729558973770077319830005517129946578866686957"),
        MontFp!(Fq, "471738898967521029133040851318449165997304108729558973770077319830005517129946578866686956"),
        MontFp!(Fq, "475922286169261325753349249653048451545124878552823515553267735739164647307408490559963136"),
        MontFp!(Fq, "4183387201740296620308398334599285547820769823264541783190415909159130177461911693276180"),
        MontFp!(Fq, "4183387201740296620308398334599285547820769823264541783190415909159130177461911693276181"),
    ];
}
