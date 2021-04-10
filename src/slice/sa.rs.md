---
data:
  _extendedDependsOn: []
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/slice/sa.rs\n"
  code: "pub fn sort_cyclic_shift<T: Copy + Default>(\n    s: &[T],\n    max: T,\n\
    \    mut key: impl FnMut(T) -> usize,\n) -> Vec<usize> {\n    let max_key = key(max);\n\
    \    let len = s.len();\n    let (mut p, mut p2) = (vec![!0; len], vec![!0; len]);\n\
    \    let (mut c, mut c2) = (vec![!0; len], vec![!0; len]);\n    let mut cnt =\
    \ vec![0; len.max(max_key)];\n    for &s in s {\n        cnt[key(s)] += 1;\n \
    \   }\n    cnt.iter_mut().fold(0, |s, e| {\n        *e += s;\n        *e\n   \
    \ });\n    for i in (0..len).rev() {\n        let c = &mut cnt[key(s[i])];\n \
    \       *c -= 1;\n        p[*c] = i;\n    }\n    let mut cls = 0;\n    c[p[0]]\
    \ = cls;\n    for i in 0..len - 1 {\n        if key(s[p[i]]) != key(s[p[i + 1]])\
    \ {\n            cls += 1;\n        }\n        c[p[i + 1]] = cls;\n    }\n   \
    \ cls += 1;\n    let mut done = 1;\n    while done < len {\n        cnt[..cls].iter_mut().for_each(|e|\
    \ *e = 0);\n        for (&p, p2) in p.iter().zip(p2.iter_mut()) {\n          \
    \  *p2 = if p < done { p + len - done } else { p - done };\n            cnt[c[*p2]]\
    \ += 1;\n        }\n        cnt.iter_mut().fold(0, |s, e| {\n            *e +=\
    \ s;\n            *e\n        });\n        for &p2 in p2.iter().rev() {\n    \
    \        let c = &mut cnt[c[p2]];\n            *c -= 1;\n            p[*c] = p2;\n\
    \        }\n        let rev = |p| if p + done < len { p + done } else { p + done\
    \ - len };\n        cls = 0;\n        c2[p[0]] = cls;\n        for (&p, &pp) in\
    \ p.iter().zip(p.iter().skip(1)) {\n            if c[p] != c[pp] || c[rev(p)]\
    \ != c[rev(pp)] {\n                cls += 1;\n            }\n            c2[pp]\
    \ = cls;\n        }\n        cls += 1;\n        std::mem::swap(&mut c, &mut c2);\n\
    \        done <<= 1;\n    }\n    p\n}\n\npub fn suffix_array<T: Copy + Default>(\n\
    \    s: &mut Vec<T>,\n    zero: T,\n    max: T,\n    key: impl FnMut(T) -> usize,\n\
    ) -> Vec<usize> {\n    s.push(zero);\n    let sa = sort_cyclic_shift(&s, max,\
    \ key);\n    s.pop();\n    sa\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/slice/sa.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-03-31 15:51:17+09:00'
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
