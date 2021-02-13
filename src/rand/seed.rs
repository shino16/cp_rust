// Codeforces
#[cfg(windows)]
mod detail {
    extern "system" {
        #[link_name = "SystemFunction036"]
        fn RtlGenRandom(buf: *mut u8, len: u32) -> u8;
    }
    fn getrand(buf: *mut u8, len: usize) {
        let ret = unsafe { RtlGenRandom(buf, len as u32) };
        assert_ne!(ret, 0);
    }
    macro_rules! def_seed {
        ($(pub fn $name:ident() -> $ret:ty;)*) => { $(
            pub fn $name() -> $ret {
                let mut v = std::mem::MaybeUninit::uninit();
                getrand(v.as_mut_ptr() as *mut u8, std::mem::size_of_val(&v));
                unsafe { v.assume_init() }
            }
        )* };
    }
    def_seed! {
        pub fn seed() -> [u64; 4];
        pub fn seed64() -> u64;
    }
}

#[cfg(not(windows))]
mod detail {
    pub fn seed() -> [u64; 4] {
        [
            // arbitrary
            0x35fee63b_fd9f69cf,
            0x9fd0680a_f9e37356,
            0x7454d5e3_d982527e,
            0x35d1849a_77925163,
        ]
    }
    pub fn seed64() -> u64 {
        0x17adfb20_0995921c
    }
}

pub use self::detail::*;
