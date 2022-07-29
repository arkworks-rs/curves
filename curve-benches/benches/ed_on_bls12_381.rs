use ark_curve_benches::*;
use ark_std::ops::{AddAssign, MulAssign, SubAssign};

use ark_ec::ProjectiveCurve;
use ark_ed_on_bls12_381::{fq::Fq, fr::Fr, EdwardsAffine as GAffine, EdwardsProjective as G};
use ark_ff::{biginteger::BigInteger256 as Repr, BigInteger, Field, PrimeField, UniformRand};

mod g {
    use super::*;
    ec_bench!(G, GAffine);
}

f_bench!(Fq, Fq, Repr, Repr, fq);
f_bench!(Fr, Fr, Repr, Repr, fr);

bencher::benchmark_main!(fq, fr, g::group_ops);
