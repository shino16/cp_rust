use crate::as_int::*;
use crate::bit::*;
use std::fmt::*;
use std::ops::*;

pub mod bisect;

pub trait ZeroOne: Copy + Eq {
	const ZERO: Self;
	fn is_zero(self) -> bool {
		self == Self::ZERO
	}
	const ONE: Self;
}

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
}

pub trait INum: Num + Neg<Output = Self> {}

pub trait Int: Num + Ord + Rem<Output = Self> + RemAssign + Bits + CastInt {
	type Signed: IInt + CastFrom<Self> + CastTo<Self>;
	type Unsigned: UInt + CastFrom<Self> + CastTo<Self>;
	fn rem_euclid(self, rhs: Self::Unsigned) -> Self::Unsigned;
}

pub trait IInt: Int + INum {}
pub trait UInt: Int {}

macro_rules! impl_int {
	(@num $t:ty) => {
		impl ZeroOne for $t {
			const ZERO: Self = 0;
			const ONE: Self = 1;
		}
		impl Num for $t {}
	};
	(@int $t:ty, $i:ty, $u:ty) => {
		impl Int for $t {
			type Signed = $i;
			type Unsigned = $u;
			fn rem_euclid(self, rhs: Self::Unsigned) -> Self::Unsigned {
				<$t>::rem_euclid(self, rhs as $t) as $u
			}
		}
	};
	({ $i:ty }, { $u:ty }) => {
		impl_int!(@num $i);
		impl_int!(@num $u);
		impl_int!(@int $i, $i, $u);
		impl_int!(@int $u, $i, $u);
		impl INum for $i {}
		impl IInt for $i {}
		impl UInt for $u {}
	};
	({ $i:ty, $($is:ty),* }, { $u:ty, $($us:ty),* }) => {
		impl_int!({ $i }, { $u });
		impl_int!({ $($is),* }, { $($us),* });
	}
}

impl_int!({ i32, i64, i128, isize }, { u32, u64, u128, usize });
