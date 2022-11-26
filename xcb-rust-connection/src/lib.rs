#![cfg_attr(not(test), no_std)]
#![allow(unused_variables, unused, unused_imports, clippy::missing_panics_doc)]

extern crate alloc;

pub mod connection;
pub mod helpers;

pub use helpers::errors::{ConnectError, ConnectionError, IdError};
