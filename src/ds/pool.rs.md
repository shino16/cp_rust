---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/lib.rs
    title: src/lib.rs
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
    RuntimeError: bundler is not specified: src/ds/pool.rs\n"
  code: "pub use crate::pool;\n\npub trait Pool<T> {\n    fn alloc() -> *mut T;\n\
    }\n\n#[macro_export]\nmacro_rules! pool {\n    ($ident:ident : [$ty:ty; $len:expr]\
    \ $(,)?) => {\n        struct $ident;\n        impl Pool<$ty> for $ident {\n \
    \           fn alloc() -> *mut $ty {\n                use std::mem::MaybeUninit\
    \ as MU;\n                use std::sync::atomic::{AtomicUsize, Ordering};\n\n\
    \                static mut DATA: MU<[MU<$ty>; $len]> = MU::uninit();\n      \
    \          static IDX: AtomicUsize = AtomicUsize::new(0);\n                let\
    \ idx = IDX.fetch_add(1, Ordering::Relaxed);\n                let data = unsafe\
    \ { &mut *DATA.as_mut_ptr() };\n                data[idx].as_mut_ptr()\n     \
    \       }\n        }\n    };\n}\n"
  dependsOn:
  - src/lib.rs
  isVerificationFile: false
  path: src/ds/pool.rs
  requiredBy: []
  timestamp: '2021-04-03 11:26:56+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/pool.rs
layout: document
redirect_from:
- /library/src/ds/pool.rs
- /library/src/ds/pool.rs.html
title: src/ds/pool.rs
---
