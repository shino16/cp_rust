---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "#[cfg(test)]\nmod tests {\n    mod fp {\n        use crate::fp::*;\n    \
    \    #[test]\n        fn test_pow() {\n            use crate::rng::*;\n      \
    \      let mut rng = Rng32::new();\n            assert_eq!(Fp17::from(2).pow(3),\
    \ Fp17::from(8));\n            for _ in 0..100 {\n                let base: Fp17\
    \ = rng.gen().into();\n                let k = rng.gen() % 100;\n            \
    \    let p = (0..k).map(|_| base).product::<Fp17>();\n                assert_eq!(p,\
    \ base.pow(k));\n            }\n        }\n        #[test]\n        fn test_inv()\
    \ {\n            use crate::rng::*;\n            let mut rng = Rng32::new();\n\
    \            for _ in 0..100 {\n                let a: Fp17 = rng.gen().into();\n\
    \                let b = a.inv();\n                assert!(a * b == Fp17::ONE,\
    \ \"{} {}\", a, b);\n            }\n        }\n    }\n\n    mod fp_naive {\n \
    \       use crate::modint::*;\n        #[test]\n        fn test_mul() {\n    \
    \        use crate::rng::*;\n            let mut rng = Rng32::new();\n       \
    \     for _ in 0..100 {\n                let a = rng.gen() as u64;\n         \
    \       let b = rng.gen() as u64;\n                assert_eq!(Modint17::from(a)\
    \ * b, Modint17::from(a * b));\n            }\n        }\n        #[test]\n  \
    \      fn test_pow() {\n            use crate::rng::*;\n            let mut rng\
    \ = Rng32::new();\n            for _ in 0..100 {\n                let base: Modint17\
    \ = rng.gen().into();\n                let k = rng.gen() % 100;\n            \
    \    let p = (0..k).map(|_| base).product::<Modint17>();\n                assert_eq!(p,\
    \ base.pow(k.into()));\n            }\n        }\n        #[test]\n        fn\
    \ test_inv() {\n            use crate::rng::*;\n            let mut rng = Rng32::new();\n\
    \            for _ in 0..100 {\n                let a: Modint17 = rng.gen().into();\n\
    \                let b = a.inv();\n                assert!(a * b == Modint17::ONE,\
    \ \"{} * {} = {}\", a, b, a * b);\n            }\n        }\n    }\n\n    mod\
    \ num {\n        use crate::num::*;\n        #[test]\n        fn types() {\n \
    \           assert_eq!(<i32 as Int>::Signed::ZERO, 0_i32);\n            assert_eq!(<i32\
    \ as Int>::Unsigned::ZERO, 0_u32);\n            assert_eq!(<u32 as Int>::Signed::ZERO,\
    \ 0_i32);\n            assert_eq!(<u32 as Int>::Unsigned::ZERO, 0_u32);\n    \
    \    }\n    }\n\n    mod make_vec {\n        use crate::make_vec::*;\n       \
    \ #[test]\n        fn test() {\n            let v = make_vec((3, (5, 8)), \"foo\"\
    );\n            assert_eq!(v, vec![vec![vec![\"foo\"; 8]; 5]; 3]);\n        }\n\
    \    }\n\n    mod math {\n        mod gcd {\n            use crate::math::gcd::*;\n\
    \            #[test]\n            fn test_gcd() {\n                assert_eq!(gcd(0,\
    \ 0), 0);\n                for a in 0..100 {\n                    for b in 1..100\
    \ {\n                        let g = gcd(a, b);\n                        for c\
    \ in g + 1..g {\n                            assert!(a % c != 0 || b % c != 0);\n\
    \                        }\n                        assert_eq!(a % g, 0);\n  \
    \                      assert_eq!(b % g, 0);\n                    }\n        \
    \        }\n            }\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/tests.rs
  requiredBy: []
  timestamp: '2020-11-17 18:39:28+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/tests.rs
layout: document
redirect_from:
- /library/src/tests.rs
- /library/src/tests.rs.html
title: src/tests.rs
---
