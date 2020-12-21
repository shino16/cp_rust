---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub trait ZeroOne: Copy + Eq {\n\tconst ZERO: Self;\n\tfn is_zero(self) ->\
    \ bool {\n\t\tself == Self::ZERO\n\t}\n\tconst ONE: Self;\n}\n\nmacro_rules! impl_zo\
    \ {\n\t($($t:ty),*) => { $(\n\t\timpl ZeroOne for $t {\n\t\t\tconst ZERO: Self\
    \ = 0;\n\t\t\tconst ONE: Self = 1;\n\t\t}\n\t)* };\n}\n\nimpl_zo!(i32, i64, i128,\
    \ isize, u32, u64, u128, usize);\n"
  dependsOn: []
  isVerificationFile: false
  path: src/int/zo.rs
  requiredBy: []
  timestamp: '2020-12-21 16:33:52+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/dfa_test.rs
  - test/src/bin/cargo_test.rs
documentation_of: src/int/zo.rs
layout: document
redirect_from:
- /library/src/int/zo.rs
- /library/src/int/zo.rs.html
title: src/int/zo.rs
---
