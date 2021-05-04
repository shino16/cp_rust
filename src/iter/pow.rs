// pow_iter(0..10, 8): 700 ms (AtC) / 900 ms (CF)
pub fn pow_iter<I: Iterator + Clone>(iter: I, k: usize) -> IterPow<I> {
    IterPow {
        iters: vec![iter.clone(); k],
        iters0: vec![iter; k],
        state: Vec::with_capacity(k),
    }
}

pub struct IterPow<I: Iterator + Clone> {
    iters: Vec<I>,
    iters0: Vec<I>,
    state: Vec<I::Item>,
}

impl<'a, I: Iterator + Clone> IterPow<I>
where
    I::Item: Clone,
{
    pub fn next(&mut self) -> Option<&Vec<I::Item>> {
        if self.state.is_empty() {
            for iter in self.iters.iter_mut() {
                self.state.push(iter.next()?);
            }
            return Some(&self.state);
        }
        for ((iter, iter0), state) in
            self.iters.iter_mut().zip(self.iters0.iter()).zip(self.state.iter_mut())
        {
            if let Some(e) = iter.next() {
                *state = e;
                return Some(&self.state);
            }
            *iter = iter0.clone();
        }
        None
    }
}
