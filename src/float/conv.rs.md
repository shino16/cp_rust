---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/complex.rs
    title: src/complex.rs
  - icon: ':heavy_check_mark:'
    path: src/conv.rs
    title: src/conv.rs
  - icon: ':heavy_check_mark:'
    path: src/ds.rs
    title: src/ds.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/uvec.rs
    title: src/ds/uvec.rs
  - icon: ':warning:'
    path: src/float.rs
    title: src/float.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/util/trait_alias.rs
    title: src/util/trait_alias.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
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
  code: "use super::*;\nuse crate::complex::*;\npub use crate::conv::*;\nuse crate::ds::uvec::*;\n\
    pub use crate::zo::*;\n\ntype Num = Complex<Float>;\nstatic mut ROOTS: UVec<Num>\
    \ = UVec::new();\n\n/// up to 2^k\npub unsafe fn reserve(k: u32) {\n    let n\
    \ = 1 << k;\n    if n * 2 <= ROOTS.len() {\n        return;\n    }\n    ROOTS.resize(n\
    \ * 2, Default::default());\n    ROOTS[1] = Num::ONE;\n    let mut i = 1;\n  \
    \  while {\n        i <<= 1;\n        i < n\n    } {\n        let u = Num::from_polar(1.0,\
    \ std::f64::consts::PI / i as f64);\n        for j in 0..i / 2 {\n           \
    \ let v = ROOTS[i / 2 + j];\n            ROOTS[i + j * 2] = v;\n            ROOTS[i\
    \ + j * 2 + 1] = v * u;\n        }\n    }\n    let i = n;\n    let u = Num::from_polar(1.0,\
    \ std::f64::consts::PI / i as f64);\n    let (v, w) = (u * u, u * u * u);\n  \
    \  for j in 0..i / 4 {\n        ROOTS[i + j * 4] = ROOTS[i / 4 + j];\n       \
    \ ROOTS[i + j * 4 + 1] = ROOTS[i / 4 + j] * u;\n        ROOTS[i + j * 4 + 2] =\
    \ ROOTS[i / 4 + j] * v;\n        ROOTS[i + j * 4 + 3] = ROOTS[i / 4 + j] * w;\n\
    \    }\n}\n\npub unsafe fn fft(a: &mut UVec<Num>) {\n    assert_eq!(a.len() &\
    \ (a.len() - 1), 0);\n    reserve(a.len().trailing_zeros());\n    let mut i =\
    \ a.len();\n    while {\n        i >>= 1;\n        i != 0\n    } {\n        for\
    \ j in (0..a.len()).step_by(i * 2) {\n            for k in 0..i {\n          \
    \      let (u, v) = (a[j + k], a[i + j + k]);\n                a[j + k] = u +\
    \ v;\n                a[i + j + k] = (u - v) / ROOTS[i + k];\n            }\n\
    \        }\n    }\n}\n\npub unsafe fn inv_fft(a: &mut UVec<Num>) {\n    assert_eq!(a.len()\
    \ & (a.len() - 1), 0);\n    reserve(a.len().trailing_zeros());\n    let mut i\
    \ = 1;\n    while i < a.len() {\n        for j in (0..a.len()).step_by(i * 2)\
    \ {\n            for k in 0..i {\n                let (u, v) = (a[j + k], a[i\
    \ + j + k] * ROOTS[i + k]);\n                a[j + k] = u + v;\n             \
    \   a[i + j + k] = u - v;\n            }\n        }\n        i <<= 1;\n    }\n\
    }\n\nimpl Conv for f64 {\n    fn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut\
    \ Vec<Self>) {\n        let len = lhs.len() + rhs.len() - 1;\n        fn ilog2(n:\
    \ usize) -> u32 {\n            std::mem::size_of::<usize>() as u32 * 8 - n.leading_zeros()\
    \ - 1\n        }\n        let n: usize = 1 << ilog2(len * 2 - 1);\n        let\
    \ mut a: Vec<_> = lhs\n            .iter()\n            .copied()\n          \
    \  .chain(std::iter::repeat(0.0))\n            .zip(rhs.iter().copied().chain(std::iter::repeat(0.0)))\n\
    \            .take(n)\n            .map(|(re, im)| Num::new(re, im))\n       \
    \     .collect();\n        unsafe {\n            fft(a.as_mut());\n        }\n\
    \        let r = Num::new(0.0, -0.25 / n as f64);\n        for i in 0..=n / 2\
    \ {\n            let j = if i == 0 { 0 } else { n - i };\n            let u =\
    \ (a[j] * a[j] - (a[i] * a[i]).conj()) * r;\n            let v = (a[i] * a[i]\
    \ - (a[j] * a[j]).conj()) * r;\n            a[i] = u;\n            a[j] = v;\n\
    \        }\n        unsafe {\n            fft(a.as_mut());\n        }\n      \
    \  lhs.clear();\n        lhs.extend(a[..len].iter().map(|z| z.re));\n    }\n}\n"
  dependsOn:
  - src/complex.rs
  - src/conv.rs
  - src/ds.rs
  - src/ds/uvec.rs
  - src/float.rs
  - src/num.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/float/conv.rs
  requiredBy: []
  timestamp: '2021-03-24 22:49:28+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/float/conv.rs
layout: document
redirect_from:
- /library/src/float/conv.rs
- /library/src/float/conv.rs.html
title: src/float/conv.rs
---
