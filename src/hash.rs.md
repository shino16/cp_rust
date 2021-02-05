---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/slice.rs
    title: src/slice.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/hash.rs\n"
  code: "use crate::slice::*;\nuse std::collections::HashMap;\nuse std::hash::Hash;\n\
    \npub struct Compress<T: Ord>(Vec<T>);\n\nimpl<T: Ord> Compress<T> {\n\tpub fn\
    \ from(mut data: Vec<T>) -> Self {\n\t\tdata.sort_unstable();\n\t\tdata.dedup();\n\
    \t\tSelf(data)\n\t}\n\tpub fn compress(&self, v: &T) -> usize {\n\t\tdebug_assert!(self.0.binary_search(v).is_ok());\n\
    \t\tlower_bound(&self.0, v)\n\t}\n\tpub fn restore(&self, i: usize) -> &T {\n\t\
    \t&self.0[i]\n\t}\n\tpub fn cache_al(&self) -> HashMap<T, usize>\n\twhere\n\t\t\
    T: Clone + Hash,\n\t{\n\t\tself.0.iter().cloned().zip(0..).collect()\n\t}\n}\n"
  dependsOn:
  - src/slice.rs
  isVerificationFile: false
  path: src/hash.rs
  requiredBy: []
  timestamp: '2021-02-06 00:52:06+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/hash.rs
layout: document
redirect_from:
- /library/src/hash.rs
- /library/src/hash.rs.html
title: src/hash.rs
---
