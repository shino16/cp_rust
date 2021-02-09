pub use crate::{scan, prtln};
pub use std::io::Write;
use std::io::{stdout, BufWriter, StdoutLock};

pub fn stdout_buf() -> BufWriter<StdoutLock<'static>> {
    let out = Box::leak(Box::new(stdout()));
    BufWriter::new(out.lock())
}

#[macro_export]
macro_rules! prtln {
    ($dst:expr) => {
        std::writeln!($dst).unwrap();
    };
    ($dst:expr, $expr:expr) => {
        std::writeln!($dst, "{}", $expr).unwrap();
    };
    ($dst:expr, $expr:expr, $($exprs:expr),*) => { {
        std::write!($dst, "{} ", $expr).unwrap();
        $crate::prtln!($dst, $($exprs),*);
    } };
    ($dst:expr, iter $expr:expr) => {
        $crate::prtln!($dst, iter $expr, sep " ");
    };
    ($dst:expr, iter $expr:expr, sep $sep:expr) => { {
        let mut iter = $expr.into_iter();
        if let Some(expr) = iter.next() {
            std::write!($dst, "{}", expr).unwrap();
            for expr in iter {
                std::write!($dst, "{}{}", $sep, expr).unwrap();
            }
        }
        $crate::prtln!($dst);
    } };
}

#[macro_export]
macro_rules! scan {
    (from $s:expr, $($r:tt)*) => {
        $crate::scan_inner!($s, $($r)*);
    };
    (save to $var:ident, $($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let $var = &mut s.split_whitespace();
        $crate::scan_inner!($var, $($r)*);
    };
    ($($r:tt)*) => {
        $crate::scan!(save to __scan, $($r)*);
    }
}

#[macro_export]
macro_rules! scan_inner {
    ($iter:expr $(,)?) => {};
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