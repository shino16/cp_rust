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
    path: src/int.rs
    title: src/int.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/util/trait_alias.rs
    title: src/util/trait_alias.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/int/gcd.rs\n"
  code: "use super::*;\nuse crate::bits::*;\n\npub mod ext;\n\npub fn gcd<I: Int +\
    \ Bits>(a: I, b: I) -> I\nwhere\n    I::Unsigned: Bits,\n{\n    ugcd(a.abs(),\
    \ b.abs()).as_()\n}\n\n// binary gcd\npub fn ugcd<I: UInt + Bits>(mut a: I, mut\
    \ b: I) -> I {\n    if a.is_zero() {\n        return b;\n    } else if b.is_zero()\
    \ {\n        return a;\n    }\n    let ak = a.trailing_zeros();\n    a >>= ak;\n\
    \    let bk = b.trailing_zeros();\n    b >>= bk;\n    while a != b {\n       \
    \ if a > b {\n            std::mem::swap(&mut a, &mut b);\n        }\n       \
    \ b -= a;\n        b >>= b.trailing_zeros();\n    }\n    a << ak.min(bk)\n}\n"
  dependsOn:
  - src/bits.rs
  - src/bounded.rs
  - src/cast.rs
  - src/int.rs
  - src/num.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/int/gcd.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-05-07 12:42:34+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/segtree_beats_test.rs
  - test/src/bin/cargo_test.rs
documentation_of: src/int/gcd.rs
layout: document
redirect_from:
- /library/src/int/gcd.rs
- /library/src/int/gcd.rs.html
title: src/int/gcd.rs
---
