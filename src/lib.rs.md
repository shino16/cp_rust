---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/ds/pool.rs
    title: src/ds/pool.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/uvec.rs
    title: src/ds/uvec.rs
  - icon: ':heavy_check_mark:'
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':warning:'
    path: src/stdio.rs
    title: src/stdio.rs
  - icon: ':warning:'
    path: src/stdio/buf.rs
    title: src/stdio/buf.rs
  - icon: ':warning:'
    path: src/util/for_loop.rs
    title: src/util/for_loop.rs
  - icon: ':heavy_check_mark:'
    path: src/util/trait_alias.rs
    title: src/util/trait_alias.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/lib.rs\n"
  code: 'pub mod alg;

    pub mod assign;

    pub mod bits;

    pub mod bool;

    pub mod bounded;

    pub mod cast;

    pub mod cmp;

    pub mod comp;

    pub mod complex;

    pub mod conv;

    pub mod dfa;

    pub mod ds;

    pub mod float;

    pub mod func;

    pub mod gf;

    pub mod graph;

    pub mod int;

    pub mod io;

    pub mod io_interactive;

    pub mod iter;

    pub mod make_vec;

    pub mod math;

    pub mod mint;

    pub mod num;

    pub mod perm;

    pub mod poly;

    pub mod rand;

    pub mod slice;

    pub mod stdio;

    pub mod u64;

    pub mod util;

    pub mod vec;

    pub mod zo;


    pub mod tests;

    '
  dependsOn: []
  isVerificationFile: false
  path: src/lib.rs
  requiredBy:
  - src/stdio.rs
  - src/stdio/buf.rs
  - src/ds/pool.rs
  - src/ds/uvec.rs
  - src/util/trait_alias.rs
  - src/util/for_loop.rs
  - src/mint.rs
  timestamp: '2021-05-04 17:50:45+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/lib.rs
layout: document
redirect_from:
- /library/src/lib.rs
- /library/src/lib.rs.html
title: src/lib.rs
---
