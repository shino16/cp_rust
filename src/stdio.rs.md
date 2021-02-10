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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/stdio.rs\n"
  code: "pub use crate::{scan, prtln};\nuse std::io::{stdout, BufWriter, StdoutLock};\n\
    \npub fn stdout_buf() -> BufWriter<StdoutLock<'static>> {\n    let out = Box::leak(Box::new(stdout()));\n\
    \    BufWriter::new(out.lock())\n}\n\n#[macro_export]\nmacro_rules! prtln {\n\
    \    (new $var:ident) => {\n        let mut $var = stdout_buf();\n    };\n   \
    \ (new $var:ident, $($t:tt)*) => { {\n        $crate::prtln!(new $var);\n    \
    \    $crate::prtln!(to $var, $($t)*);\n    } };\n    (to $var:ident, $($t:tt)*)\
    \ => { {\n        use std::io::Write;\n        $crate::prtln_inner!($var, $($t)*);\n\
    \    } };\n    ($($t:tt)*) => {\n        $crate::prtln!(new __prtln, $($t)*);\n\
    \    };\n}\n\n#[macro_export]\nmacro_rules! prtln_inner {\n    ($dst:expr $(,)?)\
    \ => {\n        ::std::writeln!($dst).unwrap();\n    };\n    ($dst:expr, $expr:expr\
    \ $(,$exprs:expr)*) => { {\n        ::std::write!($dst, \"{} \", $expr).unwrap();\n\
    \        $crate::prtln_inner!($dst, $($exprs),*);\n    } };\n    ($dst:expr, iter=$expr:expr,\
    \ sep=$sep:expr) => { {\n        let mut iter = $expr.into_iter();\n        if\
    \ let Some(expr) = iter.next() {\n            ::std::write!($dst, \"{}\", expr).unwrap();\n\
    \            for expr in iter {\n                ::std::write!($dst, \"{}{}\"\
    , $sep, expr).unwrap();\n            }\n        }\n        $crate::prtln_inner!($dst);\n\
    \    } };\n}\n\n#[macro_export]\nmacro_rules! scan {\n    (from $s:expr, $($r:tt)*)\
    \ => {\n        $crate::scan_inner!($s, $($r)*);\n    };\n    (new $var:ident,\
    \ $($r:tt)*) => {\n        let s = {\n            use ::std::io::Read;\n     \
    \       let mut s = String::new();\n            ::std::io::stdin().read_to_string(&mut\
    \ s).unwrap();\n            s\n        };\n        let $var = &mut s.split_whitespace();\n\
    \        $crate::scan_inner!($var, $($r)*);\n    };\n    ($($r:tt)*) => {\n  \
    \      $crate::scan!(nwq __scan, $($r)*);\n    }\n}\n\n#[macro_export]\nmacro_rules!\
    \ scan_inner {\n    ($iter:expr $(,)?) => {};\n    ($iter:expr, $var:ident : $t:tt\
    \ $($r:tt)*) => {\n        let $var = $crate::scan_value!($iter, $t);\n      \
    \  $crate::scan_inner!($iter $($r)*)\n    };\n    ($iter:expr, $pat:pat in $t:tt\
    \ $($r:tt)*) => {\n        let $pat = $crate::scan_value!($iter, $t);\n      \
    \  $crate::scan_inner!($iter $($r)*)\n    };\n}\n\n#[macro_export]\nmacro_rules!\
    \ scan_value {\n    ($iter:expr, ( $($t:tt),* )) => {\n        ( $($crate::scan_value!($iter,\
    \ $t)),* )\n    };\n    ($iter:expr, [ $t:tt ; $len:expr ]) => {\n        (0..$len).map(|_|\
    \ $crate::scan_value!($iter, $t)).collect::<Vec<_>>()\n    };\n    ($iter:expr,\
    \ bytes) => {\n        $iter.next().as_bytes()\n    };\n    ($iter:expr, usize1)\
    \ => {\n        $crate::scan_value!($iter, usize) - 1\n    };\n    ($iter:expr,\
    \ $t:ty) => {\n        $iter.next().unwrap().parse::<$t>().unwrap()\n    };\n\
    }\n"
  dependsOn:
  - src/lib.rs
  isVerificationFile: false
  path: src/stdio.rs
  requiredBy:
  - src/stdio/buf.rs
  timestamp: '2021-02-11 01:05:36+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/stdio.rs
layout: document
redirect_from:
- /library/src/stdio.rs
- /library/src/stdio.rs.html
title: src/stdio.rs
---
