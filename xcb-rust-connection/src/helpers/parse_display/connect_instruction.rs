//! Provides the `ConnectInstruction` structure, which allows for a `ParsedDisplay`
//! to be transformed into a server connection.

use super::ParsedDisplay;
use alloc::string::String;
use rusl::string::unix_str::UnixString;

/// Get an iterator over all of the addresses we should target with a
/// `ParsedDisplay`.
pub(super) fn connect_addresses(p: &ParsedDisplay) -> Option<UnixString> {
    let ParsedDisplay {
        protocol, display, ..
    } = p;

    if protocol.is_none() || protocol.as_deref() == Some("unix") {
        let file_name = alloc::format!("/tmp/.X11-unix/X{display}\0");

        // TODO: Try abstract socket (file name with prepended '\0')
        // Not supported on Rust right now: https://github.com/rust-lang/rust/issues/42048
        Some(UnixString::try_from_string(file_name).unwrap())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    // make sure iterator properties are clean
    use super::super::*;

    #[test]
    fn basic_test() {
        let pd = parse_display(Some(":0")).unwrap();
        let ci = pd.connect_instruction();
        let ci = ci.unwrap();

        assert_eq!(ci, UnixString::try_from_str("/tmp/.X11-unix/X0\0").unwrap());
    }

    #[test]
    fn try_over_unix_hostname() {
        let pd = parse_display(Some("unix/host:0")).unwrap();
        let ci = pd.connect_instruction();

        let ci = ci.unwrap();

        assert_eq!(ci, UnixString::try_from_str("/tmp/.X11-unix/X0\0").unwrap());
    }
}
