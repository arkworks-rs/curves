use ark_curve_benches::*;
use ark_std::ops::{AddAssign, MulAssign, SubAssign};

use ark_ec::ProjectiveCurve;
use ark_ff::{
    biginteger::{BigInteger256 as FrRepr, BigInteger256 as FqRepr},
    BigInteger, Field, PrimeField, SquareRootField, UniformRand,
};
use ark_vesta::{fq::Fq, fr::Fr, Affine as GAffine, Projective as G};

mod g {
    use super::*;
    ec_bench!(G, GAffine);
}

f_bench!(Fq, Fq, FqRepr, FqRepr, fq);
f_bench!(Fr, Fr, FrRepr, FrRepr, fr);

bencher::benchmark_main!(fq, fr, g::group_ops);
