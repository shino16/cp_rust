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
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_beats_test.rs
    title: test/src/bin/segtree_beats_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/iter.rs\n"
  code: "pub mod either;\npub mod pow;\npub mod prod;\n\npub trait Itertools: Iterator\
    \ {\n    fn collect_vec(self) -> Vec<Self::Item>\n    where\n        Self: Sized,\n\
    \    {\n        self.collect()\n    }\n}\n\nimpl<I: Iterator> Itertools for I\
    \ {}\n\n#[macro_export]\nmacro_rules! iprod {\n    ($head:expr) => {\n       \
    \ $head.into_iter()\n    };\n    ($head:expr, $($tail:expr),*) => (\n        $head.into_iter().flat_map(|e|\
    \ {\n            std::iter::repeat(e).zip(iprod!($($tail),*))\n        })\n  \
    \  );\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/iter.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/segtree_beats_test.rs
  - test/src/bin/cargo_test.rs
documentation_of: src/iter.rs
layout: document
redirect_from:
- /library/src/iter.rs
- /library/src/iter.rs.html
title: src/iter.rs
---
