---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_test.rs
    title: test/src/bin/segtree_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub use crate::alg::*;\n\npub trait Action {\n\ttype Actor: Monoid;\n\ttype\
    \ On: Monoid;\n\tfn act(\n\t\t&self,\n\t\ton: <Self::On as Alg>::Item,\n\t\tactor:\
    \ <Self::Actor as Alg>::Item,\n\t) -> <Self::On as Alg>::Item;\n}\n\npub struct\
    \ ActionImpl<On: Monoid, A: Monoid, F: Fn(On::Item, A::Item) -> On::Item>(\n\t\
    pub On,\n\tpub A,\n\tpub F,\n);\n\nimpl<On: Monoid, A: Monoid, F: Fn(On::Item,\
    \ A::Item) -> On::Item> Action\n\tfor ActionImpl<On, A, F>\n{\n\ttype Actor =\
    \ A;\n\ttype On = On;\n\tfn act(&self, on: On::Item, actor: A::Item) -> On::Item\
    \ {\n\t\tself.2(on, actor)\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/alg/action.rs
  requiredBy: []
  timestamp: '2020-12-15 00:46:43+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/segtree_test.rs
documentation_of: src/alg/action.rs
layout: document
redirect_from:
- /library/src/alg/action.rs
- /library/src/alg/action.rs.html
title: src/alg/action.rs
---
