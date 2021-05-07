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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/float/conv.rs\n"
  code: "use super::*;\nuse crate::complex::*;\npub use crate::conv::*;\nuse crate::ds::uvec::*;\n\
    pub use crate::zo::*;\n\npub type Num = Complex<Float>;\npub const TAU: Float\
    \ = 6.28318530717958647692528676655900577;\n\npub fn fft(a: &mut UVec<Num>) {\n\
    \    let n = a.len();\n    assert_eq!(n & (n - 1), 0);\n    let mut m = n >> 1;\n\
    \    while m != 0 {\n        let mut w = Num::ONE;\n        for (k, t) in (0..n).step_by(m\
    \ * 2).zip(1_u32..) {\n            for i in k..k + m {\n                let (u,\
    \ v) = (a[i], a[i + m] * w);\n                a[i] = u + v;\n                a[i\
    \ + m] = u - v;\n            }\n            w *= -Num::from_polar(1.0, TAU / (1\
    \ << (t.trailing_zeros() + 2)) as Float);\n        }\n        m >>= 1;\n    }\n\
    }\n\npub fn inv_fft(a: &mut UVec<Num>) {\n    let n = a.len();\n    assert_eq!(n\
    \ & (n - 1), 0);\n    let mut m = 1;\n    while m < n {\n        let mut w = Num::ONE;\n\
    \        for (k, t) in (0..n).step_by(m * 2).zip(1_u32..) {\n            for i\
    \ in k..k + m {\n                let (u, v) = (a[i], a[i + m]);\n            \
    \    a[i] = u + v;\n                a[i + m] = (u - v) * w;\n            }\n \
    \           w *= -Num::from_polar(1.0, TAU / (1 << (t.trailing_zeros() + 2)) as\
    \ Float).conj();\n        }\n        m <<= 1;\n    }\n    a.iter_mut().for_each(|e|\
    \ *e /= n as Float);\n}\n\nimpl Conv for Float {\n    fn conv_in_place(lhs: &mut\
    \ Vec<Self>, rhs: &mut Vec<Self>) {\n        let len = lhs.len() + rhs.len() -\
    \ 1;\n        let n = len.next_power_of_two();\n        let lhs_iter = lhs.iter().copied().chain(std::iter::repeat(0.0));\n\
    \        let rhs_iter = rhs.iter().copied().chain(std::iter::repeat(0.0));\n \
    \       let mut a: Vec<_> =\n            lhs_iter.zip(rhs_iter).take(n).map(|(re,\
    \ im)| Num::new(re, im)).collect();\n        fft(a.as_mut());\n        let r =\
    \ Num::new(0.0, -0.25);\n        for i in 0..2.min(n) {\n            a[i] = Num::from(a[i].re\
    \ * a[i].im);\n        }\n        let mut m = 1;\n        while m * 4 <= n {\n\
    \            for i in m * 2..m * 3 {\n                let j = (m * 2 - 1) ^ i;\n\
    \                let u = (a[j] * a[j] - (a[i] * a[i]).conj()) * r;\n         \
    \       let v = (a[i] * a[i] - (a[j] * a[j]).conj()) * r;\n                a[j]\
    \ = u;\n                a[i] = v;\n            }\n            m <<= 1;\n     \
    \   }\n        inv_fft(a.as_mut());\n        lhs.clear();\n        lhs.extend(a[..len].iter().map(|z|\
    \ z.re));\n    }\n}\n"
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
  timestamp: '2021-04-26 15:43:03+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/float/conv.rs
layout: document
redirect_from:
- /library/src/float/conv.rs
- /library/src/float/conv.rs.html
title: src/float/conv.rs
---
