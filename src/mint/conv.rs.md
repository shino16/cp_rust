---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':x:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_garner_test.rs
    title: test/src/bin/ntt_garner_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub use super::*;\npub use crate::conv::*;\npub use crate::ds::uvec as _;\n\
    \nmacro_rules! impl_ntt {\n    ($module:ident, $modu:ty, $kth_root:expr, $inv_kth_root:expr)\
    \ => {\n        mod $module {\n            use super::super::super::ds::uvec::*;\n\
    \            use super::super::*;\n            use super::Conv;\n\n          \
    \  type FpType = Mint<$modu>;\n\n            // modu = c * 2^K + 1\n         \
    \   const K: u32 = 20;\n            // 2^K-th root of unity (== g^c where g: primitive\
    \ root)\n            const KTH_ROOT: u32 = $kth_root;\n            const INV_KTH_ROOT:\
    \ u32 = $inv_kth_root;\n\n            static mut ROOT: UVec<FpType> = UVec(Vec::new());\
    \ // [n/2..n): n-th roots\n            static mut INV_ROOT: UVec<FpType> = UVec(Vec::new());\n\
    \            static mut REV: UVec<usize> = UVec(Vec::new()); // bit reversed\n\
    \n            // reserve for n up to 2^k\n            pub fn reserve(k: u32) {\n\
    \                unsafe {\n                    let mut n = 1_usize << k;\n   \
    \                 if n <= ROOT.len() {\n                        return;\n    \
    \                }\n                    REV.resize(n, Default::default());\n \
    \                   for i in 0..n {\n                        REV[i] = (REV[i >>\
    \ 1] >> 1) + ((i & 1) << (k - 1));\n                    }\n\n                \
    \    ROOT.resize(n, Default::default());\n                    INV_ROOT.resize(n,\
    \ Default::default());\n                    let mut w = FpType::from(KTH_ROOT);\n\
    \                    let mut wi = FpType::from(INV_KTH_ROOT);\n              \
    \      for _ in 0..(K - k) {\n                        w *= w;\n              \
    \          wi *= wi;\n                    }\n                    let mut wn =\
    \ FpType::from(-1) * w;\n                    let mut wni = FpType::from(-1) *\
    \ wi;\n                    while n >= 2 {\n                        for i in (n\
    \ / 2..n).rev() {\n                            ROOT[i] = wni;\n              \
    \              INV_ROOT[i] = wn;\n                            wn *= w;\n     \
    \                       wni *= wi;\n                        }\n              \
    \          n /= 2;\n                        w *= w;\n                        wi\
    \ *= wi;\n                        wn = FpType::from(-1) * w;\n               \
    \         wni = FpType::from(-1) * wi;\n                    }\n              \
    \  }\n            }\n\n            fn ntt(a: &mut UVec<FpType>) {\n          \
    \      unsafe {\n                    let n = a.len();\n                    let\
    \ t = ROOT.len().trailing_zeros() - n.trailing_zeros();\n                    for\
    \ i in 0..n {\n                        if i < REV[i] >> t {\n                \
    \            a.swap(i, REV[i] >> t);\n                        }\n            \
    \        }\n\n                    let mut m = 1;\n                    while m\
    \ < n {\n                        for k in (0..n).step_by(m * 2) {\n          \
    \                  for i in 0..m {\n                                let u = a[k\
    \ + i];\n                                let v = a[k + i + m] * ROOT[m + i];\n\
    \                                a[k + i] = u + v;\n                         \
    \       a[k + i + m] = u - v;\n                            }\n               \
    \         }\n                        m <<= 1;\n                    }\n       \
    \         }\n            }\n\n            fn mul<'a, 'b>(\n                a:\
    \ &'a mut UVec<FpType>,\n                b: &'b mut UVec<FpType>,\n          \
    \  ) -> &'a mut UVec<FpType> {\n                let len = a.len() + b.len() -\
    \ 1;\n                let n: usize = len.next_power_of_two();\n              \
    \  reserve(n.trailing_zeros());\n                a.resize(n, Default::default());\n\
    \                b.resize(n, Default::default());\n                ntt(a);\n \
    \               ntt(b);\n                a.iter_mut().zip(b.iter()).for_each(|(a,\
    \ b)| *a *= *b);\n                b.clear();\n                unsafe {\n     \
    \               std::mem::swap(&mut ROOT, &mut INV_ROOT);\n                }\n\
    \                ntt(a);\n                unsafe {\n                    std::mem::swap(&mut\
    \ ROOT, &mut INV_ROOT);\n                }\n                a.truncate(len);\n\
    \                let d = FpType::from(1) / FpType::from(n);\n                for\
    \ a in &mut a[..] {\n                    *a *= d;\n                }\n       \
    \         a\n            }\n\n            impl Conv for FpType {\n           \
    \     fn conv(mut lhs: Vec<Self>, mut rhs: Vec<Self>) -> Vec<Self> {\n       \
    \             mul(lhs.as_mut(), rhs.as_mut());\n                    lhs\n    \
    \            }\n                fn conv_in_place<'a, 'b>(lhs: &'a mut Vec<Self>,\
    \ rhs: &'b mut Vec<Self>) {\n                    mul(lhs.as_mut(), rhs.as_mut());\n\
    \                }\n            }\n        }\n    };\n}\n\nimpl_ntt!(impl_b, ModB,\
    \ 565_042_129, 950_391_366);\nimpl_ntt!(impl_c, ModC, 547_381_916, 603_595_182);\n\
    impl_ntt!(impl_d, ModD, 121_832_176, 323_052_423);\n\nimpl Conv for Mint17 {\n\
    \    fn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>) {\n        let\
    \ r12 = MintC::from(MintB::P).inv();\n        let r13 = MintD::from(MintB::P).inv();\n\
    \        let r23 = MintD::from(MintC::P).inv();\n        fn run<M: Mod>(lhs: &mut\
    \ Vec<Mint17>, rhs: &mut Vec<Mint17>) -> Vec<Mint<M>>\n        where\n       \
    \     Mint<M>: Conv,\n        {\n            let lhs = lhs.iter().map(|&e| Mint::from(e.value())).collect();\n\
    \            let rhs = rhs.iter().map(|&e| Mint::from(e.value())).collect();\n\
    \            Conv::conv(lhs, rhs)\n        }\n        let v1: Vec<MintB> = run(lhs,\
    \ rhs);\n        let v2: Vec<MintC> = run(lhs, rhs);\n        let v3: Vec<MintD>\
    \ = run(lhs, rhs);\n        lhs.resize(v1.len(), Default::default());\n      \
    \  for (((e0, e1), e2), e3) in lhs.iter_mut().zip(v1).zip(v2).zip(v3) {\n    \
    \        let x1 = e1;\n            let x2 = (e2 - x1.value()) * r12;\n       \
    \     let x3 = ((e3 - x1.value()) * r13 - x2.value()) * r23;\n            let\
    \ mut x = MintA::from(x1.value());\n            x += MintA::from(x2.value()) *\
    \ MintB::P;\n            x += MintA::from(x3.value()) * MintB::P * MintC::P;\n\
    \            *e0 = x.value().into();\n        }\n    }\n}\n\nimpl Conv for u64\
    \ {\n    fn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>) {\n      \
    \  let r12 = MintC::from(MintB::P).inv();\n        let r13 = MintD::from(MintB::P).inv();\n\
    \        let r23 = MintD::from(MintC::P).inv();\n        fn run<M: Mod>(lhs: &mut\
    \ Vec<u64>, rhs: &mut Vec<u64>) -> Vec<Mint<M>>\n        where\n            Mint<M>:\
    \ Conv,\n        {\n            let lhs = lhs.iter().map(|&e| Mint::from(e)).collect();\n\
    \            let rhs = rhs.iter().map(|&e| Mint::from(e)).collect();\n       \
    \     Conv::conv(lhs, rhs)\n        }\n        let v1: Vec<MintB> = run(lhs, rhs);\n\
    \        let v2: Vec<MintC> = run(lhs, rhs);\n        let v3: Vec<MintD> = run(lhs,\
    \ rhs);\n        lhs.resize(v1.len(), Default::default());\n        for (((e0,\
    \ e1), e2), e3) in lhs.iter_mut().zip(v1).zip(v2).zip(v3) {\n            let x1\
    \ = e1;\n            let x2 = (e2 - x1.value()) * r12;\n            let x3 = ((e3\
    \ - x1.value()) * r13 - x2.value()) * r23;\n            let mut x = x1.value()\
    \ as u64;\n            x += x2.value() as u64 * MintB::P as u64;\n           \
    \ x += x3.value() as u64 * MintB::P as u64 * MintC::P as u64;\n            *e0\
    \ = x;\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/mint/conv.rs
  requiredBy: []
  timestamp: '2020-11-24 01:55:32+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/cargo_test.rs
  - test/src/bin/dfa_test.rs
  - test/src/bin/ntt_garner_test.rs
documentation_of: src/mint/conv.rs
layout: document
redirect_from:
- /library/src/mint/conv.rs
- /library/src/mint/conv.rs.html
title: src/mint/conv.rs
---
