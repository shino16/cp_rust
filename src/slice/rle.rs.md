---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/slice/rle.rs\n"
  code: "pub fn rle<T: Clone + Eq>(slice: &[T]) -> Vec<(T, usize)> {\n    if let Some(v)\
    \ = slice.get(0) {\n        let mut now = v;\n        let mut cnt = 1;\n     \
    \   let mut res = Vec::with_capacity(slice.len());\n        for v in &slice[1..]\
    \ {\n            if now == v {\n                cnt += 1;\n            } else\
    \ {\n                res.push((now.clone(), cnt));\n                now = v;\n\
    \                cnt = 1;\n            }\n        }\n        res.push((now.clone(),\
    \ cnt));\n        res\n    } else {\n        Vec::new()\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/slice/rle.rs
  requiredBy: []
  timestamp: '2021-03-14 02:25:56+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/slice/rle.rs
layout: document
redirect_from:
- /library/src/slice/rle.rs
- /library/src/slice/rle.rs.html
title: src/slice/rle.rs
---
