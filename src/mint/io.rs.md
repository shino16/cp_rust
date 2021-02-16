---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':heavy_check_mark:'
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/lazy_segtree_test.rs
    title: test/src/bin/lazy_segtree_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_garner_test.rs
    title: test/src/bin/ntt_mint_garner_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/mint/io.rs\n"
  code: "pub use super::*;\nuse crate::io::*;\n\nimpl<M: Mod> Print for Mint<M> {\n\
    \    fn print(w: &mut IO, x: Self) { w.print(x.value()); }\n}\n\nimpl<M: Mod>\
    \ Scan for Mint<M> {\n    fn scan(io: &mut IO) -> Self { io.scan::<u32>().into()\
    \ }\n}\n"
  dependsOn:
  - src/io.rs
  - src/mint.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/mint/io.rs
  requiredBy: []
  timestamp: '2021-02-15 17:55:41+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_mint_garner_test.rs
  - test/src/bin/dfa_test.rs
  - test/src/bin/lazy_segtree_test.rs
documentation_of: src/mint/io.rs
layout: document
redirect_from:
- /library/src/mint/io.rs
- /library/src/mint/io.rs.html
title: src/mint/io.rs
---
