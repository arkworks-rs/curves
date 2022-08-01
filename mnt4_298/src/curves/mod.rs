use ark_ec::models::mnt4::{MNT4Parameters, MNT4};
use ark_ff::{biginteger::BigInteger320, BigInt, Field, MontFp};

use crate::{Fq, Fq2, Fq2Config, Fq4Config, Fr};

pub mod g1;
pub mod g2;

#[cfg(test)]
mod tests;

pub use self::{
    g1::{G1Affine, G1Prepared, G1Projective},
    g2::{G2Affine, G2Prepared, G2Projective},
};

pub type MNT4_298 = MNT4<Parameters>;

pub struct Parameters;

impl MNT4Parameters for Parameters {
    const TWIST: Fq2 = Fq2::new(Fq::ZERO, Fq::ONE);
    // A coefficient of MNT4-298 G2 =
    // ```
    // mnt4298_twist_coeff_a = mnt4298_Fq2(mnt4298_G1::coeff_a * non_residue, mnt6298_Fq::zero());
    //  = (A_COEFF * NONRESIDUE, ZERO)
    //  = (34, ZERO)
    // ```
    const TWIST_COEFF_A: Fq2 = Fq2::new(G1_COEFF_A_NON_RESIDUE, Fq::ZERO);

    const ATE_LOOP_COUNT: &'static [u64] = &[993502997770534912, 5071219579242586943, 2027349];
    const ATE_IS_LOOP_COUNT_NEG: bool = false;
    const FINAL_EXPONENT_LAST_CHUNK_1: BigInteger320 = BigInt::new([0x1, 0x0, 0x0, 0x0, 0x0]);
    const FINAL_EXPONENT_LAST_CHUNK_W0_IS_NEG: bool = false;
    const FINAL_EXPONENT_LAST_CHUNK_ABS_OF_W0: BigInteger320 =
        BigInt::new([993502997770534913, 5071219579242586943, 2027349, 0, 0]);
    type Fp = Fq;
    type Fr = Fr;
    type Fp2Config = Fq2Config;
    type Fp4Config = Fq4Config;
    type G1Parameters = self::g1::Parameters;
    type G2Parameters = self::g2::Parameters;
}

// 34
pub const G1_COEFF_A_NON_RESIDUE: Fq = MontFp!("34");
