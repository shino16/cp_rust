---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':heavy_check_mark:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':heavy_check_mark:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':heavy_check_mark:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_beats_test.rs
    title: test/src/bin/segtree_beats_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/int/gcd.rs\n"
  code: "use super::*;\n\npub fn gcd<I: Int>(a: I, b: I) -> I {\n\tugcd(a.abs(), b.abs()).as_()\n\
    }\n\n// binary gcd\npub fn ugcd<I: UInt>(a: I, b: I) -> I {\n\t#[target_feature(enable\
    \ = \"bmi1\")]\n\tunsafe fn ugcd_impl<I: UInt>(mut a: I, mut b: I) -> I {\n\t\t\
    if a.is_zero() {\n\t\t\treturn b;\n\t\t} else if b.is_zero() {\n\t\t\treturn a;\n\
    \t\t}\n\t\tlet a_shift = a.trailing_zeros();\n\t\ta >>= a_shift;\n\t\tlet b_shift\
    \ = b.trailing_zeros();\n\t\tb >>= b_shift;\n\t\twhile a != b {\n\t\t\tif a >\
    \ b {\n\t\t\t\tstd::mem::swap(&mut a, &mut b);\n\t\t\t}\n\t\t\tb -= a;\n\t\t\t\
    b >>= b.trailing_zeros();\n\t\t}\n\t\ta << a_shift.min(b_shift)\n\t}\n\tunsafe\
    \ {\n\t\tugcd_impl(a, b)\n\t}\n}\n\n/// (x, y, g) where ax + by = g, x >= 0\n\
    pub fn extgcd<I: IInt>(mut a: I, mut b: I) -> (I, I, I) {\n\t// A = [a, x, y;\
    \ b, u, v], k = [-1; a0; b0]\n\t// A'= [a, x, y; 0, u, v] \\therefore a0*u + b0*v\
    \ = 0\n\tlet (mut x, mut y, mut u, mut v) = (I::ONE, I::ZERO, I::ZERO, I::ONE);\n\
    \twhile !b.is_zero() {\n\t\tlet t = a / b;\n\t\ta -= t * b;\n\t\tx -= t * u;\n\
    \t\ty -= t * v;\n\t\tstd::mem::swap(&mut a, &mut b);\n\t\tstd::mem::swap(&mut\
    \ x, &mut u);\n\t\tstd::mem::swap(&mut y, &mut v);\n\t}\n\tif x < I::ZERO {\n\t\
    \tx += u;\n\t\ty -= v;\n\t\tdebug_assert_eq!(gcd(u, v), I::ONE);\n\t\tdebug_assert!(x\
    \ + u >= I::ZERO);\n\t}\n\t(x, y, a)\n}\n"
  dependsOn:
  - src/bit.rs
  - src/bounded.rs
  - src/cast.rs
  - src/int.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/int/gcd.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-02-03 21:57:49+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
  - test/src/bin/segtree_beats_test.rs
documentation_of: src/int/gcd.rs
layout: document
redirect_from:
- /library/src/int/gcd.rs
- /library/src/int/gcd.rs.html
title: src/int/gcd.rs
---
