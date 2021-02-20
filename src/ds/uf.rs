#[derive(Clone)]
pub struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl UnionFind {
    pub fn new(len: usize) -> Self {
        Self { par: vec![!0; len], size: vec![1; len], count: len }
    }
    pub fn clear(&mut self) {
        self.par.iter_mut().for_each(|e| *e = !0);
        self.size.iter_mut().for_each(|e| *e = 1);
        self.count = self.len();
    }
    pub fn len(&self) -> usize { self.par.len() }
    pub fn find(&mut self, x: usize) -> usize {
        if self.par[x] == !0 {
            x
        } else {
            self.par[x] = self.find(self.par[x]);
            self.par[x]
        }
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool { self.find(x) == self.find(y) }
    pub fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let (mut x, mut y) = (self.find(x), self.find(y));
        if x != y {
            if self.size[x] < self.size[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.par[y] = x;
            self.size[x] += self.size[y];
            self.count -= 1;
            true
        } else {
            false
        }
    }
    pub fn count(&self) -> usize { self.count }
    pub fn push(&mut self) -> usize {
        let new = self.len();
        self.par.push(new);
        self.size.push(1);
        new
    }
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut groups = vec![Vec::new(); self.len()];
        for i in 0..self.len() {
            groups[self.find(i)].push(i);
        }
        groups.retain(|v| !v.is_empty());
        groups
    }
}
