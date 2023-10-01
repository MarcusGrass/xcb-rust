//! Helpers for working with `~/.Xauthority`.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use xcb_rust_protocol::proto::xproto::FamilyEnum as X11Family;
use xcb_rust_protocol::XcbEnv;

const MIT_MAGIC_COOKIE_1: &[u8] = b"MIT-MAGIC-COOKIE-1";

/// A family describes how to interpret some bytes as an address in an `AuthEntry`.
///
/// Compared to [`super::protocol::xproto::Family`], this is a `u16` and not an `u8` since
/// that's what is used in `~/.Xauthority` files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Family(u16);

impl Family {
    /// IPv4 connection to the server
    pub const INTERNET: Self = Self(0);
    /// `DECnet`
    pub const DEC_NET: Self = Self(1);
    /// Chaosnet connection
    pub const CHAOS: Self = Self(2);
    /// Family without predefined meaning, but interpreted by the server, for example a user name
    pub const SERVER_INTERPRETED: Self = Self(5);
    /// IPv6 connection to the server
    pub const INTERNET6: Self = Self(6);
    /// Wildcard matching any protocol family
    pub const WILD: Self = Self(65535);
    /// For local non-net authentication
    pub const LOCAL: Self = Self(256);
    /// TODO: No idea what this means exactly
    pub const NETNAME: Self = Self(254);
    /// Kerberos 5 principal name
    pub const KRB5_PRINCIPAL: Self = Self(253);
    /// For local non-net authentication
    pub const LOCAL_HOST: Self = Self(252);
}

impl From<X11Family> for Family {
    fn from(value: X11Family) -> Self {
        Self(value.0.into())
    }
}

impl From<u16> for Family {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

/// A single entry of an `.Xauthority` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AuthEntry {
    /// The protocol family to which the entry applies
    family: Family,
    /// The address of the peer in a family-specific format
    address: Vec<u8>,
    /// The display number
    number: Vec<u8>,
    /// The name of the authentication method to use for the X11 server described by the previous
    /// fields.
    name: Vec<u8>,
    /// Extra data for the authentication method.
    data: Vec<u8>,
}

mod file {
    //! Code for actually reading `~/.Xauthority`.
    use alloc::string::String;
    use alloc::vec;
    use alloc::vec::Vec;
    use rusl::string::unix_str::UnixStr;

    use super::AuthEntry;

    /// Read a single `u16` from an `~/.Xauthority` file.
    ///
    /// The file stores these entries in big endian.
    #[inline]
    fn read_u16(read: &[u8], offset: usize) -> Option<u16> {
        Some(u16::from_be_bytes(
            read.get(offset..offset + 2)?.try_into().unwrap(),
        ))
    }

    /// Read a single "byte array" from an `~/.Xauthority` file.
    ///
    /// The file stores these as a length field followed by a number of bytes that contain the
    /// actual data.
    fn read_string(read: &[u8], offset: usize) -> Option<(Vec<u8>, usize)> {
        let length = read_u16(read, offset)? as usize;
        Some((
            read.get(offset + 2..offset + 2 + length)?.to_vec(),
            offset + 2 + length,
        ))
    }

    /// Read a single entry from an `~/.Xauthority` file.
    ///
    /// This function tries to return `Ok(None)` when the end of the file is reached. However, the
    /// code also treats a single byte as 'end of file', because things were simpler to implement
    /// like this.
    fn read_entry(read: &[u8], base_offset: &mut usize) -> Option<AuthEntry> {
        let family = read_u16(read, *base_offset)?.into();
        *base_offset += 2;

        let (address, offset) = read_string(read, *base_offset)?;
        let (number, offset) = read_string(read, offset)?;
        let (name, offset) = read_string(read, offset)?;
        let (data, offset) = read_string(read, offset)?;
        *base_offset = offset;
        Some(AuthEntry {
            family,
            address,
            number,
            name,
            data,
        })
    }

    /// An iterator over the entries of an `.Xauthority` file
    #[derive(Debug)]
    pub(crate) struct XAuthorityEntries(Vec<u8>, usize);

    impl XAuthorityEntries {
        /// Open `~/.Xauthority` for reading.
        ///
        /// This function returns `Ok(None)` when the location of the `.Xauthority` file could not
        /// be determined. If opening the file failed (for example, because it does not exist),
        /// that error is returned.
        pub(crate) fn new(
            auth_file: &UnixStr,
        ) -> Result<Option<XAuthorityEntries>, tiny_std::Error> {
            tiny_std::fs::read(auth_file)
                .ok()
                .map(|buf| Ok(XAuthorityEntries(buf, 0)))
                .transpose()
        }
    }

    impl Iterator for XAuthorityEntries {
        type Item = AuthEntry;

        fn next(&mut self) -> Option<Self::Item> {
            read_entry(&self.0, &mut self.1)
        }
    }

    #[cfg(test)]
    mod test {
        use alloc::vec;

        use std::io::Cursor;

        use super::super::{AuthEntry, Family};
        use super::read_entry;

        #[test]
        fn test_read() {
            // Data generated via xauth -f /tmp/file add :1 bar deadbeef
            let data = [
                0x01, 0x00, 0x00, 0x07, 0x5a, 0x77, 0x65, 0x69, 0x4c, 0x45, 0x44, 0x00, 0x01, 0x31,
                0x00, 0x03, 0x62, 0x61, 0x72, 0x00, 0x04, 0xde, 0xad, 0xbe, 0xef,
            ];
            let mut buf = data.to_vec();
            let entry = read_entry(&buf, &mut 0).unwrap();
            assert_eq!(
                entry,
                AuthEntry {
                    family: Family::LOCAL,
                    address: b"ZweiLED".to_vec(),
                    number: b"1".to_vec(),
                    name: b"bar".to_vec(),
                    data: u32::to_be_bytes(0xdead_beef).to_vec(),
                }
            );
        }

        #[test]
        fn test_read_iterate() {
            // Data generated via:
            //   xauth -f /tmp/file add :1 bar deadbeef
            //   xauth -f /tmp/file add 1.2.3.4:2 baz aabbccdd
            let data = [
                0x01, 0x00, 0x00, 0x07, 0x5a, 0x77, 0x65, 0x69, 0x4c, 0x45, 0x44, 0x00, 0x01, 0x31,
                0x00, 0x03, 0x62, 0x61, 0x72, 0x00, 0x04, 0xde, 0xad, 0xbe, 0xef, 0x00, 0x00, 0x00,
                0x04, 0x01, 0x02, 0x03, 0x04, 0x00, 0x01, 0x32, 0x00, 0x03, 0x62, 0x61, 0x7a, 0x00,
                0x04, 0xaa, 0xbb, 0xcc, 0xdd,
            ];
            let mut buf = data.to_vec();
            let mut offset = 0;
            for expected in &[
                AuthEntry {
                    family: Family::LOCAL,
                    address: b"ZweiLED".to_vec(),
                    number: b"1".to_vec(),
                    name: b"bar".to_vec(),
                    data: u32::to_be_bytes(0xdead_beef).to_vec(),
                },
                AuthEntry {
                    family: Family::INTERNET,
                    address: vec![1, 2, 3, 4],
                    number: b"2".to_vec(),
                    name: b"baz".to_vec(),
                    data: u32::to_be_bytes(0xaabb_ccdd).to_vec(),
                },
            ] {
                let entry = read_entry(&buf, &mut offset).unwrap();
                assert_eq!(&entry, expected);
            }
            let entry = read_entry(&buf, &mut offset);
            assert_eq!(entry, None);
        }
    }
}

pub(crate) type AuthInfo = (Vec<u8>, Vec<u8>);

/// Get the authentication information necessary for connecting to the given display.
///
/// - `family` is the protocol family that is used for connecting; this describes how to interpret
///   the `address`.
/// - `address` is the raw bytes describing the address that is being connected to.
/// - `display` is the display number.
///
/// If successful, this function returns that can be written to the X11 server as authorization
/// protocol name and data, respectively.
pub fn get_auth(
    xcb_env: XcbEnv,
    family: Family,
    address: &[u8],
    display: u16,
) -> Result<Option<AuthInfo>, tiny_std::Error> {
    if let Some(xauth_file) = xcb_env.x_authority {
        return match file::XAuthorityEntries::new(xauth_file)? {
            None => Ok(None),
            Some(entries) => Ok(get_auth_impl(entries, family, address, display)),
        };
    }
    Ok(None)
}

fn get_auth_impl(
    entries: impl Iterator<Item = AuthEntry>,
    family: Family,
    address: &[u8],
    display: u16,
) -> Option<AuthInfo> {
    fn address_matches(
        (family1, address1): (Family, &[u8]),
        (family2, address2): (Family, &[u8]),
    ) -> bool {
        if family1 == Family::WILD || family2 == Family::WILD {
            true
        } else if family1 != family2 {
            false
        } else {
            address1 == address2
        }
    }

    fn display_number_matches(entry_number: &[u8], display_number: &[u8]) -> bool {
        debug_assert!(!display_number.is_empty()); // This case is not handled here and would be a match
        entry_number.is_empty() || entry_number == display_number
    }

    let display = display.to_string();
    let display = display.as_bytes();

    for entry in entries {
        if address_matches((family, address), (entry.family, &entry.address))
            && display_number_matches(&entry.number, display)
            && entry.name == MIT_MAGIC_COOKIE_1
        {
            return Some((entry.name, entry.data));
        }
    }
    None
}

#[cfg(test)]
mod test {
    use alloc::vec;

    use super::{get_auth_impl, AuthEntry, Family, MIT_MAGIC_COOKIE_1};

    // Call the given function on a matching auth entry. The function can change the entry.
    // Afterwards, it should still be a match.
    fn expect_match<F>(f: F)
    where
        F: FnOnce(&mut AuthEntry),
    {
        let mut entry = AuthEntry {
            family: Family::LOCAL,
            address: b"whatever".to_vec(),
            number: b"42".to_vec(),
            name: MIT_MAGIC_COOKIE_1.to_vec(),
            data: b"1234".to_vec(),
        };
        f(&mut entry);
        let entries = vec![entry];
        let res = get_auth_impl(entries.into_iter(), Family::LOCAL, b"whatever", 42).unwrap();
        assert_eq!(res, (MIT_MAGIC_COOKIE_1.to_vec(), b"1234".to_vec()));
    }

    // Call the given function on a matching auth entry. The function can change the entry.
    // Afterwards, it should no longer match.
    fn expect_mismatch<F>(f: F)
    where
        F: FnOnce(&mut AuthEntry),
    {
        let mut entry = AuthEntry {
            family: Family::LOCAL,
            address: b"whatever".to_vec(),
            number: b"42".to_vec(),
            name: MIT_MAGIC_COOKIE_1.to_vec(),
            data: b"1234".to_vec(),
        };
        f(&mut entry);
        let entries = vec![entry];
        assert_eq!(
            get_auth_impl(entries.into_iter(), Family::LOCAL, b"whatever", 42),
            None
        );
    }

    #[test]
    fn direct_match() {
        // This checks that an auth entry where all members match, really matches
        expect_match(|_| {});
    }

    #[test]
    fn display_wildcard() {
        expect_match(|entry| entry.number = vec![]);
    }

    #[test]
    fn address_wildcard_match1() {
        expect_match(|entry| entry.family = Family::WILD);
    }

    #[test]
    fn address_wildcard_match2() {
        let entry = AuthEntry {
            family: Family::LOCAL,
            address: b"whatever".to_vec(),
            number: b"42".to_vec(),
            name: MIT_MAGIC_COOKIE_1.to_vec(),
            data: b"1234".to_vec(),
        };
        let entries = vec![entry];
        assert_eq!(
            get_auth_impl(entries.into_iter(), Family::WILD, &[], 42).unwrap(),
            (MIT_MAGIC_COOKIE_1.to_vec(), b"1234".to_vec())
        );
    }

    #[test]
    fn family_mismatch() {
        expect_mismatch(|entry| entry.family = Family::KRB5_PRINCIPAL);
    }

    #[test]
    fn address_mismatch() {
        expect_mismatch(|entry| entry.address = b"something else".to_vec());
    }

    #[test]
    fn number_mismatch() {
        expect_mismatch(|entry| entry.number = b"1337".to_vec());
    }

    #[test]
    fn protocol_mismatch() {
        expect_mismatch(|entry| entry.name = b"XDM-AUTHORIZATION-1".to_vec());
    }
}
