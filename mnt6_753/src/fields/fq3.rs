use crate::{fq::Fq, FQ_ONE, FQ_ZERO};
use ark_ff::{
    fields::fp3::{Fp3, Fp3Config},
    CubicExt, MontFp,
};

pub type Fq3 = Fp3<Fq3Config>;

pub struct Fq3Config;

impl Fp3Config for Fq3Config {
    type Fp = Fq;

    const NONRESIDUE: Fq = MontFp!(Fq, "11");

    const TWO_ADICITY: u32 = 30;

    #[rustfmt::skip]
    const TRACE_MINUS_ONE_DIV_TWO: &'static [u64] = &[
        15439605736802142541,
        18190868848461853149,
        6220121510046940818,
        10310485528612680366,
        5032137869959796540,
        3943048799800510054,
        1971151279016362045,
        6096644900171872841,
        12908407994230849218,
        4163225373804228290,
        10382959950522770522,
        9008828410264446883,
        18411821899404157689,
        12386199240837247984,
        13370099281150720481,
        11909278545073807560,
        5964354403900302648,
        15347506722065009035,
        7045354120681109597,
        14294096902719509929,
        6180325033003959541,
        14381489272445870003,
        18159920240207503954,
        17487026929061632528,
        12314108197538755669,
        12116872703077811769,
        3401400733784294722,
        13905351619889935522,
        10972472942574358218,
        6104159581753028261,
        4690139121547787552,
        4880965491878697414,
        1926648890365125214,
        13532564555356297305,
        3114545746551080,
    ];

    /// (11^T, 0, 0)
    #[rustfmt::skip]
    const QUADRATIC_NONRESIDUE_TO_T: Fq3 = CubicExt!(
        MontFp!(Fq, "22168644070733283197994897338612733221095941481265408161807376791727499343083607817089033595478370212662133368413166734396127674284827734481031659015434501966360165723728649019457855887066657739809176476252080335185730833468062"),
        FQ_ZERO,
        FQ_ZERO,
    );

    // Coefficients for the Frobenius automorphism.
    // c1[0] = 1,
    // c1[1] = 24129022407817241407134263419936114379815707076943508280977368156625538709102831814843582780138963119807143081677569721953561801075623741378629346409604471234573396989178424163772589090105392407118197799904755622897541183052132
    // c1[2] = 17769468560101711995209951371304522748355002843010440790806134764399814103468274958215310983651375801610927890210888755369611256415970113691066895445191924931148019336171640277697829047741006062493737919155152541323243293107868,
    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP3_C1: &'static [Fq] = &[
        FQ_ONE,
        MontFp!(Fq, "24129022407817241407134263419936114379815707076943508280977368156625538709102831814843582780138963119807143081677569721953561801075623741378629346409604471234573396989178424163772589090105392407118197799904755622897541183052132"),
        MontFp!(Fq, "17769468560101711995209951371304522748355002843010440790806134764399814103468274958215310983651375801610927890210888755369611256415970113691066895445191924931148019336171640277697829047741006062493737919155152541323243293107868"),
    ];

    // c2 = {c1[0], c1[2], c1[1]}
    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP3_C2: &'static [Fq] = &[
        FQ_ONE,
        Self::FROBENIUS_COEFF_FP3_C1[2],
        Self::FROBENIUS_COEFF_FP3_C1[1],
    ];
}
