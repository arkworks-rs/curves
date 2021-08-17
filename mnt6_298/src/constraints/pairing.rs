/// Specifies the constraints for computing a pairing in the MNT6-298 bilinear group.
pub use crate::MNT6_298;

#[test]
fn test() {
    use crate::MNT6_298;
    ark_curve_constraint_tests::pairing::bilinearity_test::<MNT6_298>().unwrap()
}
