//! X11 resource manager library.
//!
//! To open a database, it is recommended to use [`Database::new_from_default`], but that function
//! needs to do I/O. A wrapper to simplify usage is e.g. provided in the x11rb crate.
//!
//! This functionality is similar to what is available to C code through xcb-util-xrm and Xlib's
//! `Xrm*` function family. Not all their functionality is available in this library. Please open a
//! feature request if you need something that is not available.
//!
//! The code in this module is only available when the `resource_manager` feature of the library is
//! enabled.
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::str::FromStr;
use tiny_std::{UnixStr, UnixString};

use crate::proto::xproto::GetPropertyReply;

mod matcher;
mod parser;

/// Maximum nesting of #include directives, same value as Xlib uses.
/// After following this many `#include` directives, further includes are ignored.
const MAX_INCLUSION_DEPTH: u8 = 100;

/// How tightly does the component of an entry match a query?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Binding {
    /// We have a tight match, meaning that the next component of the entry must match the query.
    Tight,
    /// We have a loose match, meaning that any number of components can be skipped before the next
    /// match.
    Loose,
}

/// A component of a database entry.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Component {
    /// A string component
    Normal(String),
    // Actually just a-z, A-Z, 0-9 and _ or - is allowed
    /// A wildcard component ("?") that matches anything
    Wildcard,
}

/// A single entry in the resource manager database.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Entry {
    /// The components of the entry describe which queries it matches
    components: Vec<(Binding, Component)>,
    /// The value of the entry is what the caller gets after a match.
    value: Vec<u8>,
}

pub(crate) mod work_around_constant_limitations {
    // For GET_RESOURCE_DATABASE, we need Into::into() to use AtomEnum, but that is not const.
    // This module exists to work around that.

    pub(crate) const ATOM_RESOURCE_MANAGER: u32 = 23;
    pub(crate) const ATOM_STRING: u32 = 31;

    #[test]
    fn constants_are_correct() {
        use crate::proto::xproto::AtomEnum;
        assert_eq!(
            u32::from(AtomEnum::RESOURCE_MANAGER.0),
            ATOM_RESOURCE_MANAGER
        );
        assert_eq!(u32::from(AtomEnum::STRING.0), ATOM_STRING);
    }
}

/// A X11 resource database.
///
/// The recommended way to load a database is through [`Database::new_from_default`].
#[derive(Debug, Default, Clone)]
pub struct Database {
    entries: Vec<Entry>,
}

impl Database {
    /// The GetPropertyRequest to load the X11 resource database from the root window.
    ///
    /// Copy this struct, set its `window` field to the root window of the first screen send the
    /// resulting request to the X11 server. The reply can be passed to
    /// [`new_from_default`].
    /*
    pub const GET_RESOURCE_DATABASE: GetPropertyRequest = GetPropertyRequest {
        delete: false,
        window: 0,
        property: work_around_constant_limitations::ATOM_RESOURCE_MANAGER,
        type_: work_around_constant_limitations::ATOM_STRING,
        long_offset: 0,
        // This is what Xlib does, so it must be correct (tm)
        long_length: 100_000_000,
    };

     */

    /// Create a new X11 resource database from the default locations.
    ///
    /// The `reply` argument should come from [`GET_RESOURCE_DATABASE`] with its `window` field set
    /// to the window ID of the first root window. The `hostname` argument should be the hostname
    /// of the running system.
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
    pub fn new_from_default(
        reply: &GetPropertyReply,
        hostname: String,
        home_dir: Option<&UnixStr>,
        xenvironment: Option<&UnixStr>,
    ) -> Result<Self, tiny_std::Error> {
        let cur_dir = String::from(".");

        // 1. Try to load the RESOURCE_MANAGER property
        let mut entries = if let Some(db) = Self::new_from_get_property_reply(reply)? {
            db.entries
        } else {
            let mut entries = Vec::new();
            if let Some(home) = home_dir {
                // 2. Otherwise, try to load $HOME/.Xresources
                let mut path = String::from(home.as_str()?);
                path.push_str("/.Xresources\0");
                let read_something =
                    if let Ok(data) = tiny_std::fs::read(UnixStr::try_from_str(&path)?) {
                        parse_data_with_base_directory(&mut entries, &data, home, 0)?;
                        true
                    } else {
                        false
                    };
                // Restore the path so it refers to $HOME again
                let _ = path.pop();

                if !read_something {
                    // 3. Otherwise, try to load $HOME/.Xdefaults
                    path.push_str("/.Xdefaults\0");
                    if let Ok(data) = tiny_std::fs::read(UnixStr::try_from_str(&path)?) {
                        parse_data_with_base_directory(&mut entries, &data, home, 0)?;
                    }
                }
            }
            entries
        };

        // 4. If XENVIRONMENT is specified, merge the database defined by that file
        if let Some(xenv) = xenvironment {
            if let Ok(data) = tiny_std::fs::read(xenv) {
                let base = xenv
                    .as_str()?
                    .rsplit_once('/')
                    .map(|s| s.0)
                    .unwrap_or(&cur_dir);
                let base_unix = UnixString::try_from_str(base)?;
                parse_data_with_base_directory(&mut entries, &data, &base_unix, 0)?;
            }
        } else {
            // 5. Load `$HOME/.Xdefaults-[hostname]`
            let mut file = String::from(".Xdefaults-");
            file.push_str(&hostname);
            let base_path = match home_dir {
                Some(home) => home,
                None => UnixStr::EMPTY,
            };
            let base_path_utf8 = base_path.as_str()?;
            let path = format!("{base_path_utf8}/{file}\0");
            let path_unix = UnixString::try_from_str(&path)?;
            if let Ok(data) = tiny_std::fs::read(&path_unix) {
                parse_data_with_base_directory(&mut entries, &data, base_path, 0)?;
            }
        }

        Ok(Self { entries })
    }

    /// Construct a new X11 resource database from a [`GetPropertyReply`].
    ///
    /// The reply should come from [`GET_RESOURCE_DATABASE`] with its `window` field set to the
    /// window ID of the first root window.
    pub fn new_from_get_property_reply(
        reply: &GetPropertyReply,
    ) -> Result<Option<Database>, tiny_std::Error> {
        if reply.format == 8 && !reply.value.is_empty() {
            Ok(Some(Database::new_from_data(&reply.value)?))
        } else {
            Ok(None)
        }
    }

    /// Construct a new X11 resource database from raw data.
    ///
    /// This function parses data like `Some.Entry: Value\n#include "some_file"\n` and returns the
    /// resulting resource database. Parsing cannot fail since unparsable lines are simply ignored.
    ///
    /// See [`Self::new_from_data_with_base_directory`] for a version that allows to provide a path that
    /// is used for resolving relative `#include` statements.
    pub fn new_from_data(data: &[u8]) -> Result<Self, tiny_std::Error> {
        const THIS_DIR: &UnixStr = UnixStr::from_str_checked(".\0");

        let mut entries = Vec::new();
        parse_data_with_base_directory(&mut entries, data, THIS_DIR, 0)?;
        Ok(Self { entries })
    }

    /// Construct a new X11 resource database from raw data.
    ///
    /// This function parses data like `Some.Entry: Value\n#include "some_file"\n` and returns the
    /// resulting resource database. Parsing cannot fail since unparsable lines are simply ignored.
    ///
    /// When a relative `#include` statement is encountered, the file to include is searched
    /// relative to the given `base_path`.
    pub fn new_from_data_with_base_directory(
        data: &[u8],
        base_path: &UnixStr,
    ) -> Result<Self, tiny_std::Error> {
        fn helper(data: &[u8], base_path: &UnixStr) -> Result<Database, tiny_std::Error> {
            let mut entries = Vec::new();
            parse_data_with_base_directory(&mut entries, data, base_path, 0)?;
            Ok(Database { entries })
        }
        helper(data, base_path)
    }

    /// Get a value from the resource database as a byte slice.
    ///
    /// The given values describe a query to the resource database. `resource_class` can be an
    /// empty string, but otherwise must contain the same number of components as `resource_name`.
    /// Both strings may only contain alphanumeric characters or '-', '_', and '.'.
    #[must_use]
    pub fn get_bytes(&self, resource_name: &str, resource_class: &str) -> Option<&[u8]> {
        matcher::match_entry(&self.entries, resource_name, resource_class)
    }

    /// Get a value from the resource database as a byte slice.
    ///
    /// The given values describe a query to the resource database. `resource_class` can be an
    /// empty string, but otherwise must contain the same number of components as `resource_name`.
    /// Both strings may only contain alphanumeric characters or '-', '_', and '.'.
    ///
    /// If an entry is found that is not a valid utf8 `str`, `None` is returned.
    #[must_use]
    pub fn get_string(&self, resource_name: &str, resource_class: &str) -> Option<&str> {
        core::str::from_utf8(self.get_bytes(resource_name, resource_class)?).ok()
    }

    /// Get a value from the resource database as a byte slice.
    ///
    /// The given values describe a query to the resource database. `resource_class` can be an
    /// empty string, but otherwise must contain the same number of components as `resource_name`.
    /// Both strings may only contain alphanumeric characters or '-', '_', and '.'.
    ///
    /// This function interprets "true", "on", "yes" as true-ish and "false", "off", "no" als
    /// false-ish. Numbers are parsed and are true if they are not zero. Unknown values are mapped
    /// to `None`.
    #[must_use]
    pub fn get_bool(&self, resource_name: &str, resource_class: &str) -> Option<bool> {
        to_bool(self.get_string(resource_name, resource_class)?)
    }

    /// Get a value from the resource database and parse it.
    ///
    /// The given values describe a query to the resource database. `resource_class` can be an
    /// empty string, but otherwise must contain the same number of components as `resource_name`.
    /// Both strings may only contain alphanumeric characters or '-', '_', and '.'.
    ///
    /// If no value is found, `Ok(None)` is returned. Otherwise, the result from
    /// [`FromStr::from_str]` is returned with `Ok(value)` replaced with `Ok(Some(value))`.
    pub fn get_value<T>(
        &self,
        resource_name: &str,
        resource_class: &str,
    ) -> Result<Option<T>, T::Err>
    where
        T: FromStr,
    {
        self.get_string(resource_name, resource_class)
            .map(T::from_str)
            .transpose()
    }
}

/// Parse the given data as a resource database.
///
/// The parsed entries are appended to `result`. `#include`s are resolved relative to the given
/// `base_path`. `depth` is the number of includes that we are already handling. This value is used
/// to prevent endless loops when a file (directly or indirectly) includes itself.
fn parse_data_with_base_directory(
    result: &mut Vec<Entry>,
    data: &[u8],
    base_path: &UnixStr,
    depth: u8,
) -> Result<(), tiny_std::Error> {
    if depth > MAX_INCLUSION_DEPTH {
        return Ok(());
    }
    parser::parse_database(data, result, |path, entries| {
        // Construct the name of the file to include
        if let Ok(path) = core::str::from_utf8(path) {
            let base_utf8 = base_path.as_str()?;
            let extended = format!("{base_utf8}/{path}\0");
            let mut file_buf = Vec::with_capacity(4096);
            // Read the file contents
            if let Ok(data) = tiny_std::fs::read(UnixStr::from_str_checked(&extended)) {
                // Parse the file contents with the new base path
                let new_base = extended.rsplit_once('/').map(|s| s.0).unwrap_or(base_utf8);
                parse_data_with_base_directory(
                    entries,
                    &file_buf,
                    UnixStr::try_from_str(new_base)?,
                    depth + 1,
                )
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    })
}

/// Parse a value to a boolean, returning `None` if this is not possible.
fn to_bool(data: &str) -> Option<bool> {
    if let Ok(num) = i64::from_str(data) {
        return Some(num != 0);
    }
    match data.to_lowercase().as_bytes() {
        b"true" | b"on" | b"yes" => Some(true),
        b"false" | b"off" | b"no" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::{to_bool, Database};

    #[test]
    fn test_bool_true() {
        let data = ["1", "10", "true", "TRUE", "on", "ON", "yes", "YES"];
        for input in &data {
            assert_eq!(Some(true), to_bool(input));
        }
    }

    #[test]
    fn test_bool_false() {
        let data = ["0", "false", "FALSE", "off", "OFF", "no", "NO"];
        for input in &data {
            assert_eq!(Some(false), to_bool(input));
        }
    }

    #[test]
    fn test_bool_none() {
        let data = ["", "abc"];
        for input in &data {
            assert_eq!(None, to_bool(input));
        }
    }

    #[test]
    fn test_parse_i32_fail() {
        let db = Database::new_from_data(b"a:").unwrap();
        assert_eq!(db.get_string("a", "a"), Some(""));
        assert!(db.get_value::<i32>("a", "a").is_err());
    }

    #[test]
    fn test_parse_i32_success() {
        let data = [
            (&b"a: 0"[..], 0),
            (b"a: 1", 1),
            (b"a: -1", -1),
            (b"a: 100", 100),
        ];
        for (input, expected) in &data {
            let db = Database::new_from_data(input).unwrap();
            let result = db.get_value::<i32>("a", "a");
            assert_eq!(result.unwrap().unwrap(), *expected);
        }
    }
}
