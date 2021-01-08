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
    RuntimeError: bundler is not specified: src/int/bisect.rs\n"
  code: "use super::*;\n\npub fn bisect<I: Int, F: FnMut(I) -> bool>(mut l: I, mut\
    \ r: I, mut pred: F) -> I {\n\twhile l != r {\n\t\tlet mid = (l + r) >> 1;\n\t\
    \tif pred(mid) {\n\t\t\tl = mid + I::ONE;\n\t\t} else {\n\t\t\tr = mid;\n\t\t\
    }\n\t}\n\tr\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/int/bisect.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/int/bisect.rs
layout: document
redirect_from:
- /library/src/int/bisect.rs
- /library/src/int/bisect.rs.html
title: src/int/bisect.rs
---
