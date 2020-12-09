use crate::{Fq, FQ_ONE};
use ark_ff::{
    field_new,
    fields::fp2::{Fp2, Fp2Parameters},
};

pub type Fq2 = Fp2<Fq2Parameters>;

pub struct Fq2Parameters;

impl Fp2Parameters for Fq2Parameters {
    type Fp = Fq;

    // non_residue = 13
    #[rustfmt::skip]
    const NONRESIDUE: Fq = field_new!(Fq, "13");

    // qnr = (8, 1)
    const QUADRATIC_NONRESIDUE: (Self::Fp, Self::Fp) = (field_new!(Fq, "8"), FQ_ONE);

    // Coefficients:
    // [1, 41898490967918953402344214791240637128170709919953949071783502921025352812571106773058893763790338921418070971888253786114353726529584385201591605722013126468931404347949840543007986327743462853720628051692141265303114721689600]
    // see https://github.com/o1-labs/snarky/blob/2cf5ef3a14989e57c17518832b3c52590068fc48/src/camlsnark_c/libsnark-caml/depends/libff/libff/algebra/curves/mnt753/mnt4753/mnt4753_init.cpp
    const FROBENIUS_COEFF_FP2_C1: &'static [Self::Fp] = &[
        FQ_ONE,
        field_new!(Fq, "41898490967918953402344214791240637128170709919953949071783502921025352812571106773058893763790338921418070971888253786114353726529584385201591605722013126468931404347949840543007986327743462853720628051692141265303114721689600"),
    ];
}
