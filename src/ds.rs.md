---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/draft/linked_list.rs
    title: src/draft/linked_list.rs
  - icon: ':warning:'
    path: src/draft/linked_list/inner_mut.rs
    title: src/draft/linked_list/inner_mut.rs
  - icon: ':warning:'
    path: src/draft/linked_list/iter.rs
    title: src/draft/linked_list/iter.rs
  - icon: ':warning:'
    path: src/draft/linked_list/ptr.rs
    title: src/draft/linked_list/ptr.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds.rs\n"
  code: 'pub mod bitset;

    pub mod disjointst;

    pub mod fenwick;

    pub mod idx;

    pub mod list;

    pub mod segtree;

    pub mod sparsetable;

    pub mod swag;

    pub mod uf;

    pub mod uslice;

    pub mod uvec;

    pub mod uvec2d;

    pub mod vec2d;

    '
  dependsOn: []
  isVerificationFile: false
  path: src/ds.rs
  requiredBy:
  - src/draft/linked_list/ptr.rs
  - src/draft/linked_list/iter.rs
  - src/draft/linked_list/inner_mut.rs
  - src/draft/linked_list.rs
  timestamp: '2021-02-07 05:27:00+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds.rs
layout: document
redirect_from:
- /library/src/ds.rs
- /library/src/ds.rs.html
title: src/ds.rs
---
