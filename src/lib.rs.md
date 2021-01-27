---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/lib.rs\n"
  code: "#![allow(clippy::many_single_char_names)]\n#![allow(clippy::new_without_default)]\n\
    #![allow(clippy::suspicious_op_assign_impl)]\n\npub mod alg;\npub mod assign;\n\
    pub mod bit;\npub mod bool;\npub mod cast;\npub mod cmp;\npub mod conv;\npub mod\
    \ dfa;\npub mod ds;\npub mod float;\npub mod fp;\npub mod func;\npub mod fxhash;\n\
    pub mod graph;\npub mod hash;\npub mod int;\npub mod io;\npub mod io_interactive;\n\
    pub mod iter;\npub mod make_vec;\npub mod math;\npub mod mint;\npub mod num;\n\
    pub mod rand;\npub mod slice;\npub mod u64;\npub mod vec;\npub mod zo;\n\npub\
    \ mod tests;\n\n#[macro_export]\n#[cfg(debug_assertions)]\nmacro_rules! dbg {\n\
    \t($($x:expr),*) => { std::dbg!($($x),*) }\n}\n\n#[macro_export]\n#[cfg(not(debug_assertions))]\n\
    macro_rules! dbg {\n\t($($x:expr),*) => { std::convert::identity(($($x),*)) }\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: src/lib.rs
  requiredBy: []
  timestamp: '2021-01-27 17:46:37+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/lib.rs
layout: document
redirect_from:
- /library/src/lib.rs
- /library/src/lib.rs.html
title: src/lib.rs
---
