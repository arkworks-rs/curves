use ark_ff::fields::{Fp320, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "115792089237316195423570985008687907852837564279074904382605163141518161494337"]
#[generator = "7"]
pub struct FrConfig;
pub type Fr = Fp320<MontBackend<FrConfig, 5>>;
