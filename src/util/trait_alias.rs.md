---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/lib.rs
    title: src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/alg/arith.rs
    title: src/alg/arith.rs
  - icon: ':warning:'
    path: src/complex.rs
    title: src/complex.rs
  - icon: ':heavy_check_mark:'
    path: src/dfa.rs
    title: src/dfa.rs
  - icon: ':warning:'
    path: src/draft/fpacc64.rs
    title: src/draft/fpacc64.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/fwk.rs
    title: src/ds/fwk.rs
  - icon: ':warning:'
    path: src/float/conv.rs
    title: src/float/conv.rs
  - icon: ':warning:'
    path: src/graph/dijkstra.rs
    title: src/graph/dijkstra.rs
  - icon: ':warning:'
    path: src/graph/euler_tour.rs
    title: src/graph/euler_tour.rs
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
  - icon: ':heavy_check_mark:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':warning:'
    path: src/int/arith.rs
    title: src/int/arith.rs
  - icon: ':warning:'
    path: src/int/bisect.rs
    title: src/int/bisect.rs
  - icon: ':heavy_check_mark:'
    path: src/int/gcd.rs
    title: src/int/gcd.rs
  - icon: ':warning:'
    path: src/int/gcd/ext.rs
    title: src/int/gcd/ext.rs
  - icon: ':warning:'
    path: src/int/inv.rs
    title: src/int/inv.rs
  - icon: ':warning:'
    path: src/math/binom.rs
    title: src/math/binom.rs
  - icon: ':warning:'
    path: src/math/pow.rs
    title: src/math/pow.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':warning:'
    path: src/num/field.rs
    title: src/num/field.rs
  - icon: ':warning:'
    path: src/poly.rs
    title: src/poly.rs
  - icon: ':heavy_check_mark:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/edmonds_karp_test.rs
    title: test/src/bin/edmonds_karp_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ford_fulkerson_test.rs
    title: test/src/bin/ford_fulkerson_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/hlpp_test.rs
    title: test/src/bin/hlpp_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/lazy_segtree_test.rs
    title: test/src/bin/lazy_segtree_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_beats_test.rs
    title: test/src/bin/segtree_beats_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/tree_dfs_io_test.rs
    title: test/src/bin/tree_dfs_io_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/util/trait_alias.rs\n"
  code: "pub use crate::trait_alias;\n\n#[macro_export]\nmacro_rules! trait_alias\
    \ {\n    (pub trait $ident:ident = $($t:tt)*) => {\n        pub trait $ident:\
    \ $($t)* {}\n        impl<T: $($t)*> $ident for T {}\n    };\n}\n"
  dependsOn:
  - src/lib.rs
  isVerificationFile: false
  path: src/util/trait_alias.rs
  requiredBy:
  - src/complex.rs
  - src/float/conv.rs
  - src/draft/fpacc64.rs
  - src/ds/fwk.rs
  - src/num/field.rs
  - src/alg/arith.rs
  - src/poly.rs
  - src/tests.rs
  - src/num.rs
  - src/math/pow.rs
  - src/math/binom.rs
  - src/int/arith.rs
  - src/int/inv.rs
  - src/int/gcd/ext.rs
  - src/int/bisect.rs
  - src/int/gcd.rs
  - src/graph/max_flow/hlpp/edge.rs
  - src/graph/max_flow/edmonds_karp.rs
  - src/graph/max_flow/hlpp.rs
  - src/graph/max_flow/ford_fulkerson/edges.rs
  - src/graph/max_flow/ford_fulkerson/edge.rs
  - src/graph/max_flow/ford_fulkerson.rs
  - src/graph/max_flow/edmonds_karp/edge.rs
  - src/graph/dijkstra.rs
  - src/graph/euler_tour.rs
  - src/int.rs
  - src/dfa.rs
  timestamp: '2021-04-10 17:00:13+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/lazy_segtree_test.rs
  - test/src/bin/ford_fulkerson_test.rs
  - test/src/bin/segtree_beats_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/hlpp_test.rs
  - test/src/bin/tree_dfs_io_test.rs
  - test/src/bin/dfa_test.rs
  - test/src/bin/edmonds_karp_test.rs
documentation_of: src/util/trait_alias.rs
layout: document
redirect_from:
- /library/src/util/trait_alias.rs
- /library/src/util/trait_alias.rs.html
title: src/util/trait_alias.rs
---
