use ark_ff::{
    fields::fp6_2over3::{Fp6, Fp6Config},
    MontFp,
};

use crate::{Fq, Fq::ONE, Fq::ZERO, Fq3, Fq3Config};

pub type Fq6 = Fp6<Fq6Config>;

pub struct Fq6Config;

impl Fp6Config for Fq6Config {
    type Fp3Config = Fq3Config;

    /// NONRESIDUE = (0, 1, 0)
    const NONRESIDUE: Fq3 = Fq3::new(Fq::ZERO, Fq::ONE, Fq::ZERO);

    const FROBENIUS_COEFF_FP6_C1: &'static [Fq] = &[
        Fq::ONE,
        MontFp!("4922464560225523242118178942575080391082002530232324381063048548642823052024664478336818169867474395270858391911405337707247735739826664939444490469542109391530482826728203582549674992333383150446779312029624171857054392282775649"),
        MontFp!("4922464560225523242118178942575080391082002530232324381063048548642823052024664478336818169867474395270858391911405337707247735739826664939444490469542109391530482826728203582549674992333383150446779312029624171857054392282775648"),
        MontFp!("-1"),
        MontFp!("1968985824090209297278610739700577151397666382303825728450741611566800370218827257750865013421937292370006175842381275743914023380727582819905021229583192207421122272650305267822868639090213645505120388400344940985710520836292650"),
        MontFp!("1968985824090209297278610739700577151397666382303825728450741611566800370218827257750865013421937292370006175842381275743914023380727582819905021229583192207421122272650305267822868639090213645505120388400344940985710520836292651"),
    ];
}
