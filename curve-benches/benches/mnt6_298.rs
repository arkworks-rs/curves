use ark_curve_benches::*;
use ark_std::ops::{AddAssign, MulAssign, SubAssign};

use ark_ec::{PairingEngine, ProjectiveCurve};
use ark_ff::{biginteger::BigInteger320 as FqRepr, BigInteger, Field, PrimeField, UniformRand};
use ark_mnt6_298::{
    fq::Fq, fq3::Fq3, fr::Fr, Fq6, G1Affine, G1Projective as G1, G2Affine, G2Projective as G2,
    MNT6_298,
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
f_bench!(Fr, Fr, FqRepr, FqRepr, fr);
pairing_bench!(MNT6_298, Fq6);

bencher::benchmark_main!(fq, fr, fq3, fq6, g1::group_ops, g2::group_ops, pairing);
