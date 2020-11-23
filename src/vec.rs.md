---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "use std::mem::ManuallyDrop;\n\npub fn transmute_vec<T, U>(v: Vec<T>) -> Vec<U>\
    \ {\n    let mut v = ManuallyDrop::new(v);\n    unsafe { Vec::from_raw_parts(v.as_mut_ptr()\
    \ as *mut _, v.len(), v.capacity()) }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/vec.rs
  requiredBy: []
  timestamp: '2020-11-24 01:55:32+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - test/src/bin/dfa_test.rs
documentation_of: src/vec.rs
layout: document
redirect_from:
- /library/src/vec.rs
- /library/src/vec.rs.html
title: src/vec.rs
---
