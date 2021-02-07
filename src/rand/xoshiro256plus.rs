use crate::rand::seed::*;

pub struct Xoshiro256plus([u64; 4]);

impl Xoshiro256plus {
    pub fn new() -> Self {
        Self(seed())
    }
    pub fn next(&mut self) -> u64 {
        let s = &mut self.0;
        let t = s[1] << 17;
        s[2] ^= s[0];
        s[3] ^= s[1];
        s[1] ^= s[2];
        s[0] ^= s[3];
        s[2] ^= t;
        s[3] = s[3].rotate_left(45);
        s[0].wrapping_add(s[3])
    }
    /// skip 2^128 steps
    pub fn split(&mut self) -> Self {
        static JUMP: [u64; 4] =
            [0x180ec6d33cfd0aba, 0xd5a61266f0c9392c, 0xa9582618e03fc9aa, 0x39abdc4529b1661c];
        let mut s2 = [0; 4];
        for &jump in &JUMP {
            for b in 0..64 {
                if (jump >> b) & 1 != 0 {
                    for (s2, s) in s2.iter_mut().zip(&self.0) {
                        *s2 ^= s;
                    }
                }
                self.next();
            }
        }
        Self(s2)
    }
}
