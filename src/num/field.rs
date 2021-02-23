pub use super::*;

trait_alias!(pub trait Field = INum + Div<Self, Output = Self> + DivAssign<Self>);
