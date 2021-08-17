/// Specifies the constraints for computing a pairing in the MNT4-753 bilinear group.
pub use crate::MNT4_753;

#[test]
fn test() {
    use crate::MNT4_753;
    ark_curve_constraint_tests::pairing::bilinearity_test::<MNT4_753>().unwrap()
}
