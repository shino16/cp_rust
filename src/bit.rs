pub trait Lsb {
    fn lsb(self) -> Self;
}

pub trait ILog2 {
    fn ilog2(self) -> u32;
}

pub trait Msb {
    fn msb(self) -> Self;
}

macro_rules! impl_bit {
    ($($t:ty), *) => {
        $(
            impl Lsb for $t {
                fn lsb(self) -> Self {
                    self & self.wrapping_neg()
                }
            }
            impl ILog2 for $t {
                fn ilog2(self) -> u32 {
                    std::mem::size_of::<$t>() as u32 * 8 - self.leading_zeros() - 1
                }
            }
            impl Msb for $t {
                fn msb(self) -> Self {
                    (1 as $t) << self.ilog2()
                }
            }
        )*
    };
}

impl_bit!(i32, i64, u32, u64, usize);
