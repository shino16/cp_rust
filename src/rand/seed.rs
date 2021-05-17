#[cfg(unix)]
mod detail {
    extern "system" {
        #[link_name = "getrandom"]
        fn getrandom(buf: *mut u8, len: usize, flags: usize) -> isize;
    }
    pub(super) fn fill_bytes(buf: *mut u8, len: usize) {
        let ret = unsafe { getrandom(buf, len, 0) };
        assert_eq!(ret, len as isize);
    }
}

#[cfg(windows)]
mod detail {
    extern "system" {
        #[link_name = "SystemFunction036"]
        fn RtlGenRandom(buf: *mut u8, len: u32) -> u8;
    }
    pub(super) fn fill_bytes(buf: *mut u8, len: usize) {
        let ret = unsafe { RtlGenRandom(buf, len as u32) };
        assert_ne!(ret, 0);
    }
}

macro_rules! def_seed {
    ($(pub fn $name:ident() -> $ret:ty;)*) => { $(
        pub fn $name() -> $ret {
            let mut v = std::mem::MaybeUninit::uninit();
            self::detail::fill_bytes(v.as_mut_ptr() as *mut u8, std::mem::size_of_val(&v));
            unsafe { v.assume_init() }
        }
    )* };
}

def_seed! {
    pub fn seed() -> [u64; 4];
    pub fn seed64() -> u64;
}
