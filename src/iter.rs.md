---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub mod either;\npub mod pow;\npub mod prod;\n\npub trait Itertools: Iterator\
    \ {\n\tfn collect_vec(self) -> Vec<Self::Item>\n\twhere\n\t\tSelf: Sized,\n\t\
    {\n\t\tself.collect()\n\t}\n}\n\nimpl<I: Iterator> Itertools for I {}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/iter.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/iter.rs
layout: document
redirect_from:
- /library/src/iter.rs
- /library/src/iter.rs.html
title: src/iter.rs
---
