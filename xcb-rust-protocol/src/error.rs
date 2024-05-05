use crate::util::XcbError;
use core::array::TryFromSliceError;
use core::fmt::{Display, Formatter};
use core::num::TryFromIntError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Copy, Clone)]
pub enum Error {
    FromBytes,
    Serialize,
    TryFromInt,
    TryFromSlice,
    TooLargeRequest,
    MissingExtension(&'static str),
    XcbError(XcbError),
    Connection(&'static str),
    TinyStd(tiny_std::Error),
}

impl From<TryFromIntError> for Error {
    #[inline]
    fn from(_: TryFromIntError) -> Self {
        Self::TryFromInt
    }
}

impl From<TryFromSliceError> for Error {
    #[inline]
    fn from(_: TryFromSliceError) -> Self {
        Self::TryFromSlice
    }
}

impl From<tiny_std::Error> for Error {
    fn from(value: tiny_std::Error) -> Self {
        Self::TinyStd(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::FromBytes => f.write_str("Failed to deserialize from bytes"),
            Error::Serialize => f.write_str("Failed to serialize into"),
            Error::TryFromInt => f.write_str("Failed to convert number"),
            Error::TooLargeRequest => f.write_str("Failed to serialize request, too large to send"),
            Error::TryFromSlice => f.write_str("Failed to convert slice to number"),
            Error::XcbError(e) => f.write_str("Xcb Error"),
            Error::MissingExtension(ext) => f.write_fmt(format_args!("Missing extension {ext}")),
            Error::Connection(c) => f.write_fmt(format_args!("Connection err {c}")),
            Error::TinyStd(e) => f.write_fmt(format_args!("Tiny std error: {e}")),
        }
    }
}
