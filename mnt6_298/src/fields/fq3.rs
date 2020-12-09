use crate::{fq::Fq, FQ_ZERO};
use ark_ff::{
    field_new,
    fields::fp3::{Fp3, Fp3Parameters},
};

pub type Fq3 = Fp3<Fq3Parameters>;

pub struct Fq3Parameters;

impl Fp3Parameters for Fq3Parameters {
    type Fp = Fq;

    #[rustfmt::skip]
    const NONRESIDUE: Fq = field_new!(Fq, "5");

    const TWO_ADICITY: u32 = 34;

    #[rustfmt::skip]
    const T_MINUS_ONE_DIV_TWO: &'static [u64] = &[
        0x69232b75663933bd,
        0xca650efcfc00ee0,
        0x77ca3963fe36f720,
        0xe4cb46632f9bcf7e,
        0xef510453f08f9f30,
        0x9dd5b8fc72f02d83,
        0x7f8d017ed86608ab,
        0xeb2219b3697c97a4,
        0xc8663846ab96996f,
        0x833cd532053eac7d,
        0x1d5b73dfb20bd3cc,
        0x6f5f6da606b59873,
        0x62e990f43dfc42d6,
        0x6878f58,
    ];

    #[rustfmt::skip]
    const QUADRATIC_NONRESIDUE_TO_T: (Fq, Fq, Fq) = (
        field_new!(Fq, "154361449678783505076984156275977937654331103361174469632346230549735979552469642799720052"),
        FQ_ZERO,
        FQ_ZERO,
    );

    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP3_C1: &'static [Fq] = &[
        field_new!(Fq, "1"),
        field_new!(Fq, "471738898967521029133040851318449165997304108729558973770077319830005517129946578866686956"),
        field_new!(Fq, "4183387201740296620308398334599285547820769823264541783190415909159130177461911693276180"),
    ];

    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP3_C2: &'static [Fq] = &[
        Self::FROBENIUS_COEFF_FP3_C1[0],
        Self::FROBENIUS_COEFF_FP3_C1[2],
        Self::FROBENIUS_COEFF_FP3_C1[1],
    ];
}
