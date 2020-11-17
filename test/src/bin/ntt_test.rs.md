---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: src/as_int.rs
    title: src/as_int.rs
  - icon: ':x:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':x:'
    path: src/ds/uvec.rs
    title: src/ds/uvec.rs
  - icon: ':x:'
    path: src/fp.rs
    title: src/fp.rs
  - icon: ':x:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':x:'
    path: src/math/ntt.rs
    title: src/math/ntt.rs
  - icon: ':x:'
    path: src/num.rs
    title: src/num.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/convolution_mod
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "// verify-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod\n\
    \nuse lib::io::*;\nuse lib::fp::*;\nuse lib::math::ntt::*;\n\nfn main() {\n  \
    \  let mut io = IO::new();\n    let (n, m) = io.scan();\n    let a = io.scan_vec::<Fp99>(n);\n\
    \    let b = io.scan_vec::<Fp99>(m);\n    if (n, m) == (1, 1) {\n        io.println(a[0]\
    \ * b[0]);\n    } else {\n        io.iterln(Conv::conv(a, b).into_iter(), \" \"\
    );\n    }\n}\n"
  dependsOn:
  - src/as_int.rs
  - src/bit.rs
  - src/ds/uvec.rs
  - src/fp.rs
  - src/io.rs
  - src/math/ntt.rs
  - src/num.rs
  isVerificationFile: true
  path: test/src/bin/ntt_test.rs
  requiredBy: []
  timestamp: '2020-11-17 21:23:08+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: test/src/bin/ntt_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/ntt_test.rs
- /verify/test/src/bin/ntt_test.rs.html
title: test/src/bin/ntt_test.rs
---
