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
    RuntimeError: bundler is not specified: src/draft/iprod.rs\n"
  code: "macro_rules! iprod {\n\t($head:expr) => {\n\t\t$head.into_iter()\n\t};\n\t\
    ($head:expr, $($tail:expr),*) => (\n\t\t$head.into_iter().flat_map(|e| {\n\t\t\
    \tstd::iter::repeat(e).zip(iprod!($($tail),*))\n\t\t})\n\t);\n}"
  dependsOn: []
  isVerificationFile: false
  path: src/draft/iprod.rs
  requiredBy: []
  timestamp: '2021-01-12 01:50:33+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/iprod.rs
layout: document
redirect_from:
- /library/src/draft/iprod.rs
- /library/src/draft/iprod.rs.html
title: src/draft/iprod.rs
---