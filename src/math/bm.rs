use crate::num::field::*;

pub fn berlekamp_massey<T: Field>(s: &[T]) -> Vec<T> {
    let (mut c, mut b) = (vec![T::zero(); s.len()], vec![T::zero(); s.len()]);
    c[0] = T::one();
    b[0] = T::one();
    let mut db = T::one();
    let mut m = 0;
    for n in 0..s.len() {
        m += 1;
        let dc = (0..=n).fold(T::zero(), |d, i| d + c[i] * s[n - i]);
        if dc != T::zero() {
            let k = dc / db;
            for i in (m..s.len()).rev() {
                b[i] = c[i] - k * b[i - m];
            }
            b[..m].copy_from_slice(&c[..m]);
            std::mem::swap(&mut b, &mut c);
            db = dc;
            m = 0;
        }
    }
    while c.last() == Some(&T::zero()) {
        c.pop();
    }
    c.remove(0);
    c.iter_mut().for_each(|c| *c = -*c);
    c
}
