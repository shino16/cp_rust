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
    RuntimeError: bundler is not specified: src/tri.rs\n"
  code: "macro_rules! tri {\n    ($cond:expr; $a:expr; $b:expr) => {\n        if $cond\
    \ { $a } else { $b }\n    };\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/tri.rs
  requiredBy: []
  timestamp: '2021-02-03 06:11:11+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/tri.rs
layout: document
redirect_from:
- /library/src/tri.rs
- /library/src/tri.rs.html
title: src/tri.rs
---
