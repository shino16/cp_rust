---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/union_find_test.rs
    title: test/src/bin/union_find_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds/uf.rs\n"
  code: "#[derive(Clone)]\npub struct UnionFind {\n    par: Vec<usize>,\n    size:\
    \ Vec<usize>,\n    count: usize,\n}\n\nimpl UnionFind {\n    pub fn new(len: usize)\
    \ -> Self {\n        Self { par: vec![!0; len], size: vec![1; len], count: len\
    \ }\n    }\n    pub fn clear(&mut self) {\n        self.par.iter_mut().for_each(|e|\
    \ *e = !0);\n        self.size.iter_mut().for_each(|e| *e = 1);\n        self.count\
    \ = self.len();\n    }\n    pub fn len(&self) -> usize { self.par.len() }\n  \
    \  pub fn find(&mut self, x: usize) -> usize {\n        if self.par[x] == !0 {\n\
    \            x\n        } else {\n            self.par[x] = self.find(self.par[x]);\n\
    \            self.par[x]\n        }\n    }\n    pub fn is_same(&mut self, x: usize,\
    \ y: usize) -> bool { self.find(x) == self.find(y) }\n    pub fn size(&mut self,\
    \ x: usize) -> usize {\n        let root = self.find(x);\n        self.size[root]\n\
    \    }\n    pub fn unite(&mut self, x: usize, y: usize) -> bool {\n        let\
    \ (mut x, mut y) = (self.find(x), self.find(y));\n        if x != y {\n      \
    \      if self.size[x] < self.size[y] {\n                std::mem::swap(&mut x,\
    \ &mut y);\n            }\n            self.par[y] = x;\n            self.size[x]\
    \ += self.size[y];\n            self.count -= 1;\n            true\n        }\
    \ else {\n            false\n        }\n    }\n    pub fn count(&self) -> usize\
    \ { self.count }\n    pub fn push(&mut self) -> usize {\n        let new = self.len();\n\
    \        self.par.push(new);\n        self.size.push(1);\n        new\n    }\n\
    \    pub fn groups(&mut self) -> Vec<Vec<usize>> {\n        let mut groups = vec![Vec::new();\
    \ self.len()];\n        for i in 0..self.len() {\n            groups[self.find(i)].push(i);\n\
    \        }\n        groups.retain(|v| !v.is_empty());\n        groups\n    }\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/uf.rs
  requiredBy: []
  timestamp: '2021-02-20 13:28:01+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/union_find_test.rs
documentation_of: src/ds/uf.rs
layout: document
redirect_from:
- /library/src/ds/uf.rs
- /library/src/ds/uf.rs.html
title: src/ds/uf.rs
---
