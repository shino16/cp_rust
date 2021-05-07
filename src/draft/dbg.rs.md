---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/draft/dbg.rs\n"
  code: "#[cfg(debug_assertions)]\n#[macro_export]\nmacro_rules! dbg {\n    () =>\
    \ {\n        std::eprintln!(\"[{}:{}]\", std::file!(), std::line!());\n    };\n\
    \    ($val:expr) => {\n        // Use of `match` here is intentional because it\
    \ affects the lifetimes\n        // of temporaries - https://stackoverflow.com/a/48732525/1063961\n\
    \        match $val {\n            tmp => {\n                std::eprintln!(\"\
    [{}:{}] {} = {:?}\",\n                    std::file!(), std::line!(), std::stringify!($val),\
    \ &tmp);\n                tmp\n            }\n        }\n    };\n    // Trailing\
    \ comma with single argument is ignored\n    ($val:expr,) => { dbg!($val) };\n\
    \    ($($val:expr),+ $(,)?) => {\n        ($(dbg!($val)),+,)\n    };\n}\n\n#[cfg(not(debug_assertions))]\n\
    #[macro_export]\nmacro_rules! dbg {\n    ($($x:expr),*) => { std::convert::identity(($($x),*))\
    \ }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/draft/dbg.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/dbg.rs
layout: document
redirect_from:
- /library/src/draft/dbg.rs
- /library/src/draft/dbg.rs.html
title: src/draft/dbg.rs
---
