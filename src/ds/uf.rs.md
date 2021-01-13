---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/union_find_test.rs
    title: test/src/bin/union_find_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds/uf.rs\n"
  code: "#[derive(Clone)]\npub struct UnionFind {\n\tpar: Vec<usize>,\n\tsize: Vec<usize>,\n\
    \tcount: usize,\n}\n\nimpl UnionFind {\n\tpub fn new(len: usize) -> Self {\n\t\
    \tlet par: Vec<_> = (0..len).collect();\n\t\tlet size = vec![1; len];\n\t\tSelf\
    \ { par, size, count: len }\n\t}\n\tpub fn len(&self) -> usize {\n\t\tself.par.len()\n\
    \t}\n\tpub fn find(&mut self, x: usize) -> usize {\n\t\tif self.par[x] == x {\n\
    \t\t\tx\n\t\t} else {\n\t\t\tself.par[x] = self.find(self.par[x]);\n\t\t\tself.par[x]\n\
    \t\t}\n\t}\n\tpub fn is_same(&mut self, x: usize, y: usize) -> bool {\n\t\tself.find(x)\
    \ == self.find(y)\n\t}\n\tpub fn size(&mut self, x: usize) -> usize {\n\t\tlet\
    \ root = self.find(x);\n\t\tself.size[root]\n\t}\n\tpub fn unite(&mut self, x:\
    \ usize, y: usize) -> usize {\n\t\tlet (mut x, mut y) = (self.find(x), self.find(y));\n\
    \t\tif x != y {\n\t\t\tif self.size[x] < self.size[y] {\n\t\t\t\tstd::mem::swap(&mut\
    \ x, &mut y);\n\t\t\t}\n\t\t\tself.par[y] = x;\n\t\t\tself.size[x] += self.size[y];\n\
    \t\t\tself.count -= 1;\n\t\t}\n\t\tx\n\t}\n\tpub fn count(&self) -> usize {\n\t\
    \tself.count\n\t}\n\tpub fn push(&mut self) -> usize {\n\t\tlet new = self.len();\n\
    \t\tself.par.push(new);\n\t\tself.size.push(1);\n\t\tnew\n\t}\n\tpub fn group(&self)\
    \ -> Vec<Vec<usize>> {\n\t\tlet mut groups = vec![Vec::new(); self.len()];\n\t\
    \tfor (i, &x) in (0..).zip(&self.par) {\n\t\t\tgroups[x].push(i);\n\t\t}\n\t\t\
    groups.retain(|v| !v.is_empty());\n\t\tgroups\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/uf.rs
  requiredBy: []
  timestamp: '2021-01-13 14:07:03+09:00'
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
