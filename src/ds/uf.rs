pub struct UnionFind {
    par: Box<[usize]>,
    size: Box<[usize]>,
}

impl UnionFind {
    pub fn new(len: usize) -> Self {
        let par: Vec<_> = (0..len).collect();
        let size = vec![1; len];
        Self {
            par: par.into_boxed_slice(),
            size: size.into_boxed_slice(),
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.par[x] = self.find(self.par[x]);
            self.par[x]
        }
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    pub fn size(&mut self, x: usize) -> usize {
        self.size[self.find(x)]
    }
    pub fn unite(&mut self, x: usize, y: usize) {
        let (mut x, mut y) = (self.find(x), self.find(y));
        if x != y {
            if self.size[x] < self.size[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.par[y] = x;
            self.size[x] += self.size[y];
        }
    }
}
