/// Specifies the constraints for computing a pairing in the MNT6-753 bilinear group.
pub use crate::MNT6_753;

#[test]
fn test() {
    use crate::MNT6_753;
    ark_curve_constraint_tests::pairing::bilinearity_test::<MNT6_753>().unwrap()
}
