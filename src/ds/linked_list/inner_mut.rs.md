---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/ds/linked_list.rs
    title: src/ds/linked_list.rs
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
    RuntimeError: bundler is not specified: src/ds/linked_list/inner_mut.rs\n"
  code: "pub use super::*;\nuse std::cell::RefCell;\n\npub struct CursorInnerMut<'a,\
    \ T: 'a> {\n\tat: NonNull<Node<T>>,\n\tlist: &'a RefCell<LinkedList<T>>,\n}\n\n\
    impl<T> LinkedList<T> {\n\tpub fn begin_inner_mut<'a>(list: &'a RefCell<Self>)\
    \ -> CursorInnerMut<'a, T> {\n\t\tCursorInnerMut { at: list.borrow().head, list\
    \ }\n\t}\n\tpub fn end_inner_mut<'a>(list: &'a RefCell<Self>) -> CursorInnerMut<'a,\
    \ T> {\n\t\tCursorInnerMut { at: list.borrow().tail, list }\n\t}\n}\n\nimpl<'a,\
    \ T: 'a> Deref for CursorInnerMut<'a, T> {\n\ttype Target = T;\n\tfn deref(&self)\
    \ -> &Self::Target {\n\t\tunsafe { &self.at.as_ref().next_val.as_ref().unwrap().1\
    \ }\n\t}\n}\n\nimpl<'a, T: 'a> DerefMut for CursorInnerMut<'a, T> {\n\tfn deref_mut(&mut\
    \ self) -> &mut Self::Target {\n\t\tunsafe { &mut self.at.as_mut().next_val.as_mut().unwrap().1\
    \ }\n\t}\n}\n\nimpl<'a, T: 'a> CursorInnerMut<'a, T> {\n\tpub fn next(&mut self)\
    \ -> Option<&mut Self> {\n\t\tself.at = unsafe { self.at.as_ref() }.next_val.as_ref()?.0;\n\
    \t\tSome(self)\n\t}\n\tpub fn prev(&mut self) -> Option<&mut Self> {\n\t\tself.at\
    \ = unsafe { self.at.as_ref() }.prev?;\n\t\tSome(self)\n\t}\n\tpub fn advance(&mut\
    \ self, by: isize) -> Option<&mut Self> {\n\t\tif by >= 0 {\n\t\t\tfor _ in 0..by\
    \ {\n\t\t\t\tself.next()?;\n\t\t\t}\n\t\t} else {\n\t\t\tfor _ in by..0 {\n\t\t\
    \t\tself.prev()?;\n\t\t\t}\n\t\t}\n\t\tSome(self)\n\t}\n\tpub fn insert(&mut self,\
    \ val: T) {\n\t\tlet prev = unsafe { self.at.as_ref() }.prev;\n\t\tlet new_node\
    \ =\n\t\t\tself.list.borrow_mut().new_node(Node { prev, next_val: Some((self.at,\
    \ val)) });\n\t\tunsafe { self.at.as_mut() }.prev = Some(new_node);\n\t\tif let\
    \ Some(mut prev) = prev {\n\t\t\tunsafe { prev.as_mut() }.next_val.as_mut().unwrap().0\
    \ = new_node;\n\t\t} else {\n\t\t\tself.list.borrow_mut().head = new_node;\n\t\
    \t}\n\t\tself.at = new_node;\n\t\tself.list.borrow_mut().len += 1;\n\t}\n\tpub\
    \ fn remove(&mut self) -> Option<T> {\n\t\tif self.at == self.list.borrow_mut().tail\
    \ {\n\t\t\treturn None;\n\t\t}\n\t\tunsafe {\n\t\t\tlet node = std::ptr::read(self.at.as_ptr());\n\
    \t\t\tlet (mut next, val) = node.next_val?;\n\t\t\tif let Some(mut prev) = node.prev\
    \ {\n\t\t\t\t*next.as_mut().prev.as_mut().unwrap() = prev;\n\t\t\t\tprev.as_mut().next_val.as_mut().unwrap().0\
    \ = next;\n\t\t\t} else {\n\t\t\t\tnext.as_mut().prev = None;\n\t\t\t\tself.list.borrow_mut().head\
    \ = next;\n\t\t\t}\n\t\t\tself.at = next;\n\t\t\tSome(val)\n\t\t}\n\t}\n}\n"
  dependsOn:
  - src/ds/linked_list.rs
  isVerificationFile: false
  path: src/ds/linked_list/inner_mut.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-01-29 13:28:14+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/ds/linked_list/inner_mut.rs
layout: document
redirect_from:
- /library/src/ds/linked_list/inner_mut.rs
- /library/src/ds/linked_list/inner_mut.rs.html
title: src/ds/linked_list/inner_mut.rs
---
