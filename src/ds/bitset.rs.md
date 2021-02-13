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
  - icon: ':warning:'
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
    path: src/math/primes.rs
    title: src/math/primes.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ford_fulkerson_test.rs
    title: test/src/bin/ford_fulkerson_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds/bitset.rs\n"
  code: "pub trait BitSet {\n    fn get_bit(&self, i: usize) -> bool;\n    fn set_bit(&mut\
    \ self, i: usize, b: bool);\n    fn modify_bit(&mut self, i: usize, b: bool) ->\
    \ bool {\n        if self.get_bit(i) == b {\n            false\n        } else\
    \ {\n            self.set_bit(i, b);\n            true\n        }\n    }\n   \
    \ fn negate(&mut self);\n    fn reset(&mut self);\n}\n\nmacro_rules! impl_bitset\
    \ {\n    ($($type:ty),*) => { $(\n        impl BitSet for $type {\n          \
    \  fn get_bit(&self, i: usize) -> bool {\n                ((*self >> i) & 1) !=\
    \ 0\n            }\n            fn set_bit(&mut self, i: usize, b: bool) {\n \
    \               *self |= (b as $type) << i;\n            }\n            fn negate(&mut\
    \ self) {\n                *self = !*self;\n            }\n            fn reset(&mut\
    \ self) {\n                *self = 0;\n            }\n        }\n    )* };\n}\n\
    \nimpl_bitset!(i32, i64, i128, isize, u32, u64, u128, usize);\n\nimpl BitSet for\
    \ [u32] {\n    fn get_bit(&self, i: usize) -> bool {\n        self[i / 32].get_bit(i\
    \ % 32)\n    }\n    fn set_bit(&mut self, i: usize, b: bool) {\n        self[i\
    \ / 32].set_bit(i % 32, b);\n    }\n    fn negate(&mut self) {\n        for x\
    \ in self {\n            x.negate()\n        }\n    }\n    fn reset(&mut self)\
    \ {\n        for x in self {\n            x.reset();\n        }\n    }\n}\n\n\
    pub fn new_bitset(n: usize) -> Vec<u32> {\n    vec![0; (n + 31) / 32]\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/bitset.rs
  requiredBy:
  - src/math/primes.rs
  - src/graph/bfs.rs
  - src/graph/dfs.rs
  - src/graph/max_flow/ford_fulkerson/edge.rs
  - src/graph/max_flow/ford_fulkerson/edges.rs
  - src/graph/max_flow/ford_fulkerson.rs
  - src/graph/dfs_io.rs
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ford_fulkerson_test.rs
documentation_of: src/ds/bitset.rs
layout: document
redirect_from:
- /library/src/ds/bitset.rs
- /library/src/ds/bitset.rs.html
title: src/ds/bitset.rs
---
