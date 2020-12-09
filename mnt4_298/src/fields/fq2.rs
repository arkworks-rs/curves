use crate::{Fq, FQ_ONE};
use ark_ff::{
    field_new,
    fields::fp2::{Fp2, Fp2Parameters},
};

pub type Fq2 = Fp2<Fq2Parameters>;

pub struct Fq2Parameters;

impl Fp2Parameters for Fq2Parameters {
    type Fp = Fq;

    /// The quadratic non-residue (17) used to construct the extension is
    /// the same as that used in [`libff`](https://github.com/scipr-lab/libff/blob/c927821ebe02e0a24b5e0f9170cec5e211a35f08/libff/algebra/curves/mnt/mnt4/mnt4_init.cpp#L102).
    #[rustfmt::skip]
    const NONRESIDUE: Fq = field_new!(Fq, "17");

    /// The quadratic non-residue in F<sub>p</sub><sup>2</sup> that is used
    /// in the computation of square roots is (8, 1), the same as that in
    /// [`libff`](https://github.com/scipr-lab/libff/blob/c927821ebe02e0a24b5e0f9170cec5e211a35f08/libff/algebra/curves/mnt/mnt4/mnt4_init.cpp#L103)
    const QUADRATIC_NONRESIDUE: (Self::Fp, Self::Fp) = (field_new!(Fq, "8"), FQ_ONE);

    /// Precomputed coefficients:
    /// `[1, 475922286169261325753349249653048451545124879242694725395555128576210262817955800483758080]`
    const FROBENIUS_COEFF_FP2_C1: &'static [Self::Fp] = &[
        FQ_ONE,
        field_new!(Fq, "475922286169261325753349249653048451545124879242694725395555128576210262817955800483758080"),
    ];
}
