---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/lib.rs
    title: src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/util/assign_vec.rs\n"
  code: "pub use crate::assign_vec;\n\n#[macro_export]\nmacro_rules! assign_vec {\n\
    \    ($var:ident, [ $($t:tt)* ]) => {\n        $var.clear();\n\n    }\n}"
  dependsOn:
  - src/lib.rs
  isVerificationFile: false
  path: src/util/assign_vec.rs
  requiredBy: []
  timestamp: '2021-03-06 13:39:22+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/util/assign_vec.rs
layout: document
redirect_from:
- /library/src/util/assign_vec.rs
- /library/src/util/assign_vec.rs.html
title: src/util/assign_vec.rs
---
