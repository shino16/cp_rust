use crate::num::field::*;

pub fn berlekamp_massey<T: Field>(s: &[T]) -> Vec<T> {
    let (mut c, mut b) = (vec![T::ZERO; s.len()], vec![T::ZERO; s.len()]);
    c[0] = T::ONE;
    b[0] = T::ONE;
    let mut db = T::ONE;
    let mut m = 0;
    for n in 0..s.len() {
        m += 1;
        let dc = (0..=n).fold(T::ZERO, |d, i| d + c[i] * s[n - i]);
        if dc != T::ZERO {
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
    while c.last() == Some(&T::ZERO) {
        c.pop();
    }
    c.remove(0);
    c.iter_mut().for_each(|c| *c = -*c);
    c
}
