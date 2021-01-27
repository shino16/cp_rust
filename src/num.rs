pub use crate::zo::ZeroOne;
use std::fmt::*;
use std::ops::*;

pub trait Num:
	ZeroOne
	+ Add<Output = Self>
	+ AddAssign
	+ Sub<Output = Self>
	+ SubAssign
	+ Mul<Output = Self>
	+ MulAssign
	+ Div<Output = Self>
	+ DivAssign
	+ Debug
	+ Display
{
	fn wrapping_add(self, rhs: Self) -> Self;
	fn wrapping_neg(self) -> Self;
}

pub trait INum: Num + Neg<Output = Self> {}
