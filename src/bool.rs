pub trait Then: Sized {
    fn then<T>(self, f: impl FnOnce() -> T) -> Option<T>;
}

impl Then for bool {
    fn then<T>(self, f: impl FnOnce() -> T) -> Option<T> {
        if self { Some(f()) } else { None }
    }
}
