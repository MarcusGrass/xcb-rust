#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "XFIXES";
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
}
impl FixedLengthFromBytes<32> for QueryVersionReply {
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
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SaveSetModeEnum(pub u8);
impl SaveSetModeEnum {
    pub const INSERT: Self = Self(0);
    pub const DELETE: Self = Self(1);
}
impl FixedLengthSerialize<1> for SaveSetModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SaveSetModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for SaveSetModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SaveSetModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SaveSetModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SaveSetTargetEnum(pub u8);
impl SaveSetTargetEnum {
    pub const NEAREST: Self = Self(0);
    pub const ROOT: Self = Self(1);
}
impl FixedLengthSerialize<1> for SaveSetTargetEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SaveSetTargetEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for SaveSetTargetEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SaveSetTargetEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SaveSetTargetEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SaveSetMappingEnum(pub u8);
impl SaveSetMappingEnum {
    pub const MAP: Self = Self(0);
    pub const UNMAP: Self = Self(1);
}
impl FixedLengthSerialize<1> for SaveSetMappingEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SaveSetMappingEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for SaveSetMappingEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SaveSetMappingEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SaveSetMappingEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SelectionEventEnum(pub u8);
impl SelectionEventEnum {
    pub const SET_SELECTION_OWNER: Self = Self(0);
    pub const SELECTION_WINDOW_DESTROY: Self = Self(1);
    pub const SELECTION_CLIENT_CLOSE: Self = Self(2);
}
impl FixedLengthSerialize<1> for SelectionEventEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SelectionEventEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for SelectionEventEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SelectionEventEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SelectionEventEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct SelectionEventMask(pub u32);
impl SelectionEventMask {
    pub const SET_SELECTION_OWNER: Self = Self(1 << 0);
    pub const SELECTION_WINDOW_DESTROY: Self = Self(1 << 1);
    pub const SELECTION_CLIENT_CLOSE: Self = Self(1 << 2);
}
impl FixedLengthSerialize<4> for SelectionEventMask {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for SelectionEventMask {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for SelectionEventMask {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for SelectionEventMask {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for SelectionEventMask {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(SelectionEventMask);
pub const SELECTION_NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SelectionNotifyEvent {
    pub opcode: u8,
    pub subtype: SelectionEventEnum,
    pub sequence: u16,
    pub window: crate::proto::xproto::Window,
    pub owner: crate::proto::xproto::Window,
    pub selection: u32,
    pub timestamp: crate::proto::xproto::Timestamp,
    pub selection_timestamp: crate::proto::xproto::Timestamp,
}
impl FixedLengthFromBytes<32> for SelectionNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            subtype: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            owner: crate::proto::xproto::Window::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            selection: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?,
            )?,
            selection_timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct CursorNotifyEnum(pub u8);
impl CursorNotifyEnum {
    pub const DISPLAY_CURSOR: Self = Self(0);
}
impl FixedLengthSerialize<1> for CursorNotifyEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for CursorNotifyEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for CursorNotifyEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for CursorNotifyEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for CursorNotifyEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct CursorNotifyMask(pub u32);
impl CursorNotifyMask {
    pub const DISPLAY_CURSOR: Self = Self(1 << 0);
}
impl FixedLengthSerialize<4> for CursorNotifyMask {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for CursorNotifyMask {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for CursorNotifyMask {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for CursorNotifyMask {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for CursorNotifyMask {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(CursorNotifyMask);
pub const CURSOR_NOTIFY_EVENT: u8 = 1;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CursorNotifyEvent {
    pub opcode: u8,
    pub subtype: CursorNotifyEnum,
    pub sequence: u16,
    pub window: crate::proto::xproto::Window,
    pub cursor_serial: u32,
    pub timestamp: crate::proto::xproto::Timestamp,
    pub name: crate::proto::xproto::AtomEnum,
}
impl FixedLengthFromBytes<32> for CursorNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            subtype: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            cursor_serial: u32::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            name: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetCursorImageReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub xhot: u16,
    pub yhot: u16,
    pub cursor_serial: u32,
    pub cursor_image: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for GetCursorImageReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 8 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let x = i16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let y = i16::from_bytes(ptr.get(10..12).ok_or(crate::error::Error::FromBytes)?)?;
        let width = u16::from_bytes(ptr.get(12..14).ok_or(crate::error::Error::FromBytes)?)?;
        let height = u16::from_bytes(ptr.get(14..16).ok_or(crate::error::Error::FromBytes)?)?;
        let xhot = u16::from_bytes(ptr.get(16..18).ok_or(crate::error::Error::FromBytes)?)?;
        let yhot = u16::from_bytes(ptr.get(18..20).ok_or(crate::error::Error::FromBytes)?)?;
        let cursor_serial =
            u32::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(width as u16, height as u16) as usize;
        let cursor_image: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                x,
                y,
                width,
                height,
                xhot,
                yhot,
                cursor_serial,
                cursor_image,
            },
            offset,
        ))
    }
}
pub type Region = u32;
pub const BAD_REGION_ERROR: u8 = 0;
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct RegionEnum(pub Region);
impl RegionEnum {
    pub const NONE: Self = Self(0);
}
impl FixedLengthSerialize<4> for RegionEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for RegionEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(Region::from_bytes(bytes)?))
    }
}
impl From<u32> for RegionEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(Region::from(val as u32))
    }
}
impl From<u16> for RegionEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(Region::from(val as u32))
    }
}
impl From<u8> for RegionEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(Region::from(val as u32))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct FetchRegionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub extents: crate::proto::xproto::Rectangle,
    pub rectangles: alloc::vec::Vec<crate::proto::xproto::Rectangle>,
}
impl VariableLengthFromBytes for FetchRegionReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 16 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let extents = crate::proto::xproto::Rectangle::from_bytes(
            ptr.get(8..16).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let length_expr = core::ops::Div::div(length, 2) as usize;
        let rectangles: alloc::vec::Vec<crate::proto::xproto::Rectangle> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            crate::proto::xproto::Rectangle,
            length_expr,
            8
        );
        let offset = length_expr * 8 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                extents,
                rectangles,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetCursorNameReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atom: crate::proto::xproto::AtomEnum,
    pub name: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetCursorNameReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 18 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let atom = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let nbytes = u16::from_bytes(ptr.get(12..14).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = nbytes as usize;
        let name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                atom: atom.into(),
                name,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetCursorImageAndNameReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub xhot: u16,
    pub yhot: u16,
    pub cursor_serial: u32,
    pub cursor_atom: crate::proto::xproto::AtomEnum,
    pub cursor_image: alloc::vec::Vec<u32>,
    pub name: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetCursorImageAndNameReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 2 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let x = i16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let y = i16::from_bytes(ptr.get(10..12).ok_or(crate::error::Error::FromBytes)?)?;
        let width = u16::from_bytes(ptr.get(12..14).ok_or(crate::error::Error::FromBytes)?)?;
        let height = u16::from_bytes(ptr.get(14..16).ok_or(crate::error::Error::FromBytes)?)?;
        let xhot = u16::from_bytes(ptr.get(16..18).ok_or(crate::error::Error::FromBytes)?)?;
        let yhot = u16::from_bytes(ptr.get(18..20).ok_or(crate::error::Error::FromBytes)?)?;
        let cursor_serial =
            u32::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let cursor_atom = u32::from_bytes(ptr.get(24..28).ok_or(crate::error::Error::FromBytes)?)?;
        let nbytes = u16::from_bytes(ptr.get(28..30).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(width as u16, height as u16) as usize;
        let cursor_image: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 32;
        let length_expr = nbytes as usize;
        let name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        offset += length_expr;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                x,
                y,
                width,
                height,
                xhot,
                yhot,
                cursor_serial,
                cursor_atom: cursor_atom.into(),
                cursor_image,
                name,
            },
            offset,
        ))
    }
}
pub type Barrier = u32;
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct BarrierDirections(pub u32);
impl BarrierDirections {
    pub const POSITIVE_X: Self = Self(1 << 0);
    pub const POSITIVE_Y: Self = Self(1 << 1);
    pub const NEGATIVE_X: Self = Self(1 << 2);
    pub const NEGATIVE_Y: Self = Self(1 << 3);
}
impl FixedLengthSerialize<4> for BarrierDirections {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for BarrierDirections {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for BarrierDirections {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for BarrierDirections {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for BarrierDirections {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(BarrierDirections);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ClientDisconnectFlags(pub u32);
impl ClientDisconnectFlags {
    pub const DEFAULT: Self = Self(0);
    pub const TERMINATE: Self = Self(1 << 0);
}
impl FixedLengthSerialize<4> for ClientDisconnectFlags {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for ClientDisconnectFlags {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for ClientDisconnectFlags {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for ClientDisconnectFlags {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for ClientDisconnectFlags {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(ClientDisconnectFlags);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetClientDisconnectModeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub disconnect_mode: ClientDisconnectFlags,
}
impl FixedLengthFromBytes<32> for GetClientDisconnectModeReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            disconnect_mode: u32::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
        })
    }
}
