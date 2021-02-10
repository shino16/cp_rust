---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':question:'
    path: src/bits.rs
    title: src/bits.rs
  - icon: ':x:'
    path: src/ds/fenwick.rs
    title: src/ds/fenwick.rs
  - icon: ':x:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':x:'
    path: src/graph/tree.rs
    title: src/graph/tree.rs
  - icon: ':x:'
    path: src/graph/tree/dfs_io.rs
    title: src/graph/tree/dfs_io.rs
  - icon: ':question:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/778
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: test/src/bin/tree_dfs_io_test.rs\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/778\n\n\
    use lib::ds::fenwick::*;\nuse lib::graph::tree::dfs_io::*;\nuse lib::io::*;\n\n\
    fn main() {\n    let mut io = IO::new();\n    let n = io.scan();\n    let mut\
    \ graph = vec![Vec::new(); n];\n    for v in 1..n {\n        let p: usize = io.scan();\n\
    \        graph[p].push(v);\n        graph[v].push(p);\n    }\n    let mut fwk\
    \ = FenwickTree::new(vec![0; n], Addition::new());\n    let mut ans = 0;\n   \
    \ dfs_io(&graph, 0, |v, _| match v {\n        In(v) => {\n            ans += fwk.ask_prefix(v)\
    \ as u64;\n            fwk.add(v, 1_u32);\n        },\n        Out(v) => fwk.sub(v,\
    \ 1),\n    });\n    io.println(ans);\n}\n"
  dependsOn:
  - src/alg.rs
  - src/bits.rs
  - src/ds/fenwick.rs
  - src/graph.rs
  - src/graph/tree.rs
  - src/graph/tree/dfs_io.rs
  - src/io.rs
  - src/zo.rs
  isVerificationFile: true
  path: test/src/bin/tree_dfs_io_test.rs
  requiredBy: []
  timestamp: '2021-02-10 04:47:06+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: test/src/bin/tree_dfs_io_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/tree_dfs_io_test.rs
- /verify/test/src/bin/tree_dfs_io_test.rs.html
title: test/src/bin/tree_dfs_io_test.rs
---
