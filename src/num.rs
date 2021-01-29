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

macro_rules! impl_num {
	($($t:ty),*) => { $(
		impl Num for $t {
			fn wrapping_add(self, rhs: Self) -> Self {
				self.wrapping_add(rhs)
			}
			fn wrapping_neg(self) -> Self {
				self.wrapping_neg()
			}
		}
	)* };
}

impl_num!(i32, i64, i128, isize, u32, u64, u128, usize);
