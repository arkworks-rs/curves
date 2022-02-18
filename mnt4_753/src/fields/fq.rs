use ark_ff::fields::{Fp768, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "41898490967918953402344214791240637128170709919953949071783502921025352812571106773058893763790338921418070971888253786114353726529584385201591605722013126468931404347949840543007986327743462853720628051692141265303114721689601"]
#[generator = "17"]
pub struct FqConfig;
pub type Fq = Fp768<MontBackend<FqConfig, 12>>;
