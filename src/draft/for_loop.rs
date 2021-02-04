macro_rules! for_loop {
    ($init:stmt; $cond:expr; $upd:stmt; $block:block) => {
        $init
        while $cond {
            $block
            $upd
        }
    };
}
