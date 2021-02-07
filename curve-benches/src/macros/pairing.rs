#[macro_export]
macro_rules! pairing_bench {
    ($curve:ident, $pairing_field:ident) => {
        const SAMPLES: usize = 1000;

        lazy_static::lazy_static! {
            static ref PREPARED: Vec<(
                <$curve as PairingEngine>::G1Prepared,
                <$curve as PairingEngine>::G2Prepared,
            )> = {
                let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

                let g1s = (0..SAMPLES).map(|_| G1::rand(&mut rng)).collect::<Vec<_>>();
                let g2s = (0..SAMPLES).map(|_| G2::rand(&mut rng)).collect::<Vec<_>>();
                let g1s = G1::batch_normalization_into_affine(&g1s);
                let g2s = G2::batch_normalization_into_affine(&g2s);
                let prepared = g1s
                    .into_iter()
                    .zip(g2s)
                    .map(|(g1, g2)| (g1.into(), g2.into()))
                    .collect::<Vec<(
                        <$curve as PairingEngine>::G1Prepared,
                        <$curve as PairingEngine>::G2Prepared,
                    )>>();
                prepared
            };

            static ref MILLER: Vec<$pairing_field> = {
                let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

                let v: Vec<_> = (0..SAMPLES)
                    .map(|_| {
                        (
                            G1Affine::from(G1::rand(&mut rng)).into(),
                            G2Affine::from(G2::rand(&mut rng)).into(),
                        )
                    })
                    .map(|(p, q)| $curve::miller_loop(&[(p, q)]))
                    .collect();
                v
            };

            static ref G1_G2: Vec<(G1, G2)> = {
                let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

                let v: Vec<(G1, G2)> = (0..SAMPLES)
                    .map(|_| (G1::rand(&mut rng), G2::rand(&mut rng)))
                    .collect();
                v
            };
        }


        fn miller_loop(b: &mut $crate::bencher::Bencher) {
            let mut count = 0;
            let prepared = PREPARED.clone();
            b.iter(|| {
                let tmp =
                    $curve::miller_loop(&[(prepared[count].0.clone(), prepared[count].1.clone())]);
                count = (count + 1) % SAMPLES;
                tmp
            });
        }

        fn final_exponentiation(b: &mut $crate::bencher::Bencher) {
            let mut count = 0;
            let v = MILLER.clone();
            b.iter(|| {
                let tmp = $curve::final_exponentiation(&v[count]);
                count = (count + 1) % SAMPLES;
                tmp
            });
        }

        fn full_pairing(b: &mut $crate::bencher::Bencher) {
            let mut count = 0;
            let v = G1_G2.clone();
            b.iter(|| {
                let tmp = $curve::pairing(v[count].0, v[count].1);
                count = (count + 1) % SAMPLES;
                tmp
            });
        }

        $crate::benchmark_group!(pairing, miller_loop, final_exponentiation, full_pairing,);
    };
}
