---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':question:'
    path: src/as_int.rs
    title: src/as_int.rs
  - icon: ':question:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':x:'
    path: src/bitset.rs
    title: src/bitset.rs
  - icon: ':x:'
    path: src/cmp/total.rs
    title: src/cmp/total.rs
  - icon: ':question:'
    path: src/conv.rs
    title: src/conv.rs
  - icon: ':x:'
    path: src/dfa.rs
    title: src/dfa.rs
  - icon: ':x:'
    path: src/ds/disjointst.rs
    title: src/ds/disjointst.rs
  - icon: ':x:'
    path: src/ds/fenwick.rs
    title: src/ds/fenwick.rs
  - icon: ':question:'
    path: src/ds/segtree.rs
    title: src/ds/segtree.rs
  - icon: ':x:'
    path: src/ds/sparsetable.rs
    title: src/ds/sparsetable.rs
  - icon: ':question:'
    path: src/ds/uf.rs
    title: src/ds/uf.rs
  - icon: ':x:'
    path: src/ds/uslice.rs
    title: src/ds/uslice.rs
  - icon: ':question:'
    path: src/ds/uvec.rs
    title: src/ds/uvec.rs
  - icon: ':x:'
    path: src/ds/uvec2d.rs
    title: src/ds/uvec2d.rs
  - icon: ':x:'
    path: src/ds/vec2d.rs
    title: src/ds/vec2d.rs
  - icon: ':question:'
    path: src/fp.rs
    title: src/fp.rs
  - icon: ':question:'
    path: src/fp/conv.rs
    title: src/fp/conv.rs
  - icon: ':x:'
    path: src/fxhash.rs
    title: src/fxhash.rs
  - icon: ':x:'
    path: src/hash.rs
    title: src/hash.rs
  - icon: ':question:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':x:'
    path: src/io_interactive.rs
    title: src/io_interactive.rs
  - icon: ':question:'
    path: src/iter.rs
    title: src/iter.rs
  - icon: ':question:'
    path: src/iter/prod.rs
    title: src/iter/prod.rs
  - icon: ':x:'
    path: src/lib.rs
    title: src/lib.rs
  - icon: ':question:'
    path: src/make_vec.rs
    title: src/make_vec.rs
  - icon: ':question:'
    path: src/math/gcd.rs
    title: src/math/gcd.rs
  - icon: ':x:'
    path: src/math/modpow.rs
    title: src/math/modpow.rs
  - icon: ':question:'
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':question:'
    path: src/mint/conv.rs
    title: src/mint/conv.rs
  - icon: ':question:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':question:'
    path: src/rng.rs
    title: src/rng.rs
  - icon: ':x:'
    path: src/slice.rs
    title: src/slice.rs
  - icon: ':question:'
    path: src/tests.rs
    title: src/tests.rs
  - icon: ':x:'
    path: src/vec.rs
    title: src/vec.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0570
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "// verify-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0570\n\
    \nuse lib::dfa::*;\nuse lib::io::*;\nuse lib::modint::*;\n\n#[derive(Ord, PartialOrd,\
    \ Eq, PartialEq, Copy, Clone, Hash)]\nenum ZigZagState {\n    Initial,\n    First(u8),\n\
    \    Increasing(u8),\n    Decreasing(u8),\n}\n\nstruct ZigZag;\n\nimpl Dfa for\
    \ ZigZag {\n    type Alphabet = u8;\n    type State = Option<ZigZagState>;\n \
    \   fn init(&self) -> Self::State {\n        Some(ZigZagState::Initial)\n    }\n\
    \    fn next(&self, s: Self::State, a: Self::Alphabet, _: usize) -> Self::State\
    \ {\n        use ZigZagState::*;\n        if let Some(s) = s {\n            match\
    \ s {\n                Initial if a == b'0' => Some(Initial),\n              \
    \  Initial => Some(First(a)),\n                First(d) if d < a => Some(Increasing(a)),\n\
    \                First(d) if d > a => Some(Decreasing(a)),\n                Increasing(d)\
    \ if d > a => Some(Decreasing(a)),\n                Decreasing(d) if d < a =>\
    \ Some(Increasing(a)),\n                _ => None,\n            }\n        } else\
    \ {\n            None\n        }\n    }\n    fn accept(&self, s: Self::State)\
    \ -> bool {\n        s.is_some()\n    }\n    fn unsuccessful(&self, s: Self::State)\
    \ -> bool {\n        s.is_none()\n    }\n}\n\n#[derive(Default, Clone, Copy, PartialEq,\
    \ Eq)]\nstruct Modx;\n\nimpl Mod for Modx {\n    const P: u32 = 10000;\n    const\
    \ PHI: u32 = 4000;\n}\n\nfn main() {\n    let mut io = IO::new();\n    let [a0,\
    \ b]: [&[u8]; 2] = io.scan();\n    let m = io.scan();\n    let mut a = vec![b'0';\
    \ b.len()];\n    a[b.len() - a0.len()..].copy_from_slice(&a0);\n\n    let dfa\
    \ = And(ZigZag, And(MultipleOf(m), And(Leq(&b), Not(Lt(&a)))));\n    let alphabet\
    \ = \"0123456789\".as_bytes();\n    let ans: Mint<Modx> = dfa.count(a.len(), alphabet);\n\
    \    println!(\"{}\", ans);\n}\n"
  dependsOn:
  - src/lib.rs
  - src/alg.rs
  - src/as_int.rs
  - src/bit.rs
  - src/bitset.rs
  - src/cmp/total.rs
  - src/conv.rs
  - src/dfa.rs
  - src/ds/disjointst.rs
  - src/ds/fenwick.rs
  - src/ds/segtree.rs
  - src/ds/sparsetable.rs
  - src/ds/uf.rs
  - src/ds/uslice.rs
  - src/ds/uvec.rs
  - src/ds/uvec2d.rs
  - src/ds/vec2d.rs
  - src/fp.rs
  - src/fp/conv.rs
  - src/fxhash.rs
  - src/hash.rs
  - src/io.rs
  - src/io_interactive.rs
  - src/iter.rs
  - src/iter/prod.rs
  - src/make_vec.rs
  - src/math/gcd.rs
  - src/math/modpow.rs
  - src/mint.rs
  - src/mint/conv.rs
  - src/num.rs
  - src/rng.rs
  - src/slice.rs
  - src/tests.rs
  - src/vec.rs
  isVerificationFile: true
  path: test/src/bin/dfa_test.rs
  requiredBy: []
  timestamp: '2020-11-24 01:55:32+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: test/src/bin/dfa_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/dfa_test.rs
- /verify/test/src/bin/dfa_test.rs.html
title: test/src/bin/dfa_test.rs
---
