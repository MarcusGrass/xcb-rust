#![cfg_attr(not(test), no_std)]
#![allow(
    clippy::useless_conversion,
    clippy::identity_op,
    clippy::unnecessary_cast,
    clippy::derive_partial_eq_without_eq,
    clippy::too_many_arguments,
    unused_variables,
    unused_mut,
    unsafe_code
)]

/// Some types has variable lengths that need to go on the head
extern crate alloc;

#[cfg(feature = "xproto")]
pub use con::XcbConnection;
#[cfg(feature = "xproto")]
pub use helpers::XcbEnv;

pub use crate::error::{Error, Result};
#[cfg(feature = "xproto")]
use crate::proto::xproto::{Keysym, Timestamp};

#[cfg(feature = "xproto")]
pub mod con;
pub mod connection;
#[cfg(feature = "xproto")]
pub mod cookie;
pub mod error;
#[cfg(feature = "xproto")]
pub mod helpers;
pub mod proto;
pub mod util;

/// The universal null resource or null atom parameter value for many core X requests
pub const NONE: u32 = 0;

/// This constant can be used for many parameters in `create_window`
pub const COPY_FROM_PARENT: u32 = 0;

/// This constant can be used for the depth parameter in `create_window`. It indicates to use the
/// parent window's depth.
pub const COPY_DEPTH_FROM_PARENT: u8 = 0;

/// This constant can be used for the class parameter in `create_window`. It indicates to use the
/// parent window's class.
pub const COPY_CLASS_FROM_PARENT: u16 = 0;

/// This constant can be used in most request that take a timestamp argument
#[cfg(feature = "xproto")]
pub const CURRENT_TIME: Timestamp = 0;

/// This constant can be used to fill unused entries in `Keysym` tables
#[cfg(feature = "xproto")]
pub const NO_SYMBOL: Keysym = 0;
