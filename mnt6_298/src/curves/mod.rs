use ark_ec::{
    models::mnt6::{MNT6Parameters, MNT6},
    short_weierstrass::SWCurveConfig,
};
use ark_ff::{biginteger::BigInteger320, BigInt, Field, Fp3};

use crate::{Fq, Fq3Config, Fq6Config, Fr};

pub mod g1;
pub mod g2;

#[cfg(test)]
mod tests;

pub use self::{
    g1::{G1Affine, G1Prepared, G1Projective},
    g2::{G2Affine, G2Prepared, G2Projective},
};

pub type MNT6_298 = MNT6<Parameters>;

pub struct Parameters;

impl MNT6Parameters for Parameters {
    const TWIST: Fp3<Self::Fp3Config> = Fp3::<Self::Fp3Config>::new(Fq::ZERO, Fq::ONE, Fq::ZERO);
    const TWIST_COEFF_A: Fp3<Self::Fp3Config> =
        Fp3::<Self::Fp3Config>::new(Fq::ZERO, Fq::ZERO, g1::Parameters::COEFF_A);
    const ATE_LOOP_COUNT: &'static [u64] = &[0xdc9a1b671660000, 0x46609756bec2a33f, 0x1eef55];
    const ATE_IS_LOOP_COUNT_NEG: bool = true;
    const FINAL_EXPONENT_LAST_CHUNK_1: BigInteger320 = BigInt::new([0x1, 0x0, 0x0, 0x0, 0x0]);
    const FINAL_EXPONENT_LAST_CHUNK_W0_IS_NEG: bool = true;
    const FINAL_EXPONENT_LAST_CHUNK_ABS_OF_W0: BigInteger320 =
        BigInt::new([0xdc9a1b671660000, 0x46609756bec2a33f, 0x1eef55, 0x0, 0x0]);
    type Fp = Fq;
    type Fr = Fr;
    type Fp3Config = Fq3Config;
    type Fp6Config = Fq6Config;
    type G1Parameters = self::g1::Parameters;
    type G2Parameters = self::g2::Parameters;
}
