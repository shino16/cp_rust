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
    \ mut key: F) -> Vec<usize> {\n    let n = sa.len() - 1;\n    let mut rank = vec![0;\
    \ n];\n    for i in 1..n {\n        rank[sa[i]] = i;\n    }\n    let mut k = 0;\n\
    \    let mut lcp = vec![0; n - 1];\n    for i in 0..n {\n        if rank[i] ==\
    \ n - 1 {\n            k = 0;\n            continue;\n        }\n        let j\
    \ = sa[rank[i] + 1];\n        while i + k < n && j + k < n && key(&t[i + k]) ==\
    \ key(&t[j + k]) {\n            k += 1;\n        }\n        lcp[rank[i]] = k;\n\
    \        if k > 0 {\n            k -= 1;\n        }\n    }\n    lcp\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/slice/lcp.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-02-08 00:55:24+09:00'
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
