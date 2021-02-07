---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/slice/sort.rs
    title: src/slice/sort.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/slice/sa.rs\n"
  code: "use super::sort::*;\n\npub fn suffix_array(v: &mut Vec<u8>) -> Vec<usize>\
    \ {\n    let mut out = Vec::with_capacity(v.len());\n    v.extend_from_slice(&[0;\
    \ 3]);\n    suffix_array_impl(&v, &mut out, !0_u8 as usize, |&v| v as usize);\n\
    \    out\n}\n\n// reference\n// K\xE4rkk\xE4inen, Sanders and Burkhardt\n/// require\
    \ exactly 3 sentinels in the last\npub fn suffix_array_impl<T, F: FnMut(&T) ->\
    \ usize>(\n    t: &[T],\n    out: &mut Vec<usize>,\n    max_key: usize,\n    mut\
    \ key: F,\n) {\n    let n = t.len() - 3;\n\n    out.clear();\n    out.reserve(n\
    \ + 1);\n    out.push(n);\n\n    if n == 0 {\n        return;\n    } else if n\
    \ == 1 {\n        out.push(1);\n        return;\n    }\n    let (n0, n1, n2) =\
    \ ((n + 2) / 3, (n + 1) / 3, n / 3);\n    let n02 = n0 + n2;\n    let (mut r,\
    \ mut sa12) = (Vec::with_capacity(n02 + 3), Vec::with_capacity(n02));\n    for\
    \ i in 0..n + (n0 - n1) {\n        if i % 3 != 0 {\n            r.push(i);\n \
    \       }\n    }\n    debug_assert_eq!(r.len(), n02);\n\n    // radix sort [(t[i],\
    \ t[i + 1], t[i + 2]) | i % 3 != 0]\n    count_sort(&r, &mut sa12, max_key, |&v|\
    \ key(&t[v + 2]));\n    count_sort(&sa12, &mut r, max_key, |&v| key(&t[v + 1]));\n\
    \    count_sort(&r, &mut sa12, max_key, |&v| key(&t[v]));\n\n    let (mut name,\
    \ mut c0, mut c1, mut c2) = (0, !0, !0, !0);\n    for &i in &sa12 {\n        if\
    \ key(&t[i]) != c0 || key(&t[i + 1]) != c1 || key(&t[i + 2]) != c2 {\n       \
    \     name += 1;\n            c0 = key(&t[i]);\n            c1 = key(&t[i + 1]);\n\
    \            c2 = key(&t[i + 2]);\n        }\n        if i % 3 == 1 {\n      \
    \      r[i / 3] = name;\n        } else {\n            r[i / 3 + n0] = name;\n\
    \        }\n    }\n    if name < n02 {\n        fn deref(v: &usize) -> usize {\n\
    \            *v\n        }\n        r.extend_from_slice(&[0; 3]);\n        suffix_array_impl(&r,\
    \ &mut sa12, name, deref);\n        for (name, &i) in (1..).zip(&sa12) {\n   \
    \         r[i] = name;\n        }\n    } else {\n        for (i, &name) in (0..).zip(&r)\
    \ {\n            sa12[name - 1] = i;\n        }\n    }\n\n    let (mut r0, mut\
    \ sa0) = (Vec::with_capacity(n0), Vec::with_capacity(n0));\n    for &i in &sa12\
    \ {\n        if i < n0 {\n            r0.push(i * 3);\n        }\n    }\n    count_sort(&r0,\
    \ &mut sa0, max_key, |&v| key(&t[v]));\n\n    // sentinel\n    r.extend_from_slice(&[0;\
    \ 3]);\n    let enc = |i| {\n        if i < n0 { i * 3 + 1 } else { (i - n0) *\
    \ 3 + 2 }\n    };\n    let mut p = sa0.into_iter();\n    let mut q = sa12.into_iter().skip(n0\
    \ - n1);\n    let (mut pi, mut qi) = (p.next().unwrap(), q.next().unwrap());\n\
    \    loop {\n        let i = enc(qi);\n        let j = pi;\n        let cond =\
    \ if qi < n0 {\n            (key(&t[i]), r[qi + n0]) <= (key(&t[j]), r[j / 3])\n\
    \        } else {\n            (key(&t[i]), key(&t[i + 1]), r[qi - n0 + 1])\n\
    \                <= (key(&t[j]), key(&t[j + 1]), r[j / 3 + n0])\n        };\n\
    \        if cond {\n            out.push(i);\n            if let Some(q) = q.next()\
    \ {\n                qi = q;\n            } else {\n                out.push(j);\n\
    \                out.extend(p);\n                assert_eq!(out.len(), n + 1);\n\
    \                return;\n            }\n        } else {\n            out.push(j);\n\
    \            if let Some(p) = p.next() {\n                pi = p;\n          \
    \  } else {\n                out.push(i);\n                out.extend(q.map(enc));\n\
    \                assert_eq!(out.len(), n + 1);\n                return;\n    \
    \        }\n        }\n    }\n}\n"
  dependsOn:
  - src/slice/sort.rs
  isVerificationFile: false
  path: src/slice/sa.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/slice/sa.rs
layout: document
redirect_from:
- /library/src/slice/sa.rs
- /library/src/slice/sa.rs.html
title: src/slice/sa.rs
---
