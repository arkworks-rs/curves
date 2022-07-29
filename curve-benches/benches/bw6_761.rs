use ark_curve_benches::*;
use ark_std::ops::{AddAssign, MulAssign, SubAssign};

use ark_bw6_761::{
    fq::Fq, fq3::Fq3, fr::Fr, Fq6, G1Affine, G1Projective as G1, G2Affine, G2Projective as G2,
    BW6_761,
};
use ark_ec::{PairingEngine, ProjectiveCurve};
use ark_ff::{
    biginteger::{BigInteger384 as FrRepr, BigInteger768 as FqRepr},
    BigInteger, Field, PrimeField, UniformRand,
};

mod g1 {
    use super::*;
    ec_bench!(G1, G1Affine);
}
mod g2 {
    use super::*;
    ec_bench!(G2, G2Affine);
}

f_bench!(extension, Fq3, Fq3, fq3);
f_bench!(target, Fq6, Fq6, fq6);
f_bench!(Fq, Fq, FqRepr, FqRepr, fq);
f_bench!(Fr, Fr, FrRepr, FrRepr, fr);
pairing_bench!(BW6_761, Fq6);

bencher::benchmark_main!(fq, fr, fq3, fq6, g1::group_ops, g2::group_ops, pairing);
