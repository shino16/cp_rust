pub struct Xorshift32(u32);

impl Xorshift32 {
    pub fn new() -> Self {
        Self(2_463_534_242)
    }
    pub fn next(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }
}

pub struct Xorshift64(u64);

impl Xorshift64 {
    pub fn new() -> Self {
        Self(88_172_645_463_325_252)
    }
    pub fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}
