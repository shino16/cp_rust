use crate::alg::*;
use crate::bit::*;

pub struct SparseTable<A: Alg> {
    data: Vec<Vec<A::Item>>,
    alg: A,
}

// A: Band (x * x == x)
impl<A: Monoid> SparseTable<A> {
    pub fn new(data: Vec<A::Item>, alg: A) -> Self {
        let len = data.len();
        let height = len.ilog2() as usize;
        let mut data = vec![data];
        for s in 1..=height {
            let w = 1 << (s - 1);
            let mut new_row = Vec::with_capacity(data[s - 1].len() - w);
            for i in 0..data[s - 1].len() - w {
                new_row.push(alg.op(&data[s - 1][i], &data[s - 1][i + w]));
            }
            data.push(new_row);
        }
        Self { data, alg }
    }
    pub fn ask(&self, l: usize, r: usize) -> A::Item {
        if l == r {
            self.alg.unit()
        } else {
            let s = (r - l).ilog2() as usize;
            let w = 1 << s;
            self.alg.op(&self.data[s][l], &self.data[s][r - w])
        }
    }
}
