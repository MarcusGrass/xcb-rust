use alloc::string::String;
use core::fmt::Formatter;

use tiny_std::error::Error as StdError;
use xcb_rust_protocol::proto::xproto::{SetupAuthenticate, SetupFailed};
use xcb_rust_protocol::Error;

/// An error that occurred while connecting to an X11 server
#[derive(Debug)]
#[non_exhaustive]
pub enum ConnectError {
    /// An unknown error occurred.
    ///
    /// One situation were this error is used when libxcb indicates an error that does not match
    /// any of the defined error conditions. Thus, libxcb is violating its own API (or new error
    /// cases were defined, but are not yet handled by x11rb).
    UnknownError,

    /// Out of memory.
    ///
    /// This is `XCB_CONN_CLOSED_MEM_INSUFFICIENT`.
    InsufficientMemory,

    /// Error during parsing of display string.
    ///
    /// This is `XCB_CONN_CLOSED_PARSE_ERR`.
    DisplayParsingError,

    /// Server does not have a screen matching the display.
    ///
    /// This is `XCB_CONN_CLOSED_INVALID_SCREEN`.
    InvalidScreen,

    /// Invalid ID mask provided by the server.
    ///
    /// The value of `resource_id_mask` in the `Setup` provided by the server was zero.
    ZeroIdMask,

    /// The server rejected the connection with a `SetupAuthenticate` message.
    SetupAuthenticate(SetupAuthenticate),

    /// The server rejected the connection with a `SetupFailed` message.
    SetupFailed(SetupFailed),

    /// The client did not receive enough data from the server to complete
    /// the handshake.
    Incomplete {
        /// The number of bytes that were expected.
        expected: usize,
        /// The number of bytes that were received.
        received: usize,
    },

    StdError(StdError),
    Syscall(rusl::Error),

    Proto(xcb_rust_protocol::Error),
    BadValue,
    Io(&'static str),
}

impl From<xcb_rust_protocol::Error> for ConnectError {
    fn from(e: Error) -> Self {
        Self::Proto(e)
    }
}

impl From<StdError> for ConnectError {
    fn from(e: StdError) -> Self {
        Self::StdError(e)
    }
}

impl From<rusl::Error> for ConnectError {
    fn from(e: rusl::Error) -> Self {
        Self::Syscall(e)
    }
}

impl core::fmt::Display for ConnectError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fn display(
            f: &mut core::fmt::Formatter<'_>,
            prefix: &str,
            value: &[u8],
        ) -> core::fmt::Result {
            match core::str::from_utf8(value).ok() {
                Some(value) => write!(f, "{prefix}: '{value}'"),
                None => write!(f, "{prefix}: {value:?} [message is not utf8]"),
            }
        }
        match self {
            ConnectError::UnknownError => write!(f, "Unknown connection error"),
            ConnectError::InsufficientMemory => write!(f, "Insufficient memory"),
            ConnectError::DisplayParsingError => write!(f, "Display parsing error"),
            ConnectError::InvalidScreen => write!(f, "Invalid screen"),
            ConnectError::ZeroIdMask => write!(f, "XID mask was zero"),
            ConnectError::SetupFailed(err) => display(f, "X11 setup failed", &err.reason),
            ConnectError::SetupAuthenticate(err) => {
                display(f, "X11 authentication failed", &err.reason)
            }
            ConnectError::Incomplete { .. } => write!(f, "Incomplete packet"),
            ConnectError::StdError(e) => write!(f, "Syscall: {e}"),
            ConnectError::BadValue => write!(f, "Tried to parse a bad value"),
            ConnectError::Proto(p) => write!(f, "{p}"),
            ConnectError::Syscall(e) => write!(f, "Syscall: {e}"),
            ConnectError::Io(e) => write!(f, "Delegate io failed: {e}"),
        }
    }
}

/// An error that occurred on an already established X11 connection
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ConnectionError {
    /// An unknown error occurred.
    ///
    /// One situation were this error is used when libxcb indicates an error that does not match
    /// any of the defined error conditions. Thus, libxcb is violating its own API (or new error
    /// cases were defined, but are not yet handled by x11rb).
    UnknownError(String),

    /// An X11 extension was not supported by the server.
    ///
    /// This corresponds to `XCB_CONN_CLOSED_EXT_NOTSUPPORTED`.
    UnsupportedExtension(String),

    /// A request larger than the maximum request length was sent.
    ///
    /// This corresponds to `XCB_CONN_CLOSED_REQ_LEN_EXCEED`.
    MaximumRequestLengthExceeded(String),

    /// Out of memory.
    ///
    /// This is `XCB_CONN_CLOSED_MEM_INSUFFICIENT`.
    InsufficientMemory,

    Protocol(xcb_rust_protocol::Error),

    StdErr(StdError),
    Syscall(rusl::Error),

    Id(IdError),
    Io(&'static str),
}

impl From<xcb_rust_protocol::Error> for ConnectionError {
    #[inline]
    fn from(e: Error) -> Self {
        Self::Protocol(e)
    }
}

impl From<rusl::Error> for ConnectionError {
    fn from(e: rusl::Error) -> Self {
        Self::Syscall(e)
    }
}

impl From<StdError> for ConnectionError {
    fn from(e: StdError) -> Self {
        Self::StdErr(e)
    }
}

impl From<IdError> for ConnectionError {
    #[inline]
    fn from(e: IdError) -> Self {
        Self::Id(e)
    }
}

impl core::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ConnectionError::UnknownError(s) => write!(f, "Unknown connection error {s}"),
            ConnectionError::UnsupportedExtension(s) => write!(f, "Unsupported extension {s}"),
            ConnectionError::InsufficientMemory => write!(f, "Insufficient memory"),
            ConnectionError::MaximumRequestLengthExceeded(s) => {
                write!(f, "Maximum request length exceeded {s}")
            }
            ConnectionError::Protocol(e) => {
                write!(f, "Protocol error {e}")
            }
            ConnectionError::Id(id) => {
                write!(f, "IdError {id}")
            }
            ConnectionError::StdErr(s) => {
                write!(f, "Std call failed {s:?}")
            }
            ConnectionError::Syscall(s) => {
                write!(f, "Syscall failed {s:?}")
            }
            ConnectionError::Io(e) => {
                write!(f, "Delegate io failed {e}")
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum IdError {
    IdsExhausted,
}

impl core::fmt::Display for IdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            IdError::IdsExhausted => f.write_str("Ids exhausted"),
        }
    }
}
