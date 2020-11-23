---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':x:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub mod prod;\n\npub trait Itertools: Iterator {\n    fn collect_vec(self)\
    \ -> Vec<Self::Item>\n    where\n        Self: Sized,\n    {\n        self.collect()\n\
    \    }\n}\n\nimpl<I: Iterator> Itertools for I {}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/iter.rs
  requiredBy: []
  timestamp: '2020-11-17 21:49:18+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/cargo_test.rs
  - test/src/bin/dfa_test.rs
documentation_of: src/iter.rs
layout: document
redirect_from:
- /library/src/iter.rs
- /library/src/iter.rs.html
title: src/iter.rs
---
