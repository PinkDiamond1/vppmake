#[macro_export]
macro_rules! write_tag {
    ($f:expr, $tag:expr) => {
        write!($f, "[{}]", $tag)
    };
    ($f:expr, end $tag:expr) => {
        write!($f, "[/{}]", $tag)
    };
    ($f:expr, $tag:expr, $val:expr) => {
        write!($f, "[{}=\"{}\"]", $tag, $val)
    };
}

pub use write_tag;
