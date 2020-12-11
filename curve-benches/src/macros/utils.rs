#[macro_export]
macro_rules! n_fold {
    ($tmp:ident, $v:ident, $func:ident, $count:ident) => {
        $tmp.$func(&$v[$count].1);
    };

    ($tmp:ident, $func:ident) => {
        $tmp.$func();
    };
}
