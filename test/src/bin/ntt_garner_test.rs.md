---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/convolution_mod_1000000007
    links:
    - https://judge.yosupo.jp/problem/convolution_mod_1000000007
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verify-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod_1000000007\n\
    \nuse lib::io::*;\nuse lib::fp::conv::*;\n\nfn main() {\n\tlet mut io = IO::new();\n\
    \tlet (n, m) = io.scan();\n\tlet a = io.scan_vec::<F17>(n);\n\tlet b = io.scan_vec::<F17>(m);\n\
    \tif (n, m) == (1, 1) {\n\t\tio.println(a[0] * b[0]);\n\t} else {\n\t\tio.iterln(Conv::conv(a,\
    \ b).into_iter(), \" \");\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: test/src/bin/ntt_garner_test.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: test/src/bin/ntt_garner_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/ntt_garner_test.rs
- /verify/test/src/bin/ntt_garner_test.rs.html
title: test/src/bin/ntt_garner_test.rs
---
