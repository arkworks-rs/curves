use ark_ff::{
    fields::fp4::{Fp4, Fp4Config},
    MontFp,
};

use crate::{Fq, Fq2, Fq2Config, FQ_ONE, FQ_ZERO};

pub type Fq4 = Fp4<Fq4Config>;

pub struct Fq4Config;

impl Fp4Config for Fq4Config {
    type Fp2Config = Fq2Config;

    const NONRESIDUE: Fq2 = Fq2::new(FQ_ZERO, FQ_ONE);

    // Coefficients for the Frobenius automorphism.
    // c1[0] = 1,
    // c1[1] = 18691656569803771296244054523431852464958959799019013859007259692542121208304602539555350517075508287829753932558576476751900235650227380562700444433662761577027341858128610410779088384480737679672900770810745291515010467307990
    // c1[2] = 41898490967918953402344214791240637128170709919953949071783502921025352812571106773058893763790338921418070971888253786114353726529584385201591605722013126468931404347949840543007986327743462853720628051692141265303114721689600
    // c1[3] = 23206834398115182106100160267808784663211750120934935212776243228483231604266504233503543246714830633588317039329677309362453490879357004638891161288350364891904062489821230132228897943262725174047727280881395973788104254381611
    //
    // These are calculated as `FROBENIUS_COEFF_FP4_C1[i] =
    // Fp2Config::NONRESIDUE^((q^i - 1) / 4)`.
    const FROBENIUS_COEFF_FP4_C1: &'static [Fq] = &[
        FQ_ONE,
        MontFp!("18691656569803771296244054523431852464958959799019013859007259692542121208304602539555350517075508287829753932558576476751900235650227380562700444433662761577027341858128610410779088384480737679672900770810745291515010467307990"),
        MontFp!("41898490967918953402344214791240637128170709919953949071783502921025352812571106773058893763790338921418070971888253786114353726529584385201591605722013126468931404347949840543007986327743462853720628051692141265303114721689600"),
        MontFp!("23206834398115182106100160267808784663211750120934935212776243228483231604266504233503543246714830633588317039329677309362453490879357004638891161288350364891904062489821230132228897943262725174047727280881395973788104254381611"),
    ];
}
