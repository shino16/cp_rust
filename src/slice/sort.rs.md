---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/slice/sa.rs
    title: src/slice/sa.rs
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
    RuntimeError: bundler is not specified: src/slice/sort.rs\n"
  code: "use std::mem::MaybeUninit;\n\npub fn count_sort_bytes(slice: &[u8], out:\
    \ &mut Vec<u8>, max_key: usize) {\n\tcount_sort(slice, out, max_key, |&x| x as\
    \ usize);\n}\n\npub fn count_sort<T: Clone, F: FnMut(&T) -> usize>(\n\tslice:\
    \ &[T],\n\tout: &mut Vec<T>,\n\tmax_key: usize,\n\tmut key: F,\n) {\n\tlet mut\
    \ count = vec![0; max_key + 1];\n\tfor e in slice {\n\t\tcount[key(e)] += 1;\n\
    \t}\n\tfor i in 0..max_key {\n\t\tcount[i + 1] += count[i];\n\t}\n\tout.clear();\n\
    \tout.reserve(slice.len());\n\t{\n\t\t// this is safe\n\t\t// refer to https://docs.rs/uninit/0.4.0/uninit/extension_traits/trait.VecCapacity.html#method.get_backing_buffer_with_leaking_writes\n\
    \t\tlet out: &mut [MaybeUninit<T>] =\n\t\t\tunsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr()\
    \ as _, slice.len()) };\n\t\tfor e in slice.iter().rev() {\n\t\t\tcount[key(e)]\
    \ -= 1;\n\t\t\tout[count[key(e)]] = MaybeUninit::new(e.clone());\n\t\t}\n\t}\n\
    \tunsafe {\n\t\tout.set_len(slice.len());\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/slice/sort.rs
  requiredBy:
  - src/slice/sa.rs
  - src/tests.rs
  timestamp: '2021-02-06 00:52:06+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/slice/sort.rs
layout: document
redirect_from:
- /library/src/slice/sort.rs
- /library/src/slice/sort.rs.html
title: src/slice/sort.rs
---
