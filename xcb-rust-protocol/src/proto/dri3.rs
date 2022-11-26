#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "DRI3";
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
}
impl FixedLengthFromBytes<16> for QueryVersionReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            major_version: u32::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            minor_version: u32::from_bytes(
                bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct OpenReply {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub device_fd: (),
}
impl FixedLengthFromBytes<32> for OpenReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            nfd: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            device_fd: (),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct BufferFromPixmapReply {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: u32,
    pub width: u16,
    pub height: u16,
    pub stride: u16,
    pub depth: u8,
    pub bpp: u8,
    pub pixmap_fd: (),
}
impl FixedLengthFromBytes<32> for BufferFromPixmapReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            nfd: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            size: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            width: u16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
            stride: u16::from_bytes(bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?)?,
            depth: u8::from_bytes(bytes.get(18..19).ok_or(crate::error::Error::FromBytes)?)?,
            bpp: u8::from_bytes(bytes.get(19..20).ok_or(crate::error::Error::FromBytes)?)?,
            pixmap_fd: (),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct FDFromFenceReply {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub fence_fd: (),
}
impl FixedLengthFromBytes<32> for FDFromFenceReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            nfd: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            fence_fd: (),
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetSupportedModifiersReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub window_modifiers: alloc::vec::Vec<u64>,
    pub screen_modifiers: alloc::vec::Vec<u64>,
}
impl VariableLengthFromBytes for GetSupportedModifiersReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Start align 8 None
        // Padding 1 bytes
        // Padding 16 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(1..3).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(3..7).ok_or(crate::error::Error::FromBytes)?)?;
        let num_window_modifiers =
            u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let num_screen_modifiers =
            u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_window_modifiers as usize;
        let window_modifiers: alloc::vec::Vec<u64> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            u64,
            length_expr,
            8
        );
        let mut offset = length_expr * 8 + 32;
        let length_expr = num_screen_modifiers as usize;
        let screen_modifiers: alloc::vec::Vec<u64> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u64,
            length_expr,
            8
        );
        offset += length_expr * 8;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                window_modifiers,
                screen_modifiers,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct BuffersFromPixmapReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u16,
    pub height: u16,
    pub modifier: u64,
    pub depth: u8,
    pub bpp: u8,
    pub strides: alloc::vec::Vec<u32>,
    pub offsets: alloc::vec::Vec<u32>,
    pub buffers: alloc::vec::Vec<()>,
}
impl VariableLengthFromBytes for BuffersFromPixmapReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Start align 8 None
        // Padding 4 bytes
        // Padding 6 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(1..3).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(3..7).ok_or(crate::error::Error::FromBytes)?)?;
        let nfd = u8::from_bytes(ptr.get(7..8).ok_or(crate::error::Error::FromBytes)?)?;
        let width = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let height = u16::from_bytes(ptr.get(10..12).ok_or(crate::error::Error::FromBytes)?)?;
        let modifier = u64::from_bytes(ptr.get(16..24).ok_or(crate::error::Error::FromBytes)?)?;
        let depth = u8::from_bytes(ptr.get(24..25).ok_or(crate::error::Error::FromBytes)?)?;
        let bpp = u8::from_bytes(ptr.get(25..26).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = nfd as usize;
        let strides: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 32;
        let length_expr = nfd as usize;
        let offsets: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        offset += length_expr * 4;
        let buffers = alloc::vec![];
        Ok((
            Self {
                response_type,
                sequence,
                length,
                width,
                height,
                modifier,
                depth,
                bpp,
                strides,
                offsets,
                buffers,
            },
            offset,
        ))
    }
}
