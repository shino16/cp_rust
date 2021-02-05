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
  code: "use super::sort::*;\n\n// reference\n// K\xE4rkk\xE4inen, Juha, Peter Sanders,\
    \ and Stefan Burkhardt. \"Linear work suffix array construction.\" Journal of\
    \ the ACM (JACM) 53, no. 6, pp. 918-936, 2006.\npub fn suffix_array<T, F: FnMut(&T)\
    \ -> usize>(\n\tt: &[T],\n\tout: &mut Vec<usize>,\n\tmax_key: usize,\n\tmut key:\
    \ F,\n) {\n\tout.clear();\n\tout.reserve(t.len() + 1);\n\tout.push(t.len());\n\
    \n\tif t.len() == 0 {\n\t\treturn;\n\t} else if t.len() == 1 {\n\t\tout.push(1);\n\
    \t\treturn;\n\t}\n\n\tlet n = t.len();\n\tlet mut key_at = |i| if i >= n { 0 }\
    \ else { key(&t[i]) };\n\tlet (n0, n1, n2) = ((n + 2) / 3, (n + 1) / 3, n / 3);\n\
    \tlet n02 = n0 + n2;\n\tlet (mut r, mut sa12) = (Vec::with_capacity(n02 + 3),\
    \ Vec::with_capacity(n02));\n\tfor i in 0..n + (n0 - n1) {\n\t\tif i % 3 != 0\
    \ {\n\t\t\tr.push(i);\n\t\t}\n\t}\n\tassert_eq!(r.len(), n02);\n\n\t// radix sort\
    \ [(t[i], t[i + 1], t[i + 2]) | i % 3 != 0]\n\tcount_sort(&r, &mut sa12, max_key,\
    \ |&v| key_at(v + 2));\n\tcount_sort(&sa12, &mut r, max_key, |&v| key_at(v + 1));\n\
    \tcount_sort(&r, &mut sa12, max_key, |&v| key_at(v));\n\n\tlet (mut name, mut\
    \ c0, mut c1, mut c2) = (0, !0, !0, !0);\n\tfor &i in &sa12 {\n\t\tif key_at(i)\
    \ != c0 || key_at(i + 1) != c1 || key_at(i + 2) != c2 {\n\t\t\tname += 1;\n\t\t\
    \tc0 = key_at(i);\n\t\t\tc1 = key_at(i + 1);\n\t\t\tc2 = key_at(i + 2);\n\t\t\
    }\n\t\tif i % 3 == 1 {\n\t\t\tr[i / 3] = name;\n\t\t} else {\n\t\t\tr[i / 3 +\
    \ n0] = name;\n\t\t}\n\t}\n\tif name < n02 {\n\t\tfn deref(v: &usize) -> usize\
    \ {\n\t\t\t*v\n\t\t}\n\t\tsuffix_array(&r, &mut sa12, name, deref);\n\t\tfor (name,\
    \ &i) in (1..).zip(&sa12) {\n\t\t\tr[i] = name;\n\t\t}\n\t} else {\n\t\tfor (i,\
    \ &name) in (0..).zip(&r) {\n\t\t\tsa12[name - 1] = i;\n\t\t}\n\t}\n\n\tlet (mut\
    \ r0, mut sa0) = (Vec::with_capacity(n0), Vec::with_capacity(n0));\n\tfor &i in\
    \ &sa12 {\n\t\tif i < n0 {\n\t\t\tr0.push(i * 3);\n\t\t}\n\t}\n\tcount_sort(&r0,\
    \ &mut sa0, max_key, |&v| key_at(v));\n\n\t// sentinel\n\tr.extend_from_slice(&[0;\
    \ 3]);\n\tlet enc = |i| {\n\t\tif i < n0 { i * 3 + 1 } else { (i - n0) * 3 + 2\
    \ }\n\t};\n\tlet mut p = sa0.into_iter();\n\tlet mut q = sa12.into_iter().skip(n0\
    \ - n1);\n\tlet (mut pi, mut qi) = (p.next().unwrap(), q.next().unwrap());\n\t\
    loop {\n\t\tlet i = enc(qi);\n\t\tlet j = pi;\n\t\tlet cond = if qi < n0 {\n\t\
    \t\t(key_at(i), r[qi + n0]) <= (key_at(j), r[j / 3])\n\t\t} else {\n\t\t\t(key_at(i),\
    \ key_at(i + 1), r[qi - n0 + 1])\n\t\t\t\t<= (key_at(j), key_at(j + 1), r[j /\
    \ 3 + n0])\n\t\t};\n\t\tif cond {\n\t\t\tout.push(i);\n\t\t\tif let Some(q) =\
    \ q.next() {\n\t\t\t\tqi = q;\n\t\t\t} else {\n\t\t\t\tout.push(j);\n\t\t\t\t\
    out.extend(p);\n\t\t\t\treturn;\n\t\t\t}\n\t\t} else {\n\t\t\tout.push(j);\n\t\
    \t\tif let Some(p) = p.next() {\n\t\t\t\tpi = p;\n\t\t\t} else {\n\t\t\t\tout.push(i);\n\
    \t\t\t\tout.extend(q.map(enc));\n\t\t\t\treturn;\n\t\t\t}\n\t\t}\n\t}\n}\n"
  dependsOn:
  - src/slice/sort.rs
  isVerificationFile: false
  path: src/slice/sa.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-02-06 00:52:06+09:00'
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
