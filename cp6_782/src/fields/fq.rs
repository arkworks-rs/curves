use ark_ff::fields::{Fp832, MontBackend, MontConfig, MontFp};

#[derive(MontConfig)]
#[modulus = "22369874298875696930346742206501054934775599465297184582183496627646774052458024540232479018147881220178054575403841904557897715222633333372134756426301062487682326574958588001132586331462553235407484089304633076250782629492557320825577"]
#[generator = "13"]
pub struct FqConfig;
pub type Fq = Fp832<MontBackend<FqConfig, 13>>;

pub const FQ_ONE: Fq = MontFp!("1");
pub const FQ_ZERO: Fq = MontFp!("0");
