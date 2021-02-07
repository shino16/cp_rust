pub trait Then: Sized {
    fn then<T, F: FnOnce() -> T>(self, f: F) -> Option<T>;
}

impl Then for bool {
    fn then<T, F: FnOnce() -> T>(self, f: F) -> Option<T> {
        if self { Some(f()) } else { None }
    }
}
