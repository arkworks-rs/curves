use crate::Parameters;

/// Specifies the constraints for computing a pairing in the MNT6-298 bilinear
/// group.
pub type PairingVar = ark_r1cs_std::pairing::mnt6::PairingVar<Parameters>;

#[test]
fn test() {
    use crate::MNT6_298;
    ark_curve_constraint_tests::pairing::bilinearity_test::<MNT6_298, PairingVar>().unwrap()
}
