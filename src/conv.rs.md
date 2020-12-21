---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_garner_test.rs
    title: test/src/bin/ntt_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_garner_test.rs
    title: test/src/bin/ntt_mint_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_test.rs
    title: test/src/bin/ntt_mint_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_test.rs
    title: test/src/bin/ntt_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub trait Conv: Sized {\n\tfn conv(mut lhs: Vec<Self>, mut rhs: Vec<Self>)\
    \ -> Vec<Self> {\n\t\tConv::conv_in_place(&mut lhs, &mut rhs);\n\t\tlhs\n\t}\n\
    \tfn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>);\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/conv.rs
  requiredBy: []
  timestamp: '2020-11-27 14:24:44+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_test.rs
  - test/src/bin/ntt_mint_garner_test.rs
  - test/src/bin/ntt_mint_test.rs
  - test/src/bin/ntt_garner_test.rs
documentation_of: src/conv.rs
layout: document
redirect_from:
- /library/src/conv.rs
- /library/src/conv.rs.html
title: src/conv.rs
---
