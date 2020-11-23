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
  - icon: ':x:'
    path: test/src/bin/ntt_test.rs
    title: test/src/bin/ntt_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub use crate::ds::uvec as _;\npub use crate::conv::*;\n\nmacro_rules! impl_ntt\
    \ {\n    ($module:ident, $modu:ty, $kth_root:expr, $inv_kth_root:expr) => {\n\
    \        mod $module {\n            use super::super::super::ds::uvec::*;\n  \
    \          use super::super::*;\n            use super::Conv;\n\n            type\
    \ FpType = Fp<$modu>;\n\n            // modu = c * 2^K + 1\n            const\
    \ K: u32 = 20;\n            // 2^K-th root of unity (== g^c where g: primitive\
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
    \            }\n                fn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut\
    \ Vec<Self>) {\n                    mul(lhs.as_mut(), rhs.as_mut());\n       \
    \         }\n            }\n        }\n    };\n}\n\nimpl_ntt!(impl_b, ModB, 565_042_129,\
    \ 950_391_366);\nimpl_ntt!(impl_c, ModC, 547_381_916, 603_595_182);\nimpl_ntt!(impl_d,\
    \ ModD, 121_832_176, 323_052_423);\n"
  dependsOn: []
  isVerificationFile: false
  path: src/fp/conv.rs
  requiredBy: []
  timestamp: '2020-11-24 01:55:32+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/cargo_test.rs
  - test/src/bin/dfa_test.rs
  - test/src/bin/ntt_garner_test.rs
  - test/src/bin/ntt_test.rs
documentation_of: src/fp/conv.rs
layout: document
redirect_from:
- /library/src/fp/conv.rs
- /library/src/fp/conv.rs.html
title: src/fp/conv.rs
---
