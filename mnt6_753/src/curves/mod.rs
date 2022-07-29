use ark_ec::models::{
    mnt6::{MNT6Parameters, MNT6},
    short_weierstrass::SWCurveConfig,
};
use ark_ff::{biginteger::BigInteger768, BigInt, Field, Fp3};

use crate::{Fq, Fq3Config, Fq6Config, Fr};

pub mod g1;
pub mod g2;

#[cfg(test)]
mod tests;

pub use self::{
    g1::{G1Affine, G1Prepared, G1Projective},
    g2::{G2Affine, G2Prepared, G2Projective},
};

pub type MNT6_753 = MNT6<Parameters>;

pub struct Parameters;

impl MNT6Parameters for Parameters {
    const TWIST: Fp3<Self::Fp3Config> = Fp3::new(Fq::ZERO, Fq::ONE, Fq::ZERO);
    // A coefficient of MNT6-753 G2 =
    // ```
    // mnt6753_twist_coeff_a = mnt6753_Fq3(mnt6753_Fq::zero(), mnt6753_Fq::zero(),
    //                                  mnt6753_G1::coeff_a);
    //  = (ZERO, ZERO, A_COEFF);
    // ```
    const TWIST_COEFF_A: Fp3<Self::Fp3Config> =
        Fp3::new(Fq::ZERO, Fq::ZERO, g1::Parameters::COEFF_A);

    // https://github.com/o1-labs/snarky/blob/9c21ab2bb23874604640740d646a932e813432c3/snarkette/mnt6753.ml
    const ATE_LOOP_COUNT: &'static [u64] = &[
        8824542903220142080,
        7711082599397206192,
        8303354903384568230,
        5874150271971943936,
        9717849827920685054,
        95829799234282493,
    ];
    const ATE_IS_LOOP_COUNT_NEG: bool = false;
    const FINAL_EXPONENT_LAST_CHUNK_1: BigInteger768 =
        BigInt::new([0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]);
    const FINAL_EXPONENT_LAST_CHUNK_W0_IS_NEG: bool = false;
    const FINAL_EXPONENT_LAST_CHUNK_ABS_OF_W0: BigInteger768 = BigInt::new([
        8824542903220142080,
        7711082599397206192,
        8303354903384568230,
        5874150271971943936,
        9717849827920685054,
        95829799234282493,
        0,
        0,
        0,
        0,
        0,
        0,
    ]);
    type Fp = Fq;
    type Fr = Fr;
    type Fp3Config = Fq3Config;
    type Fp6Config = Fq6Config;
    type G1Parameters = self::g1::Parameters;
    type G2Parameters = self::g2::Parameters;
}
