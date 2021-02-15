pub use super::*;
use crate::io::*;

impl<M: Mod> Print for Fp<M> {
    fn print(w: &mut IO, x: Self) { w.print(x.value()); }
}
impl<M: Mod> Scan for Fp<M> {
    fn scan(io: &mut IO) -> Self { Self::new(io.scan()) }
}
