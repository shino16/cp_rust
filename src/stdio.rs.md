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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/stdio.rs\n"
  code: "pub use crate::prtln;\npub use crate::scan;\npub use crate::parse;\npub use\
    \  std::io::Write;\nuse std::io::{stdout, BufWriter, StdoutLock};\n\npub fn stdout_buf()\
    \ -> BufWriter<StdoutLock<'static>> {\n    let out = Box::leak(Box::new(stdout()));\n\
    \    BufWriter::new(out.lock())\n}\n\n#[macro_export]\nmacro_rules! prtln {\n\
    \    (@ $dst:expr, iter=$v:expr) => { $crate::prtln!(@ $dst, iter=$v, sep=\" \"\
    ); };\n    (@ $dst:expr, iter=$v:expr, sep=$sep:expr) => { {\n        let mut\
    \ iter = $v.into_iter();\n        if let Some(expr) = iter.next() {\n        \
    \    std::write!($dst, \"{}\", expr).unwrap();\n            for v in iter { std::write!($dst,\
    \ \"{}{}\", $sep, v).unwrap(); }\n        }\n        $crate::prtln!(@ $dst, \"\
    \");\n    } };\n    (@ $dst:expr, bytes=$v:expr) => {\n        $crate::prtln!(@\
    \ $dst, std::str::from_utf8(&$v).unwrap());\n    };\n    (@ $dst:expr, YesNo=$v:expr)\
    \ => { $crate::prtln!(@ $dst, if $v { \"Yes\" } else { \"No\" }); };\n    (@ $dst:expr,\
    \ YESNO=$v:expr) => { $crate::prtln!(@ $dst, if $v { \"YES\" } else { \"NO\" });\
    \ };\n    (@ $dst:expr, $v:expr, no eol) => { std::write!($dst, \"{}\", $v).unwrap();\
    \ };\n    (@ $dst:expr, $v:expr) => { std::writeln!($dst, \"{}\", $v).unwrap();\
    \ };\n    (@ $dst:expr, $v:expr, $($t:tt)*) => { {\n        std::write!($dst,\
    \ \"{} \", $v).unwrap();\n        $crate::prtln!(@ $dst, $($t)*);\n    } };\n\
    \    (new $var:ident $(,)?) => { let mut $var = stdout_buf(); };\n    (new $var:ident,\
    \ $($t:tt)*) => {\n        $crate::prtln!(new $var);\n        $crate::prtln!(to\
    \ $var, $($t)*);\n    };\n    (to $var:ident, $($t:tt)*) => { {\n        $crate::prtln!(@\
    \ $var, $($t)*);\n    } };\n    ($($t:tt)*) => { {\n        $crate::prtln!(new\
    \ __prtln, $($t)*);\n        std::mem::drop(__prtln);\n    } };\n}\n\n#[macro_export]\n\
    macro_rules! scan {\n    (@ $iter:expr $(,)?) => {};\n    (@ $iter:expr, mut $var:ident\
    \ : $t:tt $($r:tt)*) => {\n        #[allow(non_snake_case)]\n        let mut $var\
    \ = $crate::parse!($iter.into_iter(), $t);\n        $crate::scan!(@ $iter $($r)*)\n\
    \    };\n    (@ $iter:expr, $var:ident : $t:tt $($r:tt)*) => {\n        #[allow(non_snake_case)]\n\
    \        let $var = $crate::parse!($iter.into_iter(), $t);\n        $crate::scan!(@\
    \ $iter $($r)*)\n    };\n    (@ $iter:expr, $pat:pat in $t:tt $($r:tt)*) => {\n\
    \        let $pat = $crate::parse!($iter.into_iter(), $t);\n        $crate::scan!(@\
    \ $iter $($r)*)\n    };\n    (from $s:expr, $($r:tt)*) => { $crate::scan!(@ $s,\
    \ $($r)*); };\n    (new $var:ident, $($r:tt)*) => {\n        let mut __input =\
    \ String::new();\n        std::io::Read::read_to_string(&mut std::io::stdin(),\
    \ &mut __input).unwrap();\n        let $var = &mut __input.split_ascii_whitespace();\n\
    \        $crate::scan!(@ $var, $($r)*);\n    };\n    ($($r:tt)*) => { $crate::scan!(new\
    \ __scan, $($r)*); };\n}\n\n#[macro_export]\nmacro_rules! parse {\n    ($iter:expr,\
    \ ( $($t:tt),* )) => { ( $($crate::parse!($iter, $t)),* ) };\n    ($iter:expr,\
    \ [ $t:tt ; $len:expr ]) => {\n        (0..$len).map(|_| $crate::parse!($iter,\
    \ $t)).collect::<Vec<_>>()\n    };\n    ($iter:expr, bytes) => { $iter.next().expect(\"\
    no input\").as_bytes() };\n    ($iter:expr, [u8]) => { $iter.next().expect(\"\
    no input\").as_bytes().to_vec() };\n    ($iter:expr, [char]) => { $iter.next().expect(\"\
    no input\").chars().collect::<Vec<_>>() };\n    ($iter:expr, usize1) => { $crate::parse!($iter,\
    \ usize) - 1 };\n    (@graph $iter:expr, $n:expr, $m:expr) => { {\n        let\
    \ mut graph = vec![Vec::new(); $n];\n        for _ in 0..$m {\n            let\
    \ (a, b) = $crate::parse!($iter, (usize1, usize1));\n            graph[a].push(b);\n\
    \            graph[b].push(a);\n        }\n        ($n, graph)\n    } };\n   \
    \ ($iter:expr, graph) => { {\n        let (n, m) = $crate::parse!($iter, (usize,\
    \ usize));\n        $crate::parse!(@graph $iter, n, m)\n    } };\n    ($iter:expr,\
    \ tree) => { {\n        let n = $crate::parse!($iter, usize);\n        $crate::parse!(@graph\
    \ $iter, n, n - 1)\n    } };\n    ($iter:expr, $t:ty) => { $iter.next().expect(\"\
    no input\").parse::<$t>().expect(\"parse error\") };\n    ($iter:expr) => { $iter.next().expect(\"\
    no input\").parse().expect(\"parse error\") };\n}\n"
  dependsOn:
  - src/lib.rs
  isVerificationFile: false
  path: src/stdio.rs
  requiredBy:
  - src/stdio/buf.rs
  timestamp: '2021-05-17 11:32:22+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/stdio.rs
layout: document
redirect_from:
- /library/src/stdio.rs
- /library/src/stdio.rs.html
title: src/stdio.rs
---
