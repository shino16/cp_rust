---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':x:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/slice/sa.rs\n"
  code: "use std::cmp::Reverse;\n\npub struct SuffixArray<T> {\n    _data: Vec<T>,\n\
    \    sa: Vec<usize>,\n    rev: Vec<usize>,\n}\n\nimpl<T: Copy> SuffixArray<T>\
    \ {\n    pub fn new<F: FnMut(T) -> usize>(mut _data: Vec<T>, zero: T, mut key:\
    \ F) -> Self {\n        _data.push(zero);\n        let n = _data.len();\n    \
    \    let mut sa: Vec<_> = (0..n).collect();\n        sa.sort_unstable_by_key(|&i|\
    \ (key(_data[i]), Reverse(i)));\n        let mut c: Vec<_> = _data.iter().copied().map(key).collect();\n\
    \        let mut c2 = vec![0; n];\n        let mut cnt = vec![0; n];\n       \
    \ let mut len = 1;\n        while len < n {\n            for i in 0..n {\n   \
    \             let cond = i != 0\n                    && c[sa[i - 1]] == c[sa[i]]\n\
    \                    && sa[i - 1] + len < n\n                    && c[sa[i - 1]\
    \ + len / 2] == c[sa[i] + len / 2];\n                c2[sa[i]] = if cond { c2[sa[i\
    \ - 1]] } else { 1 };\n            }\n            cnt.iter_mut().for_each(|e|\
    \ *e = 0);\n            c.clear();\n            c.extend(sa.iter().copied());\n\
    \            for i in 0..n {\n                if let Some(s) = c[i].checked_sub(len)\
    \ {\n                    sa[cnt[c2[s]]] = s;\n                    cnt[c2[s]] +=\
    \ 1;\n                }\n            }\n            std::mem::swap(&mut c2, &mut\
    \ c);\n            len *= 2;\n        }\n        let mut rev = vec![!0; n];\n\
    \        for i in 0..n {\n            rev[sa[i]] = i;\n        }\n        Self\
    \ { _data, sa, rev }\n    }\n    pub fn nth(&self, i: usize) -> usize { self.sa[i]\
    \ }\n    pub fn rank(&self, i: usize) -> usize { self.rev[i] }\n}\n\nimpl<T> std::ops::Deref\
    \ for SuffixArray<T> {\n    type Target = [usize];\n    fn deref(&self) -> &Self::Target\
    \ {\n        &self.sa[..]\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/slice/sa.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-02-22 02:21:14+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/slice/sa.rs
layout: document
redirect_from:
- /library/src/slice/sa.rs
- /library/src/slice/sa.rs.html
title: src/slice/sa.rs
---
