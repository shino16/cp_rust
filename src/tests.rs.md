---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':heavy_check_mark:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':heavy_check_mark:'
    path: src/fp.rs
    title: src/fp.rs
  - icon: ':heavy_check_mark:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':heavy_check_mark:'
    path: src/int/gcd.rs
    title: src/int/gcd.rs
  - icon: ':heavy_check_mark:'
    path: src/io.rs
    title: src/io.rs
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
    path: src/slice/perm.rs
    title: src/slice/perm.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/tests.rs\n"
  code: "#[cfg(test)]\nmod tests {\n\tmod fp {\n\t\tuse crate::fp::*;\n\t\t#[test]\n\
    \t\tfn test_pow() {\n\t\t\tuse crate::rand::xoshiro256plus::*;\n\t\t\tlet mut\
    \ rng = Xoshiro256plus::new();\n\t\t\tassert_eq!(F17::new(2).pow(3), F17::new(8));\n\
    \t\t\tfor _ in 0..100 {\n\t\t\t\tlet base: F17 = rng.next().into();\n\t\t\t\t\
    let k = rng.next() % 100;\n\t\t\t\tlet p = (0..k).map(|_| base).product::<F17>();\n\
    \t\t\t\tassert_eq!(p, base.pow(k));\n\t\t\t}\n\t\t}\n\t\t#[test]\n\t\tfn test_inv()\
    \ {\n\t\t\tuse crate::rand::xoshiro256plus::*;\n\t\t\tlet mut rng = Xoshiro256plus::new();\n\
    \t\t\tfor _ in 0..100 {\n\t\t\t\tlet a: F17 = rng.next().into();\n\t\t\t\tlet\
    \ b = a.inv();\n\t\t\t\tassert!(a * b == F17::ONE, \"{} {}\", a, b);\n\t\t\t}\n\
    \t\t}\n\t}\n\n\tmod fp_naive {\n\t\tuse crate::mint::*;\n\t\t#[test]\n\t\tfn test_mul()\
    \ {\n\t\t\tuse crate::rand::xoshiro256plus::*;\n\t\t\tlet mut rng = Xoshiro256plus::new();\n\
    \t\t\tfor _ in 0..100 {\n\t\t\t\tlet a = rng.next() as u32 as u64;\n\t\t\t\tlet\
    \ b = rng.next() as u32 as u64;\n\t\t\t\tassert_eq!(Mint17::from(a) * b, Mint17::from(a\
    \ * b));\n\t\t\t}\n\t\t}\n\t\t#[test]\n\t\tfn test_pow() {\n\t\t\tuse crate::rand::xoshiro256plus::*;\n\
    \t\t\tlet mut rng = Xoshiro256plus::new();\n\t\t\tfor _ in 0..100 {\n\t\t\t\t\
    let base: Mint17 = rng.next().into();\n\t\t\t\tlet k = rng.next() % 100;\n\t\t\
    \t\tlet p = (0..k).map(|_| base).product::<Mint17>();\n\t\t\t\tassert_eq!(p, base.pow(k\
    \ as u32));\n\t\t\t}\n\t\t}\n\t\t#[test]\n\t\tfn test_inv() {\n\t\t\tuse crate::rand::xoshiro256plus::*;\n\
    \t\t\tlet mut rng = Xoshiro256plus::new();\n\t\t\tfor _ in 0..100 {\n\t\t\t\t\
    let a: Mint17 = rng.next().into();\n\t\t\t\tlet b = a.inv();\n\t\t\t\tassert!(a\
    \ * b == Mint17::ONE, \"{} * {} = {}\", a, b, a * b);\n\t\t\t}\n\t\t}\n\t}\n\n\
    \tmod iter {\n\t\tuse crate::iter::prod::*;\n\t\tuse crate::iter::*;\n\t\t#[test]\n\
    \t\tfn test() {\n\t\t\tlet lhs = (0..3).prod(b\"ab\".to_vec()).collect_vec();\n\
    \t\t\tlet rhs = vec![(0, b'a'), (0, b'b'), (1, b'a'), (1, b'b'), (2, b'a'), (2,\
    \ b'b')];\n\t\t\tassert_eq!(lhs, rhs);\n\t\t}\n\t}\n\n\tmod num {\n\t\tuse crate::int::*;\n\
    \t\t#[test]\n\t\tfn types() {\n\t\t\tassert_eq!(<i32 as Int>::Signed::ZERO, 0_i32);\n\
    \t\t\tassert_eq!(<i32 as Int>::Unsigned::ZERO, 0_u32);\n\t\t\tassert_eq!(<u32\
    \ as Int>::Signed::ZERO, 0_i32);\n\t\t\tassert_eq!(<u32 as Int>::Unsigned::ZERO,\
    \ 0_u32);\n\t\t}\n\t}\n\n\tmod make_vec {\n\t\tuse crate::make_vec::*;\n\t\t#[test]\n\
    \t\tfn test() {\n\t\t\tlet v = make_vec((3, (5, 8)), \"foo\");\n\t\t\tassert_eq!(v,\
    \ vec![vec![vec![\"foo\"; 8]; 5]; 3]);\n\t\t}\n\t}\n\n\tmod math {\n\t\tmod gcd\
    \ {\n\t\t\tuse crate::int::gcd::*;\n\t\t\t#[test]\n\t\t\tfn test_gcd() {\n\t\t\
    \t\tassert_eq!(gcd(0, 0), 0);\n\t\t\t\tfor a in 0..100 {\n\t\t\t\t\tfor b in 1..100\
    \ {\n\t\t\t\t\t\tlet g = gcd(a, b);\n\t\t\t\t\t\tfor c in g + 1..g {\n\t\t\t\t\
    \t\t\tassert!(a % c != 0 || b % c != 0);\n\t\t\t\t\t\t}\n\t\t\t\t\t\tassert_eq!(a\
    \ % g, 0);\n\t\t\t\t\t\tassert_eq!(b % g, 0);\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t\
    }\n\t\t}\n\t}\n\n\tmod slice {\n\t\tmod perm {\n\t\t\tuse crate::slice::perm::*;\n\
    \t\t\t#[test]\n\t\t\tfn test_next_permutation() {\n\t\t\t\tlet n = 5;\n\t\t\t\t\
    let mut a: Vec<_> = (0..n).collect();\n\t\t\t\tlet mut b = a.clone();\n\t\t\t\t\
    let mut cnt = 0;\n\t\t\t\twhile next_permutation(&mut b) {\n\t\t\t\t\tassert!(a\
    \ < b);\n\t\t\t\t\tnext_permutation(&mut a);\n\t\t\t\t\tcnt += 1;\n\t\t\t\t}\n\
    \t\t\t\tassert_eq!(cnt, 5 * 4 * 3 * 2 * 1 - 1);\n\t\t\t}\n\t\t}\n\t}\n}\n"
  dependsOn:
  - src/bit.rs
  - src/cast.rs
  - src/fp.rs
  - src/int.rs
  - src/int/gcd.rs
  - src/io.rs
  - src/iter.rs
  - src/iter/prod.rs
  - src/make_vec.rs
  - src/mint.rs
  - src/num.rs
  - src/rand/seed.rs
  - src/rand/xoshiro256plus.rs
  - src/slice/perm.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/tests.rs
  requiredBy: []
  timestamp: '2021-01-27 22:59:31+09:00'
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
