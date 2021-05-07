---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bits.rs
    title: src/bits.rs
  - icon: ':heavy_check_mark:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':heavy_check_mark:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':heavy_check_mark:'
    path: src/func/memo.rs
    title: src/func/memo.rs
  - icon: ':heavy_check_mark:'
    path: src/gf.rs
    title: src/gf.rs
  - icon: ':heavy_check_mark:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':heavy_check_mark:'
    path: src/int/gcd.rs
    title: src/int/gcd.rs
  - icon: ':heavy_check_mark:'
    path: src/iter.rs
    title: src/iter.rs
  - icon: ':heavy_check_mark:'
    path: src/iter/prod.rs
    title: src/iter/prod.rs
  - icon: ':heavy_check_mark:'
    path: src/make_vec.rs
    title: src/make_vec.rs
  - icon: ':heavy_check_mark:'
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/rand/seed.rs
    title: src/rand/seed.rs
  - icon: ':heavy_check_mark:'
    path: src/rand/xoshiro256plus.rs
    title: src/rand/xoshiro256plus.rs
  - icon: ':heavy_check_mark:'
    path: src/slice/lcp.rs
    title: src/slice/lcp.rs
  - icon: ':heavy_check_mark:'
    path: src/slice/perm.rs
    title: src/slice/perm.rs
  - icon: ':heavy_check_mark:'
    path: src/slice/sa.rs
    title: src/slice/sa.rs
  - icon: ':heavy_check_mark:'
    path: src/slice/sort.rs
    title: src/slice/sort.rs
  - icon: ':heavy_check_mark:'
    path: src/util/trait_alias.rs
    title: src/util/trait_alias.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/tests.rs\n"
  code: "#[cfg(test)]\nmod tests {\n    mod gf {\n        use crate::gf::*;\n    \
    \    #[test]\n        fn test_pow() {\n            use crate::rand::xoshiro256plus::*;\n\
    \            let mut rng = Xoshiro256plus::new();\n            assert_eq!(Gf17::new(2).pow(3),\
    \ Gf17::new(8));\n            for _ in 0..100 {\n                let base: Gf17\
    \ = rng.next().into();\n                let k = rng.next() % 100;\n          \
    \      let p = (0..k).map(|_| base).product::<Gf17>();\n                assert_eq!(p,\
    \ base.pow(k));\n            }\n        }\n        #[test]\n        fn test_inv()\
    \ {\n            use crate::rand::xoshiro256plus::*;\n            let mut rng\
    \ = Xoshiro256plus::new();\n            for _ in 0..100 {\n                let\
    \ a: Gf17 = rng.next().into();\n                let b = a.inv();\n           \
    \     assert!(a * b == Gf17::one(), \"{} {}\", a, b);\n            }\n       \
    \ }\n    }\n\n    mod fp_naive {\n        use crate::mint::*;\n        #[test]\n\
    \        fn test_mul() {\n            use crate::rand::xoshiro256plus::*;\n  \
    \          let mut rng = Xoshiro256plus::new();\n            for _ in 0..100 {\n\
    \                let a = rng.next() as u32 as u64;\n                let b = rng.next()\
    \ as u32 as u64;\n                assert_eq!(Mint17::from(a) * b, Mint17::from(a\
    \ * b));\n            }\n        }\n        #[test]\n        fn test_pow() {\n\
    \            use crate::rand::xoshiro256plus::*;\n            let mut rng = Xoshiro256plus::new();\n\
    \            for _ in 0..100 {\n                let base: Mint17 = rng.next().into();\n\
    \                let k = rng.next() % 100;\n                let p = (0..k).map(|_|\
    \ base).product::<Mint17>();\n                assert_eq!(p, base.pow(k as u64));\n\
    \            }\n        }\n        #[test]\n        fn test_inv() {\n        \
    \    use crate::rand::xoshiro256plus::*;\n            let mut rng = Xoshiro256plus::new();\n\
    \            for _ in 0..100 {\n                let a: Mint17 = rng.next().into();\n\
    \                let b = a.inv();\n                assert!(a * b == Mint17::one(),\
    \ \"{} * {} = {}\", a, b, a * b);\n            }\n        }\n    }\n\n    mod\
    \ func {\n        mod memo {\n            use crate::func::memo::*;\n        \
    \    #[test]\n            fn test_memo() {\n                const MOD: u32 = 1_000_000_007;\n\
    \                let mut fib = vec![1, 1];\n                for i in 2..=1000\
    \ {\n                    let a = fib[i - 1] + fib[i - 2];\n                  \
    \  fib.push(a % MOD);\n                }\n                let rhs = memoize(|fib,\
    \ n| {\n                    if n <= 1 {\n                        1\n         \
    \           } else {\n                        (fib(n - 1) + fib(n - 2)) % MOD\n\
    \                    }\n                })\n                .call(1000);\n   \
    \             assert_eq!(fib[1000], rhs);\n            }\n        }\n    }\n\n\
    \    mod iter {\n        use crate::iter::prod::*;\n        use crate::iter::*;\n\
    \        #[test]\n        fn test() {\n            let lhs = (0..3).prod(b\"ab\"\
    .to_vec()).collect_vec();\n            let rhs = vec![(0, b'a'), (0, b'b'), (1,\
    \ b'a'), (1, b'b'), (2, b'a'), (2, b'b')];\n            assert_eq!(lhs, rhs);\n\
    \        }\n    }\n\n    mod num {\n        use crate::int::*;\n        #[test]\n\
    \        fn types() {\n            assert_eq!(<i32 as Int>::Signed::zero(), 0_i32);\n\
    \            assert_eq!(<i32 as Int>::Unsigned::zero(), 0_u32);\n            assert_eq!(<u32\
    \ as Int>::Signed::zero(), 0_i32);\n            assert_eq!(<u32 as Int>::Unsigned::zero(),\
    \ 0_u32);\n        }\n    }\n\n    mod make_vec {\n        use crate::make_vec::*;\n\
    \        #[test]\n        fn test() {\n            let v = make_vec((3, (5, 8)),\
    \ \"foo\");\n            assert_eq!(v, vec![vec![vec![\"foo\"; 8]; 5]; 3]);\n\
    \        }\n    }\n\n    mod math {\n        mod gcd {\n            use crate::int::gcd::*;\n\
    \            #[test]\n            fn test_gcd() {\n                assert_eq!(gcd(0,\
    \ 0), 0);\n                for a in 0..100 {\n                    for b in 1..100\
    \ {\n                        let g = gcd(a, b);\n                        for c\
    \ in g + 1..g {\n                            assert!(a % c != 0 || b % c != 0);\n\
    \                        }\n                        assert_eq!(a % g, 0);\n  \
    \                      assert_eq!(b % g, 0);\n                    }\n        \
    \        }\n            }\n        }\n    }\n\n    mod slice {\n        mod perm\
    \ {\n            use crate::slice::perm::*;\n            #[test]\n           \
    \ fn test_next_permutation() {\n                let n = 5;\n                let\
    \ mut a: Vec<_> = (0..n).collect();\n                let mut b = a.clone();\n\
    \                let mut cnt = 0;\n                while next_permutation(&mut\
    \ b) {\n                    assert!(a < b);\n                    next_permutation(&mut\
    \ a);\n                    cnt += 1;\n                }\n                assert_eq!(cnt,\
    \ 5 * 4 * 3 * 2 * 1 - 1);\n            }\n        }\n        mod sa {\n      \
    \      use crate::slice::sa::*;\n            use crate::slice::lcp::*;\n     \
    \       #[test]\n            fn suffix_array_lcp_test() {\n                use\
    \ crate::rand::xoshiro256plus::*;\n                let mut rng = Xoshiro256plus::new();\n\
    \                let modu = rng.next() % 1000;\n                let len = rng.next()\
    \ as usize % 1000;\n                let mut v: Vec<_> = std::iter::repeat_with(||\
    \ rng.next() % modu + 1).take(len).collect();\n                let sa = suffix_array(&mut\
    \ v, 0, modu + 1, |x| x as usize);\n                v.extend_from_slice(&[0; 3]);\n\
    \                let mut ans: Vec<_> = (0..=len).collect();\n                ans.sort_unstable_by_key(|&i|\
    \ &v[i..]);\n                assert_eq!(sa, ans);\n                let lcp = lcp_impl(&v,\
    \ &sa, |v| v as usize);\n                for ((&i, &j), lcp) in sa.iter().skip(1).zip(&sa).zip(lcp)\
    \ {\n                    assert_eq!(v[i..i + lcp], v[j..j + lcp]);\n         \
    \           if i.max(j) + lcp < len {\n                        assert_ne!(v[i..i\
    \ + lcp + 1], v[j..j + lcp + 1]);\n                    }\n                }\n\
    \            }\n        }\n        mod sort {\n            use crate::slice::sort::*;\n\
    \            #[test]\n            fn test_count_sort() {\n                use\
    \ crate::rand::xoshiro256plus::*;\n                let mut rng = Xoshiro256plus::new();\n\
    \                let len = rng.next() as usize % 100;\n                let modu\
    \ = rng.next() % len as u64 + 50;\n                let mut a: Vec<_> = std::iter::repeat_with(||\
    \ (rng.next() % modu, rng.next()))\n                    .take(len)\n         \
    \           .collect();\n                let mut b = Vec::new();\n           \
    \     count_sort(&a, &mut b, modu as usize, |x| x.0 as usize);\n             \
    \   a.sort_by_key(|&x| x.0);\n                assert_eq!(a, b);\n            }\n\
    \        }\n    }\n}\n"
  dependsOn:
  - src/bits.rs
  - src/bounded.rs
  - src/cast.rs
  - src/func/memo.rs
  - src/gf.rs
  - src/int.rs
  - src/int/gcd.rs
  - src/iter.rs
  - src/iter/prod.rs
  - src/make_vec.rs
  - src/mint.rs
  - src/num.rs
  - src/rand/seed.rs
  - src/rand/xoshiro256plus.rs
  - src/slice/lcp.rs
  - src/slice/perm.rs
  - src/slice/sa.rs
  - src/slice/sort.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/tests.rs
  requiredBy: []
  timestamp: '2021-05-07 12:42:34+09:00'
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
