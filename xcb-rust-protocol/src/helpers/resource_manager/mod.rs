//! X11 resource manager library.
//! The code in this module is only available when the `resource_manager` feature of the library is
//! enabled.

use alloc::string::ToString;
use protocol::Database;

use crate::con::{SocketIo, XcbState};
use crate::helpers::resource_manager::protocol::work_around_constant_limitations;
use crate::proto::xproto::{GetPropertyReply, GetPropertyTypeEnum};
use crate::Error;

pub mod protocol;

#[inline]
fn send_request<IO, XS>(io: &mut IO, state: &mut XS) -> Result<GetPropertyReply, Error>
where
    IO: SocketIo,
    XS: XcbState,
{
    let window = state.setup().roots[0].root;
    let cookie = crate::connection::xproto::get_property(
        io,
        state,
        0,
        window,
        work_around_constant_limitations::ATOM_RESOURCE_MANAGER,
        GetPropertyTypeEnum(work_around_constant_limitations::ATOM_STRING),
        0,
        // This is what Xlib does, so it must be correct (tm)
        100_000_000,
        false,
    )?;
    cookie.reply(io, state)
}

/// Create a new X11 resource database from the `RESOURCE_MANAGER` property of the first
/// screen's root window.
///
/// This function returns an error if the `GetProperty` request to get the `RESOURCE_MANAGER`
/// property fails. It returns `Ok(None)` if the property does not exist, has the wrong format,
/// or is empty.
pub fn new_from_resource_manager<IO, XS>(
    io: &mut IO,
    state: &mut XS,
) -> Result<Option<Database>, Error>
where
    IO: SocketIo,
    XS: XcbState,
{
    Ok(Database::new_from_get_property_reply(&send_request(
        io, state,
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
pub fn new_from_default<IO, XS>(
    io: &mut IO,
    state: &mut XS,
    home_dir: Option<&str>,
    xenvironment: Option<&str>,
) -> Result<Database, Error>
where
    IO: SocketIo,
    XS: XcbState,
{
    Ok(
        Database::new_from_default(
            &send_request(io, state)?,
            tiny_std::env::host_name().unwrap_or_else(|_| "localhost".to_string()),
            home_dir,
            xenvironment,
        ), // TODO: Maybe fix?
    )
}
