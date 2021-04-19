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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds/uf.rs\n"
  code: "#[derive(Clone)]\npub struct UnionFind {\n    par_or_size: Vec<usize>,\n\
    \    count: usize,\n}\n\nimpl UnionFind {\n    pub fn new(len: usize) -> Self\
    \ {\n        let par_or_size = vec![1_usize.wrapping_neg(); len];\n        Self\
    \ { par_or_size, count: len }\n    }\n    pub fn clear(&mut self) {\n        self.par_or_size.iter_mut().for_each(|t|\
    \ *t = 1_usize.wrapping_neg());\n        self.count = self.len();\n    }\n   \
    \ pub fn len(&self) -> usize {\n        self.par_or_size.len()\n    }\n    pub\
    \ fn find(&mut self, x: usize) -> usize {\n        if self.par_or_size[x] >> 31\
    \ != 0 {\n            x\n        } else {\n            self.par_or_size[x] = self.find(self.par_or_size[x]);\n\
    \            self.par_or_size[x]\n        }\n    }\n    pub fn is_same(&mut self,\
    \ x: usize, y: usize) -> bool {\n        self.find(x) == self.find(y)\n    }\n\
    \    pub fn size(&mut self, x: usize) -> usize {\n        let root = self.find(x);\n\
    \        self.par_or_size[root].wrapping_neg()\n    }\n    pub fn unite(&mut self,\
    \ x: usize, y: usize) -> bool {\n        let (mut x, mut y) = (self.find(x), self.find(y));\n\
    \        if x != y {\n            if self.par_or_size[x] > self.par_or_size[y]\
    \ {\n                std::mem::swap(&mut x, &mut y);\n            }\n        \
    \    self.par_or_size[x] = self.par_or_size[x].wrapping_add(self.par_or_size[y]);\n\
    \            self.par_or_size[y] = x;\n            self.count -= 1;\n        \
    \    true\n        } else {\n            false\n        }\n    }\n    pub fn count(&self)\
    \ -> usize {\n        self.count\n    }\n    pub fn groups(&mut self) -> Vec<Vec<usize>>\
    \ {\n        let mut groups = vec![Vec::new(); self.len()];\n        for i in\
    \ 0..self.len() {\n            groups[self.find(i)].push(i);\n        }\n    \
    \    groups.retain(|v| !v.is_empty());\n        groups\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/uf.rs
  requiredBy: []
  timestamp: '2021-04-19 13:13:02+09:00'
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
