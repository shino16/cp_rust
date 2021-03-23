pub use super::*;
use crate::io::*;

impl<M: Mod> Print for Gf<M> {
    fn print(w: &mut IO, x: Self) { w.print(x.value()); }
}
impl<M: Mod> Scan for Gf<M> {
    fn scan(io: &mut IO) -> Self { Self::new(io.scan()) }
}
