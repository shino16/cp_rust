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
  code: "use std::ops::*;\n\npub trait Bits:\n    Sized + Shl<u32, Output = Self>\
    \ + ShlAssign<u32> + Shr<u32, Output = Self> + ShrAssign<u32>\n{\n    fn trailing_zeros(self)\
    \ -> u32;\n    fn lsb(self) -> Self;\n    fn ilog2(self) -> u32;\n    fn msb(self)\
    \ -> Self;\n}\n\nmacro_rules! impl_bit {\n    ($($t:ty), *) => { $(\n        impl\
    \ Bits for $t {\n            fn trailing_zeros(self) -> u32 {\n              \
    \  <$t>::trailing_zeros(self)\n            }\n            fn lsb(self) -> Self\
    \ {\n                self & self.wrapping_neg()\n            }\n            fn\
    \ ilog2(self) -> u32 {\n                std::mem::size_of::<$t>() as u32 * 8 -\
    \ self.leading_zeros() - 1\n            }\n            fn msb(self) -> Self {\n\
    \                (1 as $t) << self.ilog2()\n            }\n        }\n    )* };\n\
    }\n\nimpl_bit!(i32, u32, i64, u64, i128, u128, isize, usize);\n"
  dependsOn: []
  isVerificationFile: false
  path: src/bit.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/bit.rs
layout: document
redirect_from:
- /library/src/bit.rs
- /library/src/bit.rs.html
title: src/bit.rs
---
