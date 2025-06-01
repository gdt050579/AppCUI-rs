macro_rules! should_not_use {
    ($param:expr, $msg:literal) => {
        if $param.is_some() {
            panic!($msg);
        }
    };
}

pub(super) use should_not_use;
