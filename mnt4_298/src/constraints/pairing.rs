/// Specifies the constraints for computing a pairing in the MNT4-298 bilinear group.
pub use crate::MNT4_298;

#[test]
fn test() {
    use crate::MNT4_298;
    ark_curve_constraint_tests::pairing::bilinearity_test::<MNT4_298>().unwrap()
}
