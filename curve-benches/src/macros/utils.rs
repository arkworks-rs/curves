#[macro_export]
macro_rules! n_fold {
    ($tmp:ident, $v:ident, $func:ident, $count:ident) => {
        const ITERS: usize = 1000;
        let other = &$v[$count].1;

        #[cfg(not(feature = "n_fold"))]
        $tmp.$func(other);
        #[cfg(feature = "n_fold")]
        for _ in 0..ITERS {
            $tmp.$func(other);
        }
    };

    ($tmp:ident, $func:ident) => {
        const ITERS: usize = 1000;

        #[cfg(not(feature = "n_fold"))]
        $tmp.$func();
        #[cfg(feature = "n_fold")]
        for _ in 0..ITERS {
            $tmp.$func();
        }
    };
}

/// Defines a function called `$group_name` that returns the test description
/// values for the listed functions `$function`.
#[macro_export]
macro_rules! benchmark_group {
    ($group_name:ident, $($function:path),+) => {
        pub fn $group_name() -> ::std::vec::Vec<$crate::TestDescAndFn> {
            use $crate::{TestDescAndFn, TestFn, TestDesc};
            use std::borrow::Cow;
            let mut benches = ::std::vec::Vec::new();
            $(
                benches.push(TestDescAndFn {
                    desc: TestDesc {
                        name: Cow::from(module_path!().to_string() + "::" + stringify!($function)),
                        ignore: false,
                    },
                    testfn: TestFn::StaticBenchFn($function),
                });
            )+
            benches
        }
    };
    ($group_name:ident, $($function:path,)+) => {
        benchmark_group!($group_name, $($function),+);
    };
}
