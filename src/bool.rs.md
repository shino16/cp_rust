---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/bool.rs\n"
  code: "pub trait Then: Sized {\n\tfn then<T, F: FnOnce() -> T>(self, f: F) -> Option<T>;\n\
    }\n\nimpl Then for bool {\n\tfn then<T, F: FnOnce() -> T>(self, f: F) -> Option<T>\
    \ {\n\t\tif self { Some(f()) } else { None }\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/bool.rs
  requiredBy: []
  timestamp: '2021-01-12 14:31:17+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/bool.rs
layout: document
redirect_from:
- /library/src/bool.rs
- /library/src/bool.rs.html
title: src/bool.rs
---
