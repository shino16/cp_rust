---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/conv.rs
    title: src/conv.rs
  - icon: ':heavy_check_mark:'
    path: src/ds.rs
    title: src/ds.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/uvec.rs
    title: src/ds/uvec.rs
  - icon: ':heavy_check_mark:'
    path: src/gf.rs
    title: src/gf.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/u64/conv.rs
    title: src/u64/conv.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_garner_test.rs
    title: test/src/bin/ntt_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_test.rs
    title: test/src/bin/ntt_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/gf/conv.rs\n"
  code: "pub use super::*;\npub use crate::conv::*;\npub use crate::ds::uvec::*;\n\
    \nmacro_rules! impl_ntt {\n    ($module:ident, $modu:ty, $prim:expr) => {\n  \
    \      pub mod $module {\n            use super::*;\n\n            type GfType\
    \ = Gf<$modu>;\n\n            pub fn ntt(a: &mut UVec<GfType>) {\n           \
    \     let n = a.len();\n                let r = GfType::new($prim);\n        \
    \        let roots: Vec<_> = (0..n.trailing_zeros())\n                    .map(|i|\
    \ -r.pow(((GfType::P - 1) >> (i + 2)) as u64))\n                    .collect();\n\
    \                let mut m = n >> 1;\n                while m != 0 {\n       \
    \             let mut w = GfType::ONE;\n                    for (k, t) in (0..n).step_by(m\
    \ * 2).zip(1_u32..) {\n                        for i in k..k + m {\n         \
    \                   let u = a[i];\n                            let v = a[i + m]\
    \ * w;\n                            a[i] = u + v;\n                          \
    \  a[i + m] = u - v;\n                        }\n                        w *=\
    \ roots[t.trailing_zeros() as usize];\n                    }\n               \
    \     m >>= 1;\n                }\n            }\n\n            pub fn inv_ntt(a:\
    \ &mut UVec<GfType>) {\n                let n = a.len();\n                let\
    \ r = GfType::new($prim);\n                let inv_roots: Vec<_> = (0..n.trailing_zeros())\n\
    \                    .map(|i| -r.pow((GfType::P - 1 - ((GfType::P - 1) >> (i +\
    \ 2))) as u64))\n                    .collect();\n                let mut m =\
    \ 1;\n                while m < n {\n                    let mut w = GfType::ONE;\n\
    \                    for (k, t) in (0..n).step_by(m * 2).zip(1_u32..) {\n    \
    \                    for i in k..k + m {\n                            let u =\
    \ a[i];\n                            let v = a[i + m];\n                     \
    \       a[i] = u + v;\n                            a[i + m] = (u - v) * w;\n \
    \                       }\n                        w *= inv_roots[t.trailing_zeros()\
    \ as usize];\n                    }\n                    m <<= 1;\n          \
    \      }\n                let d = GfType::from(n as u32).inv();\n            \
    \    a.iter_mut().for_each(|e| *e *= d);\n            }\n\n            pub fn\
    \ conv<'a, 'b>(a: &'a mut UVec<GfType>, b: &'b mut UVec<GfType>) {\n         \
    \       let len = a.len() + b.len() - 1;\n                fn ilog2(n: usize) ->\
    \ u32 {\n                    std::mem::size_of::<usize>() as u32 * 8 - n.leading_zeros()\
    \ - 1\n                }\n                let n: usize = 1 << ilog2(len * 2 -\
    \ 1);\n                a.resize(n, Default::default());\n                b.resize(n,\
    \ Default::default());\n                ntt(a);\n                ntt(b);\n   \
    \             a.iter_mut().zip(b.iter()).for_each(|(a, b)| *a *= *b);\n      \
    \          b.clear();\n                inv_ntt(a);\n                a.truncate(len);\n\
    \            }\n\n            impl Conv for GfType {\n                fn conv(mut\
    \ lhs: Vec<Self>, mut rhs: Vec<Self>) -> Vec<Self> {\n                    conv(lhs.as_mut(),\
    \ rhs.as_mut());\n                    lhs\n                }\n               \
    \ fn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>) {\n             \
    \       conv(lhs.as_mut(), rhs.as_mut());\n                }\n            }\n\
    \        }\n    };\n}\n\nimpl_ntt!(impl_b, ModB, 3);\nimpl_ntt!(impl_c, ModC,\
    \ 5);\nimpl_ntt!(impl_d, ModD, 5);\n\nimpl Conv for Gf17 {\n    fn conv_in_place(lhs:\
    \ &mut Vec<Self>, rhs: &mut Vec<Self>) {\n        let r12 = GfC::from(GfB::P).inv();\n\
    \        let r13 = GfD::from(GfB::P).inv();\n        let r23 = GfD::from(GfC::P).inv();\n\
    \        fn run<M: Mod>(lhs: &[Gf17], rhs: &[Gf17]) -> Vec<Gf<M>>\n        where\n\
    \            Gf<M>: Conv,\n        {\n            let lhs = lhs.iter().map(|&e|\
    \ Gf::from(e.value())).collect();\n            let rhs = rhs.iter().map(|&e| Gf::from(e.value())).collect();\n\
    \            Conv::conv(lhs, rhs)\n        }\n        let v1: Vec<GfB> = run(lhs,\
    \ rhs);\n        let v2: Vec<GfC> = run(lhs, rhs);\n        let v3: Vec<GfD> =\
    \ run(lhs, rhs);\n        lhs.resize(v1.len(), Default::default());\n        for\
    \ (((e0, e1), e2), e3) in lhs.iter_mut().zip(v1).zip(v2).zip(v3) {\n         \
    \   let x1 = e1;\n            let x2 = (e2 - x1.value()) * r12;\n            let\
    \ x3 = ((e3 - x1.value()) * r13 - x2.value()) * r23;\n            let mut x =\
    \ GfA::from(x1.value());\n            x += GfA::from(x2.value()) * GfB::P;\n \
    \           x += GfA::from(x3.value()) * GfB::P * GfC::P;\n            *e0 = x.value().into();\n\
    \        }\n    }\n}\n"
  dependsOn:
  - src/conv.rs
  - src/ds.rs
  - src/ds/uvec.rs
  - src/gf.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/gf/conv.rs
  requiredBy:
  - src/u64/conv.rs
  timestamp: '2021-03-26 09:38:33+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_garner_test.rs
  - test/src/bin/ntt_test.rs
documentation_of: src/gf/conv.rs
layout: document
redirect_from:
- /library/src/gf/conv.rs
- /library/src/gf/conv.rs.html
title: src/gf/conv.rs
---
