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
  code: "use crate::slice::*;\n\npub struct Hash<T>(Vec<T>);\n\nimpl<T: Copy + Ord>\
    \ Hash<T> {\n\tpub fn from(mut data: Vec<T>) -> Self {\n\t\tdata.sort_unstable();\n\
    \t\tdata.dedup();\n\t\tSelf(data)\n\t}\n\tpub fn run(&self, v: &T) -> usize {\n\
    \t\tdebug_assert!(self.0.binary_search(v).is_ok());\n\t\tself.0.lower_bound(v)\n\
    \t}\n\tpub fn restore(&self, i: usize) -> &T {\n\t\t&self.0[i]\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/hash.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/hash.rs
layout: document
redirect_from:
- /library/src/hash.rs
- /library/src/hash.rs.html
title: src/hash.rs
---
