pub struct Rng32(u32);

impl Rng32 {
    pub fn new() -> Self {
        Rng32(2_463_534_242)
    }
    pub fn gen(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }
}

pub struct Rng64(u64);

impl Rng64 {
    pub fn new() -> Self {
        Rng64(88_172_645_463_325_252)
    }
    pub fn gen(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}
