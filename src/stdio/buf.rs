pub use super::*;
pub use crate::prtln;
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
    ($dst:expr, $expr:expr) => { {
        $crate::prt!($dst, $expr);
        std::writeln!($dst).unwrap();
    } };
    ($dst:expr, $expr:expr, $($exprs:expr),*) => { {
        $crate::prt!($dst, $expr);
        $crate::prt!($dst, " ");
        $crate::prtln($dst, $($exprs),*);
    } };
}

#[macro_export]
macro_rules! prt {
    ($dst:expr,) => {};
    ($dst:expr, $expr:expr) => {
        std::write!($dst, "{}", $expr).unwrap();
    };
    ($dst:expr,iter($expr:expr)) => {
        $crate::prt!($dst, iter($expr, " "));
    };
    ($dst:expr,iter($expr:expr, $delim:expr)) => {
        let mut iter = $expr.into_iter();
        if let Some(expr) = iter.next() {
            $crate::prt!($dst, expr);
            $iter.for_each(|expr| $crate::prt($dst, " ", expr));
        }
    };
}
