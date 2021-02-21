pub use crate::for_loop;

#[macro_export]
macro_rules! for_loop {
    ($init:stmt; $cond:expr; $upd:stmt; $block:block) => {
        $init
        while $cond {
            $block
            $upd
        }
    };
}
