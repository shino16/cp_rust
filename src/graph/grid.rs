pub use super::*;

pub struct Grid<F> {
    pub h: usize,
    pub w: usize,
    pub is_edge: F,
    shift: u32,
}

impl<F: Fn((usize, usize), (usize, usize)) -> bool> Grid<F> {
    pub fn new(h: usize, w: usize, is_edge: F) -> Self {
        let shift = w.next_power_of_two().trailing_zeros();
        Self { h, w, is_edge, shift }
    }
    pub fn at(&self, r: usize, c: usize) -> usize { (r << self.shift) + c }
    pub fn r(&self, v: usize) -> usize { v >> self.shift }
    pub fn c(&self, v: usize) -> usize { v & ((1 << self.shift) - 1) }
    pub fn pos(&self, v: usize) -> Option<(usize, usize)> {
        let (r, c) = (self.r(v), self.c(v));
        if r < self.h && c < self.w { Some((r, c)) } else { None }
    }
}

impl<F: Fn((usize, usize), (usize, usize)) -> bool> Graph for Grid<F> {
    fn len(&self) -> usize { self.h << self.shift }
    fn adj(&self, v: usize, mut f: impl FnMut(usize)) {
        if let Some((r, c)) = self.pos(v) {
            const DR: [usize; 4] = [1, !0, 0, 0];
            const DC: [usize; 4] = [0, 0, 1, !0];
            for (&dr, &dc) in DR.iter().zip(&DC) {
                let (r2, c2) = (r.wrapping_add(dr), c.wrapping_add(dc));
                if r2 < self.h && c2 < self.w && (self.is_edge)((r, c), (r2, c2)) {
                    f(self.at(r2, c2));
                }
            }
        }
    }
}
