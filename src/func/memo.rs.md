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
    RuntimeError: bundler is not specified: src/func/memo.rs\n"
  code: "use std::cell::RefCell;\nuse std::collections::HashMap;\nuse std::hash::Hash;\n\
    \n#[must_use]\npub struct Memo<F, Arg, Ret>(F, RefCell<HashMap<Arg, Ret>>);\n\n\
    impl<F, Arg, Ret> Memo<F, Arg, Ret>\nwhere\n    F: Fn(&dyn Fn(Arg) -> Ret, Arg)\
    \ -> Ret,\n    Arg: Clone + Eq + Hash,\n    Ret: Clone,\n{\n    pub fn call(&self,\
    \ arg: Arg) -> Ret {\n        if let Some(ret) = self.1.borrow().get(&arg) {\n\
    \            return ret.clone();\n        }\n        let ret = self.0(&|arg| self.call(arg),\
    \ arg.clone());\n        self.1.borrow_mut().insert(arg, ret.clone());\n     \
    \   ret\n    }\n}\n\npub fn memo<Arg: Eq + Hash, Ret, F>(f: F) -> Memo<F, Arg,\
    \ Ret>\nwhere\n    F: Fn(&dyn Fn(Arg) -> Ret, Arg) -> Ret,\n{\n    Memo(f, RefCell::new(HashMap::new()))\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: src/func/memo.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/func/memo.rs
layout: document
redirect_from:
- /library/src/func/memo.rs
- /library/src/func/memo.rs.html
title: src/func/memo.rs
---
