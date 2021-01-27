use crate::bit::*;
use crate::cast::*;
pub use crate::num::*;
pub use crate::zo::*;
use std::ops::*;

pub mod arith;
pub mod bisect;
pub mod gcd;
pub mod inv;

pub trait Int: Num + Ord + Rem<Output = Self> + RemAssign + Bits + PrimCast {
	type Signed: IInt + CastFrom<Self> + CastTo<Self>;
	type Unsigned: UInt + CastFrom<Self> + CastTo<Self>;
	const MIN: Self;
	const MAX: Self;
	fn abs(self) -> Self::Unsigned;
	fn rem_euclid(self, rhs: Self::Unsigned) -> Self::Unsigned;
}

pub trait IInt: Int + INum {}
pub trait UInt: Int {}

macro_rules! impl_int {
	(@num $t:ident) => {
		impl Num for $t {
			fn wrapping_add(self, rhs: Self) -> Self {
				self.wrapping_add(rhs)
			}
			fn wrapping_neg(self) -> Self {
				self.wrapping_neg()
			}
		}
	};
	(@int $t:ident, $i:ident, $u:ident) => {
		impl Int for $t {
			type Signed = $i;
			type Unsigned = $u;
			const MIN: Self = std::$t::MIN;
			const MAX: Self = std::$t::MAX;
			#[allow(unconditional_recursion)] // it's not
			fn abs(self) -> Self::Unsigned {
				self.abs() as $u
			}
			fn rem_euclid(self, rhs: Self::Unsigned) -> Self::Unsigned {
				<$t>::rem_euclid(self, rhs as $t) as $u
			}
		}
	};
	({ $i:ident }, { $u:ident }) => {
		impl_int!(@num $i);
		impl_int!(@num $u);
		impl_int!(@int $i, $i, $u);
		impl_int!(@int $u, $i, $u);
		impl INum for $i {}
		impl IInt for $i {}
		impl UInt for $u {}
	};
	({ $i:ident, $($is:ident),* }, { $u:ident, $($us:ident),* }) => {
		impl_int!({ $i }, { $u });
		impl_int!({ $($is),* }, { $($us),* });
	}
}

impl_int!({ i32, i64, i128, isize }, { u32, u64, u128, usize });
