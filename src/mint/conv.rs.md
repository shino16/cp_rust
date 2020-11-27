---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_garner_test.rs
    title: test/src/bin/ntt_mint_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_test.rs
    title: test/src/bin/ntt_mint_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub use crate::conv::*;\npub use crate::ds::uvec::*;\npub use crate::mint::*;\n\
    \nmacro_rules! impl_ntt {\n    ($module:ident, $modu:ty, $prim:expr) => {\n  \
    \      pub mod $module {\n            use super::*;\n\n            type FpType\
    \ = Mint<$modu>;\n\n            static mut ROOT: UVec<FpType> = UVec(Vec::new());\n\
    \            static mut INV_ROOT: UVec<FpType> = UVec(Vec::new());\n\n       \
    \     // reserve for n up to 2^k\n            pub fn reserve(k: usize) {\n   \
    \             unsafe {\n                    if k <= ROOT.len() { return; }\n \
    \                   ROOT.resize(k, Default::default());\n                    INV_ROOT.resize(k,\
    \ Default::default());\n                    let m = FpType::P - 1;\n         \
    \           let proot = FpType::from($prim);\n                    for i in 0..k\
    \ {\n                        ROOT[i] = -proot.pow(m >> (i + 2));\n           \
    \             INV_ROOT[i] = ROOT[i].inv();\n                    }\n          \
    \      }\n            }\n\n            pub fn ntt(a: &mut UVec<FpType>) {\n  \
    \              let n = a.len();\n                let mut m = n >> 1;\n       \
    \         while m != 0 {\n                    let mut w = FpType::ONE;\n     \
    \               for (k, t) in (0..n).step_by(m * 2).zip(1_u32..) {\n         \
    \               for i in 0..m {\n                            let u = a[k + i];\n\
    \                            let v = a[k + i + m] * w;\n                     \
    \       a[k + i] = u + v;\n                            a[k + i + m] = u - v;\n\
    \                        }\n                        w *= unsafe { ROOT[t.trailing_zeros()\
    \ as usize] };\n                    }\n                    m >>= 1;\n        \
    \        }\n            }\n\n            pub fn inv_ntt(a: &mut UVec<FpType>)\
    \ {\n                let n = a.len();\n                let mut m = 1;\n      \
    \          while m < n {\n                    let mut w = FpType::ONE;\n     \
    \               for (k, t) in (0..n).step_by(m * 2).zip(1_u32..) {\n         \
    \               for i in 0..m {\n                            let u = a[k + i];\n\
    \                            let v = a[k + i + m];\n                         \
    \   a[k + i] = u + v;\n                            a[k + i + m] = (u - v) * w;\n\
    \                        }\n                        w *= unsafe { INV_ROOT[t.trailing_zeros()\
    \ as usize] };\n                    }\n                    m <<= 1;\n        \
    \        }\n                let d = FpType::from(n).inv();\n                a.iter_mut().for_each(|e|\
    \ *e *= d);\n            }\n\n            pub fn conv<'a, 'b>(a: &'a mut UVec<FpType>,\
    \ b: &'b mut UVec<FpType>) {\n                let len = a.len() + b.len() - 1;\n\
    \                fn ilog2(n: usize) -> u32 {\n                    std::mem::size_of::<usize>()\
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
    \ Mint17 {\n\tfn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>) {\n\t\
    \tlet r12 = MintC::from(MintB::P).inv();\n\t\tlet r13 = MintD::from(MintB::P).inv();\n\
    \t\tlet r23 = MintD::from(MintC::P).inv();\n\t\tfn run<M: Mod>(lhs: &mut Vec<Mint17>,\
    \ rhs: &mut Vec<Mint17>) -> Vec<Mint<M>>\n\t\twhere\n\t\t\tMint<M>: Conv,\n\t\t\
    {\n\t\t\tlet lhs = lhs.iter().map(|&e| Mint::from(e.value())).collect();\n\t\t\
    \tlet rhs = rhs.iter().map(|&e| Mint::from(e.value())).collect();\n\t\t\tConv::conv(lhs,\
    \ rhs)\n\t\t}\n\t\tlet v1: Vec<MintB> = run(lhs, rhs);\n\t\tlet v2: Vec<MintC>\
    \ = run(lhs, rhs);\n\t\tlet v3: Vec<MintD> = run(lhs, rhs);\n\t\tlhs.resize(v1.len(),\
    \ Default::default());\n\t\tfor (((e0, e1), e2), e3) in lhs.iter_mut().zip(v1).zip(v2).zip(v3)\
    \ {\n\t\t\tlet x1 = e1;\n\t\t\tlet x2 = (e2 - x1.value()) * r12;\n\t\t\tlet x3\
    \ = ((e3 - x1.value()) * r13 - x2.value()) * r23;\n\t\t\tlet mut x = MintA::from(x1.value());\n\
    \t\t\tx += MintA::from(x2.value()) * MintB::P;\n\t\t\tx += MintA::from(x3.value())\
    \ * MintB::P * MintC::P;\n\t\t\t*e0 = x.value().into();\n\t\t}\n\t}\n}\n\n// impl\
    \ Conv for u64 {\n// \tfn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>)\
    \ {\n// \t\tlet r12 = MintC::from(MintB::P).inv();\n// \t\tlet r13 = MintD::from(MintB::P).inv();\n\
    // \t\tlet r23 = MintD::from(MintC::P).inv();\n// \t\tfn run<M: Mod>(lhs: &mut\
    \ Vec<u64>, rhs: &mut Vec<u64>) -> Vec<Mint<M>>\n// \t\twhere\n// \t\t\tMint<M>:\
    \ Conv,\n// \t\t{\n// \t\t\tlet lhs = lhs.iter().map(|&e| Mint::from(e)).collect();\n\
    // \t\t\tlet rhs = rhs.iter().map(|&e| Mint::from(e)).collect();\n// \t\t\tConv::conv(lhs,\
    \ rhs)\n// \t\t}\n// \t\tlet v1: Vec<MintB> = run(lhs, rhs);\n// \t\tlet v2: Vec<MintC>\
    \ = run(lhs, rhs);\n// \t\tlet v3: Vec<MintD> = run(lhs, rhs);\n// \t\tlhs.resize(v1.len(),\
    \ Default::default());\n// \t\tfor (((e0, e1), e2), e3) in lhs.iter_mut().zip(v1).zip(v2).zip(v3)\
    \ {\n// \t\t\tlet x1 = e1;\n// \t\t\tlet x2 = (e2 - x1.value()) * r12;\n// \t\t\
    \tlet x3 = ((e3 - x1.value()) * r13 - x2.value()) * r23;\n// \t\t\tlet mut x =\
    \ x1.value() as u64;\n// \t\t\tx += x2.value() as u64 * MintB::P as u64;\n// \t\
    \t\tx += x3.value() as u64 * MintB::P as u64 * MintC::P as u64;\n// \t\t\t*e0\
    \ = x;\n// \t\t}\n// \t}\n// }\n"
  dependsOn: []
  isVerificationFile: false
  path: src/mint/conv.rs
  requiredBy: []
  timestamp: '2020-11-27 14:24:44+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/ntt_mint_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/dfa_test.rs
  - test/src/bin/ntt_mint_garner_test.rs
documentation_of: src/mint/conv.rs
layout: document
redirect_from:
- /library/src/mint/conv.rs
- /library/src/mint/conv.rs.html
title: src/mint/conv.rs
---
