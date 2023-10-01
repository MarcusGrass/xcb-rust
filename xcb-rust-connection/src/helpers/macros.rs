#[macro_export]
#[cfg(feature = "debug")]
macro_rules! debug {
    ($($arg:tt)*) => {{
        tiny_std::eprintln!("[{}:L#{}] {}", file!(), line!(), format_args!($($arg)*));
    }}
}
#[macro_export]
#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($($arg:tt)*) => {{}};
}
