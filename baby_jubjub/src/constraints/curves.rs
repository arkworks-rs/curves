use ark_r1cs_std::groups::curves::twisted_edwards::{AffineVar, MontgomeryAffineVar};

use crate::{constraints::FqVar, *};

/// A variable that is the R1CS equivalent of `crate::EdwardsAffine`.
pub type EdwardsVar = AffineVar<BabyJubConfig, FqVar>;

/// A variable that is the R1CS equivalent of `crate::NonZeroMontgomeryAffine`.
pub type NonZeroMontgomeryVar = MontgomeryAffineVar<BabyJubConfig, FqVar>;

#[test]
fn test() {
    ark_curve_constraint_tests::curves::te_test::<BabyJubConfig, EdwardsVar>().unwrap();
}
