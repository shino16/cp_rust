pub use super::*;
use std::cell::RefCell;

pub struct CursorInnerMut<'a, T: 'a> {
    at: NonNull<Node<T>>,
    list: &'a RefCell<LinkedList<T>>,
}

impl<T> LinkedList<T> {
    pub fn begin_inner_mut<'a>(list: &'a RefCell<Self>) -> CursorInnerMut<'a, T> {
        CursorInnerMut { at: list.borrow().head, list }
    }
    pub fn end_inner_mut<'a>(list: &'a RefCell<Self>) -> CursorInnerMut<'a, T> {
        CursorInnerMut { at: list.borrow().tail, list }
    }
}

impl<'a, T: 'a> Deref for CursorInnerMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.at.as_ref().next_val.as_ref().unwrap().1 }
    }
}

impl<'a, T: 'a> DerefMut for CursorInnerMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.at.as_mut().next_val.as_mut().unwrap().1 }
    }
}

impl<'a, T: 'a> CursorInnerMut<'a, T> {
    pub fn next(&mut self) -> Option<&mut Self> {
        self.at = unsafe { self.at.as_ref() }.next_val.as_ref()?.0;
        Some(self)
    }
    pub fn prev(&mut self) -> Option<&mut Self> {
        self.at = unsafe { self.at.as_ref() }.prev?;
        Some(self)
    }
    pub fn advance(&mut self, by: isize) -> Option<&mut Self> {
        if by >= 0 {
            for _ in 0..by {
                self.next()?;
            }
        } else {
            for _ in by..0 {
                self.prev()?;
            }
        }
        Some(self)
    }
    pub fn insert(&mut self, val: T) {
        let prev = unsafe { self.at.as_ref() }.prev;
        let new_node =
            self.list.borrow_mut().new_node(Node { prev, next_val: Some((self.at, val)) });
        unsafe { self.at.as_mut() }.prev = Some(new_node);
        if let Some(mut prev) = prev {
            unsafe { prev.as_mut() }.next_val.as_mut().unwrap().0 = new_node;
        } else {
            self.list.borrow_mut().head = new_node;
        }
        self.at = new_node;
        self.list.borrow_mut().len += 1;
    }
    pub fn remove(&mut self) -> Option<T> {
        if self.at == self.list.borrow_mut().tail {
            return None;
        }
        unsafe {
            let node = std::ptr::read(self.at.as_ptr());
            let (mut next, val) = node.next_val?;
            if let Some(mut prev) = node.prev {
                *next.as_mut().prev.as_mut().unwrap() = prev;
                prev.as_mut().next_val.as_mut().unwrap().0 = next;
            } else {
                next.as_mut().prev = None;
                self.list.borrow_mut().head = next;
            }
            self.at = next;
            Some(val)
        }
    }
}
