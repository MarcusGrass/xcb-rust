pub(crate) mod codegen;
pub(crate) mod relationship;
pub(crate) mod type_resolve;
pub(crate) mod types;

pub(crate) trait PushAny<T> {
    fn push(&mut self, s: T);
}

impl PushAny<&str> for String {
    #[inline]
    fn push(&mut self, s: &str) {
        self.push_str(s);
    }
}

impl PushAny<char> for String {
    #[inline]
    fn push(&mut self, s: char) {
        self.push(s);
    }
}

#[macro_export]
macro_rules! dump {
    ($buf:expr, $fmt:expr) => {
        $crate::generator::PushAny::push(&mut $buf, $fmt)
    };
    ($buf:expr, $fmt:expr, $($args:tt)*) => {
        let _ = $buf.write_fmt(format_args!($fmt, $($args)*));
    };
}
