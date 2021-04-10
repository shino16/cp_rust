---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/math/convex.rs\n"
  code: "pub type Int = usize;\n\n/// return (f(x), x) where f(x) is (locally) minimal\n\
    // verification: https://codeforces.com/contest/1479/submission/109458067\npub\
    \ fn convex_min<T: Ord>(\n    mut l: Int,\n    mut r: Int,\n    mut f: impl FnMut(Int)\
    \ -> T,\n) -> (T, Int) {\n    r -= 1;\n    // f(r) < f(r + 1)\n    while l !=\
    \ r {\n        let m = (l + r) / 2;\n        if f(m) < f(m + 1) {\n          \
    \  r = m;\n        } else {\n            l = m + 1;\n        }\n    }\n    (f(l),\
    \ l)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/math/convex.rs
  requiredBy: []
  timestamp: '2021-03-31 15:51:17+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/convex.rs
layout: document
redirect_from:
- /library/src/math/convex.rs
- /library/src/math/convex.rs.html
title: src/math/convex.rs
---
