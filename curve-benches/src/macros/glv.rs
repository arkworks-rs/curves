#[macro_export]
macro_rules! glv_bench {
    ($affine:ty, $params:ty) => {

        fn glv_mul(b: &mut $crate::bencher::Bencher) {
            use ark_ec::glv::GLVParameters;
            const SAMPLES: usize = 100;

            let mut rng = ark_std::test_rng();

            let v: Vec<($affine, Fr)> = (0..SAMPLES)
                .map(|_| (<$affine>::rand(&mut rng), Fr::rand(&mut rng)))
                .collect();

            let mut count = 0;
            b.iter(|| {
                let mut tmp = v[count].0;
                let tmp  = <$params>::glv_mul(&tmp, v[count].1);
                count = (count + 1) % SAMPLES;
                tmp
            });
        }

        $crate::benchmark_group!(
            group_glv_ops,
            glv_mul
        );
    };
}