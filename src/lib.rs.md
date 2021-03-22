---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/ds/pool.rs
    title: src/ds/pool.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/uvec.rs
    title: src/ds/uvec.rs
  - icon: ':heavy_check_mark:'
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':warning:'
    path: src/stdio.rs
    title: src/stdio.rs
  - icon: ':warning:'
    path: src/stdio/buf.rs
    title: src/stdio/buf.rs
  - icon: ':warning:'
    path: src/util/for_loop.rs
    title: src/util/for_loop.rs
  - icon: ':heavy_check_mark:'
    path: src/util/trait_alias.rs
    title: src/util/trait_alias.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/lib.rs\n"
  code: "pub mod alg;\npub mod assign;\npub mod bits;\npub mod bool;\npub mod bounded;\n\
    pub mod cast;\npub mod cmp;\npub mod complex;\npub mod conv;\npub mod dfa;\npub\
    \ mod ds;\npub mod float;\npub mod fp;\npub mod func;\npub mod fxhash;\npub mod\
    \ graph;\npub mod hash;\npub mod int;\npub mod io;\npub mod io_interactive;\n\
    pub mod iter;\npub mod make_vec;\npub mod math;\npub mod mint;\npub mod num;\n\
    pub mod poly;\npub mod rand;\npub mod slice;\npub mod stdio;\npub mod u64;\npub\
    \ mod util;\npub mod vec;\npub mod zo;\n\npub mod tests;\n\n#[cfg(debug_assertions)]\n\
    #[macro_export]\nmacro_rules! dbg {\n    ($($val:expr),* $(,)?) => {\n       \
    \ ($( match $val {\n            tmp => {\n                std::eprintln!(\"[{}:{}]\
    \ {} = {:?}\",\n                    std::file!(), std::line!(), std::stringify!($val),\
    \ &tmp);\n                tmp\n            }\n        } ),*)\n    };\n}\n\n#[cfg(not(debug_assertions))]\n\
    #[macro_export]\nmacro_rules! dbg {\n    ($($x:expr),*) => {};\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/lib.rs
  requiredBy:
  - src/stdio.rs
  - src/util/for_loop.rs
  - src/util/trait_alias.rs
  - src/ds/pool.rs
  - src/ds/uvec.rs
  - src/mint.rs
  - src/stdio/buf.rs
  timestamp: '2021-03-22 00:48:45+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/lib.rs
layout: document
redirect_from:
- /library/src/lib.rs
- /library/src/lib.rs.html
title: src/lib.rs
---
