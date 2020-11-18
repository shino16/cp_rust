---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':warning:'
    path: src/ds/segtree.rs
    title: src/ds/segtree.rs
  - icon: ':heavy_check_mark:'
    path: src/io.rs
    title: src/io.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    PROBLEM: http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "// verify-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A\n\
    \nuse lib::ds::segtree::*;\nuse lib::io::*;\n\nfn main() {\n    let mut io = IO::new();\n\
    \    let (n, q): (usize, usize) = io.scan();\n\n    let inf = (1_u32 << 31) -\
    \ 1;\n\n    let data = vec![inf; n];\n    let mut st = SegmentTree::new(&data,\
    \ MonoidImpl(|| inf, |a, b| *a.min(b)));\n\n    for _ in 0..q {\n        let [c,\
    \ x, y]: [usize; 3] = io.scan();\n        if c == 0 {\n            st.exec(x,\
    \ |v| *v = y as u32);\n        } else {\n            io.println(st.ask(x, y +\
    \ 1));\n        }\n    }\n}\n"
  dependsOn:
  - src/alg.rs
  - src/ds/segtree.rs
  - src/io.rs
  isVerificationFile: false
  path: test/src/bin/segtree.rs
  requiredBy: []
  timestamp: '2020-11-18 23:11:02+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: test/src/bin/segtree.rs
layout: document
redirect_from:
- /library/test/src/bin/segtree.rs
- /library/test/src/bin/segtree.rs.html
title: test/src/bin/segtree.rs
---
