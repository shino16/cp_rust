pub mod alg;
pub mod assign;
pub mod bits;
pub mod bool;
pub mod bounded;
pub mod cast;
pub mod cmp;
pub mod conv;
pub mod dfa;
pub mod ds;
pub mod float;
pub mod fp;
pub mod func;
pub mod fxhash;
pub mod graph;
pub mod hash;
pub mod int;
pub mod io;
pub mod io_interactive;
pub mod iter;
pub mod make_vec;
pub mod math;
pub mod mint;
pub mod num;
pub mod rand;
pub mod slice;
pub mod stdio;
pub mod u64;
pub mod vec;
pub mod zo;

pub mod tests;

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! dbg {
    () => {
        std::eprintln!("[{}:{}]", std::file!(), std::line!());
    };
    ($val:expr) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                std::eprintln!("[{}:{}] {} = {:?}",
                    std::file!(), std::line!(), std::stringify!($val), &tmp);
                tmp
            }
        }
    };
    // Trailing comma with single argument is ignored
    ($val:expr,) => { dbg!($val) };
    ($($val:expr),+ $(,)?) => {
        ($(dbg!($val)),+,)
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! dbg {
    ($($x:expr),*) => { std::convert::identity(($($x),*)) }
}
