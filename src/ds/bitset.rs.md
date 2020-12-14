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
  code: "pub trait BitSet {\n\tfn get_bit(&self, i: usize) -> bool;\n\tfn set_bit(&mut\
    \ self, i: usize, b: bool);\n\tfn modify_bit(&mut self, i: usize, b: bool) ->\
    \ bool {\n\t\tlet ret = self.get_bit(i);\n\t\tself.set_bit(i, b);\n\t\tret\n\t\
    }\n}\n\nmacro_rules! impl_bitset {\n\t($($type:ty),*) => { $(\n\t\timpl BitSet\
    \ for $type {\n\t\t\tfn get_bit(&self, i: usize) -> bool {\n\t\t\t\t((*self >>\
    \ i) & 1) != 0\n\t\t\t}\n\t\t\tfn set_bit(&mut self, i: usize, b: bool) {\n\t\t\
    \t\t*self |= (b as $type) << i;\n\t\t\t}\n\t\t}\n\t)* };\n}\n\nimpl_bitset!(i32,\
    \ i64, i128, isize, u32, u64, u128, usize);\n\nimpl BitSet for [u32] {\n\tfn get_bit(&self,\
    \ i: usize) -> bool {\n\t\tself[i / 32].get_bit(i % 32)\n\t}\n\tfn set_bit(&mut\
    \ self, i: usize, b: bool) {\n\t\tself[i / 32].set_bit(i % 32, b);\n\t}\n}\n\n\
    pub fn new_bitset(n: usize) -> Vec<u32> {\n\tvec![0; n / 32]\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/bitset.rs
  requiredBy: []
  timestamp: '2020-12-15 00:46:43+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/bitset.rs
layout: document
redirect_from:
- /library/src/ds/bitset.rs
- /library/src/ds/bitset.rs.html
title: src/ds/bitset.rs
---
