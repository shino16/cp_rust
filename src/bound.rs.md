---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/graph/max_flow/edmonds_karp.rs
    title: src/graph/max_flow/edmonds_karp.rs
  - icon: ':warning:'
    path: src/graph/max_flow/edmonds_karp/edge.rs
    title: src/graph/max_flow/edmonds_karp/edge.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/max_flow/ford_fulkerson.rs
    title: src/graph/max_flow/ford_fulkerson.rs
  - icon: ':warning:'
    path: src/graph/max_flow/ford_fulkerson/edge.rs
    title: src/graph/max_flow/ford_fulkerson/edge.rs
  - icon: ':warning:'
    path: src/graph/max_flow/ford_fulkerson/edges.rs
    title: src/graph/max_flow/ford_fulkerson/edges.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/max_flow/hlpp.rs
    title: src/graph/max_flow/hlpp.rs
  - icon: ':warning:'
    path: src/graph/max_flow/hlpp/edge.rs
    title: src/graph/max_flow/hlpp/edge.rs
  - icon: ':warning:'
    path: src/graph/max_flow/push_relabel.rs
    title: src/graph/max_flow/push_relabel.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/edmonds_karp_test.rs
    title: test/src/bin/edmonds_karp_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ford_fulkerson_test.rs
    title: test/src/bin/ford_fulkerson_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/hlpp_test.rs
    title: test/src/bin/hlpp_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/bound.rs\n"
  code: "pub trait Bound: Ord {\n\tconst MIN: Self;\n\tconst MAX: Self;\n}\n\nmacro_rules!\
    \ impl_bound {\n\t($($t:ident),*) => { $(\n\t\timpl Bound for $t {\n\t\t\tconst\
    \ MIN: Self = std::$t::MIN;\n\t\t\tconst MAX: Self = std::$t::MAX;\n\t\t}\n\t\
    )* };\n}\n\nimpl_bound!(i32, i64, i128, isize, u32, u64, u128, usize);\n"
  dependsOn: []
  isVerificationFile: false
  path: src/bound.rs
  requiredBy:
  - src/graph/max_flow/push_relabel.rs
  - src/graph/max_flow/ford_fulkerson.rs
  - src/graph/max_flow/edmonds_karp.rs
  - src/graph/max_flow/hlpp/edge.rs
  - src/graph/max_flow/hlpp.rs
  - src/graph/max_flow/edmonds_karp/edge.rs
  - src/graph/max_flow/ford_fulkerson/edge.rs
  - src/graph/max_flow/ford_fulkerson/edges.rs
  timestamp: '2021-01-29 12:22:27+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/hlpp_test.rs
  - test/src/bin/ford_fulkerson_test.rs
  - test/src/bin/edmonds_karp_test.rs
documentation_of: src/bound.rs
layout: document
redirect_from:
- /library/src/bound.rs
- /library/src/bound.rs.html
title: src/bound.rs
---
