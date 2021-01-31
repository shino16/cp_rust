---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':question:'
    path: src/alg/arith.rs
    title: src/alg/arith.rs
  - icon: ':question:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':question:'
    path: src/ds/bitset.rs
    title: src/ds/bitset.rs
  - icon: ':x:'
    path: src/ds/fenwick.rs
    title: src/ds/fenwick.rs
  - icon: ':x:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':x:'
    path: src/graph/dfs_io.rs
    title: src/graph/dfs_io.rs
  - icon: ':question:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':question:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://yukicoder.me/submissions/611960
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: test/src/bin/dfs_io_test.rs\n"
  code: "// verify-helper: PROBLEM https://yukicoder.me/submissions/611960\n\nuse\
    \ lib::ds::fenwick::*;\nuse lib::graph::dfs_io::*;\nuse lib::io::*;\n\nfn main()\
    \ {\n\tlet mut io = IO::new();\n\tlet n = io.scan();\n\tlet mut graph = vec![Vec::new();\
    \ n];\n\tfor v in 1..n {\n\t\tlet p: usize = io.scan();\n\t\tgraph[p].push(v);\n\
    \t\tgraph[v].push(p);\n\t}\n\tlet mut fwk = FenwickTree::new(vec![0; n], Addition::new());\n\
    \tlet mut ans = 0;\n\tdfs_io(&graph, 0, |v, _| match v {\n\t\tIn(v) => {\n\t\t\
    \tans += fwk.ask_prefix(v) as u64;\n\t\t\tfwk.add(v, 1_u32);\n\t\t},\n\t\tOut(v)\
    \ => fwk.sub(v, 1),\n\t});\n\tio.println(ans);\n}\n"
  dependsOn:
  - src/alg.rs
  - src/alg/arith.rs
  - src/bit.rs
  - src/ds/bitset.rs
  - src/ds/fenwick.rs
  - src/graph.rs
  - src/graph/dfs_io.rs
  - src/io.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: true
  path: test/src/bin/dfs_io_test.rs
  requiredBy: []
  timestamp: '2021-01-31 20:22:45+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: test/src/bin/dfs_io_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/dfs_io_test.rs
- /verify/test/src/bin/dfs_io_test.rs.html
title: test/src/bin/dfs_io_test.rs
---
