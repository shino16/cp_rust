pub trait BitSet {
    fn get_bit(&self, i: usize) -> bool;
    fn set_bit(&mut self, i: usize) -> bool;
    fn negate(&mut self);
    fn clear(&mut self);
}

macro_rules! impl_bitset {
    ($($type:ty),*) => { $(
        impl BitSet for $type {
            fn get_bit(&self, i: usize) -> bool {
                *self >> i & 1 != 0
            }
            fn set_bit(&mut self, i: usize) -> bool {
                let ret = *self >> i & 1 == 0;
                *self |= 1 << i;
                ret
            }
            fn negate(&mut self) {
                *self = !*self;
            }
            fn clear(&mut self) {
                *self = 0;
            }
        }
    )* };
}

impl_bitset!(i32, i64, i128, isize, u32, u64, u128, usize);

impl BitSet for [u32] {
    fn get_bit(&self, i: usize) -> bool {
        self[i / 32].get_bit(i % 32)
    }
    fn set_bit(&mut self, i: usize) -> bool {
        self[i / 32].set_bit(i % 32)
    }
    fn negate(&mut self) {
        for x in self {
            x.negate()
        }
    }
    fn clear(&mut self) {
        for x in self {
            x.clear();
        }
    }
}

pub fn new_bitset(n: usize) -> Vec<u32> {
    vec![0; (n + 31) / 32]
}
