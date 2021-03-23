use super::*;
use crate::complex::*;
pub use crate::conv::*;
use crate::ds::uvec::*;
pub use crate::zo::*;

type Num = Complex<Float>;
static mut ROOTS: UVec<Num> = UVec::new();

/// up to 2^k
pub unsafe fn reserve(k: u32) {
    let n = 1 << k;
    if n * 2 <= ROOTS.len() {
        return;
    }
    ROOTS.resize(n * 2, Default::default());
    ROOTS[1] = Num::ONE;
    let mut i = 1;
    while {
        i <<= 1;
        i < n
    } {
        let u = Num::from_polar(1.0, std::f64::consts::PI / i as f64);
        for j in 0..i / 2 {
            let v = ROOTS[i / 2 + j];
            ROOTS[i + j * 2] = v;
            ROOTS[i + j * 2 + 1] = v * u;
        }
    }
    let i = n;
    let u = Num::from_polar(1.0, std::f64::consts::PI / i as f64);
    let (v, w) = (u * u, u * u * u);
    for j in 0..i / 4 {
        ROOTS[i + j * 4] = ROOTS[i / 4 + j];
        ROOTS[i + j * 4 + 1] = ROOTS[i / 4 + j] * u;
        ROOTS[i + j * 4 + 2] = ROOTS[i / 4 + j] * v;
        ROOTS[i + j * 4 + 3] = ROOTS[i / 4 + j] * w;
    }
}

pub unsafe fn fft(a: &mut UVec<Num>) {
    assert_eq!(a.len() & (a.len() - 1), 0);
    reserve(a.len().trailing_zeros());
    let mut i = a.len();
    while {
        i >>= 1;
        i != 0
    } {
        for j in (0..a.len()).step_by(i * 2) {
            for k in 0..i {
                let (u, v) = (a[j + k], a[i + j + k]);
                a[j + k] = u + v;
                a[i + j + k] = (u - v) / ROOTS[i + k];
            }
        }
    }
}

pub unsafe fn inv_fft(a: &mut UVec<Num>) {
    assert_eq!(a.len() & (a.len() - 1), 0);
    reserve(a.len().trailing_zeros());
    let mut i = 1;
    while i < a.len() {
        for j in (0..a.len()).step_by(i * 2) {
            for k in 0..i {
                let (u, v) = (a[j + k], a[i + j + k] * ROOTS[i + k]);
                a[j + k] = u + v;
                a[i + j + k] = u - v;
            }
        }
        i <<= 1;
    }
}

impl Conv for f64 {
    fn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>) {
        let len = lhs.len() + rhs.len() - 1;
        fn ilog2(n: usize) -> u32 {
            std::mem::size_of::<usize>() as u32 * 8 - n.leading_zeros() - 1
        }
        let n: usize = 1 << ilog2(len * 2 - 1);
        let mut a: Vec<_> = lhs
            .iter()
            .copied()
            .chain(std::iter::repeat(0.0))
            .zip(rhs.iter().copied().chain(std::iter::repeat(0.0)))
            .take(n)
            .map(|(re, im)| Num::new(re, im))
            .collect();
        unsafe {
            fft(a.as_mut());
        }
        let r = Num::new(0.0, -0.25 / n as f64);
        for i in 0..=n / 2 {
            let j = if i == 0 { 0 } else { n - i };
            let u = (a[j] * a[j] - (a[i] * a[i]).conj()) * r;
            let v = (a[i] * a[i] - (a[j] * a[j]).conj()) * r;
            a[i] = u;
            a[j] = v;
        }
        unsafe {
            fft(a.as_mut());
        }
        lhs.clear();
        lhs.extend(a[..len].iter().map(|z| z.re));
    }
}
