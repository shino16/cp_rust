---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/lib.rs
    title: src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/stdio/buf.rs
    title: src/stdio/buf.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/stdio.rs\n"
  code: "pub use crate::prtln;\npub use crate::scan;\nuse std::io::{stdout, BufWriter,\
    \ StdoutLock};\n\npub fn stdout_buf() -> BufWriter<StdoutLock<'static>> {\n  \
    \  let out = Box::leak(Box::new(stdout()));\n    BufWriter::with_capacity(1 <<\
    \ 25, out.lock())\n}\n\n#[macro_export]\nmacro_rules! prtln {\n    (@ $dst:expr,\
    \ iter=$expr:expr) => { $crate::prtln!(@ $dst, iter=$expr, sep=\" \"); };\n  \
    \  (@ $dst:expr, iter=$expr:expr, sep=$sep:expr) => { {\n        let mut iter\
    \ = $expr.into_iter();\n        if let Some(expr) = iter.next() {\n          \
    \  std::write!($dst, \"{}\", expr).unwrap();\n            for expr in iter { std::write!($dst,\
    \ \"{}{}\", $sep, expr).unwrap(); }\n        }\n        $crate::prtln!(@ $dst,\
    \ \"\");\n    } };\n    (@ $dst:expr, $expr:expr) => { std::writeln!($dst, \"\
    {}\", $expr).unwrap(); };\n    (@ $dst:expr, $expr:expr, $($exprs:expr),*) =>\
    \ { {\n        std::write!($dst, \"{} \", $expr).unwrap();\n        $crate::prtln!(@\
    \ $dst, $($exprs),*);\n    } };\n    (new $var:ident) => { let mut $var = stdout_buf();\
    \ };\n    (new $var:ident, $($t:tt)*) => {\n        $crate::prtln!(new $var);\n\
    \        $crate::prtln!(to $var, $($t)*);\n    };\n    (to $var:ident, $($t:tt)*)\
    \ => { {\n        use std::io::Write;\n        $crate::prtln!(@ $var, $($t)*);\n\
    \    } };\n    ($($t:tt)*) => { $crate::prtln!(new __prtln, $($t)*); };\n}\n\n\
    #[macro_export]\nmacro_rules! scan {\n    (@ $iter:expr $(,)?) => {};\n    (@\
    \ $iter:expr, mut $var:ident : $t:tt $($r:tt)*) => {\n        let mut $var = $crate::scan_value!($iter,\
    \ $t);\n        $crate::scan!(@ $iter $($r)*)\n    };\n    (@ $iter:expr, $var:ident\
    \ : $t:tt $($r:tt)*) => {\n        let $var = $crate::scan_value!($iter, $t);\n\
    \        $crate::scan!(@ $iter $($r)*)\n    };\n    (@ $iter:expr, $pat:pat in\
    \ $t:tt $($r:tt)*) => {\n        let $pat = $crate::scan_value!($iter, $t);\n\
    \        $crate::scan!(@ $iter $($r)*)\n    };\n    (from $s:expr, $($r:tt)*)\
    \ => { $crate::scan!(@ $s, $($r)*); };\n    (new $var:ident, $($r:tt)*) => {\n\
    \        let mut __input = String::new();\n        std::io::Read::read_to_string(&mut\
    \ std::io::stdin(), &mut __input).unwrap();\n        let $var = &mut __input.split_ascii_whitespace();\n\
    \        $crate::scan!(@ $var, $($r)*);\n    };\n    ($($r:tt)*) => { $crate::scan!(new\
    \ __scan, $($r)*); };\n}\n\n#[macro_export]\nmacro_rules! scan_value {\n    ($iter:expr,\
    \ ( $($t:tt),* )) => { ( $($crate::scan_value!($iter, $t)),* ) };\n    ($iter:expr,\
    \ [ $t:tt ; $len:expr ]) => {\n        (0..$len).map(|_| $crate::scan_value!($iter,\
    \ $t)).collect::<Vec<_>>()\n    };\n    ($iter:expr, &[u8]) => { $iter.next().unwrap().as_bytes()\
    \ };\n    ($iter:expr, usize1) => { $crate::scan_value!($iter, usize) - 1 };\n\
    \    ($iter:expr, $t:ty) => { $iter.next().unwrap().parse::<$t>().unwrap() };\n\
    }\n"
  dependsOn:
  - src/lib.rs
  isVerificationFile: false
  path: src/stdio.rs
  requiredBy:
  - src/stdio/buf.rs
  timestamp: '2021-03-26 09:38:33+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/stdio.rs
layout: document
redirect_from:
- /library/src/stdio.rs
- /library/src/stdio.rs.html
title: src/stdio.rs
---
