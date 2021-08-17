use crate::*;
use ark_r1cs_std::groups::curves::twisted_edwards::AffineVar;

/// A variable that is the R1CS equivalent of `crate::EdwardsAffine`.
pub type EdwardsVar = AffineVar<EdwardsParameters>;

#[test]
fn test() {
    ark_curve_constraint_tests::curves::te_test::<EdwardsParameters>().unwrap();
}
