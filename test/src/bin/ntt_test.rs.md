---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
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
  dependsOn: []
  isVerificationFile: true
  path: test/src/bin/ntt_test.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: test/src/bin/ntt_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/ntt_test.rs
- /verify/test/src/bin/ntt_test.rs.html
title: test/src/bin/ntt_test.rs
---
