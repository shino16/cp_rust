---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/slice/perm.rs\n"
  code: "pub fn next_permutation<T: Ord>(a: &mut [T]) -> bool {\n\tif a.len() <= 1\
    \ {\n\t\treturn false;\n\t}\n\tlet mut k = a.len() - 1;\n\twhile k != 0 && a[k\
    \ - 1] >= a[k] {\n\t\tk -= 1;\n\t}\n\tif k == 0 {\n\t\ta.reverse();\n\t\treturn\
    \ false;\n\t}\n\tk -= 1;\n\tlet mut l = a.len() - 1;\n\twhile a[k] >= a[l] {\n\
    \t\tl -= 1;\n\t}\n\ta.swap(k, l);\n\ta[k + 1..].reverse();\n\ttrue\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/slice/perm.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-01-12 01:50:14+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/slice/perm.rs
layout: document
redirect_from:
- /library/src/slice/perm.rs
- /library/src/slice/perm.rs.html
title: src/slice/perm.rs
---
