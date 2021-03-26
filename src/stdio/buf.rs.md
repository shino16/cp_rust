---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/lib.rs
    title: src/lib.rs
  - icon: ':warning:'
    path: src/stdio.rs
    title: src/stdio.rs
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
    RuntimeError: bundler is not specified: src/stdio/buf.rs\n"
  code: "pub use super::*;\npub use crate::prtln;\npub use std::io::Write;\nuse std::io::{stdout,\
    \ BufWriter, StdoutLock};\n\npub fn stdout_buf() -> BufWriter<StdoutLock<'static>>\
    \ {\n    let out = Box::leak(Box::new(stdout()));\n    BufWriter::new(out.lock())\n\
    }\n\n#[macro_export]\nmacro_rules! prtln {\n    ($dst:expr) => {\n        std::writeln!($dst).unwrap();\n\
    \    };\n    ($dst:expr, $expr:expr) => { {\n        $crate::prt!($dst, $expr);\n\
    \        std::writeln!($dst).unwrap();\n    } };\n    ($dst:expr, $expr:expr,\
    \ $($exprs:expr),*) => { {\n        $crate::prt!($dst, $expr);\n        $crate::prt!($dst,\
    \ \" \");\n        $crate::prtln($dst, $($exprs),*);\n    } };\n}\n\n#[macro_export]\n\
    macro_rules! prt {\n    ($dst:expr,) => {};\n    ($dst:expr, $expr:expr) => {\n\
    \        std::write!($dst, \"{}\", $expr).unwrap();\n    };\n    ($dst:expr,iter($expr:expr))\
    \ => {\n        $crate::prt!($dst, iter($expr, \" \"));\n    };\n    ($dst:expr,iter($expr:expr,\
    \ $delim:expr)) => {\n        let mut iter = $expr.into_iter();\n        if let\
    \ Some(expr) = iter.next() {\n            $crate::prt!($dst, expr);\n        \
    \    $iter.for_each(|expr| $crate::prt($dst, \" \", expr));\n        }\n    };\n\
    }\n"
  dependsOn:
  - src/lib.rs
  - src/stdio.rs
  isVerificationFile: false
  path: src/stdio/buf.rs
  requiredBy: []
  timestamp: '2021-03-26 09:38:33+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/stdio/buf.rs
layout: document
redirect_from:
- /library/src/stdio/buf.rs
- /library/src/stdio/buf.rs.html
title: src/stdio/buf.rs
---
