pub use crate::{scan, prtln};
use std::io::{stdout, BufWriter, StdoutLock};

pub fn stdout_buf() -> BufWriter<StdoutLock<'static>> {
    let out = Box::leak(Box::new(stdout()));
    BufWriter::new(out.lock())
}

#[macro_export]
macro_rules! prtln {
    (new $var:ident) => {
        let mut $var = stdout_buf();
    };
    (new $var:ident, $($t:tt)*) => { {
        $crate::prtln!(new $var);
        $crate::prtln!(to $var, $($t)*);
    } };
    (to $var:ident, $($t:tt)*) => { {
        use std::io::Write;
        $crate::prtln_inner!($var, $($t)*);
    } };
    ($($t:tt)*) => {
        $crate::prtln!(new __prtln, $($t)*);
    };
}

#[macro_export]
macro_rules! prtln_inner {
    ($dst:expr $(,)?) => {
        ::std::writeln!($dst).unwrap();
    };
    ($dst:expr, $expr:expr $(,$exprs:expr)*) => { {
        ::std::write!($dst, "{} ", $expr).unwrap();
        $crate::prtln_inner!($dst, $($exprs),*);
    } };
    ($dst:expr, iter=$expr:expr, sep=$sep:expr) => { {
        let mut iter = $expr.into_iter();
        if let Some(expr) = iter.next() {
            ::std::write!($dst, "{}", expr).unwrap();
            for expr in iter {
                ::std::write!($dst, "{}{}", $sep, expr).unwrap();
            }
        }
        $crate::prtln_inner!($dst);
    } };
}

#[macro_export]
macro_rules! scan {
    (from $s:expr, $($r:tt)*) => {
        $crate::scan_inner!($s, $($r)*);
    };
    (new $var:ident, $($r:tt)*) => {
        let s = {
            use ::std::io::Read;
            let mut s = String::new();
            ::std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let $var = &mut s.split_whitespace();
        $crate::scan_inner!($var, $($r)*);
    };
    ($($r:tt)*) => {
        $crate::scan!(nwq __scan, $($r)*);
    }
}

#[macro_export]
macro_rules! scan_inner {
    ($iter:expr $(,)?) => {};
    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = $crate::scan_value!($iter, $t);
        $crate::scan_inner!($iter $($r)*)
    };
    ($iter:expr, $pat:pat in $t:tt $($r:tt)*) => {
        let $pat = $crate::scan_value!($iter, $t);
        $crate::scan_inner!($iter $($r)*)
    };
}

#[macro_export]
macro_rules! scan_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $($crate::scan_value!($iter, $t)),* )
    };
    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| $crate::scan_value!($iter, $t)).collect::<Vec<_>>()
    };
    ($iter:expr, bytes) => {
        $iter.next().as_bytes()
    };
    ($iter:expr, usize1) => {
        $crate::scan_value!($iter, usize) - 1
    };
    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().unwrap()
    };
}
