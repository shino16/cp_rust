pub use super::*;

pub struct Grid<F: Fn(usize, usize) -> bool> {
    pub h: usize,
    pub w: usize,
    pub wall: F,
    shift: u32,
}

impl<F: Fn(usize, usize) -> bool> Grid<F> {
    pub fn new(h: usize, w: usize, wall: F) -> Self {
        let shift = w.next_power_of_two().trailing_zeros();
        Self { h, w, wall, shift }
    }
    pub fn at(&self, r: usize, c: usize) -> usize {
        (r << self.shift) + c
    }
    pub fn r(&self, v: usize) -> usize {
        v >> self.shift
    }
    pub fn c(&self, v: usize) -> usize {
        v & ((1 << self.shift) - 1)
    }
    pub fn pos(&self, v: usize) -> (usize, usize) {
        (self.r(v), self.c(v))
    }
}

impl<F: Fn(usize, usize) -> bool> Graph for Grid<F> {
    fn len(&self) -> usize {
        self.h << self.shift
    }
    fn adj<G: FnMut(usize)>(&self, v: usize, mut f: G) {
        let (r, c) = self.pos(v);
        const DR: [usize; 4] = [1, !0, 0, 0];
        const DC: [usize; 4] = [0, 0, 1, !0];
        for (&dr, &dc) in DR.iter().zip(&DC) {
            let (r, c) = (r.wrapping_add(dr), c.wrapping_add(dc));
            if r < self.h && c < self.w && !(self.wall)(r, c) {
                f(self.at(r, c));
            }
        }
    }
}
