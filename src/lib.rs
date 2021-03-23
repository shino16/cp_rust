pub mod alg;
pub mod assign;
pub mod bits;
pub mod bool;
pub mod bounded;
pub mod cast;
pub mod cmp;
pub mod complex;
pub mod conv;
pub mod dfa;
pub mod ds;
pub mod float;
pub mod gf;
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
pub mod poly;
pub mod rand;
pub mod slice;
pub mod stdio;
pub mod u64;
pub mod util;
pub mod vec;
pub mod zo;

pub mod tests;

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! dbg {
    ($($val:expr),* $(,)?) => {
        ($( match $val {
            tmp => {
                std::eprintln!("[{}:{}] {} = {:?}",
                    std::file!(), std::line!(), std::stringify!($val), &tmp);
                tmp
            }
        } ),*)
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! dbg {
    ($($x:expr),*) => {};
}
