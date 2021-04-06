pub use crate::alg::*;
use crate::bits::*;
use crate::ds::uvec::*;

#[derive(Clone)]
pub struct SparseTable<T, M> {
    data: UVec<UVec<T>>,
    alg: M,
}

/// M: Band (x * x == x)
impl<T: Copy, M: Monoid<T>> SparseTable<T, M> {
    pub fn new(data: Vec<T>, alg: M) -> Self {
        let height = data.len().ilog2() as usize;
        let mut data = uvec![UVec(data)];
        for s in 1..=height {
            let w = 1 << (s - 1);
            let mut new_row = UVec(Vec::with_capacity(data[s - 1].len() - w));
            for i in 0..data[s - 1].len() - w {
                new_row.push(alg.op(data[s - 1][i], data[s - 1][i + w]));
            }
            data.push(new_row);
        }
        Self { data, alg }
    }
    pub fn ask(&self, l: usize, r: usize) -> T {
        if l == r {
            self.alg.unit()
        } else {
            let s = (r - l).ilog2() as usize;
            self.alg.op(self.data[s][l], self.data[s][r - (1 << s)])
        }
    }
}
