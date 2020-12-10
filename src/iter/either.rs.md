---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub enum Either<L, R> {\n\tLeft(L),\n\tRight(R),\n}\n\nimpl<A, L, R> Iterator\
    \ for Either<L, R>\nwhere\n\tL: Iterator<Item = A>,\n\tR: Iterator<Item = A>,\n\
    {\n\ttype Item = A;\n\tfn next(&mut self) -> Option<Self::Item> {\n\t\tmatch self\
    \ {\n\t\t\tSelf::Left(l) => l.next(),\n\t\t\tSelf::Right(r) => r.next(),\n\t\t\
    }\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/iter/either.rs
  requiredBy: []
  timestamp: '2020-12-10 17:35:58+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/iter/either.rs
layout: document
redirect_from:
- /library/src/iter/either.rs
- /library/src/iter/either.rs.html
title: src/iter/either.rs
---
