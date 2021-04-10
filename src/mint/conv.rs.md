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
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_garner_test.rs
    title: test/src/bin/ntt_mint_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_test.rs
    title: test/src/bin/ntt_mint_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/mint/conv.rs\n"
  code: "pub use super::*;\npub use crate::conv::*;\npub use crate::ds::uvec::*;\n\
    \nmacro_rules! impl_ntt {\n    ($module:ident, $modu:ty, $prim:expr) => {\n  \
    \      pub mod $module {\n            use super::*;\n\n            type Type =\
    \ Mint<$modu>;\n\n            pub fn ntt(a: &mut UVec<Type>) {\n             \
    \   let n = a.len();\n                assert_eq!(n.count_ones(), 1);\n       \
    \         let r = Type::new($prim);\n                let roots: Vec<_> = (0..n.trailing_zeros())\n\
    \                    .map(|i| -r.pow(((Type::M - 1) >> (i + 2)) as u64))\n   \
    \                 .collect();\n                let mut m = n >> 1;\n         \
    \       while m != 0 {\n                    let mut w = Type::ONE;\n         \
    \           for (k, t) in (0..n).step_by(m * 2).zip(1_u32..) {\n             \
    \           for i in k..k + m {\n                            let (u, v) = (a[i],\
    \ a[i + m] * w);\n                            a[i] = u + v;\n                \
    \            a[i + m] = u - v;\n                        }\n                  \
    \      w *= roots[t.trailing_zeros() as usize];\n                    }\n     \
    \               m >>= 1;\n                }\n            }\n\n            pub\
    \ fn inv_ntt(a: &mut UVec<Type>) {\n                let n = a.len();\n       \
    \         assert_eq!(n.count_ones(), 1);\n                let r = Type::new($prim);\n\
    \                let inv_roots: Vec<_> = (0..n.trailing_zeros())\n           \
    \         .map(|i| -r.pow((Type::M - 1 - ((Type::M - 1) >> (i + 2))) as u64))\n\
    \                    .collect();\n                let mut m = 1;\n           \
    \     while m < n {\n                    let mut w = Type::ONE;\n            \
    \        for (k, t) in (0..n).step_by(m * 2).zip(1_u32..) {\n                \
    \        for i in k..k + m {\n                            let u = a[i];\n    \
    \                        let v = a[i + m];\n                            a[i] =\
    \ u + v;\n                            a[i + m] = (u - v) * w;\n              \
    \          }\n                        w *= inv_roots[t.trailing_zeros() as usize];\n\
    \                    }\n                    m <<= 1;\n                }\n    \
    \            let d = Type::from(n as u32).inv();\n                a.iter_mut().for_each(|e|\
    \ *e *= d);\n            }\n\n            pub fn conv(a: &mut UVec<Type>, b: &mut\
    \ UVec<Type>) {\n                let len = a.len() + b.len() - 1;\n          \
    \      let n = len.next_power_of_two();\n                a.resize_with(n, Default::default);\n\
    \                b.resize_with(n, Default::default);\n                ntt(a);\n\
    \                ntt(b);\n                a.iter_mut().zip(b.iter()).for_each(|(a,\
    \ b)| *a *= *b);\n                b.clear();\n                inv_ntt(a);\n  \
    \              a.truncate(len);\n            }\n\n            impl Conv for Type\
    \ {\n                fn conv(mut lhs: Vec<Self>, mut rhs: Vec<Self>) -> Vec<Self>\
    \ {\n                    conv(lhs.as_mut(), rhs.as_mut());\n                 \
    \   lhs\n                }\n                fn conv_in_place(lhs: &mut Vec<Self>,\
    \ rhs: &mut Vec<Self>) {\n                    conv(lhs.as_mut(), rhs.as_mut());\n\
    \                }\n            }\n        }\n    };\n}\n\nimpl_ntt!(impl_b, ModB,\
    \ 3);\nimpl_ntt!(impl_c, ModC, 5);\nimpl_ntt!(impl_d, ModD, 5);\n\nimpl Conv for\
    \ Mint17 {\n    fn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>) {\n\
    \        let r12 = MintC::from(MintB::M).inv();\n        let r13 = MintD::from(MintB::M).inv();\n\
    \        let r23 = MintD::from(MintC::M).inv();\n        fn run<M: Mod>(lhs: &mut\
    \ Vec<Mint17>, rhs: &mut Vec<Mint17>) -> Vec<Mint<M>>\n        where\n       \
    \     Mint<M>: Conv,\n        {\n            let lhs = lhs.iter().map(|&e| Mint::from(e.value())).collect();\n\
    \            let rhs = rhs.iter().map(|&e| Mint::from(e.value())).collect();\n\
    \            Conv::conv(lhs, rhs)\n        }\n        let v1: Vec<MintB> = run(lhs,\
    \ rhs);\n        let v2: Vec<MintC> = run(lhs, rhs);\n        let v3: Vec<MintD>\
    \ = run(lhs, rhs);\n        lhs.resize_with(v1.len(), Default::default);\n   \
    \     for (((e0, e1), e2), e3) in lhs.iter_mut().zip(v1).zip(v2).zip(v3) {\n \
    \           let x1 = e1;\n            let x2 = (e2 - x1.value()) * r12;\n    \
    \        let x3 = ((e3 - x1.value()) * r13 - x2.value()) * r23;\n            let\
    \ mut x = MintA::from(x1.value());\n            x += MintA::from(x2.value()) *\
    \ MintB::M;\n            x += MintA::from(x3.value()) * MintB::M * MintC::M;\n\
    \            *e0 = x.value().into();\n        }\n    }\n}\n"
  dependsOn:
  - src/conv.rs
  - src/ds.rs
  - src/ds/uvec.rs
  - src/mint.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/mint/conv.rs
  requiredBy: []
  timestamp: '2021-04-10 17:00:13+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_mint_garner_test.rs
  - test/src/bin/ntt_mint_test.rs
documentation_of: src/mint/conv.rs
layout: document
redirect_from:
- /library/src/mint/conv.rs
- /library/src/mint/conv.rs.html
title: src/mint/conv.rs
---
