---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/swag.rs
    title: src/ds/swag.rs
  - icon: ':question:'
    path: src/io.rs
    title: src/io.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_3_D&lang=ja
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: test/src/bin/swag_test.rs\n"
  code: "// verification-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_3_D&lang=ja\n\
    \nuse lib::ds::swag::*;\nuse lib::io::*;\n\nfn main() {\n    let mut io = IO::new();\n\
    \    let (n, Usize1(l)) = io.scan();\n    let a = io.scan_vec::<u32>(n);\n   \
    \ let mut swag = Swag::new(MonoidImpl(|| !0, u32::min));\n    swag.extend_from_slice(&a[..l]);\n\
    \    io.iterln(\n        a[l..].iter().map(|&a| {\n            swag.push(a);\n\
    \            let ans = swag.ask();\n            swag.pop();\n            ans\n\
    \        }),\n        \" \",\n    );\n}\n"
  dependsOn:
  - src/alg.rs
  - src/ds/swag.rs
  - src/io.rs
  isVerificationFile: true
  path: test/src/bin/swag_test.rs
  requiredBy: []
  timestamp: '2021-02-20 13:37:47+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: test/src/bin/swag_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/swag_test.rs
- /verify/test/src/bin/swag_test.rs.html
title: test/src/bin/swag_test.rs
---
