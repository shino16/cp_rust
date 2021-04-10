---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/draft/for_loop.rs\n"
  code: "#[macro_export]\nmacro_rules! for_loop {\n    ($init:stmt; $cond:expr; $upd:stmt;\
    \ $block:block) => {\n        $init\n        while $cond {\n            $block\n\
    \            $upd\n        }\n    };\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/draft/for_loop.rs
  requiredBy: []
  timestamp: '2021-02-22 02:21:06+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/for_loop.rs
layout: document
redirect_from:
- /library/src/draft/for_loop.rs
- /library/src/draft/for_loop.rs.html
title: src/draft/for_loop.rs
---
