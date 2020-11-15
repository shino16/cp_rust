---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/lib.rs
    title: src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_test.rs
    title: test/src/bin/ntt_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub use crate::ds::uvec as _;\n\npub trait Conv: Sized {\n    fn conv(lhs:\
    \ Vec<Self>, rhs: Vec<Self>) -> Vec<Self>;\n    fn conv_in_place<'a, 'b>(\n  \
    \      lhs: &'a mut Vec<Self>,\n        rhs: &'b mut Vec<Self>,\n    ) -> &'a\
    \ mut Vec<Self>;\n}\n\nmacro_rules! impl_ntt {\n    ($module:ident, $modu:ty,\
    \ $log2k:expr, $kth_root:expr, $inv_kth_root:expr) => {\n        mod $module {\n\
    \            use super::super::super::ds::uvec::*;\n            use super::super::super::fp::*;\n\
    \            use super::Conv;\n\n            type FpType = Fp<$modu>;\n\n    \
    \        // modu = c * 2^log2k + 1\n            const LOG2K: u32 = $log2k;\n \
    \           // 2^log2k-th root of unity (== g^c where g: primitive root)\n   \
    \         const KTH_ROOT: u32 = $kth_root;\n            const INV_KTH_ROOT: u32\
    \ = $inv_kth_root;\n\n            static mut ROOT: UVec<FpType> = UVec(Vec::new());\
    \ // [n/2..n): n-th roots\n            static mut INV_ROOT: UVec<FpType> = UVec(Vec::new());\n\
    \            static mut REV: UVec<usize> = UVec(Vec::new()); // bit reversed\n\
    \n            fn log2(n: usize) -> u32 {\n                std::mem::size_of::<usize>()\
    \ as u32 * 8 - (n - 1).leading_zeros()\n            }\n\n            // reserve\
    \ for n up to 2^k\n            pub fn reserve(k: u32) {\n                unsafe\
    \ {\n                    let mut n = 1_usize << k;\n                    if n <=\
    \ ROOT.len() {\n                        return;\n                    }\n     \
    \               REV.resize(n, Default::default());\n                    for i\
    \ in 0..n {\n                        REV[i] = (REV[i >> 1] >> 1) + ((i & 1) <<\
    \ (k - 1));\n                    }\n\n                    ROOT.resize(n, Default::default());\n\
    \                    INV_ROOT.resize(n, Default::default());\n               \
    \     let mut w = FpType::from(KTH_ROOT);\n                    let mut wi = FpType::from(INV_KTH_ROOT);\n\
    \                    for _ in 0..(LOG2K - k) {\n                        w *= w;\n\
    \                        wi *= wi;\n                    }\n                  \
    \  let mut wn = FpType::from(-1) * w;\n                    let mut wni = FpType::from(-1)\
    \ * wi;\n                    while n >= 2 {\n                        for i in\
    \ (n / 2..n).rev() {\n                            ROOT[i] = wni;\n           \
    \                 INV_ROOT[i] = wn;\n                            wn *= w;\n  \
    \                          wni *= wi;\n                        }\n           \
    \             n /= 2;\n                        w *= w;\n                     \
    \   wi *= wi;\n                        wn = FpType::from(-1) * w;\n          \
    \              wni = FpType::from(-1) * wi;\n                    }\n         \
    \       }\n            }\n\n            fn ntt(a: &mut UVec<FpType>) {\n     \
    \           unsafe {\n                    let n = a.len();\n                 \
    \   let t = ROOT.len().trailing_zeros() - n.trailing_zeros();\n              \
    \      for i in 0..n {\n                        if i < REV[i] >> t {\n       \
    \                     a.swap(i, REV[i] >> t);\n                        }\n   \
    \                 }\n\n                    let mut m = 1;\n                  \
    \  while m < n {\n                        for k in (0..n).step_by(m * 2) {\n \
    \                           for i in 0..m {\n                                let\
    \ u = a[k + i];\n                                let v = a[k + i + m] * ROOT[m\
    \ + i];\n                                a[k + i] = u + v;\n                 \
    \               a[k + i + m] = u - v;\n                            }\n       \
    \                 }\n                        m <<= 1;\n                    }\n\
    \                }\n            }\n\n            fn mul<'a, 'b>(\n           \
    \     a: &'a mut UVec<FpType>,\n                b: &'b mut UVec<FpType>,\n   \
    \         ) -> &'a mut UVec<FpType> {\n                let len = a.len() + b.len()\
    \ - 1;\n                let n: usize = 1 << log2(len);\n                reserve(n.trailing_zeros());\n\
    \                a.resize(n, Default::default());\n                b.resize(n,\
    \ Default::default());\n                ntt(a);\n                ntt(b);\n   \
    \             a.iter_mut().zip(b.iter()).for_each(|(a, b)| *a *= *b);\n      \
    \          b.clear();\n                unsafe {\n                    std::mem::swap(&mut\
    \ ROOT, &mut INV_ROOT);\n                }\n                ntt(a);\n        \
    \        unsafe {\n                    std::mem::swap(&mut ROOT, &mut INV_ROOT);\n\
    \                }\n                a.truncate(len);\n                let d =\
    \ FpType::from(1) / FpType::from(n);\n                for a in &mut a[..] {\n\
    \                    *a *= d;\n                }\n                a\n        \
    \    }\n\n            impl Conv for FpType {\n                fn conv(mut lhs:\
    \ Vec<Self>, mut rhs: Vec<Self>) -> Vec<Self> {\n                    mul(lhs.as_mut(),\
    \ rhs.as_mut());\n                    lhs\n                }\n               \
    \ fn conv_in_place<'a, 'b>(\n                    lhs: &'a mut Vec<Self>,\n   \
    \                 rhs: &'b mut Vec<Self>,\n                ) -> &'a mut Vec<Self>\
    \ {\n                    mul(lhs.as_mut(), rhs.as_mut())\n                }\n\
    \            }\n        }\n    };\n}\n\nimpl_ntt!(impl99, Mod99, 23, 15_311_432,\
    \ 469_870_224);\n"
  dependsOn: []
  isVerificationFile: false
  path: src/math/ntt.rs
  requiredBy:
  - src/lib.rs
  timestamp: '2020-11-16 03:39:01+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_test.rs
documentation_of: src/math/ntt.rs
layout: document
redirect_from:
- /library/src/math/ntt.rs
- /library/src/math/ntt.rs.html
title: src/math/ntt.rs
---
