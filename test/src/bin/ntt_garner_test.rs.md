---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/as_int.rs
    title: src/as_int.rs
  - icon: ':heavy_check_mark:'
    path: src/bit.rs
    title: src/bit.rs
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
    path: src/num.rs
    title: src/num.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/convolution_mod_1000000007
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "// verify-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod_1000000007\n\
    \nuse lib::io::*;\nuse lib::fp::conv::*;\n\nfn main() {\n\tlet mut io = IO::new();\n\
    \tlet (n, m) = io.scan();\n\tlet a = io.scan_vec::<F17>(n);\n\tlet b = io.scan_vec::<F17>(m);\n\
    \tif (n, m) == (1, 1) {\n\t\tio.println(a[0] * b[0]);\n\t} else {\n\t\tio.iterln(Conv::conv(a,\
    \ b).into_iter(), \" \");\n\t}\n}\n"
  dependsOn:
  - src/as_int.rs
  - src/bit.rs
  - src/conv.rs
  - src/ds/uvec.rs
  - src/fp.rs
  - src/fp/conv.rs
  - src/io.rs
  - src/num.rs
  isVerificationFile: true
  path: test/src/bin/ntt_garner_test.rs
  requiredBy: []
  timestamp: '2020-11-28 19:21:23+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: test/src/bin/ntt_garner_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/ntt_garner_test.rs
- /verify/test/src/bin/ntt_garner_test.rs.html
title: test/src/bin/ntt_garner_test.rs
---
