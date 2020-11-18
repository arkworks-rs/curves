use crate::*;
use ark_r1cs_std::groups::curves::short_weierstrass::ProjectiveVar;

use crate::constraints::FBaseVar;

/// A group element in the Tweedledum prime order group.
pub type GVar = ProjectiveVar<TweedledumParameters, FBaseVar>;

#[test]
fn test() {
    ark_curve_constraint_tests::curves::sw_test::<TweedledumParameters, GVar>().unwrap();
}
