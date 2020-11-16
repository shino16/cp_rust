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
  code: "#[derive(Clone)]\npub struct UnionFind {\n    par: Vec<usize>,\n    size:\
    \ Vec<usize>,\n    count: usize,\n}\n\nimpl UnionFind {\n    pub fn new(len: usize)\
    \ -> Self {\n        let par: Vec<_> = (0..len).collect();\n        let size =\
    \ vec![1; len];\n        Self { par, size, count: len }\n    }\n    pub fn find(&mut\
    \ self, x: usize) -> usize {\n        if self.par[x] == x {\n            x\n \
    \       } else {\n            self.par[x] = self.find(self.par[x]);\n        \
    \    self.par[x]\n        }\n    }\n    pub fn is_same(&mut self, x: usize, y:\
    \ usize) -> bool {\n        self.find(x) == self.find(y)\n    }\n    pub fn size(&mut\
    \ self, x: usize) -> usize {\n        let root = self.find(x);\n        self.size[root]\n\
    \    }\n    pub fn unite(&mut self, x: usize, y: usize) {\n        let (mut x,\
    \ mut y) = (self.find(x), self.find(y));\n        if x != y {\n            if\
    \ self.size[x] < self.size[y] {\n                std::mem::swap(&mut x, &mut y);\n\
    \            }\n            self.par[y] = x;\n            self.size[x] += self.size[y];\n\
    \            self.count -= 1;\n        }\n    }\n    pub fn count(&self) -> usize\
    \ {\n        self.count\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/uf.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/uf.rs
layout: document
redirect_from:
- /library/src/ds/uf.rs
- /library/src/ds/uf.rs.html
title: src/ds/uf.rs
---
