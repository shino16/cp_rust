pub use super::*;
use crate::io::*;

impl<M: Mod> Print for Mint<M> {
    fn print(w: &mut IO, x: Self) { w.print(x.value()); }
}

impl<M: Mod> Scan for Mint<M> {
    fn scan(io: &mut IO) -> Self { io.scan::<u32>().into() }
}
