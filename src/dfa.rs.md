---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/bits.rs
    title: src/bits.rs
  - icon: ':heavy_check_mark:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':heavy_check_mark:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':heavy_check_mark:'
    path: src/fxhash.rs
    title: src/fxhash.rs
  - icon: ':heavy_check_mark:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/rand/seed.rs
    title: src/rand/seed.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/dfa.rs\n"
  code: "use crate::fxhash::FxHashMap as HashMap;\nuse crate::int::*;\nuse std::cmp::Ordering;\n\
    use std::hash::Hash;\n\npub const DIGITS: &[u8] = b\"0123456789\";\n\npub trait\
    \ Dfa {\n    type Alphabet;\n    type State;\n    fn init(&self) -> Self::State;\n\
    \    fn next(&self, s: Self::State, a: Self::Alphabet, i: usize) -> Self::State;\n\
    \    fn accept(&self, s: Self::State) -> bool;\n    fn successful(&self, _: Self::State)\
    \ -> bool {\n        false\n    }\n    fn unsuccessful(&self, _: Self::State)\
    \ -> bool {\n        false\n    }\n\n    fn count<I: Num>(&self, n: usize, alphabet:\
    \ &[Self::Alphabet]) -> I\n    where\n        Self::Alphabet: Copy,\n        Self::State:\
    \ Eq + Hash + Copy,\n    {\n        count(self, n, alphabet)\n    }\n}\n\npub\
    \ struct And<X, Y>(pub X, pub Y);\n\nimpl<X: Dfa<Alphabet = A>, Y: Dfa<Alphabet\
    \ = A>, A: Copy> Dfa for And<X, Y> {\n    type Alphabet = A;\n    type State =\
    \ (X::State, Y::State);\n    fn init(&self) -> Self::State {\n        (self.0.init(),\
    \ self.1.init())\n    }\n    fn next(&self, (s0, s1): Self::State, a: Self::Alphabet,\
    \ i: usize) -> Self::State {\n        (self.0.next(s0, a, i), self.1.next(s1,\
    \ a, i))\n    }\n    fn accept(&self, (s0, s1): Self::State) -> bool {\n     \
    \   self.0.accept(s0) && self.1.accept(s1)\n    }\n    fn successful(&self, (s0,\
    \ s1): Self::State) -> bool {\n        self.0.successful(s0) && self.1.successful(s1)\n\
    \    }\n    fn unsuccessful(&self, (s0, s1): Self::State) -> bool {\n        self.0.unsuccessful(s0)\
    \ || self.1.unsuccessful(s1)\n    }\n}\n\npub struct Prod<X, Y>(pub X, pub Y);\n\
    \nimpl<X: Dfa, Y: Dfa> Dfa for Prod<X, Y> {\n    type Alphabet = (X::Alphabet,\
    \ Y::Alphabet);\n    type State = (X::State, Y::State);\n    fn init(&self) ->\
    \ Self::State {\n        (self.0.init(), self.1.init())\n    }\n    fn next(&self,\
    \ (s0, s1): Self::State, (a0, a1): Self::Alphabet, i: usize) -> Self::State {\n\
    \        (self.0.next(s0, a0, i), self.1.next(s1, a1, i))\n    }\n    fn accept(&self,\
    \ (s0, s1): Self::State) -> bool {\n        self.0.accept(s0) && self.1.accept(s1)\n\
    \    }\n    fn successful(&self, (s0, s1): Self::State) -> bool {\n        self.0.successful(s0)\
    \ && self.1.successful(s1)\n    }\n    fn unsuccessful(&self, (s0, s1): Self::State)\
    \ -> bool {\n        self.0.unsuccessful(s0) || self.1.unsuccessful(s1)\n    }\n\
    }\n\npub struct Not<X>(pub X);\n\nimpl<X: Dfa> Dfa for Not<X> {\n    type Alphabet\
    \ = X::Alphabet;\n    type State = X::State;\n    fn init(&self) -> Self::State\
    \ {\n        self.0.init()\n    }\n    fn next(&self, s: Self::State, a: Self::Alphabet,\
    \ i: usize) -> Self::State {\n        self.0.next(s, a, i)\n    }\n    fn accept(&self,\
    \ s: Self::State) -> bool {\n        !self.0.accept(s)\n    }\n    fn successful(&self,\
    \ s: Self::State) -> bool {\n        self.0.unsuccessful(s)\n    }\n    fn unsuccessful(&self,\
    \ s: Self::State) -> bool {\n        self.0.successful(s)\n    }\n}\n\npub struct\
    \ Lt<'a>(pub &'a [u8]);\n\nimpl Dfa for Lt<'_> {\n    type Alphabet = u8;\n  \
    \  type State = Ordering;\n    fn init(&self) -> Self::State {\n        Ordering::Equal\n\
    \    }\n    fn next(&self, s: Self::State, a: Self::Alphabet, i: usize) -> Self::State\
    \ {\n        s.then(a.cmp(&self.0[i]))\n    }\n    fn accept(&self, s: Self::State)\
    \ -> bool {\n        s == Ordering::Less\n    }\n    fn successful(&self, s: Self::State)\
    \ -> bool {\n        s == Ordering::Less\n    }\n    fn unsuccessful(&self, s:\
    \ Self::State) -> bool {\n        s == Ordering::Greater\n    }\n}\n\npub struct\
    \ Leq<'a>(pub &'a [u8]);\n\nimpl Dfa for Leq<'_> {\n    type Alphabet = u8;\n\
    \    type State = Ordering;\n    fn init(&self) -> Self::State {\n        Ordering::Equal\n\
    \    }\n    fn next(&self, s: Self::State, a: Self::Alphabet, i: usize) -> Self::State\
    \ {\n        s.then(a.cmp(&self.0[i]))\n    }\n    fn accept(&self, s: Self::State)\
    \ -> bool {\n        s != Ordering::Greater\n    }\n    fn successful(&self, s:\
    \ Self::State) -> bool {\n        s == Ordering::Less\n    }\n    fn unsuccessful(&self,\
    \ s: Self::State) -> bool {\n        s == Ordering::Greater\n    }\n}\n\npub struct\
    \ MultipleOf(pub u32);\n\nimpl Dfa for MultipleOf {\n    type Alphabet = u8;\n\
    \    type State = u32;\n    fn init(&self) -> Self::State {\n        0\n    }\n\
    \    fn next(&self, s: Self::State, a: Self::Alphabet, _: usize) -> Self::State\
    \ {\n        (s * 10 + (a - b'0') as u32) % self.0\n    }\n    fn accept(&self,\
    \ s: Self::State) -> bool {\n        s == 0\n    }\n}\n\nfn count<I: Num, X: Dfa\
    \ + ?Sized>(dfa: &X, n: usize, alphabet: &[X::Alphabet]) -> I\nwhere\n    X::Alphabet:\
    \ Copy,\n    X::State: Eq + Hash + Copy,\n{\n    let mut dp = HashMap::default();\n\
    \    let mut dp2 = HashMap::default();\n    dp.insert(dfa.init(), I::ONE);\n \
    \   for i in 0..n {\n        dp2.clear();\n        for (s, k) in dp.drain() {\n\
    \            for &a in alphabet {\n                let s1 = dfa.next(s, a, i);\n\
    \                if dfa.unsuccessful(s1) {\n                    continue;\n  \
    \              }\n                *dp2.entry(s1).or_insert(I::ZERO) += k;\n  \
    \          }\n        }\n        std::mem::swap(&mut dp, &mut dp2);\n    }\n \
    \   let mut sum = I::ZERO;\n    for (s, k) in dp {\n        if dfa.accept(s) {\n\
    \            sum += k;\n        }\n    }\n    sum\n}\n"
  dependsOn:
  - src/bits.rs
  - src/bounded.rs
  - src/cast.rs
  - src/fxhash.rs
  - src/int.rs
  - src/num.rs
  - src/rand/seed.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/dfa.rs
  requiredBy: []
  timestamp: '2021-02-10 04:47:06+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/dfa_test.rs
documentation_of: src/dfa.rs
layout: document
redirect_from:
- /library/src/dfa.rs
- /library/src/dfa.rs.html
title: src/dfa.rs
---
