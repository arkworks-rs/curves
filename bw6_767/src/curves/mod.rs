use ark_ec::{
    bw6,
    bw6::{BW6Config, TwistType, BW6},
};
use ark_ff::{biginteger::BigInteger768 as BigInteger, BigInt};

use crate::*;

pub mod g1;
pub mod g2;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Eq)]
pub struct Config;

impl BW6Config for Config {
    // X is the same as in bls12_381
    const X: BigInteger = BigInt::new([
        0xd201000000010000,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
    ]);
    const X_IS_NEGATIVE: bool = true;
    // X+1
    const ATE_LOOP_COUNT_1: &'static [u64] = &[0xd20100000000ffff];
    const ATE_LOOP_COUNT_1_IS_NEGATIVE: bool = true;
    // X^3-X^2-X = -3465144826073652319005258340840392356319973669502814453760
    const ATE_LOOP_COUNT_2: &'static [i8] = &[
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0,
        -1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, -1,
        0, -1, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0,
        0, 0, 0, -1, 0, 1, 0, -1, 0, 1, 0, 0, 1, 0, -1, 0, 1, 0, 1, 0, 0, 0, 0, 0, -1, 0, 1, 0, 0,
        0, 0, 0, 0, -1, 0, -1, 0, 0, 0, 1, 0, -1, 0, 0, 1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0, 0,
        1, 0, 0, 1, 0, 1, 0, 1, 0, -1, 0, 1, 0, 0, 1,
    ];
    const ATE_LOOP_COUNT_2_IS_NEGATIVE: bool = true;
    const TWIST_TYPE: TwistType = TwistType::M;
    type Fp = Fq;
    type Fp3Config = Fq3Config;
    type Fp6Config = Fq6Config;
    type G1Config = g1::Config;
    type G2Config = g2::Config;
}

pub type BW6_767 = BW6<Config>;

pub type G1Affine = bw6::G1Affine<Config>;
pub type G1Projective = bw6::G1Projective<Config>;
pub type G2Affine = bw6::G2Affine<Config>;
pub type G2Projective = bw6::G2Projective<Config>;
