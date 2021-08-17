use crate::*;
use ark_r1cs_std::groups::curves::short_weierstrass::ProjectiveVar;

/// A group element in the Pallas prime-order group.
pub type GVar = ProjectiveVar<PallasParameters>;

#[test]
fn test() {
    ark_curve_constraint_tests::curves::sw_test::<PallasParameters>().unwrap();
}
