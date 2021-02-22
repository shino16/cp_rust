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
    path: src/fp.rs
    title: src/fp.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/fp/conv.rs\n"
  code: "pub use super::*;\npub use crate::conv::*;\npub use crate::ds::uvec::*;\n\
    \nmacro_rules! impl_ntt {\n    ($module:ident, $modu:ty, $prim:expr) => {\n  \
    \      pub mod $module {\n            use super::*;\n\n            type FpType\
    \ = Fp<$modu>;\n\n            static mut ROOT: UVec<FpType> = UVec(Vec::new());\n\
    \            static mut INV_ROOT: UVec<FpType> = UVec(Vec::new());\n\n       \
    \     /// reserve for n up to 2^k\n            pub fn reserve(k: usize) {\n  \
    \              unsafe {\n                    if k <= ROOT.len() {\n          \
    \              return;\n                    }\n                    ROOT.resize(k,\
    \ Default::default());\n                    INV_ROOT.resize(k, Default::default());\n\
    \                    let m = FpType::P - 1;\n                    let proot = FpType::new($prim);\n\
    \                    for i in 0..k {\n                        ROOT[i] = -proot.pow((m\
    \ >> (i + 2)) as u64);\n                        INV_ROOT[i] = ROOT[i].inv();\n\
    \                    }\n                }\n            }\n\n            pub fn\
    \ ntt(a: &mut UVec<FpType>) {\n                let n = a.len();\n            \
    \    let mut m = n >> 1;\n                while m != 0 {\n                   \
    \ let mut w = FpType::ONE;\n                    for (k, t) in (0..n).step_by(m\
    \ * 2).zip(1_u32..) {\n                        for i in k..k + m {\n         \
    \                   let u = a[i];\n                            let v = a[i + m]\
    \ * w;\n                            a[i] = u + v;\n                          \
    \  a[i + m] = u - v;\n                        }\n                        w *=\
    \ unsafe { ROOT[t.trailing_zeros() as usize] };\n                    }\n     \
    \               m >>= 1;\n                }\n            }\n\n            pub\
    \ fn inv_ntt(a: &mut UVec<FpType>) {\n                let n = a.len();\n     \
    \           let mut m = 1;\n                while m < n {\n                  \
    \  let mut w = FpType::ONE;\n                    for (k, t) in (0..n).step_by(m\
    \ * 2).zip(1_u32..) {\n                        for i in k..k + m {\n         \
    \                   let u = a[i];\n                            let v = a[i + m];\n\
    \                            a[i] = u + v;\n                            a[i +\
    \ m] = (u - v) * w;\n                        }\n                        w *= unsafe\
    \ { INV_ROOT[t.trailing_zeros() as usize] };\n                    }\n        \
    \            m <<= 1;\n                }\n                let d = FpType::from(n\
    \ as u32).inv();\n                a.iter_mut().for_each(|e| *e *= d);\n      \
    \      }\n\n            pub fn conv<'a, 'b>(a: &'a mut UVec<FpType>, b: &'b mut\
    \ UVec<FpType>) {\n                let len = a.len() + b.len() - 1;\n        \
    \        fn ilog2(n: usize) -> u32 {\n                    std::mem::size_of::<usize>()\
    \ as u32 * 8 - n.leading_zeros() - 1\n                }\n                let n:\
    \ usize = 1 << ilog2(len * 2 - 1);\n                reserve(n.trailing_zeros()\
    \ as usize);\n                a.resize(n, Default::default());\n             \
    \   b.resize(n, Default::default());\n                ntt(a);\n              \
    \  ntt(b);\n                a.iter_mut().zip(b.iter()).for_each(|(a, b)| *a *=\
    \ *b);\n                b.clear();\n                inv_ntt(a);\n            \
    \    a.truncate(len);\n            }\n\n            impl Conv for FpType {\n \
    \               fn conv(mut lhs: Vec<Self>, mut rhs: Vec<Self>) -> Vec<Self> {\n\
    \                    conv(lhs.as_mut(), rhs.as_mut());\n                    lhs\n\
    \                }\n                fn conv_in_place(lhs: &mut Vec<Self>, rhs:\
    \ &mut Vec<Self>) {\n                    conv(lhs.as_mut(), rhs.as_mut());\n \
    \               }\n            }\n        }\n    };\n}\n\nimpl_ntt!(impl_b, ModB,\
    \ 3);\nimpl_ntt!(impl_c, ModC, 5);\nimpl_ntt!(impl_d, ModD, 5);\n\nimpl Conv for\
    \ F17 {\n    fn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>) {\n  \
    \      let r12 = FpC::from(FpB::P).inv();\n        let r13 = FpD::from(FpB::P).inv();\n\
    \        let r23 = FpD::from(FpC::P).inv();\n        fn run<M: Mod>(lhs: &[F17],\
    \ rhs: &[F17]) -> Vec<Fp<M>>\n        where\n            Fp<M>: Conv,\n      \
    \  {\n            let lhs = lhs.iter().map(|&e| Fp::from(e.value())).collect();\n\
    \            let rhs = rhs.iter().map(|&e| Fp::from(e.value())).collect();\n \
    \           Conv::conv(lhs, rhs)\n        }\n        let v1: Vec<FpB> = run(lhs,\
    \ rhs);\n        let v2: Vec<FpC> = run(lhs, rhs);\n        let v3: Vec<FpD> =\
    \ run(lhs, rhs);\n        lhs.resize(v1.len(), Default::default());\n        for\
    \ (((e0, e1), e2), e3) in lhs.iter_mut().zip(v1).zip(v2).zip(v3) {\n         \
    \   let x1 = e1;\n            let x2 = (e2 - x1.value()) * r12;\n            let\
    \ x3 = ((e3 - x1.value()) * r13 - x2.value()) * r23;\n            let mut x =\
    \ FpA::from(x1.value());\n            x += FpA::from(x2.value()) * FpB::P;\n \
    \           x += FpA::from(x3.value()) * FpB::P * FpC::P;\n            *e0 = x.value().into();\n\
    \        }\n    }\n}\n"
  dependsOn:
  - src/conv.rs
  - src/ds.rs
  - src/ds/uvec.rs
  - src/fp.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/fp/conv.rs
  requiredBy:
  - src/u64/conv.rs
  timestamp: '2021-02-20 13:28:01+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_garner_test.rs
  - test/src/bin/ntt_test.rs
documentation_of: src/fp/conv.rs
layout: document
redirect_from:
- /library/src/fp/conv.rs
- /library/src/fp/conv.rs.html
title: src/fp/conv.rs
---
