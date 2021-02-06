---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/math/gcd.rs\n"
  code: "type Int = i32;\ntype UInt = u64;\n\npub fn gcd(a: Int, b: Int) -> Int {\n\
    \tugcd(a.abs() as _, b.abs() as _) as _\n}\n\n// binary gcd\npub fn ugcd(a: UInt,\
    \ b: UInt) -> UInt {\n\t#[target_feature(enable = \"bmi1\")]\n\tunsafe fn ugcd_impl(mut\
    \ a: UInt, mut b: UInt) -> UInt {\n\t\tif a == 0 {\n\t\t\treturn b;\n\t\t} else\
    \ if b == 0 {\n\t\t\treturn a;\n\t\t}\n\t\tlet a_shift = a.trailing_zeros();\n\
    \t\ta >>= a_shift;\n\t\tlet b_shift = b.trailing_zeros();\n\t\tb >>= b_shift;\n\
    \t\twhile a != b {\n\t\t\tif a > b {\n\t\t\t\tstd::mem::swap(&mut a, &mut b);\n\
    \t\t\t}\n\t\t\tb -= a;\n\t\t\tb >>= b.trailing_zeros();\n\t\t}\n\t\ta << a_shift.min(b_shift)\n\
    \t}\n\tunsafe {\n\t\tugcd_impl(a, b)\n\t}\n}\n\n/// (x, y, g) where ax + by =\
    \ g, x >= 0\npub fn extgcd(mut a: Int, mut b: Int) -> (Int, Int, Int) {\n\t//\
    \ A = [a, x, y; b, u, v], k = [-1; a0; b0]\n\t// A'= [a, x, y; 0, u, v] \\therefore\
    \ a0*u + b0*v = 0\n\tlet (mut x, mut y, mut u, mut v) = (1, 0, 0, 1);\n\twhile\
    \ b != 0 {\n\t\tlet t = a / b;\n\t\ta -= t * b;\n\t\tx -= t * u;\n\t\ty -= t *\
    \ v;\n\t\tstd::mem::swap(&mut a, &mut b);\n\t\tstd::mem::swap(&mut x, &mut u);\n\
    \t\tstd::mem::swap(&mut y, &mut v);\n\t}\n\tif x < 0 {\n\t\tx += u;\n\t\ty -=\
    \ v;\n\t\tdebug_assert_eq!(gcd(u, v), 1);\n\t\tdebug_assert!(x + u >= 0);\n\t\
    }\n\t(x, y, a)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/math/gcd.rs
  requiredBy: []
  timestamp: '2021-02-07 05:27:00+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/gcd.rs
layout: document
redirect_from:
- /library/src/math/gcd.rs
- /library/src/math/gcd.rs.html
title: src/math/gcd.rs
---
