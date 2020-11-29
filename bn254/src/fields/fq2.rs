use super::*;
use ark_ff::{field_new, fields::*};

pub type Fq2 = Fp2<Fq2Parameters>;

pub struct Fq2Parameters;

impl Fp2Parameters for Fq2Parameters {
    type Fp = Fq;

    /// NONRESIDUE = -1
    #[rustfmt::skip]
    const NONRESIDUE: Fq = field_new!(Fq, "-1");

    /// QUADRATIC_NONRESIDUE = U+2
    #[rustfmt::skip]
    const QUADRATIC_NONRESIDUE: (Fq, Fq) = (
        field_new!(Fq, "2"),
        field_new!(Fq, "1"),
    );

    /// Coefficients for the Frobenius automorphism.
    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP2_C1: &'static [Fq] = &[
        // NONRESIDUE**(((q^0) - 1) / 2)
        field_new!(Fq, "1"),
        // NONRESIDUE**(((q^1) - 1) / 2)
        field_new!(Fq, "-1"),
    ];

    #[inline(always)]
    fn mul_fp_by_nonresidue(fe: &Self::Fp) -> Self::Fp {
        -(*fe)
    }
}

pub const FQ2_ZERO: Fq2 = field_new!(Fq2, FQ_ZERO, FQ_ZERO);
pub const FQ2_ONE: Fq2 = field_new!(Fq2, FQ_ONE, FQ_ZERO);
