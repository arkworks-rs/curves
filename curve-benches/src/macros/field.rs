#[macro_export]
macro_rules! f_bench {
    // Use this for base fields
    ($f:ident, $f_type:ty, $f_repr:ident, $f_repr_type:ty, $modname:ident) => {
        $crate::paste!{
            pub mod [<_ $modname>] {
                use super::*;
                field_common!($f, $f_type);
                sqrt!($f, $f_type);
                prime_field!($f, $f_type, $f_repr, $f_repr_type);
            }
            $crate::criterion::criterion_group!($modname, [<_ $modname>]::bench_common_field_ops, [<_ $modname>]::bench_sqrt, [<_ $modname>]::bench_prime_field_ops);
        }
    };
    // use this for intermediate fields
    (extension, $f:ident, $f_type:ty, $modname:ident) => {
        $crate::paste!{
            mod [<_ $modname>] {
                use super::*;
                field_common!($f, $f_type);
                sqrt!($f, $f_type);
            }
            $crate::criterion::criterion_group!($modname, [<_ $modname>]::bench_common_field_ops, [<_ $modname>]::bench_sqrt);
        }
    };
    // Use this for the full extension field Fqk
    (target, $f:ident, $f_type:ty, $modname:ident) => {
        crate::paste! {
            mod [<_ $modname>] {
                use super::*;
                field_common!($f, $f_type);
            }
            $crate::criterion::criterion_group!($modname, [<_ $modname>]::bench_common_field_ops);
        }
    };
}

#[macro_export]
macro_rules! field_common {
    ($f:ident, $f_type:ty) => {
        pub fn bench_common_field_ops(c: &mut $crate::criterion::Criterion) {
            let mut group = c.benchmark_group("Common field operations for ".to_string() + core::stringify!($f));
            group.bench_function("AddAssign", add_assign);
            group.bench_function("SubAssign", sub_assign);
            group.bench_function("Double", double);
            group.bench_function("Negate", negate);
            group.bench_function("MulAssign", mul_assign);
            group.bench_function("Square", square);
            group.bench_function("Inverse", inverse);
            group.bench_function("Serialize w/ compression", ser);
            group.bench_function("Deserialize w/ compression", deser);
            group.bench_function("Serialize unchecked", ser_unchecked);
            group.bench_function("Deserialize unchecked", deser_unchecked);
        }

        fn add_assign(b: &mut $crate::criterion::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<_> = (0..SAMPLES)
                .map(|_| ($f::rand(&mut rng), $f::rand(&mut rng)))
                .collect();

            let mut count = 0;
            b.iter(|| {
                let mut tmp = v[count].0;
                n_fold!(tmp, v, add_assign, count);
                count = (count + 1) % SAMPLES;
                tmp
            });
        }


        fn sub_assign(b: &mut $crate::criterion::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<_> = (0..SAMPLES)
                .map(|_| ($f::rand(&mut rng), $f::rand(&mut rng)))
                .collect();

            let mut count = 0;
            b.iter(|| {
                let mut tmp = v[count].0;
                n_fold!(tmp, v, sub_assign, count);
                count = (count + 1) % SAMPLES;
                tmp
            });
        }

        fn double(b: &mut $crate::criterion::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<$f_type> = (0..SAMPLES).map(|_| $f::rand(&mut rng)).collect();

            let mut count = 0;
            b.iter(|| {
                let mut tmp = v[count];
                n_fold!(tmp, double_in_place);
                count = (count + 1) % SAMPLES;
                tmp
            });
        }


        fn negate(b: &mut $crate::criterion::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<$f_type> = (0..SAMPLES).map(|_| $f::rand(&mut rng)).collect();

            let mut count = 0;
            b.iter(|| {
                let mut tmp = v[count];
                tmp = -tmp;
                count = (count + 1) % SAMPLES;
                tmp
            });
        }

        fn mul_assign(b: &mut $crate::criterion::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<_> = (0..SAMPLES)
                .map(|_| ($f::rand(&mut rng), $f::rand(&mut rng)))
                .collect();

            let mut count = 0;
            b.iter(|| {
                let mut tmp = v[count].0;
                n_fold!(tmp, v, mul_assign, count);
                count = (count + 1) % SAMPLES;
                tmp
            });
        }

        fn square(b: &mut $crate::criterion::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<$f_type> = (0..SAMPLES).map(|_| $f::rand(&mut rng)).collect();

            let mut count = 0;
            b.iter(|| {
                let mut tmp = v[count];
                n_fold!(tmp, square_in_place);
                count = (count + 1) % SAMPLES;
                tmp
            });
        }


        fn inverse(b: &mut $crate::criterion::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<$f_type> = (0..SAMPLES).map(|_| $f::rand(&mut rng)).collect();

            let mut count = 0;
            b.iter(|| {
                let tmp = v[count].inverse();
                count = (count + 1) % SAMPLES;
                tmp
            });
        }


        fn deser(b: &mut $crate::criterion::Bencher) {
            use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let mut num_bytes = 0;
            let v: Vec<_> = (0..SAMPLES).flat_map(|_| {
                let mut bytes = Vec::with_capacity(1000);
                let tmp = $f::rand(&mut rng);
                tmp.serialize(&mut bytes).unwrap();
                num_bytes = bytes.len();
                bytes
            }).collect();

            let mut count = 0;
            b.iter(|| {
                count = (count + 1) % SAMPLES;
                let index = count * num_bytes;
                <$f_type>::deserialize(&v[index..(index + num_bytes)]).unwrap()
            });
        }


        fn ser(b: &mut $crate::criterion::Bencher) {
            use ark_serialize::CanonicalSerialize;
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<$f_type> = (0..SAMPLES).map(|_| $f::rand(&mut rng)).collect();
            let mut bytes = Vec::with_capacity(1000);

            let mut count = 0;
            b.iter(|| {
                let tmp = v[count];
                count = (count + 1) % SAMPLES;
                bytes.clear();
                tmp.serialize(&mut bytes)

            });
        }


        fn deser_unchecked(b: &mut $crate::criterion::Bencher) {
            use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let mut num_bytes = 0;
            let v: Vec<_> = (0..SAMPLES).flat_map(|_| {
                let mut bytes = Vec::with_capacity(1000);
                let tmp = $f::rand(&mut rng);
                tmp.serialize_unchecked(&mut bytes).unwrap();
                num_bytes = bytes.len();
                bytes
            }).collect();

            let mut count = 0;
            b.iter(|| {
                count = (count + 1) % SAMPLES;
                let index = count * num_bytes;
                <$f_type>::deserialize_unchecked(&v[index..(index + num_bytes)]).unwrap()
            });
        }


        fn ser_unchecked(b: &mut $crate::criterion::Bencher) {
            use ark_serialize::CanonicalSerialize;
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<$f_type> = (0..SAMPLES).map(|_| $f::rand(&mut rng)).collect();
            let mut bytes = Vec::with_capacity(1000);

            let mut count = 0;
            b.iter(|| {
                let tmp = v[count];
                count = (count + 1) % SAMPLES;
                bytes.clear();
                tmp.serialize_unchecked(&mut bytes)

            });
        }
    }
}

#[macro_export]
macro_rules! sqrt {
    ($f:ident, $f_type:ty) => {
        pub fn bench_sqrt(b: &mut $crate::criterion::Criterion) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<$f_type> = (0..SAMPLES)
                .map(|_| {
                    let mut tmp = $f::rand(&mut rng);
                    tmp.square_in_place();
                    tmp
                })
            .collect();

            let mut count = 0;
            b.bench_function(
                core::concat!("Square-roots for ", core::stringify!($f)),
                |_| {
                count = (count + 1) % SAMPLES;
                let _ = v[count].sqrt();
            });
        }
    }
}

#[macro_export]
macro_rules! prime_field {
    ($f:ident, $f_type:ty, $f_repr:ident, $f_repr_type:ty) => {
        pub fn bench_prime_field_ops(c: &mut $crate::criterion::Criterion) {
            let mut group = c.benchmark_group("Prime field operations for ".to_string() + core::stringify!($f));
            group.bench_function("AddNoCarry for BigInteger", repr_add_nocarry);
            group.bench_function("SubNoBorrow for BigInteger", repr_sub_noborrow);
            group.bench_function("NumBits for BigInteger", repr_num_bits);
            group.bench_function("MulBy2 for BigInteger", repr_mul2);
            group.bench_function("DivBy2 for BigInteger", repr_div2);
            group.bench_function("Into BigInteger", into_repr);
            group.bench_function("From BigInteger", from_repr);
        }

        fn repr_add_nocarry(b: &mut $crate::criterion::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<_> = (0..SAMPLES)
                .map(|_| {
                    let mut tmp1 = $f_repr::rand(&mut rng);
                    let mut tmp2 = $f_repr::rand(&mut rng);
                    // Shave a few bits off to avoid overflow.
                    for _ in 0..3 {
                        tmp1.div2();
                        tmp2.div2();
                    }
                    (tmp1, tmp2)
                })
            .collect();

            let mut count = 0;
            b.iter(|| {
                let mut tmp = v[count].0;
                n_fold!(tmp, v, add_nocarry, count);
                count = (count + 1) % SAMPLES;
                tmp
            });
        }

        fn repr_sub_noborrow(b: &mut $crate::criterion::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<_> = (0..SAMPLES)
                .map(|_| {
                    let tmp1 = $f_repr::rand(&mut rng);
                    let mut tmp2 = tmp1;
                    // Ensure tmp2 is smaller than tmp1.
                    for _ in 0..10 {
                        tmp2.div2();
                    }
                    (tmp1, tmp2)
                })
            .collect();

            let mut count = 0;
            b.iter(|| {
                let mut tmp = v[count].0;
                n_fold!(tmp, v, sub_noborrow, count);
                count = (count + 1) % SAMPLES;
                tmp;
            });
        }

        fn repr_num_bits(b: &mut $crate::criterion::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<$f_repr_type> = (0..SAMPLES).map(|_| $f_repr::rand(&mut rng)).collect();

            let mut count = 0;
            b.iter(|| {
                let tmp = v[count].num_bits();
                count = (count + 1) % SAMPLES;
                tmp;
            });
        }

        fn repr_mul2(b: &mut $crate::criterion::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<$f_repr_type> = (0..SAMPLES).map(|_| $f_repr::rand(&mut rng)).collect();

            let mut count = 0;
            b.iter(|| {
                let mut tmp = v[count];
                n_fold!(tmp, mul2);
                count = (count + 1) % SAMPLES;
                tmp;
            });
        }

        fn repr_div2(b: &mut $crate::criterion::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<$f_repr_type> = (0..SAMPLES).map(|_| $f_repr::rand(&mut rng)).collect();

            let mut count = 0;
            b.iter(|| {
                let mut tmp = v[count];
                n_fold!(tmp, div2);
                count = (count + 1) % SAMPLES;
                tmp;
            });
        }

        fn into_repr(b: &mut $crate::criterion::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<$f_type> = (0..SAMPLES).map(|_| $f::rand(&mut rng)).collect();

            let mut count = 0;
            b.iter(|| {
                count = (count + 1) % SAMPLES;
                v[count].into_repr();
            });
        }

        fn from_repr(b: &mut $crate::criterion::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<$f_repr_type> = (0..SAMPLES)
                .map(|_| $f::rand(&mut rng).into_repr())
                .collect();

            let mut count = 0;
            b.iter(|| {
                count = (count + 1) % SAMPLES;
                $f::from(v[count]);
            });
        }
    }
}
