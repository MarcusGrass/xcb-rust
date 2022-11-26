//! Contains utilities for connection to the X11 server.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use xcb_rust_protocol::proto::xproto::{Setup, SetupAuthenticate, SetupFailed, SetupRequest};
use xcb_rust_protocol::util::{VariableLengthFromBytes, VariableLengthSerialize};
use xcb_rust_protocol::XcbEnv;

use crate::helpers::xauth::{get_auth, Family};
use crate::ConnectError;

pub struct Connect {
    // input buffer
    buffer: Vec<u8>,
    // position in the buffer that has been filled
    advanced: usize,
}

const INITIAL_CAPACITY: usize = 8;

// X11 interprets capital B as big endian, and lowercase l as little endian.
#[cfg(target_endian = "little")]
const BYTE_ORDER: u8 = b'l';
#[cfg(not(target_endian = "little"))]
const BYTE_ORDER: u8 = b'B';

// protocol version
const PROTOCOL_MAJOR_VERSION: u16 = 11;
const PROTOCOL_MINOR_VERSION: u16 = 0;

impl Connect {
    /// The initial state of a `Connect`.
    fn blank() -> Self {
        Self {
            buffer: vec![0; INITIAL_CAPACITY],
            advanced: 0,
        }
    }

    /// Create a new `Connect` from the information necessary to connect to the X11 server.
    ///
    /// This returns the connection handshake object as well as the setup request to send to the server.
    pub fn new(
        env: XcbEnv,
        family: Family,
        address: &[u8],
        display: u16,
    ) -> Result<(Self, Vec<u8>), ConnectError> {
        match get_auth(env, family, address, display)? {
            Some((name, data)) => Ok(Self::with_authorization(name, data)),
            None => {
                // fall through to no authorization
                Ok(Self::with_authorization(Vec::new(), Vec::new()))
            }
        }
    }

    /// Create a new `Connect` from the given authorization data.
    ///
    /// This uses the provided protocol name and data to establish the connection,
    /// rather than the default protocol name and data found in `Xauthority`.
    #[must_use]
    pub fn with_authorization(protocol_name: Vec<u8>, protocol_data: Vec<u8>) -> (Self, Vec<u8>) {
        // craft the setup request
        let mut container = vec![0u8; protocol_data.len() + protocol_name.len() + 128];
        let sr = SetupRequest {
            byte_order: BYTE_ORDER,
            protocol_major_version: PROTOCOL_MAJOR_VERSION,
            protocol_minor_version: PROTOCOL_MINOR_VERSION,
            authorization_protocol_name: protocol_name,
            authorization_protocol_data: protocol_data,
        };
        let bytes = sr.serialize_into(container.as_mut_slice()).unwrap();
        container = container[..bytes].to_vec();
        // return it
        (Self::blank(), container)
    }

    /// Returns the buffer that needs to be filled with incoming data from the server.
    ///
    /// After filling this buffer (using a method like `Read::read`), call [`advance`] with
    /// the number of bytes read to indicate that the buffer has been filled.
    pub fn buffer(&mut self) -> &mut [u8] {
        &mut self.buffer[self.advanced..]
    }

    #[must_use]
    pub fn needs_bytes(&self) -> bool {
        self.buffer.len() != self.advanced
    }

    /// Advance the internal buffer, given the number of bytes that have been read.
    pub fn advance(&mut self) -> bool {
        self.advanced = self.buffer.len();
        debug_assert!(self.buffer.len() >= self.advanced);

        // if we've read up to the initial capacity, tell how many more bytes
        // we need to read
        if self.advanced == INITIAL_CAPACITY {
            // remaining length is at byte range 6-7 in 4-bytes
            let length = u16::from_ne_bytes([self.buffer[6], self.buffer[7]]);
            let length = length as usize * 4;
            // allocate more room
            // use reserve_exact because this will be the final
            // length of the vector
            self.buffer.reserve_exact(length);
            self.buffer.resize(length + self.buffer.len(), 0);
            false
        } else {
            self.advanced == self.buffer.len()
        }
    }

    /// Returns the setup provided by the server.
    ///
    /// # Errors
    ///
    /// - If this method is called before the server returns all of the required data,
    ///   it returns `ConnectError::NotEnoughData`.
    /// - If the server fails to establish the X11 connection, the `ConnectError::SetupFailed`
    ///   variant is returned.
    /// - If the server failed to authenticate the user, the `ConnectError::SetupAuthenticate`
    ///   error is returned.
    /// - If the server failed to parse any of the above responses, the
    ///   `ConnectError::ParseError` error is returned.
    #[allow(clippy::match_on_vec_items)]
    pub fn into_setup(self) -> Result<Setup, ConnectError> {
        // if we aren't full yet, panic
        if self.advanced != self.buffer.len() {
            return Err(ConnectError::Incomplete {
                expected: self.buffer.len(),
                received: self.advanced,
            });
        }
        // parse the setup response
        match self.buffer[0] {
            0 => {
                // an error has occurred
                let (failed, _) = SetupFailed::from_bytes(&self.buffer)?;
                Err(ConnectError::SetupFailed(failed))
            }
            1 => {
                // the setup is valid!
                let (success, _) = Setup::from_bytes(&self.buffer)?;
                Ok(success)
            }
            2 => {
                // we need further authentication
                let (more_auth, _) = SetupAuthenticate::from_bytes(&self.buffer)?;
                Err(ConnectError::SetupAuthenticate(more_auth))
            }
            _ => {
                // this is undefined
                Err(ConnectError::BadValue)
            }
        }
    }
}

impl core::fmt::Debug for Connect {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Connect")
            .field(
                "buffer",
                &format_args!("{}/{}", self.advanced, self.buffer.len()),
            )
            .finish()
    }
}

impl TryFrom<Connect> for Setup {
    type Error = ConnectError;

    fn try_from(connect: Connect) -> Result<Self, Self::Error> {
        connect.into_setup()
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use core::mem::drop;

    use xcb_rust_protocol::proto::xproto::{ImageOrderEnum, Setup, SetupAuthenticate, SetupFailed};
    use xcb_rust_protocol::util::VariableLengthSerialize;

    use super::*;

    fn test_setup() -> Setup {
        let vendor = b"Testing Setup".to_vec();
        let mut s = Setup {
            status: 1,
            protocol_major_version: 11,
            protocol_minor_version: 0,
            length: 0,
            release_number: 0,
            resource_id_base: 1,
            resource_id_mask: 1,
            motion_buffer_size: 0,
            vendor,
            maximum_request_length: 0,
            image_byte_order: ImageOrderEnum::L_S_B_FIRST,
            bitmap_format_bit_order: ImageOrderEnum::L_S_B_FIRST,
            bitmap_format_scanline_unit: 32,
            bitmap_format_scanline_pad: 32,
            min_keycode: 0,
            max_keycode: 0,
            pixmap_formats: vec![],
            roots: vec![],
        };
        let mut dummy = vec![0u8; 2048];
        let len = s.clone().serialize_into(&mut dummy).unwrap();
        // +3 so it rounds up
        s.length = ((len as u16 - 8 + 3) / 4) as u16;
        s
    }

    fn try_receive_bytes(item: impl VariableLengthSerialize) -> Result<Setup, ConnectError> {
        let mut connect = Connect::blank();

        // feed in a setup
        let mut item_bytes = vec![0; 2048];
        let bytes = item.serialize_into(&mut item_bytes).unwrap();
        item_bytes = item_bytes[..bytes].to_vec();

        let mut i = 0;
        loop {
            i += 1;
            if i > 500 {
                panic!("too many iterations");
            }

            // copy bytes to connect
            let buffer = connect.buffer();
            let bytes_to_copy = std::cmp::min(item_bytes.len(), buffer.len());
            buffer[..bytes_to_copy].copy_from_slice(&item_bytes[..bytes_to_copy]);

            // drain the bytes that we've already copied
            drop(item_bytes.drain(..bytes_to_copy));

            // check advance
            if connect.advance() {
                break;
            }
        }

        connect.into_setup()
    }

    #[test]
    fn test_connect_receive_setup() {
        let setup = test_setup();
        let b = try_receive_bytes(setup.clone());

        match b {
            Ok(s) => assert_eq!(s, setup),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn test_connect_receive_setup_authenticate() {
        let reason = b"Needs more auth.".to_vec();
        let setup = SetupAuthenticate { status: 2, reason };

        let b = try_receive_bytes(setup.clone());
        match b {
            Ok(s) => panic!("{:?}", s),
            Err(ConnectError::SetupAuthenticate(e)) => assert_eq!(e, setup),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn test_connect_receive_setup_failed() {
        let mut setup = SetupFailed {
            status: 0,
            protocol_major_version: 11,
            protocol_minor_version: 0,
            length: 0,
            reason: b"whatever".to_vec(),
        };
        let mut dummy = vec![0u8; 256];
        let bytes = setup.clone().serialize_into(&mut dummy).unwrap();
        setup.length = ((bytes - 8) / 4) as _;

        let b = try_receive_bytes(setup.clone());
        match b {
            Ok(s) => panic!("{:?}", s),
            Err(ConnectError::SetupFailed(e)) => assert_eq!(e, setup),
            Err(e) => panic!("{:?}", e),
        }
    }
}
