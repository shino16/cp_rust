pub use crate::graph::*;

pub struct Grid {
	pub h: usize,
	pub w: usize,
}

pub struct GridAdj((usize, usize), (usize, usize), usize);
impl Iterator for GridAdj {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
		const DR: [usize; 4] = [!0, 1, 0, 0];
		const DC: [usize; 4] = [0, 0, !0, 1];
		let (r, c) = self.0;
		let (h, w) = self.1;
		let mut i = self.2;
		loop {
			i = i.wrapping_add(1);
			if i >= 4 {
				return None;
			}
			let (r, c) = (r.wrapping_add(DR[i]), c.wrapping_add(DC[i]));
			if r < h && c < w {
				return Some((r, c));
			}
		}
    }
}

impl<'g> Graph<'g> for Grid {
    type V = (usize, usize);
    type E = ();
    type VIter = GridAdj;
    type EIter = std::iter::Take<std::iter::Repeat<()>>;
    fn add_edge(&mut self, _: Self::V, _: Self::V, _: Self::E) {
		panic!()
	}
    fn adj_v<'a: 'g>(&'a self, v: Self::V) -> Self::VIter {
		GridAdj(v, self.dim(), !0)
    }
    fn adj_e<'a: 'g>(&'a self, _: Self::V) -> Self::EIter {
		std::iter::repeat(()).take(4)
    }
    fn dim(&self) -> Self::V {
		(self.h, self.w)
    }
}
