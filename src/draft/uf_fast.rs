// 2x faster

#[derive(Clone)]
pub struct UnionFind {
    par_or_size: Vec<usize>,
    count: usize,
}

impl UnionFind {
    pub fn new(len: usize) -> Self {
        let par_or_size = vec![1_usize.wrapping_neg(); len];
        Self { par_or_size, count: len }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.par_or_size[x] >> 31 != 0 {
            x
        } else {
            self.par_or_size[x] = self.find(self.par_or_size[x]);
            self.par_or_size[x]
        }
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    pub fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.par_or_size[root].wrapping_neg()
    }
    pub fn unite(&mut self, x: usize, y: usize) -> usize {
        let (mut x, mut y) = (self.find(x), self.find(y));
        if x != y {
            if self.par_or_size[x] > self.par_or_size[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.par_or_size[x] += self.par_or_size[y];
            self.par_or_size[y] = x;
            self.count -= 1;
        }
        x
    }
    pub fn count(&self) -> usize {
        self.count
    }
}
