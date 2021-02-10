---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/ds.rs
    title: src/ds.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/draft/linked_list/inner_mut.rs
    title: src/draft/linked_list/inner_mut.rs
  - icon: ':warning:'
    path: src/draft/linked_list/iter.rs
    title: src/draft/linked_list/iter.rs
  - icon: ':warning:'
    path: src/draft/linked_list/ptr.rs
    title: src/draft/linked_list/ptr.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/draft/linked_list.rs\n"
  code: "use std::iter::FromIterator;\nuse std::marker::PhantomData;\nuse std::ops::{Deref,\
    \ DerefMut};\nuse std::ptr::NonNull;\n\npub mod inner_mut;\npub mod ptr;\n\n///\
    \ FIXME: double free\n/// https://github.com/shino16/cpr/runs/1796088138?check_suite_focus=true#step:8:64\n\
    #[derive(PartialEq, PartialOrd, Hash)]\npub struct LinkedList<T> {\n    pub head:\
    \ NonNull<Node<T>>,\n    pub tail: NonNull<Node<T>>,\n    arenas: Vec<Vec<Node<T>>>,\n\
    \    arena_idx: usize,\n    len: usize,\n}\n\n#[derive(Debug, PartialEq, PartialOrd,\
    \ Clone, Copy, Hash, Default)]\npub struct Node<T> {\n    prev: Option<NonNull<Node<T>>>,\n\
    \    next_val: Option<(NonNull<Node<T>>, T)>,\n}\n\nimpl<T> Node<T> {\n    fn\
    \ new() -> Self {\n        Self { prev: None, next_val: None }\n    }\n}\n\npub\
    \ struct Iter<'a, T: 'a> {\n    head: NonNull<Node<T>>,\n    len: usize,\n   \
    \ _marker: PhantomData<&'a Node<T>>,\n}\n\npub struct IntoIter<T> {\n    list:\
    \ LinkedList<T>,\n}\n\npub struct CursorMut<'a, T: 'a> {\n    at: NonNull<Node<T>>,\n\
    \    list: &'a mut LinkedList<T>,\n}\n\nimpl<T> LinkedList<T> {\n    pub fn new()\
    \ -> Self {\n        let mut arenas = vec![vec![Node::new()]];\n        let head\
    \ = (&mut arenas[0][0]).into();\n        Self { head, tail: head, arenas, arena_idx:\
    \ 0, len: 0 }\n    }\n    pub fn with_capacity(cap: usize) -> Self {\n       \
    \ let mut arenas = vec![Vec::with_capacity(cap)];\n        arenas[0].push(Node::new());\n\
    \        let head = (&mut arenas[0][0]).into();\n        Self { head, tail: head,\
    \ arenas, arena_idx: 0, len: 0 }\n    }\n    pub fn len(&self) -> usize {\n  \
    \      self.len\n    }\n    pub fn is_empty(&self) -> bool {\n        self.head\
    \ == self.tail\n    }\n    pub fn clear(&mut self) {\n        self.drop_impl();\n\
    \        self.arena_idx = 0;\n    }\n    fn new_node(&mut self, node: Node<T>)\
    \ -> NonNull<Node<T>> {\n        let arena = &self.arenas[self.arena_idx];\n \
    \       if arena.len() == arena.capacity() {\n            self.arena_idx += 1;\n\
    \            if self.arena_idx == self.arenas.len() {\n                let new_arena\
    \ = Vec::with_capacity(arena.capacity() * 2);\n                self.arenas.push(new_arena);\n\
    \            }\n        }\n        let arena = &mut self.arenas[self.arena_idx];\n\
    \        arena.push(node);\n        arena.last_mut().unwrap().into()\n    }\n\
    \    pub fn begin_mut(&mut self) -> CursorMut<'_, T> {\n        CursorMut { at:\
    \ self.head, list: self }\n    }\n    pub fn end_mut(&mut self) -> CursorMut<'_,\
    \ T> {\n        CursorMut { at: self.tail, list: self }\n    }\n    pub fn push_front(&mut\
    \ self, val: T) {\n        self.begin_mut().insert(val)\n    }\n    pub fn push_back(&mut\
    \ self, val: T) {\n        self.end_mut().insert(val)\n    }\n    pub fn pop_front(&mut\
    \ self) -> Option<T> {\n        self.begin_mut().remove()\n    }\n    pub fn pop_back(&mut\
    \ self) -> Option<T> {\n        self.end_mut().prev()?.remove()\n    }\n    pub\
    \ fn iter(&self) -> Iter<'_, T> {\n        Iter { head: self.head, len: self.len,\
    \ _marker: PhantomData }\n    }\n    fn drop_impl(&mut self) {\n        for v\
    \ in &mut self.arenas[1..] {\n            unsafe {\n                v.set_len(0);\n\
    \            }\n        }\n        let mut cursor = self.begin_mut();\n      \
    \  while cursor.remove().is_some() {}\n    }\n}\n\nimpl<T> FromIterator<T> for\
    \ LinkedList<T> {\n    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self\
    \ {\n        let iter = iter.into_iter();\n        let mut res = Self::with_capacity(iter.size_hint().0);\n\
    \        for val in iter {\n            res.end_mut().insert(val);\n        }\n\
    \        res\n    }\n}\n\nimpl<T> IntoIterator for LinkedList<T> {\n    type Item\
    \ = T;\n    type IntoIter = IntoIter<T>;\n    fn into_iter(self) -> Self::IntoIter\
    \ {\n        IntoIter { list: self }\n    }\n}\n\nimpl<T: Clone + std::fmt::Debug>\
    \ Clone for LinkedList<T> {\n    fn clone(&self) -> Self {\n        self.iter().cloned().collect()\n\
    \    }\n}\n\nimpl<T> Drop for LinkedList<T> {\n    fn drop(&mut self) {\n    \
    \    self.drop_impl();\n    }\n}\n\nimpl<'a, T: 'a + std::fmt::Debug> Iterator\
    \ for Iter<'a, T> {\n    type Item = &'a T;\n    fn next(&mut self) -> Option<Self::Item>\
    \ {\n        let next_val = unsafe { &*self.head.as_ptr() }.next_val.as_ref()?;\n\
    \        let res = &next_val.1;\n        assert!(next_val.0 != self.head);\n \
    \       self.head = next_val.0;\n        Some(res)\n    }\n    fn size_hint(&self)\
    \ -> (usize, Option<usize>) {\n        (self.len, Some(self.len))\n    }\n}\n\n\
    impl<T> Iterator for IntoIter<T> {\n    type Item = T;\n    fn next(&mut self)\
    \ -> Option<Self::Item> {\n        self.list.pop_front()\n    }\n}\n\nimpl<'a,\
    \ T: 'a> Deref for CursorMut<'a, T> {\n    type Target = T;\n    fn deref(&self)\
    \ -> &Self::Target {\n        unsafe { &self.at.as_ref().next_val.as_ref().unwrap().1\
    \ }\n    }\n}\n\nimpl<'a, T: 'a> DerefMut for CursorMut<'a, T> {\n    fn deref_mut(&mut\
    \ self) -> &mut Self::Target {\n        unsafe { &mut self.at.as_mut().next_val.as_mut().unwrap().1\
    \ }\n    }\n}\n\nimpl<'a, T: 'a> CursorMut<'a, T> {\n    pub fn next(&mut self)\
    \ -> Option<&mut Self> {\n        self.at = unsafe { self.at.as_ref() }.next_val.as_ref()?.0;\n\
    \        Some(self)\n    }\n    pub fn prev(&mut self) -> Option<&mut Self> {\n\
    \        self.at = unsafe { self.at.as_ref() }.prev?;\n        Some(self)\n  \
    \  }\n    pub fn advance(&mut self, by: isize) -> Option<&mut Self> {\n      \
    \  if by >= 0 {\n            for _ in 0..by {\n                self.next()?;\n\
    \            }\n        } else {\n            for _ in by..0 {\n             \
    \   self.prev()?;\n            }\n        }\n        Some(self)\n    }\n    pub\
    \ fn insert(&mut self, val: T) {\n        let prev = unsafe { self.at.as_ref()\
    \ }.prev;\n        let new_node = self.list.new_node(Node { prev, next_val: Some((self.at,\
    \ val)) });\n        unsafe { self.at.as_mut() }.prev = Some(new_node);\n    \
    \    if let Some(mut prev) = prev {\n            unsafe { prev.as_mut() }.next_val.as_mut().unwrap().0\
    \ = new_node;\n        } else {\n            self.list.head = new_node;\n    \
    \    }\n        self.at = new_node;\n        self.list.len += 1;\n        unsafe\
    \ {\n            if let Some(prev) = self.at.as_ref().prev {\n               \
    \ assert!(Some(prev) != prev.as_ref().next_val.as_ref().map(|t| t.0));\n     \
    \       }\n            assert!(Some(self.at) != self.at.as_ref().next_val.as_ref().map(|t|\
    \ t.0));\n            if let Some((next, _)) = self.at.as_ref().next_val {\n \
    \               assert!(Some(next) != next.as_ref().next_val.as_ref().map(|t|\
    \ t.0));\n            }\n        }\n    }\n    pub fn remove(&mut self) -> Option<T>\
    \ {\n        if self.at == self.list.tail {\n            return None;\n      \
    \  }\n        unsafe {\n            let node = std::ptr::read(self.at.as_ptr());\n\
    \            let (mut next, val) = node.next_val?;\n            if let Some(mut\
    \ prev) = node.prev {\n                *next.as_mut().prev.as_mut().unwrap() =\
    \ prev;\n                prev.as_mut().next_val.as_mut().unwrap().0 = next;\n\
    \            } else {\n                next.as_mut().prev = None;\n          \
    \      self.list.head = next;\n            }\n            self.at = next;\n\n\
    \            if let Some(prev) = self.at.as_ref().prev {\n                assert!(Some(prev)\
    \ != prev.as_ref().next_val.as_ref().map(|t| t.0));\n            }\n         \
    \   assert!(Some(self.at) != self.at.as_ref().next_val.as_ref().map(|t| t.0));\n\
    \            if let Some((next, _)) = self.at.as_ref().next_val {\n          \
    \      assert!(Some(next) != next.as_ref().next_val.as_ref().map(|t| t.0));\n\
    \            }\n            Some(val)\n        }\n    }\n}\n\nimpl<T: std::fmt::Debug>\
    \ std::fmt::Debug for LinkedList<T> {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)\
    \ -> std::fmt::Result {\n        f.debug_list().entries(self.iter()).finish()\n\
    \    }\n}\n\n#[cfg(test)]\nmod test {\n    #[test]\n    fn test_linked_list()\
    \ {\n        use crate::ds::linked_list::inner_mut::*;\n        use std::cell::RefCell;\n\
    \        use std::sync::atomic::{AtomicU32, Ordering};\n\n        static DROP_CNT:\
    \ AtomicU32 = AtomicU32::new(0);\n        #[derive(PartialEq, Eq, Clone, Debug)]\n\
    \        struct S(u32);\n        impl Drop for S {\n            fn drop(&mut self)\
    \ {\n                DROP_CNT.fetch_add(1, Ordering::SeqCst);\n            }\n\
    \        }\n\n        let mut v = Vec::new();\n        let mut l = LinkedList::new();\n\
    \        let mut l2 = l.clone();\n        let mut cur = l2.begin_mut();\n    \
    \    for n in 0..10 {\n            v.push(Box::new(S(n)));\n            l.push_back(Box::new(S(n)));\n\
    \            cur.insert(Box::new(S(n)));\n            cur.next().unwrap();\n \
    \       }\n        assert_eq!(v, l.clone().into_iter().collect::<Vec<_>>());\n\
    \        assert_eq!(v, l2.into_iter().collect::<Vec<_>>());\n\n        let l =\
    \ RefCell::new(l);\n        let mut cur = LinkedList::begin_inner_mut(&l);\n \
    \       cur.advance(7).unwrap().remove();\n        v.remove(7);\n        assert_eq!(v,\
    \ l.borrow().clone().into_iter().collect::<Vec<_>>());\n        cur.advance(-2).unwrap().insert(Box::new(S(100)));\n\
    \        v.insert(5, Box::new(S(100)));\n        assert_eq!(v, l.borrow().clone().into_iter().collect::<Vec<_>>());\n\
    \        let mut cur = LinkedList::end_inner_mut(&l);\n        cur.advance(-8).unwrap().remove();\n\
    \        v.remove(2);\n        assert_eq!(v, l.borrow().clone().into_iter().collect::<Vec<_>>());\n\
    \        cur.advance(-2).unwrap();\n        assert!(cur.prev().is_none());\n \
    \       std::mem::drop((v, l));\n        assert_eq!(DROP_CNT.load(Ordering::SeqCst),\
    \ 70);\n    }\n    #[test]\n    fn test_linked_list_ptr() {\n        use crate::ds::linked_list::ptr::*;\n\
    \        use std::sync::atomic::{AtomicU32, Ordering};\n\n        static DROP_CNT:\
    \ AtomicU32 = AtomicU32::new(0);\n        #[derive(PartialEq, Eq, Clone, Debug)]\n\
    \        struct S(u32);\n        impl Drop for S {\n            fn drop(&mut self)\
    \ {\n                DROP_CNT.fetch_add(1, Ordering::SeqCst);\n            }\n\
    \        }\n\n        let mut v = Vec::new();\n        let mut l = LinkedList::new();\n\
    \        let mut l2 = l.clone();\n        let mut cur = l2.begin_mut();\n    \
    \    for n in 0..10 {\n            v.push(Box::new(S(n)));\n            l.push_back(Box::new(S(n)));\n\
    \            cur.insert(Box::new(S(n)));\n            cur.next().unwrap();\n \
    \       }\n        assert_eq!(v, l.clone().into_iter().collect::<Vec<_>>());\n\
    \        assert_eq!(v, l2.into_iter().collect::<Vec<_>>());\n\n        let mut\
    \ cur = l.begin_ptr();\n        unsafe { cur.advance(7).unwrap().remove(); }\n\
    \        v.remove(7);\n        assert_eq!(v, l.clone().into_iter().collect::<Vec<_>>());\n\
    \        unsafe { cur.advance(-2).unwrap().insert(Box::new(S(100))); }\n     \
    \   v.insert(5, Box::new(S(100)));\n        assert_eq!(v, l.clone().into_iter().collect::<Vec<_>>());\n\
    \        let mut cur = l.end_ptr();\n        unsafe { cur.advance(-8).unwrap().remove();\
    \ }\n        v.remove(2);\n        assert_eq!(v, l.clone().into_iter().collect::<Vec<_>>());\n\
    \        cur.advance(-2).unwrap();\n        assert!(cur.prev().is_none());\n \
    \       for (v, l) in v.iter().zip(l.iter()) {\n            assert_eq!(v, l);\n\
    \        }\n        std::mem::drop((v, l));\n        assert_eq!(DROP_CNT.load(Ordering::SeqCst),\
    \ 70);\n    }\n}"
  dependsOn:
  - src/ds.rs
  isVerificationFile: false
  path: src/draft/linked_list.rs
  requiredBy:
  - src/draft/linked_list/ptr.rs
  - src/draft/linked_list/iter.rs
  - src/draft/linked_list/inner_mut.rs
  timestamp: '2021-02-09 16:24:56+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/linked_list.rs
layout: document
redirect_from:
- /library/src/draft/linked_list.rs
- /library/src/draft/linked_list.rs.html
title: src/draft/linked_list.rs
---
