use ark_ff::{
    fields::fp3::{Fp3, Fp3Config},
    MontFp,
};

use crate::{fields::{FQ_ONE, FQ_ZERO}, Fq};

pub type Fq3 = Fp3<Fq3Config>;

pub struct Fq3Config;

impl Fp3Config for Fq3Config {
    type Fp = Fq;

    /// NONRESIDUE = -4
    // Fq3 = Fq\[u\]/u^3+4
    const NONRESIDUE: Fq = MontFp!("-4");

    // (MODULUS^3 - 1) % 2^TWO_ADICITY == 0
    const TWO_ADICITY: u32 = 1;

    // (T-1)/2 with T = (MODULUS^3-1) / 2^TWO_ADICITY
    #[rustfmt::skip]
    const TRACE_MINUS_ONE_DIV_TWO: &'static [u64] = &[
        0xb5e7c000000a3eac,
        0xf79b99dbf41cf4ab,
        0xe9372b1919e55ee5,
        0xbb7bbc4936c1980b,
        0x7c0cb9d4399b36e1,
        0x73304a5507bb1ae0,
        0x92f639be8963936f,
        0x4f574ac2439ba816,
        0x670d9bd389dd29ef,
        0x606ddf900d2124f1,
        0x928fb14985ec3270,
        0x6b2f2428c5f420f3,
        0xac9ade29d5ab5fbe,
        0xec0d0434c4005822,
        0x973f10d7f3c5c108,
        0x6d5e83fc81095979,
        0xdac3e6e4e1647752,
        0x227febf93994603e,
        0x4ab8755d894167d1,
        0x4fd2d3f67d8b537a,
        0x33e196a4d5f4030a,
        0x88b51fb72092df1a,
        0xa67e5b1e8fc48316,
        0xb0855eb2a00d7dab,
        0xe875dd2da6751442,
        0x777594a243e25676,
        0x294e0f70376a85a8,
        0x83f431c7988e4f18,
        0x8e8fb6af3ca2f5f1,
        0x7297896b4b9e90f1,
        0xff38f54664d66123,
        0xb5ecf80bfff41e13,
        0x1662a3666bb8392a,
        0x07a0968e8742d3e1,
        0xf12927e564bcdfdc,
        0x5de9825a0e,
    ];

    // NONRESIDUE^T % q
    const QUADRATIC_NONRESIDUE_TO_T: Fq3 = Fq3::new(
        MontFp!("6891450384315732539396789682275657542479668912536150109513790160209623422243491736087683183289411687640864567753786613451161759120554247759349511699125301598951605099378508850372543631423596795951899700429969112842764913119068298"),
        FQ_ZERO,
        FQ_ZERO,
    );

    // NQR ^ (MODULUS^i - 1)/3, i=0,1,2 with NQR = u = (0,1,0)
    const FROBENIUS_COEFF_FP3_C1: &'static [Fq] = &[
        FQ_ONE,
        MontFp!("4922464560225523242118178942575080391082002530232324381063048548642823052024664478336818169867474395270858391911405337707247735739826664939444490469542109391530482826728203582549674992333383150446779312029624171857054392282775648"),
        MontFp!("1968985824090209297278610739700577151397666382303825728450741611566800370218827257750865013421937292370006175842381275743914023380727582819905021229583192207421122272650305267822868639090213645505120388400344940985710520836292650"),
    ];

    // NQR ^ (2*MODULUS^i - 2)/3, i=0,1,2 with NQR = u = (0,1,0)
    const FROBENIUS_COEFF_FP3_C2: &'static [Fq] = &[
        FQ_ONE,
        MontFp!("1968985824090209297278610739700577151397666382303825728450741611566800370218827257750865013421937292370006175842381275743914023380727582819905021229583192207421122272650305267822868639090213645505120388400344940985710520836292650"),
        MontFp!("4922464560225523242118178942575080391082002530232324381063048548642823052024664478336818169867474395270858391911405337707247735739826664939444490469542109391530482826728203582549674992333383150446779312029624171857054392282775648"),
    ];

    #[inline(always)]
    fn mul_fp_by_nonresidue(fe: &Self::Fp) -> Self::Fp {
        let original = -(*fe);
        let double = original + &original;
        double + &double
    }
}
