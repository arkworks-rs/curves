use ark_curve_benches::*;
use ark_std::ops::{AddAssign, MulAssign, SubAssign};

use ark_bn254::{
    fq::Fq, fq2::Fq2, fr::Fr, Bn254, Fq12, G1Affine, G1Projective as G1, G2Affine,
    G2Projective as G2,
};
use ark_ec::{PairingEngine, ProjectiveCurve};
use ark_ff::{
    biginteger::BigInteger256 as Repr, BigInteger, Field, PrimeField, SquareRootField, UniformRand,
};

mod g1 {
    use super::*;
    use ark_bn254::g1::Parameters;
    ec_bench!(G1, G1Affine);
    glv_bench!(G1Affine, Parameters);
}
mod g2 {
    use super::*;
    use ark_bn254::g2::Parameters;
    ec_bench!(G2, G2Affine);
    glv_bench!(G2Affine, Parameters);
}

f_bench!(Fq, Fq, Repr, Repr, fq);
f_bench!(Fr, Fr, Repr, Repr, fr);
f_bench!(extension, Fq2, Fq2, fq2);
f_bench!(target, Fq12, Fq12, fq12);

pairing_bench!(Bn254, Fq12);

bencher::benchmark_main!(
    fq,
    fr,
    fq2,
    fq12,
    g1::group_ops,
    g1::group_glv_ops,
    g2::group_ops,
    g2::group_glv_ops,
    pairing
);
