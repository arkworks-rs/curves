use crate::*;
use ark_r1cs_std::groups::curves::short_weierstrass::ProjectiveVar;

use crate::constraints::FBaseVar;

/// A group element in the Pluto prime-order group.
pub type GVar = ProjectiveVar<PlutoParameters, FBaseVar>;

#[test]
fn test() {
    ark_curve_constraint_tests::curves::sw_test::<PlutoParameters, GVar>().unwrap();
}
