use super::*;
use ark_ff::{field_new, fields::*};

pub type Fq2 = Fp2<Fq2Parameters>;

pub struct Fq2Parameters;

impl Fp2Parameters for Fq2Parameters {
    type Fp = Fq;

    /// NONRESIDUE = -5
    #[rustfmt::skip]
    const NONRESIDUE: Fq = field_new!(Fq, "-5");

    /// QUADRATIC_NONRESIDUE = U
    #[rustfmt::skip]
    const QUADRATIC_NONRESIDUE: (Fq, Fq) = (FQ_ZERO, FQ_ONE);

    /// Coefficients for the Frobenius automorphism.
    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP2_C1: &'static [Fq] = &[
        // NONRESIDUE**(((q^0) - 1) / 2)
        FQ_ONE,
        // NONRESIDUE**(((q^1) - 1) / 2)
        field_new!(Fq, "-1"),
    ];

    // Mul by -5
    #[inline(always)]
    fn mul_fp_by_nonresidue(fe: &Self::Fp) -> Self::Fp {
        let original = fe;
        let mut fe = -fe.double();
        fe.double_in_place();
        fe - original
    }

    // x + -5 * y, computed as x - 5*y
    #[inline(always)]
    fn add_and_mul_fp_by_nonresidue(x: &Self::Fp, y: &Self::Fp) -> Self::Fp {
        // c becomes 5 * y
        let mut c = y.double();
        c.double_in_place();
        c += y;
        *x - c
    }

    // x + y + (-5 * y), computed as x - 4*y
    #[inline(always)]
    fn add_and_mul_fp_by_nonresidue_plus_one(x: &Self::Fp, y: &Self::Fp) -> Self::Fp {
        // c becomes 4 * y
        let mut c = y.double();
        c.double_in_place();
        *x - c
    }

    // x - (-5 * y), computed as x + 5*y
    #[inline(always)]
    fn sub_and_mul_fp_by_nonresidue(x: &Self::Fp, y: &Self::Fp) -> Self::Fp {
        // c becomes 5 * y
        let mut c = y.double();
        c.double_in_place();
        c += y;
        *x + c
    }
}

pub const FQ2_ZERO: Fq2 = field_new!(Fq2, FQ_ZERO, FQ_ZERO);
pub const FQ2_ONE: Fq2 = field_new!(Fq2, FQ_ONE, FQ_ZERO);
