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
  code: "pub trait BitSet {\n\tfn bit_at(&self, i: usize) -> bool;\n\tfn set_bit_at(&mut\
    \ self, i: usize, b: bool);\n}\n\nmacro_rules! impl_bitset {\n\t($($type:ty),*)\
    \ => { $(\n\t\timpl BitSet for $type {\n\t\t\tfn bit_at(&self, i: usize) -> bool\
    \ {\n\t\t\t\t((*self >> i) & 1) != 0\n\t\t\t}\n\t\t\tfn set_bit_at(&mut self,\
    \ i: usize, b: bool) {\n\t\t\t\t*self |= (b as $type) << i;\n\t\t\t}\n\t\t}\n\t\
    )* };\n}\n\nimpl_bitset!(i32, i64, i128, isize, u32, u64, u128, usize);\n\nimpl\
    \ BitSet for [u32] {\n\tfn bit_at(&self, i: usize) -> bool {\n\t\tself[i / 32].bit_at(i\
    \ % 32)\n\t}\n\tfn set_bit_at(&mut self, i: usize, b: bool) {\n\t\tself[i / 32].set_bit_at(i\
    \ % 32, b);\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/bitset.rs
  requiredBy: []
  timestamp: '2020-11-27 14:24:44+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/bitset.rs
layout: document
redirect_from:
- /library/src/bitset.rs
- /library/src/bitset.rs.html
title: src/bitset.rs
---
