---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/conv.rs
    title: src/conv.rs
  - icon: ':heavy_check_mark:'
    path: src/ds.rs
    title: src/ds.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/uvec.rs
    title: src/ds/uvec.rs
  - icon: ':heavy_check_mark:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':heavy_check_mark:'
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':heavy_check_mark:'
    path: src/mint/conv.rs
    title: src/mint/conv.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/convolution_mod
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: test/src/bin/ntt_mint_test.rs\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod\n\
    \nuse lib::io::*;\nuse lib::mint::conv::*;\n\nfn main() {\n    let mut io = IO::new();\n\
    \    let (n, m) = io.scan();\n    let a = io.scan_vec::<Mint99>(n);\n    let b\
    \ = io.scan_vec::<Mint99>(m);\n    if (n, m) == (1, 1) {\n        io.println(a[0]\
    \ * b[0]);\n    } else {\n        io.iterln(Conv::conv(a, b).into_iter(), \" \"\
    );\n    }\n}\n"
  dependsOn:
  - src/conv.rs
  - src/ds.rs
  - src/ds/uvec.rs
  - src/io.rs
  - src/mint.rs
  - src/mint/conv.rs
  - src/zo.rs
  isVerificationFile: true
  path: test/src/bin/ntt_mint_test.rs
  requiredBy: []
  timestamp: '2021-04-06 23:05:21+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: test/src/bin/ntt_mint_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/ntt_mint_test.rs
- /verify/test/src/bin/ntt_mint_test.rs.html
title: test/src/bin/ntt_mint_test.rs
---
