#[macro_export]
macro_rules! uprint {
    ($($arg:tt)*) => {
        $crate::uart::write_fmt(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        uprint!(concat!($fmt, "\r\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        uprint!(concat!($fmt, "\r\n"), $($arg)*)
    };
}

#[macro_export]
macro_rules! print {
    ($fmt:expr) => {
        uprint!(concat!($fmt))
    };
    ($fmt:expr, $($arg:tt)*) => {
        uprint!(concat!($fmt), $($arg)*)
    };
}
