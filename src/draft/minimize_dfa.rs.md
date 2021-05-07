---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/draft/minimize_dfa.rs\n"
  code: "fn get_q() -> Vec<Vec<bool>> {\n    unimplemented!()\n}\n\nfn next_state(mut\
    \ s: Vec<bool>, op: Op) -> Vec<bool> {\n    unimplemented!()\n}\n\nfn is_accepted(v:\
    \ &Vec<bool>) -> bool {\n    unimplemented!()\n}\n\nfn run<F, G, H, State: Clone\
    \ + Ord, Alphabet: Clone>(\n    get_q: impl FnOnce() -> Vec<State>,\n    is_accepted:\
    \ impl Fn(&State) -> bool,\n    next_state: impl Fn(State, Alphabet) -> State,\n\
    \    alphabet: &[Alphabet],\n) -> Vec<BTreeSet<State>>\n{\n    let q = get_q();\n\
    \    let (f, notf) = q\n        .iter()\n        .cloned()\n        .partition::<BTreeSet<_>,\
    \ _>(|v| is_accepted(v));\n\n    let mut p = vec![f.clone(), notf.clone()];\n\
    \    let mut p2 = Vec::new();\n    let mut w = vec![f, notf];\n    while let Some(a)\
    \ = w.pop() {\n        for op in alphabet {\n            let x: BTreeSet<_> =\
    \ q\n                .iter()\n                .cloned()\n                .filter(|s|\
    \ a.get(&next_state(s.clone(), op.clone())).is_some())\n                .collect();\n\
    \            for y in p.drain(..) {\n                let inter = x.intersection(&y);\n\
    \                let inter: BTreeSet<_> = inter.cloned().collect();\n        \
    \        let imply = y.difference(&x);\n                let imply: BTreeSet<_>\
    \ = imply.cloned().collect();\n                if inter.is_empty() || imply.is_empty()\
    \ {\n                    p2.push(y);\n                    continue;\n        \
    \        }\n                p2.push(inter.clone());\n                p2.push(imply.clone());\n\
    \                if let Some((i, _)) = w.iter().enumerate().filter(|&(_, s)| s\
    \ == &y).next() {\n                    w.swap_remove(i);\n                   \
    \ w.push(inter);\n                    w.push(imply);\n                } else if\
    \ inter.len() <= imply.len() {\n                    w.push(inter);\n         \
    \       } else {\n                    w.push(imply);\n                }\n    \
    \        }\n            std::mem::swap(&mut p, &mut p2);\n        }\n    }\n \
    \   p.sort_unstable();\n    p\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/draft/minimize_dfa.rs
  requiredBy: []
  timestamp: '2021-03-31 15:51:17+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/minimize_dfa.rs
layout: document
redirect_from:
- /library/src/draft/minimize_dfa.rs
- /library/src/draft/minimize_dfa.rs.html
title: src/draft/minimize_dfa.rs
---
