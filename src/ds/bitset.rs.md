---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/graph/bfs.rs
    title: src/graph/bfs.rs
  - icon: ':warning:'
    path: src/graph/dfs.rs
    title: src/graph/dfs.rs
  - icon: ':x:'
    path: src/graph/dfs_io.rs
    title: src/graph/dfs_io.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/max_flow/ford_fulkerson.rs
    title: src/graph/max_flow/ford_fulkerson.rs
  - icon: ':warning:'
    path: src/graph/max_flow/ford_fulkerson/edge.rs
    title: src/graph/max_flow/ford_fulkerson/edge.rs
  - icon: ':warning:'
    path: src/graph/max_flow/ford_fulkerson/edges.rs
    title: src/graph/max_flow/ford_fulkerson/edges.rs
  - icon: ':warning:'
    path: src/graph/tree/dfs_io.rs
    title: src/graph/tree/dfs_io.rs
  - icon: ':warning:'
    path: src/graph/tree/reroot.rs
    title: src/graph/tree/reroot.rs
  - icon: ':warning:'
    path: src/math/primes.rs
    title: src/math/primes.rs
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/dfs_io_test.rs
    title: test/src/bin/dfs_io_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ford_fulkerson_test.rs
    title: test/src/bin/ford_fulkerson_test.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds/bitset.rs\n"
  code: "pub trait BitSet {\n\tfn get_bit(&self, i: usize) -> bool;\n\tfn set_bit(&mut\
    \ self, i: usize, b: bool);\n\tfn modify_bit(&mut self, i: usize, b: bool) ->\
    \ bool {\n\t\tif self.get_bit(i) == b {\n\t\t\tfalse\n\t\t} else {\n\t\t\tself.set_bit(i,\
    \ b);\n\t\t\ttrue\n\t\t}\n\t}\n\tfn negate(&mut self);\n\tfn reset(&mut self);\n\
    }\n\nmacro_rules! impl_bitset {\n\t($($type:ty),*) => { $(\n\t\timpl BitSet for\
    \ $type {\n\t\t\tfn get_bit(&self, i: usize) -> bool {\n\t\t\t\t((*self >> i)\
    \ & 1) != 0\n\t\t\t}\n\t\t\tfn set_bit(&mut self, i: usize, b: bool) {\n\t\t\t\
    \t*self |= (b as $type) << i;\n\t\t\t}\n\t\t\tfn negate(&mut self) {\n\t\t\t\t\
    *self = !*self;\n\t\t\t}\n\t\t\tfn reset(&mut self) {\n\t\t\t\t*self = 0;\n\t\t\
    \t}\n\t\t}\n\t)* };\n}\n\nimpl_bitset!(i32, i64, i128, isize, u32, u64, u128,\
    \ usize);\n\nimpl BitSet for [u32] {\n\tfn get_bit(&self, i: usize) -> bool {\n\
    \t\tself[i / 32].get_bit(i % 32)\n\t}\n\tfn set_bit(&mut self, i: usize, b: bool)\
    \ {\n\t\tself[i / 32].set_bit(i % 32, b);\n\t}\n\tfn negate(&mut self) {\n\t\t\
    for x in self {\n\t\t\tx.negate()\n\t\t}\n\t}\n\tfn reset(&mut self) {\n\t\tfor\
    \ x in self {\n\t\t\tx.reset();\n\t\t}\n\t}\n}\n\npub fn new_bitset(n: usize)\
    \ -> Vec<u32> {\n\tvec![0; (n + 31) / 32]\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/bitset.rs
  requiredBy:
  - src/graph/dfs.rs
  - src/graph/tree/dfs_io.rs
  - src/graph/tree/reroot.rs
  - src/graph/dfs_io.rs
  - src/graph/bfs.rs
  - src/graph/max_flow/ford_fulkerson.rs
  - src/graph/max_flow/ford_fulkerson/edge.rs
  - src/graph/max_flow/ford_fulkerson/edges.rs
  - src/math/primes.rs
  timestamp: '2021-01-29 12:22:27+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/ford_fulkerson_test.rs
  - test/src/bin/dfs_io_test.rs
documentation_of: src/ds/bitset.rs
layout: document
redirect_from:
- /library/src/ds/bitset.rs
- /library/src/ds/bitset.rs.html
title: src/ds/bitset.rs
---
