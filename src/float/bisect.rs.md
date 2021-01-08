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
    RuntimeError: bundler is not specified: src/float/bisect.rs\n"
  code: "use super::*;\n\npub fn bisect<F: FnMut(Float) -> bool>(\n\tmut l: Float,\n\
    \tmut r: Float,\n\te: Float,\n\tmut pred: F,\n) -> Float {\n\tlet k = ((r - l)\
    \ / e).log2() as u32 + 2;\n\tfor _ in 0..k {\n\t\tlet mid = (l + r) / 2.0;\n\t\
    \tif pred(mid) {\n\t\t\tl = mid;\n\t\t} else {\n\t\t\tr = mid;\n\t\t}\n\t}\n\t\
    r\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/float/bisect.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/float/bisect.rs
layout: document
redirect_from:
- /library/src/float/bisect.rs
- /library/src/float/bisect.rs.html
title: src/float/bisect.rs
---
