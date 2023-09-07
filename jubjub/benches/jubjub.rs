use ark_algebra_bench_templates::*;
use ark_jubjub::{EdwardsProjective as G, Fq, Fr};

bench!(
    Name = "jubjub",
    Group = G,
    ScalarField = Fr,
    PrimeBaseField = Fq,
);
