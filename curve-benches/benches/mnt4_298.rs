use ark_curve_benches::*;
use ark_std::ops::{AddAssign, MulAssign, SubAssign};

use ark_ec::{PairingEngine, ProjectiveCurve};
use ark_ff::{biginteger::BigInteger320 as FqRepr, BigInteger, Field, PrimeField, UniformRand};
use ark_mnt4_298::{
    fq::Fq, fq2::Fq2, fr::Fr, Fq4, G1Affine, G1Projective as G1, G2Affine, G2Projective as G2,
    MNT4_298,
};

mod g1 {
    use super::*;
    ec_bench!(G1, G1Affine);
}
mod g2 {
    use super::*;
    ec_bench!(G2, G2Affine);
}

f_bench!(extension, Fq2, Fq2, fq2);
f_bench!(target, Fq4, Fq4, fq4);
f_bench!(Fq, Fq, FqRepr, FqRepr, fq);
f_bench!(Fr, Fr, FqRepr, FqRepr, fr);
pairing_bench!(MNT4_298, Fq4);

bencher::benchmark_main!(fq, fr, fq2, fq4, g1::group_ops, g2::group_ops, pairing);
