---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/conv.rs
    title: src/conv.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/uvec.rs
    title: src/ds/uvec.rs
  - icon: ':heavy_check_mark:'
    path: src/fp.rs
    title: src/fp.rs
  - icon: ':heavy_check_mark:'
    path: src/fp/conv.rs
    title: src/fp/conv.rs
  - icon: ':heavy_check_mark:'
    path: src/io.rs
    title: src/io.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: test/src/bin/ntt_test.rs\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod\n\
    \nuse lib::fp::conv::*;\nuse lib::io::*;\n\nfn main() {\n    let mut io = IO::new();\n\
    \    let (n, m) = io.scan();\n    let a = io.scan_vec::<F99>(n);\n    let b =\
    \ io.scan_vec::<F99>(m);\n    if (n, m) == (1, 1) {\n        io.println(a[0] *\
    \ b[0]);\n    } else {\n        io.iterln(Conv::conv(a, b).into_iter(), \" \"\
    );\n    }\n}\n"
  dependsOn:
  - src/conv.rs
  - src/ds/uvec.rs
  - src/fp.rs
  - src/fp/conv.rs
  - src/io.rs
  - src/zo.rs
  isVerificationFile: true
  path: test/src/bin/ntt_test.rs
  requiredBy: []
  timestamp: '2021-02-11 01:05:36+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: test/src/bin/ntt_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/ntt_test.rs
- /verify/test/src/bin/ntt_test.rs.html
title: test/src/bin/ntt_test.rs
---
