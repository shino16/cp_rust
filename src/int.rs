use crate::bit::*;
pub use crate::bounded::*;
use crate::cast::*;
pub use crate::num::*;
pub use crate::zo::*;
use std::ops::*;

pub mod arith;
pub mod bisect;
pub mod gcd;
pub mod inv;
pub mod saturate;

pub trait Int: Num + Ord + Rem<Output = Self> + RemAssign + Bounded + Bits + PrimCast {
    type Signed: IInt + CastFrom<Self> + CastTo<Self>;
    type Unsigned: UInt + CastFrom<Self> + CastTo<Self>;
    fn abs(self) -> Self::Unsigned;
    fn rem_euclid(self, rhs: Self::Unsigned) -> Self::Unsigned;
}

pub trait IInt: Int + INum {}
pub trait UInt: Int {}

macro_rules! impl_int {
    (@ $t:ident, $i:ident, $u:ident, $abs:expr) => {
        impl Int for $t {
            type Signed = $i;
            type Unsigned = $u;
            fn abs(self) -> Self::Unsigned {
                $abs(self) as $u
            }
            fn rem_euclid(self, rhs: Self::Unsigned) -> Self::Unsigned {
                <$t>::rem_euclid(self, rhs as $t) as $u
            }
        }
    };
    ({ $i:ident }, { $u:ident }) => {
        impl_int!(@ $i, $i, $u, |x| <$i>::abs(x));
        impl_int!(@ $u, $i, $u, |x| x);
        impl IInt for $i {}
        impl UInt for $u {}
    };
    ({ $i:ident, $($is:ident),* }, { $u:ident, $($us:ident),* }) => {
        impl_int!({ $i }, { $u });
        impl_int!({ $($is),* }, { $($us),* });
    }
}

impl_int!({ i32, i64, i128, isize }, { u32, u64, u128, usize });
