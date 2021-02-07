use ark_curve_benches::*;
use ark_std::ops::{AddAssign, MulAssign, SubAssign};

use ark_bn254::{
    fq::Fq, fq2::Fq2, fr::Fr, Bn254, Fq12, G1Affine, G1Projective as G1, G2Affine,
    G2Projective as G2,
};
use ark_ec::{PairingEngine, ProjectiveCurve};
use ark_ff::{
    biginteger::{BigInteger256 as FrRepr, BigInteger256 as FqRepr},
    BigInteger, Field, PrimeField, SquareRootField, UniformRand,
};

mod g1 {
    use super::*;
    ec_bench!(G1, G1Affine);
}
mod g2 {
    use super::*;
    ec_bench!(G2, G2Affine);
}

f_bench!(Fq, Fq, FqRepr, FqRepr, fq);
f_bench!(Fr, Fr, FrRepr, FrRepr, fr);
f_bench!(extension, Fq2, Fq2, fq2);
f_bench!(target, Fq12, Fq12, fq12);

pairing_bench!(Bn254, Fq12);

bencher::benchmark_main!(fq, fr, fq2, fq12, g1::group_ops, g2::group_ops, pairing);
