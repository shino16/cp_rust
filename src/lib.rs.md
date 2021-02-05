---
data:
  _extendedDependsOn: []
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
    RuntimeError: bundler is not specified: src/lib.rs\n"
  code: '#![allow(clippy::many_single_char_names)]

    #![allow(clippy::new_without_default)]

    #![allow(clippy::suspicious_op_assign_impl)]


    pub mod alg;

    pub mod assign;

    pub mod bit;

    pub mod bool;

    pub mod bounded;

    pub mod cast;

    pub mod cmp;

    pub mod conv;

    pub mod dfa;

    pub mod ds;

    pub mod float;

    pub mod fp;

    pub mod func;

    pub mod fxhash;

    pub mod graph;

    pub mod hash;

    pub mod int;

    pub mod io;

    pub mod io_interactive;

    pub mod iter;

    pub mod macros;

    pub mod make_vec;

    pub mod math;

    pub mod mint;

    pub mod num;

    pub mod rand;

    pub mod slice;

    pub mod u64;

    pub mod vec;

    pub mod zo;


    pub mod tests;

    '
  dependsOn: []
  isVerificationFile: false
  path: src/lib.rs
  requiredBy: []
  timestamp: '2021-02-06 00:52:06+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/lib.rs
layout: document
redirect_from:
- /library/src/lib.rs
- /library/src/lib.rs.html
title: src/lib.rs
---
