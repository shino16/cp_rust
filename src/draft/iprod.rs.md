---
data:
  _extendedDependsOn: []
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
    RuntimeError: bundler is not specified: src/draft/iprod.rs\n"
  code: "macro_rules! iprod {\n    ($head:expr) => {\n        $head.into_iter()\n\
    \    };\n    ($head:expr, $($tail:expr),*) => (\n        $head.into_iter().flat_map(|e|\
    \ {\n            std::iter::repeat(e).zip(iprod!($($tail),*))\n        })\n  \
    \  );\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/draft/iprod.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/iprod.rs
layout: document
redirect_from:
- /library/src/draft/iprod.rs
- /library/src/draft/iprod.rs.html
title: src/draft/iprod.rs
---
