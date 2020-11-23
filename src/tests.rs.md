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
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "#[cfg(test)]\nmod tests {\n    mod fp {\n        use crate::fp::*;\n    \
    \    #[test]\n        fn test_pow() {\n            use crate::rng::*;\n      \
    \      let mut rng = Rng32::new();\n            assert_eq!(F17::from(2).pow(3),\
    \ F17::from(8));\n            for _ in 0..100 {\n                let base: F17\
    \ = rng.gen().into();\n                let k = rng.gen() % 100;\n            \
    \    let p = (0..k).map(|_| base).product::<F17>();\n                assert_eq!(p,\
    \ base.pow(k));\n            }\n        }\n        #[test]\n        fn test_inv()\
    \ {\n            use crate::rng::*;\n            let mut rng = Rng32::new();\n\
    \            for _ in 0..100 {\n                let a: F17 = rng.gen().into();\n\
    \                let b = a.inv();\n                assert!(a * b == F17::ONE,\
    \ \"{} {}\", a, b);\n            }\n        }\n    }\n\n    mod fp_naive {\n \
    \       use crate::mint::*;\n        #[test]\n        fn test_mul() {\n      \
    \      use crate::rng::*;\n            let mut rng = Rng32::new();\n         \
    \   for _ in 0..100 {\n                let a = rng.gen() as u64;\n           \
    \     let b = rng.gen() as u64;\n                assert_eq!(Mint17::from(a) *\
    \ b, Mint17::from(a * b));\n            }\n        }\n        #[test]\n      \
    \  fn test_pow() {\n            use crate::rng::*;\n            let mut rng =\
    \ Rng32::new();\n            for _ in 0..100 {\n                let base: Mint17\
    \ = rng.gen().into();\n                let k = rng.gen() % 100;\n            \
    \    let p = (0..k).map(|_| base).product::<Mint17>();\n                assert_eq!(p,\
    \ base.pow(k.into()));\n            }\n        }\n        #[test]\n        fn\
    \ test_inv() {\n            use crate::rng::*;\n            let mut rng = Rng32::new();\n\
    \            for _ in 0..100 {\n                let a: Mint17 = rng.gen().into();\n\
    \                let b = a.inv();\n                assert!(a * b == Mint17::ONE,\
    \ \"{} * {} = {}\", a, b, a * b);\n            }\n        }\n    }\n\n    mod\
    \ iter {\n        use crate::iter::prod::*;\n        use crate::iter::*;\n   \
    \     #[test]\n        fn test() {\n            let lhs = (0..3).prod(b\"ab\"\
    .to_vec()).collect_vec();\n            let rhs = vec![\n                (0, b'a'),\n\
    \                (0, b'b'),\n                (1, b'a'),\n                (1, b'b'),\n\
    \                (2, b'a'),\n                (2, b'b'),\n            ];\n    \
    \        assert_eq!(lhs, rhs);\n        }\n    }\n\n    mod num {\n        use\
    \ crate::num::*;\n        #[test]\n        fn types() {\n            assert_eq!(<i32\
    \ as Int>::Signed::ZERO, 0_i32);\n            assert_eq!(<i32 as Int>::Unsigned::ZERO,\
    \ 0_u32);\n            assert_eq!(<u32 as Int>::Signed::ZERO, 0_i32);\n      \
    \      assert_eq!(<u32 as Int>::Unsigned::ZERO, 0_u32);\n        }\n    }\n\n\
    \    mod make_vec {\n        use crate::make_vec::*;\n        #[test]\n      \
    \  fn test() {\n            let v = make_vec((3, (5, 8)), \"foo\");\n        \
    \    assert_eq!(v, vec![vec![vec![\"foo\"; 8]; 5]; 3]);\n        }\n    }\n\n\
    \    mod math {\n        mod gcd {\n            use crate::math::gcd::*;\n   \
    \         #[test]\n            fn test_gcd() {\n                assert_eq!(gcd(0,\
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
  timestamp: '2020-11-24 01:55:32+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/cargo_test.rs
  - test/src/bin/dfa_test.rs
documentation_of: src/tests.rs
layout: document
redirect_from:
- /library/src/tests.rs
- /library/src/tests.rs.html
title: src/tests.rs
---
