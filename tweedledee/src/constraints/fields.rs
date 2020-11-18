use crate::fp::Fp;
use ark_r1cs_std::fields::fp::FpVar;

/// A variable that is the R1CS equivalent of `crate::Fp`.
pub type FBaseVar = FpVar<Fp>;

#[test]
fn test() {
    ark_curve_constraint_tests::fields::field_test::<_, _, FBaseVar>().unwrap();
}
