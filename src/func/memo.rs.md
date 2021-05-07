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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/func/memo.rs\n"
  code: "use std::cell::RefCell;\nuse std::hash::Hash;\nuse lib2::fxhash::*;\n// (otherwise)\
    \ use std::collections::HashMap;\n\n#[must_use]\npub struct Memo<F, Arg, Ret>(F,\
    \ RefCell<HashMap<Arg, Ret>>);\n\nimpl<F, Arg, Ret> Memo<F, Arg, Ret>\nwhere\n\
    \    F: Fn(&dyn Fn(Arg) -> Ret, Arg) -> Ret,\n    Arg: Clone + Eq + Hash,\n  \
    \  Ret: Clone,\n{\n    pub fn call(&self, arg: Arg) -> Ret {\n        if let Some(ret)\
    \ = self.1.borrow().get(&arg) {\n            return ret.clone();\n        }\n\
    \        let ret = self.0(&|arg| self.call(arg), arg.clone());\n        self.1.borrow_mut().insert(arg,\
    \ ret.clone());\n        ret\n    }\n}\n\npub fn memoize<Arg: Eq + Hash, Ret,\
    \ F>(f: F) -> Memo<F, Arg, Ret>\nwhere\n    F: Fn(&dyn Fn(Arg) -> Ret, Arg) ->\
    \ Ret,\n{\n    Memo(f, RefCell::new(HashMap::default()))\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/func/memo.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-04-03 13:43:51+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/func/memo.rs
layout: document
redirect_from:
- /library/src/func/memo.rs
- /library/src/func/memo.rs.html
title: src/func/memo.rs
---
