---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/alg/arith.rs
    title: src/alg/arith.rs
  - icon: ':x:'
    path: src/dfa.rs
    title: src/dfa.rs
  - icon: ':warning:'
    path: src/draft/fpacc64.rs
    title: src/draft/fpacc64.rs
  - icon: ':warning:'
    path: src/draft/graph/ds.rs
    title: src/draft/graph/ds.rs
  - icon: ':warning:'
    path: src/draft/graph/graph.rs
    title: src/draft/graph/graph.rs
  - icon: ':warning:'
    path: src/draft/graph/path.rs
    title: src/draft/graph/path.rs
  - icon: ':warning:'
    path: src/ds/fenwick.rs
    title: src/ds/fenwick.rs
  - icon: ':x:'
    path: src/fp.rs
    title: src/fp.rs
  - icon: ':x:'
    path: src/fp/conv.rs
    title: src/fp/conv.rs
  - icon: ':warning:'
    path: src/fp/num.rs
    title: src/fp/num.rs
  - icon: ':warning:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':warning:'
    path: src/graph/bfs.rs
    title: src/graph/bfs.rs
  - icon: ':warning:'
    path: src/graph/dfs.rs
    title: src/graph/dfs.rs
  - icon: ':warning:'
    path: src/graph/dfs_io.rs
    title: src/graph/dfs_io.rs
  - icon: ':warning:'
    path: src/graph/dijkstra.rs
    title: src/graph/dijkstra.rs
  - icon: ':warning:'
    path: src/graph/euler_tour.rs
    title: src/graph/euler_tour.rs
  - icon: ':warning:'
    path: src/graph/grid.rs
    title: src/graph/grid.rs
  - icon: ':warning:'
    path: src/graph/tree.rs
    title: src/graph/tree.rs
  - icon: ':warning:'
    path: src/graph/tree/dfs.rs
    title: src/graph/tree/dfs.rs
  - icon: ':warning:'
    path: src/graph/tree/dfs_io.rs
    title: src/graph/tree/dfs_io.rs
  - icon: ':warning:'
    path: src/graph/tree/reroot.rs
    title: src/graph/tree/reroot.rs
  - icon: ':x:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':warning:'
    path: src/int/arith.rs
    title: src/int/arith.rs
  - icon: ':warning:'
    path: src/int/bisect.rs
    title: src/int/bisect.rs
  - icon: ':x:'
    path: src/int/gcd.rs
    title: src/int/gcd.rs
  - icon: ':warning:'
    path: src/int/inv.rs
    title: src/int/inv.rs
  - icon: ':warning:'
    path: src/math/binom.rs
    title: src/math/binom.rs
  - icon: ':warning:'
    path: src/math/pow.rs
    title: src/math/pow.rs
  - icon: ':question:'
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':heavy_check_mark:'
    path: src/mint/conv.rs
    title: src/mint/conv.rs
  - icon: ':warning:'
    path: src/mint/num.rs
    title: src/mint/num.rs
  - icon: ':question:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':warning:'
    path: src/slice/cum.rs
    title: src/slice/cum.rs
  - icon: ':x:'
    path: src/tests.rs
    title: src/tests.rs
  - icon: ':warning:'
    path: src/u64/conv.rs
    title: src/u64/conv.rs
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':x:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/lazy_segtree_test.rs
    title: test/src/bin/lazy_segtree_test.rs
  - icon: ':x:'
    path: test/src/bin/ntt_garner_test.rs
    title: test/src/bin/ntt_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_garner_test.rs
    title: test/src/bin/ntt_mint_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_test.rs
    title: test/src/bin/ntt_mint_test.rs
  - icon: ':x:'
    path: test/src/bin/ntt_test.rs
    title: test/src/bin/ntt_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/zo.rs\n"
  code: "pub trait ZeroOne: Copy + Eq {\n\tconst ZERO: Self;\n\tfn is_zero(self) ->\
    \ bool {\n\t\tself == Self::ZERO\n\t}\n\tconst ONE: Self;\n}\n\nmacro_rules! impl_zo\
    \ {\n\t($($t:ty),*) => { $(\n\t\timpl ZeroOne for $t {\n\t\t\tconst ZERO: Self\
    \ = 0;\n\t\t\tconst ONE: Self = 1;\n\t\t}\n\t)* };\n}\n\nimpl_zo!(i32, i64, i128,\
    \ isize, u32, u64, u128, usize);\n"
  dependsOn: []
  isVerificationFile: false
  path: src/zo.rs
  requiredBy:
  - src/fp.rs
  - src/slice/cum.rs
  - src/fp/conv.rs
  - src/fp/num.rs
  - src/ds/fenwick.rs
  - src/u64/conv.rs
  - src/tests.rs
  - src/int.rs
  - src/graph/dfs_io.rs
  - src/graph/tree.rs
  - src/graph/dfs.rs
  - src/graph/tree/reroot.rs
  - src/graph/tree/dfs_io.rs
  - src/graph/tree/dfs.rs
  - src/graph/dijkstra.rs
  - src/graph/bfs.rs
  - src/graph/grid.rs
  - src/graph/euler_tour.rs
  - src/graph.rs
  - src/alg/arith.rs
  - src/num.rs
  - src/int/bisect.rs
  - src/int/inv.rs
  - src/int/gcd.rs
  - src/int/arith.rs
  - src/mint/conv.rs
  - src/mint/num.rs
  - src/dfa.rs
  - src/mint.rs
  - src/math/pow.rs
  - src/math/binom.rs
  - src/draft/fpacc64.rs
  - src/draft/graph/ds.rs
  - src/draft/graph/graph.rs
  - src/draft/graph/path.rs
  timestamp: '2020-12-21 16:49:17+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/dfa_test.rs
  - test/src/bin/ntt_test.rs
  - test/src/bin/ntt_mint_garner_test.rs
  - test/src/bin/ntt_mint_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/ntt_garner_test.rs
  - test/src/bin/lazy_segtree_test.rs
documentation_of: src/zo.rs
layout: document
redirect_from:
- /library/src/zo.rs
- /library/src/zo.rs.html
title: src/zo.rs
---
