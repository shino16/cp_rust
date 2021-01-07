---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::*;\n\npub trait Bits:\n\tSized\n\t+ BitAnd<Output = Self> +\
    \ BitAndAssign\n\t+ BitOr<Output = Self> + BitOrAssign\n\t+ BitXor<Output = Self>\
    \ + BitXorAssign\n\t+ Shl<u32, Output = Self> + ShlAssign<u32>\n\t+ Shr<u32, Output\
    \ = Self> + ShrAssign<u32>\n\t+ Not<Output = Self>\n{\n\tfn trailing_zeros(self)\
    \ -> u32;\n\tfn lsb(self) -> Self;\n\tfn ilog2(self) -> u32;\n\tfn msb(self) ->\
    \ Self;\n}\n\nmacro_rules! impl_bit {\n\t($($t:ty), *) => { $(\n\t\timpl Bits\
    \ for $t {\n\t\t\tfn trailing_zeros(self) -> u32 {\n\t\t\t\t<$t>::trailing_zeros(self)\n\
    \t\t\t}\n\t\t\tfn lsb(self) -> Self {\n\t\t\t\tself & self.wrapping_neg()\n\t\t\
    \t}\n\t\t\tfn ilog2(self) -> u32 {\n\t\t\t\tstd::mem::size_of::<$t>() as u32 *\
    \ 8 - self.leading_zeros() - 1\n\t\t\t}\n\t\t\tfn msb(self) -> Self {\n\t\t\t\t\
    (1 as $t) << self.ilog2()\n\t\t\t}\n\t\t}\n\t)* };\n}\n\nimpl_bit!(i32, i64, i128,\
    \ isize, u32, u64, u128, usize);\n"
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
