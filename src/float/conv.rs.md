---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/float/conv.rs\n"
  code: '// use super::*;

    // pub use crate::conv::*;

    // use crate::ds::uvec::*;

    // pub use crate::zo::*;


    // pub mod complex;

    // use complex::*;


    // pub type Num = Complex<Float>;

    // pub const TAU: Float = 6.28318530717958647692528676655900577;


    // pub fn fft(a: &mut UVec<Num>) {

    //     let n = a.len();

    //     assert_eq!(n & (n - 1), 0);

    //     let mut m = n >> 1;

    //     while m != 0 {

    //         let mut w = Num::ONE;

    //         for (k, t) in (0..n).step_by(m * 2).zip(1_u32..) {

    //             for i in k..k + m {

    //                 let (u, v) = (a[i], a[i + m] * w);

    //                 a[i] = u + v;

    //                 a[i + m] = u - v;

    //             }

    //             w *= -Num::from_polar(1.0, TAU / (1 << (t.trailing_zeros() + 2))
    as Float);

    //         }

    //         m >>= 1;

    //     }

    // }


    // pub fn inv_fft(a: &mut UVec<Num>) {

    //     let n = a.len();

    //     assert_eq!(n & (n - 1), 0);

    //     let mut m = 1;

    //     while m < n {

    //         let mut w = Num::ONE;

    //         for (k, t) in (0..n).step_by(m * 2).zip(1_u32..) {

    //             for i in k..k + m {

    //                 let (u, v) = (a[i], a[i + m] * w);

    //                 a[i] = u + v;

    //                 a[i + m] = u - v;

    //             }

    //             w *= -Num::from_polar(1.0, TAU / (1 << (t.trailing_zeros() + 2))
    as Float).conj();

    //         }

    //         m <<= 1;

    //     }

    //     a.iter_mut().for_each(|e| *e /= n as Float);

    // }


    // impl Conv for Float {

    //     fn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>) {

    //         let len = lhs.len() + rhs.len() - 1;

    //         fn ilog2(n: usize) -> u32 {

    //             std::mem::size_of::<usize>() as u32 * 8 - n.leading_zeros() - 1

    //         }

    //         let n = 1 << ilog2(len);

    //         let lhs_iter = lhs.iter().copied().chain(std::iter::repeat(0.0));

    //         let rhs_iter = rhs.iter().copied().chain(std::iter::repeat(0.0));

    //         let mut a: Vec<_> =

    //             lhs_iter.zip(rhs_iter).take(n).map(|(re, im)| Num::new(re, im)).collect();

    //         dbg!(&a);

    //         fft(a.as_mut());

    //         dbg!(&a);

    //         let r = Num::new(0.0, -0.25 / n as Float);

    //         for i in 0..2.min(n) {

    //             a[i] = (a[i] * a[i] - (a[i] * a[i]).conj()) * r;

    //         }

    //         let mut m = 1;

    //         while m * 4 <= n {

    //             for i in m * 2..m * 3 {

    //                 let j = (m * 2 - 1) ^ i;

    //                 let u = (a[j] * a[j] - (a[i] * a[i]).conj()) * r;

    //                 let v = (a[i] * a[i] - (a[j] * a[j]).conj()) * r;

    //                 a[i] = u;

    //                 a[j] = v;

    //             }

    //             m <<= 1;

    //         }

    //         fft(a.as_mut());

    //         lhs.clear();

    //         lhs.extend(a[..len].iter().map(|z| z.re));

    //     }

    // }

    '
  dependsOn: []
  isVerificationFile: false
  path: src/float/conv.rs
  requiredBy: []
  timestamp: '2021-04-03 11:26:56+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/float/conv.rs
layout: document
redirect_from:
- /library/src/float/conv.rs
- /library/src/float/conv.rs.html
title: src/float/conv.rs
---
