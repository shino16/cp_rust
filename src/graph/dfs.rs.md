---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/ds/bitset.rs
    title: src/ds/bitset.rs
  - icon: ':warning:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/dfs.rs\n"
  code: "pub use super::*;\nuse crate::ds::bitset::*;\n\n/// f: (v, par)\npub fn dfs<G:\
    \ Graph, F: FnMut(usize, usize)>(g: &G, s: usize, mut f: F) {\n\tlet mut togo\
    \ = vec![(s, !0)];\n\tlet mut visited = new_bitset(g.len());\n\tvisited.set_bit(s,\
    \ true);\n\twhile let Some((v, par)) = togo.pop() {\n\t\tf(v, par);\n\t\tg.adj(v,\
    \ |w| {\n\t\t\tif visited.modify_bit(w, true) {\n\t\t\t\ttogo.push((w, v));\n\t\
    \t\t}\n\t\t});\n\t}\n}\n\npub fn is_connected<G: Graph>(g: &G) -> bool {\n\tlet\
    \ mut cnt = 0;\n\tdfs(g, 0, |_, _| cnt += 1);\n\tcnt == g.len()\n}\n\npub fn dfs_ord_par<G:\
    \ Graph>(g: &G, s: usize) -> (Vec<(usize, usize)>, Vec<usize>) {\n\tlet mut ord\
    \ = Vec::with_capacity(g.len());\n\tlet mut par = vec![!0; g.len()];\n\tdfs(g,\
    \ s, |v, p| {\n\t\tord.push((v, p));\n\t\tpar[v] = p;\n\t});\n\t(ord, par)\n}\n"
  dependsOn:
  - src/ds/bitset.rs
  - src/graph.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/dfs.rs
  requiredBy: []
  timestamp: '2021-01-29 12:22:27+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/dfs.rs
layout: document
redirect_from:
- /library/src/graph/dfs.rs
- /library/src/graph/dfs.rs.html
title: src/graph/dfs.rs
---
