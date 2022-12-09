use crate::{constraints::FqVar, *};
use ark_r1cs_std::groups::curves::short_weierstrass::ProjectiveVar;

/// A group element in the secq256k1 curve.
pub type GVar = ProjectiveVar<Parameters, FqVar>;

#[test]
fn test() {
    ark_curve_constraint_tests::curves::sw_test::<Parameters, GVar>().unwrap();
}
