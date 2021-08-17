/// Specifies the constraints for computing a pairing in the BLS12-377 bilinear group.
pub use crate::Bls12_377;

#[test]
fn test() {
    use crate::Bls12_377;
    ark_curve_constraint_tests::pairing::bilinearity_test::<Bls12_377>().unwrap()
}
