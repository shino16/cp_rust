use crate::util::trait_alias::*;
pub use crate::zo::ZeroOne;
use std::fmt::*;
use std::ops::*;

pub mod field;

trait_alias!(pub trait Ring =
    ZeroOne
    + Add<Output = Self> + AddAssign
    + Mul<Output = Self> + MulAssign
    + Debug
    + Display);

trait_alias!(pub trait Num =
    Ring
    + Sub<Output = Self> + SubAssign
    + Div<Output = Self> + DivAssign);

trait_alias!(pub trait INum = Num + Neg<Output = Self>);
