use ark_algebra_bench_templates::*;
use ark_ed_on_bls12_381_bandersnatch::{fq::Fq, fr::Fr, EdwardsProjective as G};

bench!(
    Name = "EdOnBls12Bandersnatch_381",
    Group = G,
    ScalarField = Fr,
    PrimeBaseField = Fq,
);
