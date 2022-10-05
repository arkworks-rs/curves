use crate::Parameters;

/// Specifies the constraints for computing a pairing in the MNT4-753 bilinear
/// group.
pub type PairingVar = ark_r1cs_std::pairing::mnt4::PairingVar<Parameters>;

#[test]
fn test() {
    use crate::MNT4_753;
    ark_curve_constraint_tests::pairing::bilinearity_test::<MNT4_753, PairingVar>().unwrap()
}
