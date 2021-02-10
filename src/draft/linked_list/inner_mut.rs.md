---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/draft/linked_list.rs
    title: src/draft/linked_list.rs
  - icon: ':warning:'
    path: src/ds.rs
    title: src/ds.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/draft/linked_list/inner_mut.rs\n"
  code: "pub use super::*;\nuse std::cell::RefCell;\n\npub struct CursorInnerMut<'a,\
    \ T: 'a> {\n    at: NonNull<Node<T>>,\n    list: &'a RefCell<LinkedList<T>>,\n\
    }\n\nimpl<T> LinkedList<T> {\n    pub fn begin_inner_mut<'a>(list: &'a RefCell<Self>)\
    \ -> CursorInnerMut<'a, T> {\n        CursorInnerMut { at: list.borrow().head,\
    \ list }\n    }\n    pub fn end_inner_mut<'a>(list: &'a RefCell<Self>) -> CursorInnerMut<'a,\
    \ T> {\n        CursorInnerMut { at: list.borrow().tail, list }\n    }\n}\n\n\
    impl<'a, T: 'a> Deref for CursorInnerMut<'a, T> {\n    type Target = T;\n    fn\
    \ deref(&self) -> &Self::Target {\n        unsafe { &self.at.as_ref().next_val.as_ref().unwrap().1\
    \ }\n    }\n}\n\nimpl<'a, T: 'a> DerefMut for CursorInnerMut<'a, T> {\n    fn\
    \ deref_mut(&mut self) -> &mut Self::Target {\n        unsafe { &mut self.at.as_mut().next_val.as_mut().unwrap().1\
    \ }\n    }\n}\n\nimpl<'a, T: 'a> CursorInnerMut<'a, T> {\n    pub fn next(&mut\
    \ self) -> Option<&mut Self> {\n        self.at = unsafe { self.at.as_ref() }.next_val.as_ref()?.0;\n\
    \        Some(self)\n    }\n    pub fn prev(&mut self) -> Option<&mut Self> {\n\
    \        self.at = unsafe { self.at.as_ref() }.prev?;\n        Some(self)\n  \
    \  }\n    pub fn advance(&mut self, by: isize) -> Option<&mut Self> {\n      \
    \  if by >= 0 {\n            for _ in 0..by {\n                self.next()?;\n\
    \            }\n        } else {\n            for _ in by..0 {\n             \
    \   self.prev()?;\n            }\n        }\n        Some(self)\n    }\n    pub\
    \ fn insert(&mut self, val: T) {\n        let prev = unsafe { self.at.as_ref()\
    \ }.prev;\n        let new_node =\n            self.list.borrow_mut().new_node(Node\
    \ { prev, next_val: Some((self.at, val)) });\n        unsafe { self.at.as_mut()\
    \ }.prev = Some(new_node);\n        if let Some(mut prev) = prev {\n         \
    \   unsafe { prev.as_mut() }.next_val.as_mut().unwrap().0 = new_node;\n      \
    \  } else {\n            self.list.borrow_mut().head = new_node;\n        }\n\
    \        self.at = new_node;\n        self.list.borrow_mut().len += 1;\n    }\n\
    \    pub fn remove(&mut self) -> Option<T> {\n        if self.at == self.list.borrow_mut().tail\
    \ {\n            return None;\n        }\n        unsafe {\n            let node\
    \ = std::ptr::read(self.at.as_ptr());\n            let (mut next, val) = node.next_val?;\n\
    \            if let Some(mut prev) = node.prev {\n                *next.as_mut().prev.as_mut().unwrap()\
    \ = prev;\n                prev.as_mut().next_val.as_mut().unwrap().0 = next;\n\
    \            } else {\n                next.as_mut().prev = None;\n          \
    \      self.list.borrow_mut().head = next;\n            }\n            self.at\
    \ = next;\n            Some(val)\n        }\n    }\n}\n"
  dependsOn:
  - src/draft/linked_list.rs
  - src/ds.rs
  isVerificationFile: false
  path: src/draft/linked_list/inner_mut.rs
  requiredBy: []
  timestamp: '2021-02-09 16:24:56+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/linked_list/inner_mut.rs
layout: document
redirect_from:
- /library/src/draft/linked_list/inner_mut.rs
- /library/src/draft/linked_list/inner_mut.rs.html
title: src/draft/linked_list/inner_mut.rs
---
