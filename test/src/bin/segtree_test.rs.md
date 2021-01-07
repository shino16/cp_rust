---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A
    links:
    - http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verify-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A\n\
    \nuse lib::ds::segtree::*;\nuse lib::io::*;\n\nfn main() {\n\tlet mut io = IO::new();\n\
    \tlet (n, q): (usize, usize) = io.scan();\n\n\tlet inf = (1_u32 << 31) - 1;\n\n\
    \tlet data = vec![inf; n];\n\tlet mut st = SegmentTree::new(&data, MonoidImpl(||\
    \ inf, |a, b| a.min(b)));\n\n\tfor _ in 0..q {\n\t\tlet [c, x, y]: [usize; 3]\
    \ = io.scan();\n\t\tif c == 0 {\n\t\t\tst.exec(x, |v| *v = y as u32);\n\t\t} else\
    \ {\n\t\t\tio.println(st.ask(x, y + 1));\n\t\t}\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: test/src/bin/segtree_test.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: test/src/bin/segtree_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/segtree_test.rs
- /verify/test/src/bin/segtree_test.rs.html
title: test/src/bin/segtree_test.rs
---
