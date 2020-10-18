use crate::bit::*;

/// op(x, x) == x
pub trait Band {
    type Item: Copy;
    fn unit(&self) -> Self::Item;
    fn op(&self, x: Self::Item, y: Self::Item) -> Self::Item;
}

pub struct BandImpl<T, F>(pub T, pub F);

impl<T: Copy, F: Fn(T, T) -> T> Band for BandImpl<T, F> {
    type Item = T;
    fn unit(&self) -> Self::Item {
        self.0
    }
    fn op(&self, x: Self::Item, y: Self::Item) -> Self::Item {
        (self.1)(x, y)
    }
}

pub struct SparseTable<T, Alg> {
    data: Vec<Vec<T>>,
    alg: Alg,
}

impl<Alg: Band> SparseTable<Alg::Item, Alg> {
    pub fn new(data: Vec<Alg::Item>, alg: Alg) -> Self {
        let len = data.len();
        let height = len.ilog2() as usize;
        let mut data = vec![data];
        for s in 1..=height {
            let w = 1 << (s - 1);
            let mut new_row = Vec::with_capacity(data[s - 1].len() - w);
            for i in 0..data[s - 1].len() - w {
                new_row.push(alg.op(data[s - 1][i], data[s - 1][i + w]));
            }
            data.push(new_row);
        }
        Self { data, alg }
    }
    pub fn ask(&self, l: usize, r: usize) -> Alg::Item {
        if l == r {
            self.alg.unit()
        } else {
            let s = (r - l).ilog2() as usize;
            let w = 1 << s;
            self.alg.op(self.data[s][l], self.data[s][r - w])
        }
    }
}
