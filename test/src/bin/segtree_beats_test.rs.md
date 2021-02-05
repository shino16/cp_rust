---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':heavy_check_mark:'
    path: src/bit.rs
    title: src/bit.rs
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
    \n#[derive(Debug, Clone, Copy)]\nstruct E {\n\tlen: usize,\n\tsum: u64,\n\tmax:\
    \ u32,\n\tlcm: u32,\n}\n\n#[derive(Debug, Clone, Copy)]\nenum F {\n\tAsgn(u32),\n\
    \tGcd(u32),\n\tUnit,\n}\nuse F::*;\n\nstruct M;\nimpl Monoid for M {\n\ttype Item\
    \ = E;\n\tfn unit(&self) -> Self::Item {\n\t\tE { len: 0, sum: 0, max: 0, lcm:\
    \ 1 }\n\t}\n\tfn op(&self, x: Self::Item, y: Self::Item) -> Self::Item {\n\t\t\
    if x.len == 0 {\n\t\t\ty\n\t\t} else if y.len == 0 {\n\t\t\tx\n\t\t} else {\n\t\
    \t\tE {\n\t\t\t\tlen: x.len + y.len,\n\t\t\t\tsum: x.sum + y.sum,\n\t\t\t\tmax:\
    \ x.max.max(y.max),\n\t\t\t\tlcm: lcm(x.lcm, y.lcm),\n\t\t\t}\n\t\t}\n\t}\n}\n\
    \nstruct A;\nimpl Monoid for A {\n\ttype Item = F;\n\tfn unit(&self) -> Self::Item\
    \ {\n\t\tUnit\n\t}\n\tfn op(&self, x: Self::Item, y: Self::Item) -> Self::Item\
    \ {\n\t\tmatch y {\n\t\t\tAsgn(_) => y,\n\t\t\tGcd(y) => match x {\n\t\t\t\tAsgn(a)\
    \ => Asgn(gcd(a, y)),\n\t\t\t\tGcd(x) => Gcd(gcd(x, y)),\n\t\t\t\t_ => Gcd(y),\n\
    \t\t\t},\n\t\t\t_ => x,\n\t\t}\n\t}\n}\n\nfn lcm(x: u32, y: u32) -> u32 {\n\t\
    let lcm = x as u64 * y as u64 / gcd(x, y) as u64;\n\t(1 << 30).min(lcm) as u32\n\
    }\n\nfn fill(a: u32, len: usize) -> E {\n\tE { len, sum: a as u64 * len as u64,\
    \ max: a, lcm: a }\n}\n\nfn act(e: E, a: F) -> Option<E> {\n\tmatch a {\n\t\t\
    Asgn(a) => Some(fill(a, e.len)),\n\t\tGcd(a) =>\n\t\t\tif e.len == 1 {\n\t\t\t\
    \tSome(fill(gcd(e.max, a), 1))\n\t\t\t} else if e.lcm != 1 << 30 && a % e.lcm\
    \ == 0 {\n\t\t\t\tSome(e)\n\t\t\t} else {\n\t\t\t\tNone\n\t\t\t},\n\t\t_ => Some(e),\n\
    \t}\n}\n\nfn main() {\n\tlet mut io = IO::new();\n\tlet [n, q]: [usize; 2] = io.scan();\n\
    \tlet a = io\n\t\t.scan_iter::<u32>(n)\n\t\t.map(|a| E { len: 1, sum: a as u64,\
    \ max: a, lcm: a })\n\t\t.collect_vec();\n\tlet mut st = SegmentTreeBeats::from_slice(&a,\
    \ M, A, act);\n\tfor _ in 0..q {\n\t\tlet (c, Usize1(l), r) = io.scan();\n\t\t\
    match c {\n\t\t\t1 => {\n\t\t\t\tst.act_over(l, r, Asgn(io.scan()));\n\t\t\t},\n\
    \t\t\t2 => {\n\t\t\t\tst.act_over(l, r, Gcd(io.scan()));\n\t\t\t},\n\t\t\t3 =>\
    \ {\n\t\t\t\tio.println(st.ask(l, r).max);\n\t\t\t},\n\t\t\t_ => {\n\t\t\t\tio.println(st.ask(l,\
    \ r).sum);\n\t\t\t},\n\t\t}\n\t}\n}\n"
  dependsOn:
  - src/alg.rs
  - src/bit.rs
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
  timestamp: '2021-02-06 00:52:06+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: test/src/bin/segtree_beats_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/segtree_beats_test.rs
- /verify/test/src/bin/segtree_beats_test.rs.html
title: test/src/bin/segtree_beats_test.rs
---
