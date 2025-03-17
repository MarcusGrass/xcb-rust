use crate::proto::xproto::{Atom, GetPropertyReply, Window, CLIENT_MESSAGE_EVENT};
use crate::Error;
use tiny_std::UnixStr;

pub mod cursor;
pub mod properties;
pub mod resource_manager;

#[derive(Debug, Default, Copy, Clone)]
pub struct XcbEnv<'a> {
    /// `HOME` dir path
    pub home_dir: Option<&'a UnixStr>,
    /// `XENVIRONMENT` file path
    pub x_environment: Option<&'a UnixStr>,
    /// `XAUTHORITY` file path
    pub x_authority: Option<&'a UnixStr>,
    /// `DISPLAY` number
    pub display: Option<&'a UnixStr>,
    /// `XCURSOR_SIZE` cursor size
    pub x_cursor_size: Option<&'a UnixStr>,
}

#[must_use]
pub fn new_client_message32(window: Window, r#type: Atom, data: [u32; 5]) -> [u8; 32] {
    let window = window.to_ne_bytes();
    let kind = r#type.to_ne_bytes();
    let d1 = data[0].to_ne_bytes();
    let d2 = data[1].to_ne_bytes();
    let d3 = data[2].to_ne_bytes();
    let d4 = data[3].to_ne_bytes();
    let d5 = data[4].to_ne_bytes();
    [
        CLIENT_MESSAGE_EVENT,
        32,
        0,
        0,
        window[0],
        window[1],
        window[2],
        window[3],
        kind[0],
        kind[1],
        kind[2],
        kind[3],
        d1[0],
        d1[1],
        d1[2],
        d1[3],
        d2[0],
        d2[1],
        d2[2],
        d2[3],
        d3[0],
        d3[1],
        d3[2],
        d3[3],
        d4[0],
        d4[1],
        d4[2],
        d4[3],
        d5[0],
        d5[1],
        d5[2],
        d5[3],
    ]
}

#[must_use]
pub fn new_client_message16(window: Window, r#type: Atom, data: [u16; 10]) -> [u8; 32] {
    let window = window.to_ne_bytes();
    let kind = r#type.to_ne_bytes();
    let d1 = data[0].to_ne_bytes();
    let d2 = data[1].to_ne_bytes();
    let d3 = data[2].to_ne_bytes();
    let d4 = data[3].to_ne_bytes();
    let d5 = data[4].to_ne_bytes();
    let d6 = data[5].to_ne_bytes();
    let d7 = data[6].to_ne_bytes();
    let d8 = data[7].to_ne_bytes();
    let d9 = data[8].to_ne_bytes();
    let d10 = data[9].to_ne_bytes();
    [
        CLIENT_MESSAGE_EVENT,
        32,
        0,
        0,
        window[0],
        window[1],
        window[2],
        window[3],
        kind[0],
        kind[1],
        kind[2],
        kind[3],
        d1[0],
        d1[1],
        d2[0],
        d2[1],
        d3[0],
        d3[1],
        d4[0],
        d4[1],
        d5[0],
        d5[1],
        d6[0],
        d6[1],
        d7[0],
        d7[1],
        d8[0],
        d8[1],
        d9[0],
        d9[1],
        d10[0],
        d10[1],
    ]
}

#[must_use]
pub fn new_client_message8(window: Window, r#type: Atom, data: [u8; 20]) -> [u8; 32] {
    let window = window.to_ne_bytes();
    let kind = r#type.to_ne_bytes();
    [
        CLIENT_MESSAGE_EVENT,
        32,
        0,
        0,
        window[0],
        window[1],
        window[2],
        window[3],
        kind[0],
        kind[1],
        kind[2],
        kind[3],
        data[0],
        data[1],
        data[2],
        data[3],
        data[4],
        data[5],
        data[6],
        data[7],
        data[8],
        data[9],
        data[10],
        data[11],
        data[12],
        data[13],
        data[14],
        data[15],
        data[16],
        data[17],
        data[18],
        data[19],
    ]
}

/// Serialize a client message, if it's in u8, you don't need to serialize it.
pub fn client_message_16(data: &[u16]) -> crate::Result<[u8; 20]> {
    if data.len() != 10 {
        return Err(Error::Serialize);
    }
    let mut buf = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (ind, entry) in data.iter().enumerate() {
        let split = entry.to_ne_bytes();
        let start = ind * 2;
        buf[start..start + 2].copy_from_slice(&split);
    }
    Ok(buf)
}

/// Serialize a client message, if it's in u8, you don't need to serialize it.
pub fn client_message_32(data: &[u32]) -> crate::Result<[u8; 20]> {
    if data.len() != 5 {
        return Err(Error::Serialize);
    }
    let mut buf = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (ind, entry) in data.iter().enumerate() {
        let split = entry.to_ne_bytes();
        let split = entry.to_ne_bytes();
        let start = ind * 4;
        buf[start..start + 4].copy_from_slice(&split);
    }
    Ok(buf)
}

pub trait CanIterFormats {
    fn value32(&self) -> Option<Iter32>;
    fn value16(&self) -> Option<Iter16>;
    fn value8(&self) -> Option<&[u8]>;
}

impl CanIterFormats for GetPropertyReply {
    fn value32(&self) -> Option<Iter32> {
        (self.format == 32).then(|| Iter32::new(&self.value))
    }

    fn value16(&self) -> Option<Iter16> {
        (self.format == 16).then(|| Iter16::new(&self.value))
    }

    fn value8(&self) -> Option<&[u8]> {
        (self.format == 8).then_some(self.value.as_slice())
    }
}

pub struct Iter32<'a> {
    buf: &'a [u8],
    ind: usize,
}

impl<'a> Iter32<'a> {
    #[must_use]
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, ind: 0 }
    }
}

impl Iterator for Iter32<'_> {
    type Item = u32;

    #[allow(unsafe_code)]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_u32) = self.buf.get(self.ind..self.ind + 4) {
            self.ind += 4;
            // SAFETY: We just checked that it is four bytes
            unsafe { Some(u32::from_ne_bytes(next_u32.try_into().unwrap_unchecked())) }
        } else {
            None
        }
    }
}

pub struct Iter16<'a> {
    buf: &'a [u8],
    ind: usize,
}

impl<'a> Iter16<'a> {
    #[must_use]
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, ind: 0 }
    }
}

impl Iterator for Iter16<'_> {
    type Item = u32;

    #[allow(unsafe_code)]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_u32) = self.buf.get(self.ind..self.ind + 2) {
            self.ind += 2;
            // SAFETY: We just checked that it is four bytes
            unsafe { Some(u32::from_ne_bytes(next_u32.try_into().unwrap_unchecked())) }
        } else {
            None
        }
    }
}
