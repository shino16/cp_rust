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
    RuntimeError: bundler is not specified: src/math/factorize.rs\n"
  code: "pub fn factorize(mut n: u32) -> Vec<(u32, u32)> {\n    if n == 1 {\n    \
    \    return Vec::new();\n    }\n    if n < 4 {\n        return vec![(n, 1)];\n\
    \    }\n    let mut res = Vec::new();\n    if n % 2 == 0 {\n        let t = n.trailing_zeros();\n\
    \        res.push((2, t));\n        n >>= t;\n    }\n    for d in (3..).step_by(2)\
    \ {\n        if d * d > n {\n            break;\n        }\n        if n % d ==\
    \ 0 {\n            let mut cnt = 1;\n            n /= d;\n            while n\
    \ % d == 0 {\n                cnt += 1;\n                n /= d;\n           \
    \ }\n            res.push((d, cnt));\n        }\n    }\n    if n != 1 {\n    \
    \    res.push((n, 1));\n    }\n    res\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/math/factorize.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/factorize.rs
layout: document
redirect_from:
- /library/src/math/factorize.rs
- /library/src/math/factorize.rs.html
title: src/math/factorize.rs
---
