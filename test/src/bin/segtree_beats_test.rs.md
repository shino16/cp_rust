---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':heavy_check_mark:'
    path: src/bits.rs
    title: src/bits.rs
  - icon: ':heavy_check_mark:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':heavy_check_mark:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/segtree/beats.rs
    title: src/ds/segtree/beats.rs
  - icon: ':heavy_check_mark:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':heavy_check_mark:'
    path: src/int/gcd.rs
    title: src/int/gcd.rs
  - icon: ':heavy_check_mark:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':heavy_check_mark:'
    path: src/iter.rs
    title: src/iter.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/880
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: test/src/bin/segtree_beats_test.rs\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/880\n\n\
    use lib::ds::segtree::beats::*;\nuse lib::int::gcd::*;\nuse lib::io::*;\nuse lib::iter::Itertools;\n\
    \n#[derive(Debug, Clone, Copy)]\nstruct E {\n    len: usize,\n    sum: u64,\n\
    \    max: u32,\n    lcm: u32,\n}\n\n#[derive(Debug, Clone, Copy)]\nenum F {\n\
    \    Asgn(u32),\n    Gcd(u32),\n    Unit,\n}\nuse F::*;\n\nstruct M;\nimpl Monoid<E>\
    \ for M {\n    fn unit(&self) -> E {\n        E { len: 0, sum: 0, max: 0, lcm:\
    \ 1 }\n    }\n    fn op(&self, x: E, y: E) -> E {\n        if x.len == 0 {\n \
    \           y\n        } else if y.len == 0 {\n            x\n        } else {\n\
    \            E {\n                len: x.len + y.len,\n                sum: x.sum\
    \ + y.sum,\n                max: x.max.max(y.max),\n                lcm: lcm(x.lcm,\
    \ y.lcm),\n            }\n        }\n    }\n}\n\nstruct A;\nimpl Monoid<F> for\
    \ A {\n    fn unit(&self) -> F {\n        Unit\n    }\n    fn op(&self, x: F,\
    \ y: F) -> F {\n        match y {\n            Asgn(_) => y,\n            Gcd(y)\
    \ => match x {\n                Asgn(a) => Asgn(gcd(a, y)),\n                Gcd(x)\
    \ => Gcd(gcd(x, y)),\n                _ => Gcd(y),\n            },\n         \
    \   _ => x,\n        }\n    }\n}\n\nfn lcm(x: u32, y: u32) -> u32 {\n    let lcm\
    \ = x as u64 * y as u64 / gcd(x, y) as u64;\n    (1 << 30).min(lcm) as u32\n}\n\
    \nfn fill(a: u32, len: usize) -> E {\n    E { len, sum: a as u64 * len as u64,\
    \ max: a, lcm: a }\n}\n\nfn act(e: E, a: F) -> Option<E> {\n    match a {\n  \
    \      Asgn(a) => Some(fill(a, e.len)),\n        Gcd(a) =>\n            if e.len\
    \ == 1 {\n                Some(fill(gcd(e.max, a), 1))\n            } else if\
    \ e.lcm != 1 << 30 && a % e.lcm == 0 {\n                Some(e)\n            }\
    \ else {\n                None\n            },\n        _ => Some(e),\n    }\n\
    }\n\nfn main() {\n    let mut io = IO::new();\n    let [n, q]: [usize; 2] = io.scan();\n\
    \    let a = io\n        .scan_iter::<u32>(n)\n        .map(|a| E { len: 1, sum:\
    \ a as u64, max: a, lcm: a })\n        .collect_vec();\n    let mut st = SegmentTreeBeats::from_slice(&a,\
    \ M, A, act);\n    for _ in 0..q {\n        let (c, Usize1(l), r) = io.scan();\n\
    \        match c {\n            1 => {\n                st.act_over(l, r, Asgn(io.scan()));\n\
    \            },\n            2 => {\n                st.act_over(l, r, Gcd(io.scan()));\n\
    \            },\n            3 => {\n                io.println(st.ask(l, r).max);\n\
    \            },\n            _ => {\n                io.println(st.ask(l, r).sum);\n\
    \            },\n        }\n    }\n}\n"
  dependsOn:
  - src/alg.rs
  - src/bits.rs
  - src/bounded.rs
  - src/cast.rs
  - src/ds/segtree/beats.rs
  - src/int.rs
  - src/int/gcd.rs
  - src/io.rs
  - src/iter.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: true
  path: test/src/bin/segtree_beats_test.rs
  requiredBy: []
  timestamp: '2021-02-20 13:37:47+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: test/src/bin/segtree_beats_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/segtree_beats_test.rs
- /verify/test/src/bin/segtree_beats_test.rs.html
title: test/src/bin/segtree_beats_test.rs
---
