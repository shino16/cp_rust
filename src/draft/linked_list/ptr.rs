pub use super::*;

/// FIXME: double free
/// https://github.com/shino16/cpr/runs/1796042987?check_suite_focus=true#step:8:65
#[derive(Clone)]
pub struct CursorPtr<T> {
    at: NonNull<Node<T>>,
    list: NonNull<LinkedList<T>>,
}

impl<T> LinkedList<T> {
    pub fn begin_ptr<'a>(&mut self) -> CursorPtr<T> {
        CursorPtr { at: self.head, list: self.into() }
    }
    pub fn end_ptr<'a>(&mut self) -> CursorPtr<T> {
        CursorPtr { at: self.tail, list: self.into() }
    }
}

impl<T> Deref for CursorPtr<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.at.as_ref().next_val.as_ref().unwrap().1 }
    }
}

impl<T> DerefMut for CursorPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.at.as_mut().next_val.as_mut().unwrap().1 }
    }
}

impl<T> CursorPtr<T> {
    pub fn dangling() -> Self {
        Self { at: NonNull::dangling(), list: NonNull::dangling() }
    }
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
    /// safety: must not alias list, prev, at, next
    pub unsafe fn insert(&mut self, val: T) -> Self {
        let prev = self.at.as_ref().prev;
        let new_node =
            self.list.as_mut().new_node(Node { prev, next_val: Some((self.at, val)) });
        self.at.as_mut().prev = Some(new_node);
        if let Some(mut prev) = prev {
            prev.as_mut().next_val.as_mut().unwrap().0 = new_node;
        } else {
            self.list.as_mut().head = new_node;
        }
        self.at = new_node;
        self.list.as_mut().len += 1;
        let next = new_node.as_ref().next_val.as_ref().map(|t| t.0);
        assert_ne!(Some(new_node), next);
        Self { at: new_node, list: self.list }
    }
    /// safety: must not alias list, prev, at, next
    pub unsafe fn remove(&mut self) -> Option<T> {
        if self.at == self.list.as_mut().tail {
            return None;
        }
        let node = std::ptr::read(self.at.as_ptr());
        let (mut next, val) = node.next_val?;
        if let Some(mut prev) = node.prev {
            *next.as_mut().prev.as_mut().unwrap() = prev;
            prev.as_mut().next_val.as_mut().unwrap().0 = next;
        } else {
            next.as_mut().prev = None;
            self.list.as_mut().head = next;
        }
        self.at = next;
        self.list.as_mut().len -= 1;
        Some(val)
    }
}
