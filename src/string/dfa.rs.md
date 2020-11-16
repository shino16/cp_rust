---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "use std::cmp::Ordering;\nuse std::collections::HashMap;\nuse std::hash::Hash;\n\
    \npub trait Dfa {\n    type Alphabet;\n    type State;\n    fn init(&self) ->\
    \ Self::State;\n    fn next(&self, s: Self::State, a: Self::Alphabet, i: usize)\
    \ -> Self::State;\n    fn accept(&self, s: Self::State) -> bool;\n    fn successful(&self,\
    \ _: Self::State) -> bool {\n        false\n    }\n    fn unsuccessful(&self,\
    \ _: Self::State) -> bool {\n        false\n    }\n}\n\npub struct And<X, Y>(pub\
    \ X, pub Y);\n\nimpl<X: Dfa<Alphabet = A>, Y: Dfa<Alphabet = A>, A: Copy> Dfa\
    \ for And<X, Y> {\n    type Alphabet = A;\n    type State = (X::State, Y::State);\n\
    \    fn init(&self) -> Self::State {\n        (self.0.init(), self.1.init())\n\
    \    }\n    fn next(&self, (s0, s1): Self::State, a: Self::Alphabet, i: usize)\
    \ -> Self::State {\n        (self.0.next(s0, a, i), self.1.next(s1, a, i))\n \
    \   }\n    fn accept(&self, (s0, s1): Self::State) -> bool {\n        self.0.accept(s0)\
    \ && self.1.accept(s1)\n    }\n    fn successful(&self, (s0, s1): Self::State)\
    \ -> bool {\n        self.0.successful(s0) && self.1.successful(s1)\n    }\n \
    \   fn unsuccessful(&self, (s0, s1): Self::State) -> bool {\n        self.0.unsuccessful(s0)\
    \ || self.1.unsuccessful(s1)\n    }\n}\n\npub struct Not<X>(pub X);\n\nimpl<X:\
    \ Dfa> Dfa for Not<X> {\n    type Alphabet = X::Alphabet;\n    type State = X::State;\n\
    \    fn init(&self) -> Self::State {\n        self.0.init()\n    }\n    fn next(&self,\
    \ s: Self::State, a: Self::Alphabet, i: usize) -> Self::State {\n        self.0.next(s,\
    \ a, i)\n    }\n    fn accept(&self, s: Self::State) -> bool {\n        !self.0.accept(s)\n\
    \    }\n    fn successful(&self, s: Self::State) -> bool {\n        self.0.unsuccessful(s)\n\
    \    }\n    fn unsuccessful(&self, s: Self::State) -> bool {\n        self.0.successful(s)\n\
    \    }\n}\n\npub struct Lt<'a>(pub &'a [u8]);\n\nimpl Dfa for Lt<'_> {\n    type\
    \ Alphabet = u8;\n    type State = Ordering;\n    fn init(&self) -> Self::State\
    \ {\n        Ordering::Equal\n    }\n    // assumes i moves from 0 to self.0.len()\
    \ - 1\n    fn next(&self, s: Self::State, a: Self::Alphabet, i: usize) -> Self::State\
    \ {\n        s.then(a.cmp(&self.0[i]))\n    }\n    fn accept(&self, s: Self::State)\
    \ -> bool {\n        s == Ordering::Less\n    }\n    fn successful(&self, s: Self::State)\
    \ -> bool {\n        s == Ordering::Less\n    }\n    fn unsuccessful(&self, s:\
    \ Self::State) -> bool {\n        s == Ordering::Greater\n    }\n}\n\npub struct\
    \ Leq<'a>(pub &'a [u8]);\n\nimpl Dfa for Leq<'_> {\n    type Alphabet = u8;\n\
    \    type State = Ordering;\n    fn init(&self) -> Self::State {\n        Ordering::Equal\n\
    \    }\n    // assumes i moves from 0 to self.0.len() - 1\n    fn next(&self,\
    \ s: Self::State, a: Self::Alphabet, i: usize) -> Self::State {\n        s.then(a.cmp(&self.0[i]))\n\
    \    }\n    fn accept(&self, s: Self::State) -> bool {\n        s != Ordering::Greater\n\
    \    }\n    fn successful(&self, s: Self::State) -> bool {\n        s == Ordering::Less\n\
    \    }\n    fn unsuccessful(&self, s: Self::State) -> bool {\n        s == Ordering::Greater\n\
    \    }\n}\n\npub struct MultipleOf(pub u32);\n\nimpl Dfa for MultipleOf {\n  \
    \  type Alphabet = u8;\n    type State = u32;\n    fn init(&self) -> Self::State\
    \ {\n        0\n    }\n    fn next(&self, s: Self::State, a: Self::Alphabet, _:\
    \ usize) -> Self::State {\n        (s * 10 + (a - b'0') as u32) % self.0\n   \
    \ }\n    fn accept(&self, s: Self::State) -> bool {\n        s == 0\n    }\n}\n\
    \n#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]\npub enum ZigZagState\
    \ {\n    Initial,\n    First(u8),\n    Increasing(u8),\n    Decreasing(u8),\n\
    }\n\npub struct ZigZag;\n\nimpl Dfa for ZigZag {\n    type Alphabet = u8;\n  \
    \  type State = Option<ZigZagState>;\n    fn init(&self) -> Self::State {\n  \
    \      Some(ZigZagState::Initial)\n    }\n    fn next(&self, s: Self::State, a:\
    \ Self::Alphabet, _: usize) -> Self::State {\n        use ZigZagState::*;\n  \
    \      if let Some(s) = s {\n            match s {\n                Initial if\
    \ a == b'0' => Some(Initial),\n                Initial => Some(First(a)),\n  \
    \              First(d) if d < a => Some(Increasing(a)),\n                First(d)\
    \ if d > a => Some(Decreasing(a)),\n                Increasing(d) if d > a =>\
    \ Some(Decreasing(a)),\n                Decreasing(d) if d < a => Some(Increasing(a)),\n\
    \                _ => None,\n            }\n        } else {\n            None\n\
    \        }\n    }\n    fn accept(&self, s: Self::State) -> bool {\n        s.is_some()\n\
    \    }\n    fn unsuccessful(&self, s: Self::State) -> bool {\n        s.is_none()\n\
    \    }\n}\n\npub fn count<X: Dfa>(dfa: &X, n: usize, modulo: u32, alphabet: &[X::Alphabet])\
    \ -> u32\nwhere\n    X::Alphabet: Copy,\n    X::State: Eq + Hash + Copy,\n{\n\
    \    let mut dp = HashMap::new();\n    let mut dp2 = HashMap::new();\n    dp.insert(dfa.init(),\
    \ 1_u64);\n    for i in 0..n {\n        dp2.clear();\n        for (s, k) in dp.drain()\
    \ {\n            let k = k % modulo as u64;\n            for &a in alphabet {\n\
    \                let s1 = dfa.next(s, a, i);\n                if dfa.unsuccessful(s1)\
    \ {\n                    continue;\n                }\n                *dp2.entry(s1).or_insert(0)\
    \ += k;\n            }\n        }\n        std::mem::swap(&mut dp, &mut dp2);\n\
    \    }\n    let mut sum = 0;\n    let cap = dp.capacity().max(dp2.capacity());\n\
    \    dbg!(cap);\n    for (s, k) in dp {\n        if dfa.accept(s) {\n        \
    \    sum += k;\n            sum %= modulo as u64\n        }\n    }\n    sum as\
    \ u32\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/string/dfa.rs
  requiredBy: []
  timestamp: '2020-11-15 11:59:15+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/string/dfa.rs
layout: document
redirect_from:
- /library/src/string/dfa.rs
- /library/src/string/dfa.rs.html
title: src/string/dfa.rs
---
