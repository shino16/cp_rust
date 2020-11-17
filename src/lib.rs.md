---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: '#![allow(clippy::many_single_char_names)]

    #![allow(clippy::new_without_default)]

    #![allow(clippy::suspicious_op_assign_impl)]


    pub mod alg;

    pub mod as_int;

    pub mod bit;

    pub mod bitset;

    pub mod cmp;

    pub mod dfa;

    pub mod ds;

    pub mod fp;

    pub mod hash;

    pub mod io;

    pub mod io_interactive;

    pub mod make_vec;

    pub mod math;

    pub mod modint;

    pub mod num;

    pub mod rng;

    pub mod slice;


    pub mod tests;

    '
  dependsOn: []
  isVerificationFile: false
  path: src/lib.rs
  requiredBy: []
  timestamp: '2020-11-17 18:39:28+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/lib.rs
layout: document
redirect_from:
- /library/src/lib.rs
- /library/src/lib.rs.html
title: src/lib.rs
---
