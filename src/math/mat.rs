use crate::num::*;

pub type Mat<T> = Vec<Vec<T>>;

pub fn eye<T: Ring>(n: usize) -> Mat<T> {
    let mut res = vec![vec![T::zero(); n]; n];
    for i in 0..n {
        res[i][i] = T::one();
    }
    res
}

pub fn mat_mul<T: Ring>(a: &Mat<T>, b: &Mat<T>) -> Mat<T> {
    let mut res = vec![vec![T::zero(); b[0].len()]; a.len()];
    for (res, a) in res.iter_mut().zip(a) {
        for (&a, b) in a.iter().zip(b) {
            for (res, &b) in res.iter_mut().zip(b) {
                *res += a * b;
            }
        }
    }
    res
}

pub fn mat_pow<T: Ring>(mut e: Mat<T>, mut k: u64) -> Mat<T> {
    let mut res = eye(e.len());
    while k != 0 {
        if k % 2 != 0 {
            res = mat_mul(&res, &e);
        }
        e = mat_mul(&e, &e);
        k /= 2;
    }
    res
}
