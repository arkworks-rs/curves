macro_rules! ec_bench {
    ($projective:ty, $affine:ty) => {
        #[bench]
        fn bench_rand(b: &mut ::test::Bencher) {
            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);
            b.iter(|| <$projective>::rand(&mut rng));
        }

        #[bench]
        fn bench_mul_assign(b: &mut ::test::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<($projective, Fr)> = (0..SAMPLES)
                .map(|_| (<$projective>::rand(&mut rng), Fr::rand(&mut rng)))
                .collect();

            let mut count = 0;
            b.iter(|| {
                let mut tmp = v[count].0;
                tmp *= v[count].1;
                count = (count + 1) % SAMPLES;
                tmp
            });
        }

        #[bench]
        fn bench_add_assign(b: &mut ::test::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<($projective, $projective)> = (0..SAMPLES)
                .map(|_| (<$projective>::rand(&mut rng), <$projective>::rand(&mut rng)))
                .collect();

            let mut count = 0;
            b.iter(|| {
                let mut tmp = v[count].0;
                n_fold!(tmp, v, add_assign, count);
                count = (count + 1) % SAMPLES;
                tmp
            });
        }

        #[bench]
        fn bench_add_assign_mixed(b: &mut ::test::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<($projective, $affine)> = (0..SAMPLES)
                .map(|_| {
                    (
                        <$projective>::rand(&mut rng),
                        <$projective>::rand(&mut rng).into(),
                    )
                })
                .collect();

            let mut count = 0;
            b.iter(|| {
                let mut tmp = v[count].0;
                n_fold!(tmp, v, add_assign_mixed, count);
                count = (count + 1) % SAMPLES;
                tmp
            });
        }

        #[bench]
        fn bench_double(b: &mut ::test::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<($projective, $projective)> = (0..SAMPLES)
                .map(|_| (<$projective>::rand(&mut rng), <$projective>::rand(&mut rng)))
                .collect();

            let mut count = 0;
            b.iter(|| {
                let mut tmp = v[count].0;
                n_fold!(tmp, double_in_place);
                count = (count + 1) % SAMPLES;
                tmp
            });
        }

        #[bench]
        fn bench_deser(b: &mut ::test::Bencher) {
            use ark_ec::ProjectiveCurve;
            use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let mut num_bytes = 0;
            let tmp = <$projective>::rand(&mut rng).into_affine();
            let v: Vec<_> = (0..SAMPLES)
                .flat_map(|_| {
                    let mut bytes = Vec::with_capacity(1000);
                    tmp.serialize(&mut bytes).unwrap();
                    num_bytes = bytes.len();
                    bytes
                })
                .collect();

            let mut count = 0;
            b.iter(|| {
                count = (count + 1) % SAMPLES;
                let index = count * num_bytes;
                <$affine>::deserialize(&v[index..(index + num_bytes)]).unwrap()
            });
        }

        #[bench]
        fn bench_ser(b: &mut ::test::Bencher) {
            use ark_ec::ProjectiveCurve;
            use ark_serialize::CanonicalSerialize;
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let mut v: Vec<_> = (0..SAMPLES)
                .map(|_| <$projective>::rand(&mut rng))
                .collect();
            let v = <$projective>::batch_normalization_into_affine(v.as_mut_slice());
            let mut bytes = Vec::with_capacity(1000);

            let mut count = 0;
            b.iter(|| {
                let tmp = v[count];
                count = (count + 1) % SAMPLES;
                bytes.clear();
                tmp.serialize(&mut bytes)
            });
        }

        #[bench]
        fn bench_deser_unchecked(b: &mut ::test::Bencher) {
            use ark_ec::ProjectiveCurve;
            use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let mut num_bytes = 0;
            let tmp = <$projective>::rand(&mut rng).into_affine();
            let v: Vec<_> = (0..SAMPLES)
                .flat_map(|_| {
                    let mut bytes = Vec::with_capacity(1000);
                    tmp.serialize_unchecked(&mut bytes).unwrap();
                    num_bytes = bytes.len();
                    bytes
                })
                .collect();

            let mut count = 0;
            b.iter(|| {
                count = (count + 1) % SAMPLES;
                let index = count * num_bytes;
                <$affine>::deserialize_unchecked(&v[index..(index + num_bytes)]).unwrap()
            });
        }

        #[bench]
        fn bench_ser_unchecked(b: &mut ::test::Bencher) {
            use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let mut v: Vec<_> = (0..SAMPLES)
                .map(|_| <$projective>::rand(&mut rng))
                .collect();
            let v = <$projective>::batch_normalization_into_affine(v.as_mut_slice());
            let mut bytes = Vec::with_capacity(1000);

            let mut count = 0;
            b.iter(|| {
                let tmp = v[count];
                count = (count + 1) % SAMPLES;
                bytes.clear();
                tmp.serialize_unchecked(&mut bytes)
            });
        }
    };
}
