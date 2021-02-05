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
    RuntimeError: bundler is not specified: src/slice/lcp.rs\n"
  code: "pub fn lcp(t: &[u8], sa: &[usize]) -> Vec<usize> {\n    lcp_impl(t, sa, |&v|\
    \ v as usize)\n}\n\npub fn lcp_impl<T, F: FnMut(&T) -> usize>(t: &[T], sa: &[usize],\
    \ mut key: F) -> Vec<usize> {\n\tlet n = sa.len() - 1;\n\tlet mut rank = vec![0;\
    \ n];\n\tfor i in 1..n {\n\t\trank[sa[i]] = i;\n\t}\n\tlet mut k = 0;\n\tlet mut\
    \ lcp = vec![0; n - 1];\n\tfor i in 0..n {\n\t\tif rank[i] == n - 1 {\n\t\t\t\
    k = 0;\n\t\t\tcontinue;\n\t\t}\n\t\tlet j = sa[rank[i] + 1];\n\t\twhile i + k\
    \ < n && j + k < n && key(&t[i + k]) == key(&t[j + k]) {\n\t\t\tk += 1;\n\t\t\
    }\n\t\tlcp[rank[i]] = k;\n\t\tif k > 0 {\n\t\t\tk -= 1;\n\t\t}\n\t}\n\tlcp\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/slice/lcp.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-02-06 03:32:36+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/slice/lcp.rs
layout: document
redirect_from:
- /library/src/slice/lcp.rs
- /library/src/slice/lcp.rs.html
title: src/slice/lcp.rs
---
