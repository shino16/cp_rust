macro_rules! tri {
    ($cond:expr; $a:expr; $b:expr) => {
        if $cond { $a } else { $b }
    };
}
