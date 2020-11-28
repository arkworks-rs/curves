use super::*;
use ark_ff::{field_new, fields::*};

pub type Fq6 = Fp6<Fq6Parameters>;

#[derive(Clone, Copy)]
pub struct Fq6Parameters;

impl Fp6Parameters for Fq6Parameters {
    type Fp2Params = Fq2Parameters;

    /// NONRESIDUE = U
    #[rustfmt::skip]
    const NONRESIDUE: Fq2 = field_new!(Fq2, FQ_ZERO, FQ_ONE);

    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP6_C1: &'static [Fq2] = &[
        // Fp2::NONRESIDUE^(((q^0) - 1) / 3)
        field_new!(Fq2, FQ_ONE, FQ_ZERO),
        // Fp2::NONRESIDUE^(((q^1) - 1) / 3)
        field_new!(Fq2,
            field_new!(Fq, "80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410946"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^2) - 1) / 3)
        field_new!(Fq2,
            field_new!(Fq, "80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410945"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^3) - 1) / 3)
        field_new!(Fq2, field_new!(Fq, "-1"), FQ_ZERO),
        // Fp2::NONRESIDUE^(((q^4) - 1) / 3)
        field_new!(Fq2,
            field_new!(Fq, "258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^5) - 1) / 3)
        field_new!(Fq2,
            field_new!(Fq, "258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047232"),
            FQ_ZERO,
        ),
    ];
    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP6_C2: &'static [Fq2] = &[
        // Fp2::NONRESIDUE^((2*(q^0) - 2) / 3)
        field_new!(Fq2, FQ_ONE, FQ_ZERO),
        // Fp2::NONRESIDUE^((2*(q^1) - 2) / 3)
        field_new!(Fq2,
            field_new!(Fq, "80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410945"),
            FQ_ZERO
        ),
        // Fp2::NONRESIDUE^((2*(q^2) - 2) / 3)
        field_new!(Fq2,
            field_new!(Fq, "258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^((2*(q^3) - 2) / 3)
        field_new!(Fq2, FQ_ONE, FQ_ZERO),
        // Fp2::NONRESIDUE^((2*(q^4) - 2) / 3)
        field_new!(Fq2,
            field_new!(Fq, "80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410945"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^((2*(q^5) - 2) / 3)
        field_new!(Fq2,
            field_new!(Fq, "258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231"),
            FQ_ZERO,
        ),
    ];

    #[inline(always)]
    fn mul_fp2_by_nonresidue(fe: &Fq2) -> Fq2 {
        // Karatsuba multiplication with constant other = u.
        let c0 = Fq2Parameters::mul_fp_by_nonresidue(&fe.c1);
        let c1 = fe.c0;
        field_new!(Fq2, c0, c1)
    }
}
