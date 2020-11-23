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
  code: "pub use crate::alg::*;\nuse crate::bit::*;\n\n#[derive(Clone)]\npub struct\
    \ DisjointSparseTable<A: Alg> {\n    data: Vec<Vec<A::Item>>,\n    alg: A,\n}\n\
    \nimpl<A: Monoid> DisjointSparseTable<A> {\n    pub fn new(data: Vec<A::Item>,\
    \ alg: A) -> Self {\n        let len = data.len();\n        let height = len.ilog2()\
    \ as usize;\n        let mut data = vec![data; height + 1];\n        for s in\
    \ 1..=height {\n            for z in (0..len).step_by(1 << (s + 1)) {\n      \
    \          let m = z + (1 << s);\n                if m >= len {\n            \
    \        break;\n                }\n                data[s][m - 1] = data[0][m\
    \ - 1].clone();\n                data[s][m] = data[0][m].clone();\n          \
    \      for i in (z..m - 1).rev() {\n                    data[s][i] = alg.op(&data[0][i],\
    \ &data[s][i + 1]);\n                }\n                for i in m + 1..(m + (1\
    \ << s)).min(len) {\n                    data[s][i] = alg.op(&data[s][i - 1],\
    \ &data[0][i]);\n                }\n            }\n        }\n        Self { data,\
    \ alg }\n    }\n    pub fn ask(&self, l: usize, r: usize) -> A::Item {\n     \
    \   if l == r {\n            self.alg.unit()\n        } else if l + 1 == r {\n\
    \            self.data[0][l].clone()\n        } else {\n            let s = (l\
    \ ^ r).ilog2() as usize;\n            self.alg.op(&self.data[s][l], &self.data[s][r])\n\
    \        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/disjointst.rs
  requiredBy: []
  timestamp: '2020-11-17 22:19:43+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - test/src/bin/dfa_test.rs
documentation_of: src/ds/disjointst.rs
layout: document
redirect_from:
- /library/src/ds/disjointst.rs
- /library/src/ds/disjointst.rs.html
title: src/ds/disjointst.rs
---
