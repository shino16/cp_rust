---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "use crate::int::*;\n\n// binary gcd\npub fn gcd<I: Int>(mut a: I, mut b:\
    \ I) -> I {\n\tif a.is_zero() {\n\t\treturn b;\n\t} else if b.is_zero() {\n\t\t\
    return a;\n\t}\n\tlet a_shift = a.trailing_zeros();\n\ta >>= a_shift;\n\tlet b_shift\
    \ = b.trailing_zeros();\n\tb >>= b_shift;\n\tlet shift = a_shift.min(b_shift);\n\
    \tloop {\n\t\tif a > b {\n\t\t\tstd::mem::swap(&mut a, &mut b);\n\t\t}\n\t\tb\
    \ -= a;\n\t\tif b.is_zero() {\n\t\t\treturn a << shift;\n\t\t}\n\t\tb >>= b.trailing_zeros();\n\
    \t}\n}\n\n// (x, y, g) where ax + by = g\npub fn extgcd<I: IInt>(mut a: I, mut\
    \ b: I) -> (I, I, I) {\n\tlet (mut x, mut y, mut u, mut v) = (I::ONE, I::ZERO,\
    \ I::ZERO, I::ONE);\n\twhile !b.is_zero() {\n\t\tlet t = a / b;\n\t\ta -= t *\
    \ b;\n\t\tx -= t * u;\n\t\ty -= t * v;\n\t\tstd::mem::swap(&mut a, &mut b);\n\t\
    \tstd::mem::swap(&mut x, &mut u);\n\t\tstd::mem::swap(&mut y, &mut v);\n\t}\n\t\
    (x, y, a)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/math/gcd.rs
  requiredBy: []
  timestamp: '2020-12-10 17:35:58+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/math/gcd.rs
layout: document
redirect_from:
- /library/src/math/gcd.rs
- /library/src/math/gcd.rs.html
title: src/math/gcd.rs
---
