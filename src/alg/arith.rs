pub use super::*;
pub use crate::num::*;

#[derive(Default, Clone, Copy)]
pub struct Addition();

impl<T: Num> Monoid<T> for Addition {
    fn unit(&self) -> T { T::ZERO }
    fn op(&self, x: T, y: T) -> T { x.wrapping_add(y) }
}
impl<T: Num> Group<T> for Addition {
    fn inv(&self, x: T) -> T { x.wrapping_neg() }
}
