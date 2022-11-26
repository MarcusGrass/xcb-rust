#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "MIT-SHM";
pub type Seg = u32;
pub const COMPLETION_EVENT: u8 = 0;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CompletionEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub drawable: crate::proto::xproto::Drawable,
    pub minor_event: u16,
    pub major_event: u8,
    pub shmseg: Seg,
    pub offset: u32,
}
impl FixedLengthFromBytes<20> for CompletionEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            drawable: crate::proto::xproto::Drawable::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            minor_event: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            major_event: u8::from_bytes(bytes.get(10..11).ok_or(crate::error::Error::FromBytes)?)?,
            shmseg: Seg::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            offset: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const BAD_SEG_ERROR: u8 = 0;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub shared_pixmaps: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub uid: u16,
    pub gid: u16,
    pub pixmap_format: u8,
}
impl FixedLengthFromBytes<32> for QueryVersionReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            shared_pixmaps: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            major_version: u16::from_bytes(
                bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?,
            )?,
            minor_version: u16::from_bytes(
                bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            uid: u16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
            gid: u16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
            pixmap_format: u8::from_bytes(
                bytes.get(16..17).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetImageReply {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub visual: crate::proto::xproto::Visualid,
    pub size: u32,
}
impl FixedLengthFromBytes<16> for GetImageReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            depth: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            visual: crate::proto::xproto::Visualid::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            size: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CreateSegmentReply {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub shm_fd: (),
}
impl FixedLengthFromBytes<32> for CreateSegmentReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            nfd: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            shm_fd: (),
        })
    }
}
