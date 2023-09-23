use ark_algebra_bench_templates::*;
use ark_babyjub::{EdwardsProjective as G, Fq, Fr};

bench!(
    Name = "babyjub",
    Group = G,
    ScalarField = Fr,
    PrimeBaseField = Fq,
);
