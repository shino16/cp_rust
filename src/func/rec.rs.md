---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/func/rec.rs\n"
  code: "pub struct Recurse<F>(F);\n\nimpl<F> Recurse<F> {\n\tpub fn call<Arg, Ret>(&self,\
    \ arg: Arg) -> Ret\n\twhere\n\t\tF: Fn(&dyn Fn(Arg) -> Ret, Arg) -> Ret,\n\t{\n\
    \t\tself.0(&|arg| self.call(arg), arg)\n\t}\n}\n\npub fn recurse<Arg, Ret, F:\
    \ Fn(&dyn Fn(Arg) -> Ret, Arg) -> Ret>(f: F) -> Recurse<F> {\n\tRecurse(f)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/func/rec.rs
  requiredBy: []
  timestamp: '2020-12-21 16:30:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/func/rec.rs
layout: document
redirect_from:
- /library/src/func/rec.rs
- /library/src/func/rec.rs.html
title: src/func/rec.rs
---
