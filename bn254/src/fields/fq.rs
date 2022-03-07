use ark_ff::fields::{Fp256, MontBackend, MontConfig, MontFp};

#[derive(MontConfig)]
#[modulus = "21888242871839275222246405745257275088696311157297823662689037894645226208583"]
#[generator = "3"]
pub struct FqConfig;
pub type Fq = Fp256<MontBackend<FqConfig, 4>>;

pub const FQ_ONE: Fq = MontFp!(Fq, "1");
pub const FQ_ZERO: Fq = MontFp!(Fq, "0");
