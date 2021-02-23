use crate::util::trait_alias::*;
pub use crate::zo::ZeroOne;
use std::fmt::*;
use std::ops::*;

pub mod field;

trait_alias!(pub trait Num =
    ZeroOne
    + Add<Output = Self> + AddAssign
    + Sub<Output = Self> + SubAssign
    + Mul<Output = Self> + MulAssign
    + Div<Output = Self> + DivAssign
    + Debug
    + Display);

trait_alias!(pub trait INum = Num + Neg<Output = Self>);
