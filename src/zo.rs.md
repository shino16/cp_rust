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
  code: "pub trait ZeroOne: Copy + Eq {\n\tconst ZERO: Self;\n\tfn is_zero(self) ->\
    \ bool {\n\t\tself == Self::ZERO\n\t}\n\tconst ONE: Self;\n}\n\nmacro_rules! impl_zo\
    \ {\n\t($($t:ty),*) => { $(\n\t\timpl ZeroOne for $t {\n\t\t\tconst ZERO: Self\
    \ = 0;\n\t\t\tconst ONE: Self = 1;\n\t\t}\n\t)* };\n}\n\nimpl_zo!(i32, i64, i128,\
    \ isize, u32, u64, u128, usize);\n"
  dependsOn: []
  isVerificationFile: false
  path: src/zo.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/zo.rs
layout: document
redirect_from:
- /library/src/zo.rs
- /library/src/zo.rs.html
title: src/zo.rs
---
