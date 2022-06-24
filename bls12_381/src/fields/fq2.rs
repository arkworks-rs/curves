use ark_ff::{fields::*, MontFp, QuadExt};

use crate::*;

pub type Fq2 = Fp2<Fq2Config>;

pub struct Fq2Config;

impl Fp2Config for Fq2Config {
    type Fp = Fq;

    /// NONRESIDUE = -1
    const NONRESIDUE: Fq = MontFp!(Fq, "-1");

    /// Coefficients for the Frobenius automorphism.
    const FROBENIUS_COEFF_FP2_C1: &'static [Fq] = &[
        // Fq(-1)**(((q^0) - 1) / 2)
        MontFp!(Fq, "1"),
        // Fq(-1)**(((q^1) - 1) / 2)
        MontFp!(Fq, "-1"),
    ];

    #[inline(always)]
    fn mul_fp_by_nonresidue(fp: &Self::Fp) -> Self::Fp {
        -(*fp)
    }
}

pub const FQ2_ZERO: Fq2 = QuadExt!(FQ_ZERO, FQ_ZERO);
pub const FQ2_ONE: Fq2 = QuadExt!(FQ_ONE, FQ_ZERO);
