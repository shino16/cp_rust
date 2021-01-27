---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/draft/uf_fast.rs\n"
  code: "// 2x faster\n\n#[derive(Clone)]\npub struct UnionFind {\n\tpar_or_size:\
    \ Vec<usize>,\n\tcount: usize,\n}\n\nimpl UnionFind {\n\tpub fn new(len: usize)\
    \ -> Self {\n\t\tlet par_or_size = vec![1_usize.wrapping_neg(); len];\n\t\tSelf\
    \ { par_or_size, count: len }\n\t}\n\tpub fn find(&mut self, x: usize) -> usize\
    \ {\n\t\tif self.par_or_size[x] >> 31 != 0 {\n\t\t\tx\n\t\t} else {\n\t\t\tself.par_or_size[x]\
    \ = self.find(self.par_or_size[x]);\n\t\t\tself.par_or_size[x]\n\t\t}\n\t}\n\t\
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {\n\t\tself.find(x) == self.find(y)\n\
    \t}\n\tpub fn size(&mut self, x: usize) -> usize {\n\t\tlet root = self.find(x);\n\
    \t\tself.par_or_size[root].wrapping_neg()\n\t}\n\tpub fn unite(&mut self, x: usize,\
    \ y: usize) -> usize {\n\t\tlet (mut x, mut y) = (self.find(x), self.find(y));\n\
    \t\tif x != y {\n\t\t\tif self.par_or_size[x] > self.par_or_size[y] {\n\t\t\t\t\
    std::mem::swap(&mut x, &mut y);\n\t\t\t}\n\t\t\tself.par_or_size[x] += self.par_or_size[y];\n\
    \t\t\tself.par_or_size[y] = x;\n\t\t\tself.count -= 1;\n\t\t}\n\t\tx\n\t}\n\t\
    pub fn count(&self) -> usize {\n\t\tself.count\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/draft/uf_fast.rs
  requiredBy: []
  timestamp: '2021-01-13 14:07:03+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/uf_fast.rs
layout: document
redirect_from:
- /library/src/draft/uf_fast.rs
- /library/src/draft/uf_fast.rs.html
title: src/draft/uf_fast.rs
---
