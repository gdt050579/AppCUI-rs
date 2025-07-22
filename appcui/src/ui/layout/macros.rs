macro_rules! should_not_use {
    ($param:expr, $error:expr) => {
        if $param.is_some() {
            return Err($error);
        }
    };
}

macro_rules! should_use {
    ($param:expr, $error:expr) => {
        if $param.is_none() {
            return Err($error);
        }
    };
}

pub (super) use should_not_use;
pub (super) use should_use;