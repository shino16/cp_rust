pub struct Rng32(u32);

impl Rng32 {
    pub fn new() -> Self {
        Rng32(2463534242)
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

pub struct Rng64(u64);

impl Rng64 {
    pub fn new() -> Self {
        Rng64(88172645463325252_u64)
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
