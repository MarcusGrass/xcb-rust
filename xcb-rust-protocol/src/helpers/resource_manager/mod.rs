//! X11 resource manager library.
//! The code in this module is only available when the `resource_manager` feature of the library is
//! enabled.

use alloc::string::ToString;
use protocol::Database;

use crate::connection::xproto::XprotoConnection;
use crate::helpers::resource_manager::protocol::work_around_constant_limitations;
use crate::proto::xproto::{GetPropertyReply, GetPropertyTypeEnum};
use crate::{Error, XcbConnection};

pub mod protocol;

fn send_request<C>(conn: &mut C, in_buffer: &mut [u8], out_buffer: &mut [u8]) -> Result<GetPropertyReply, Error>
where
    C: XcbConnection,
{
    let window = conn.setup().roots[0].root;
    let cookie = conn.get_property(
        out_buffer,
        0,
        window,
        work_around_constant_limitations::ATOM_RESOURCE_MANAGER,
        GetPropertyTypeEnum(work_around_constant_limitations::ATOM_STRING),
        0,
        // This is what Xlib does, so it must be correct (tm)
        100_000_000,
        false,
    )?;
    cookie.reply(conn, in_buffer, out_buffer)
}

/// Create a new X11 resource database from the `RESOURCE_MANAGER` property of the first
/// screen's root window.
///
/// This function returns an error if the `GetProperty` request to get the `RESOURCE_MANAGER`
/// property fails. It returns `Ok(None)` if the property does not exist, has the wrong format,
/// or is empty.
pub fn new_from_resource_manager<C>(
    conn: &mut C,
    in_buffer: &mut [u8],
    out_buffer: &mut [u8],
) -> Result<Option<Database>, Error>
where
    C: XcbConnection,
{
    Ok(Database::new_from_get_property_reply(&send_request(
        conn, in_buffer, out_buffer
    )?))
}

/// Create a new X11 resource database from the default locations.
///
/// The default location is a combination of two places. First, the following places are
/// searched for data:
/// - The `RESOURCE_MANAGER` property of the first screen's root window (See
///   [`Self::new_from_resource_manager`]).
/// - If not found, the file `$HOME/.Xresources` is loaded.
/// - If not found, the file `$HOME/.Xdefaults` is loaded.
///
/// The result of the above search of the above search is combined with:
/// - The contents of the file `$XENVIRONMENT`, if this environment variable is set.
/// - Otherwise, the contents of `$HOME/.Xdefaults-[hostname]`.
///
/// This function only returns an error if communication with the X11 server fails. All other
/// errors are ignored. It might be that an empty database is returned.
///
/// The behaviour of this function is mostly equivalent to Xlib's `XGetDefault()`. The
/// exception is that `XGetDefault()` does not load `$HOME/.Xresources`.
///
/// The behaviour of this function is equivalent to xcb-util-xrm's
/// `xcb_xrm_database_from_default()`.
pub fn new_from_default<C>(
    conn: &mut C,
    in_buffer: &mut [u8],
    out_buffer: &mut [u8],
    home_dir: Option<&str>,
    xenvironment: Option<&str>,
) -> Result<Database, Error>
where
    C: XcbConnection,
{
    Ok(
        Database::new_from_default(
            &send_request(conn, in_buffer, out_buffer)?,
            tiny_std::env::host_name().unwrap_or_else(|_| "localhost".to_string()),
            home_dir,
            xenvironment,
        ), // TODO: Maybe fix?
    )
}
