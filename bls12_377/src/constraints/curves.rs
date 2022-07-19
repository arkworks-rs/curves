use ark_ec::{bls12::Bls12Parameters, CurveConfig};
use ark_r1cs_std::{
    fields::fp::FpVar,
    groups::{bls12, curves::twisted_edwards::AffineVar as TEAffineVar},
};

use crate::Parameters;

/// An element of G1 in the BLS12-377 bilinear group.
pub type G1Var = bls12::G1Var<Parameters>;
/// An element of G2 in the BLS12-377 bilinear group.
pub type G2Var = bls12::G2Var<Parameters>;

/// An element of G1 (in TE Affine form) in the BLS12-377 bilinear group.
pub type G1TEAffineVar = TEAffineVar<
    <Parameters as Bls12Parameters>::G1Parameters,
    FpVar<<<Parameters as Bls12Parameters>::G1Parameters as CurveConfig>::BaseField>,
>;

/// Represents the cached precomputation that can be performed on a G1 element
/// which enables speeding up pairing computation.
pub type G1PreparedVar = bls12::G1PreparedVar<Parameters>;
/// Represents the cached precomputation that can be performed on a G2 element
/// which enables speeding up pairing computation.
pub type G2PreparedVar = bls12::G2PreparedVar<Parameters>;

#[test]
fn test() {
    use ark_ec::models::bls12::Bls12Parameters;
    ark_curve_constraint_tests::curves::sw_test::<
        <Parameters as Bls12Parameters>::G1Parameters,
        G1Var,
    >()
    .unwrap();
    ark_curve_constraint_tests::curves::te_test::<
        <Parameters as Bls12Parameters>::G1Parameters,
        G1TEAffineVar,
    >()
    .unwrap();
    ark_curve_constraint_tests::curves::sw_test::<
        <Parameters as Bls12Parameters>::G2Parameters,
        G2Var,
    >()
    .unwrap();
}
