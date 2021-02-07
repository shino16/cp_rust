pub use super::*;
use crate::num::*;

impl<M: Mod> Num for Mint<M> {
    fn wrapping_add(self, rhs: Self) -> Self {
        self + rhs
    }
    fn wrapping_neg(self) -> Self {
        -self
    }
}
