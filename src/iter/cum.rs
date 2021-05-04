use crate::zo::*;

pub trait Cumsum: Sized + Iterator {
    fn cumsum(self) -> Vec<Self::Item>
    where
        Self::Item: Copy + ZeroOne + std::ops::Add<Output = Self::Item>,
    {
        let (lb, _) = self.size_hint();
        let mut res = Vec::with_capacity(lb + 1);
        let mut sum = Self::Item::ZERO;
        res.push(sum);
        for v in self {
            sum = sum + v;
            res.push(sum);
        }
        res
    }
}

impl<T: Sized + Iterator> Cumsum for T {}
