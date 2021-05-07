---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/iter/cum.rs\n"
  code: "use crate::zo::*;\n\npub trait Cumsum: Sized + Iterator {\n    fn cumsum(self)\
    \ -> Vec<Self::Item>\n    where\n        Self::Item: Copy + ZeroOne + std::ops::Add<Output\
    \ = Self::Item>,\n    {\n        let (lb, _) = self.size_hint();\n        let\
    \ mut res = Vec::with_capacity(lb + 1);\n        let mut sum = Self::Item::ZERO;\n\
    \        res.push(sum);\n        for v in self {\n            sum = sum + v;\n\
    \            res.push(sum);\n        }\n        res\n    }\n}\n\nimpl<T: Sized\
    \ + Iterator> Cumsum for T {}\n"
  dependsOn:
  - src/zo.rs
  isVerificationFile: false
  path: src/iter/cum.rs
  requiredBy: []
  timestamp: '2021-05-04 17:50:45+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/iter/cum.rs
layout: document
redirect_from:
- /library/src/iter/cum.rs
- /library/src/iter/cum.rs.html
title: src/iter/cum.rs
---
