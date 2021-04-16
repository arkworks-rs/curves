use ark_curve_benches::*;
use ark_std::ops::{AddAssign, MulAssign, SubAssign};

use ark_ec::ProjectiveCurve;
use ark_eris::{fq::Fq, fr::Fr, Affine as GAffine, Projective as G};
use ark_ff::{
    biginteger::{BigInteger448 as FrRepr, BigInteger448 as FqRepr},
    BigInteger, Field, PrimeField, SquareRootField, UniformRand,
};

mod g {
    use super::*;
    ec_bench!(G, GAffine);
}

f_bench!(Fq, Fq, FqRepr, FqRepr, fq);
f_bench!(Fr, Fr, FrRepr, FrRepr, fr);

bencher::benchmark_main!(fq, fr, g::group_ops);
