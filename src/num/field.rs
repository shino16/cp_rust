pub use super::*;

pub trait Field: Num + Neg<Output = Self> + Div<Self, Output = Self> + DivAssign<Self> {}
impl<T: Num + Neg<Output = Self> + Div<Self, Output = Self> + DivAssign<Self>> Field for T {}
