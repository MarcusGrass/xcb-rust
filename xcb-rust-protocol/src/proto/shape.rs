#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "SHAPE";
pub type Op = u8;
pub type Kind = u8;
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SoEnum(pub Op);
impl SoEnum {
    pub const SET: Self = Self(0);
    pub const UNION: Self = Self(1);
    pub const INTERSECT: Self = Self(2);
    pub const SUBTRACT: Self = Self(3);
    pub const INVERT: Self = Self(4);
}
impl FixedLengthSerialize<1> for SoEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SoEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(Op::from_bytes(bytes)?))
    }
}
impl From<u32> for SoEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SoEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SoEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SkEnum(pub Kind);
impl SkEnum {
    pub const BOUNDING: Self = Self(0);
    pub const CLIP: Self = Self(1);
    pub const INPUT: Self = Self(2);
}
impl FixedLengthSerialize<1> for SkEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SkEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(Kind::from_bytes(bytes)?))
    }
}
impl From<u32> for SkEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SkEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SkEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
pub const NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct NotifyEvent {
    pub opcode: u8,
    pub shape_kind: SkEnum,
    pub sequence: u16,
    pub affected_window: crate::proto::xproto::Window,
    pub extents_x: i16,
    pub extents_y: i16,
    pub extents_width: u16,
    pub extents_height: u16,
    pub server_time: crate::proto::xproto::Timestamp,
    pub shaped: u8,
}
impl FixedLengthFromBytes<32> for NotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            shape_kind: Kind::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            affected_window: crate::proto::xproto::Window::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            extents_x: i16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            extents_y: i16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
            extents_width: u16::from_bytes(
                bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?,
            )?,
            extents_height: u16::from_bytes(
                bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            server_time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?,
            )?,
            shaped: u8::from_bytes(bytes.get(20..21).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
}
impl FixedLengthFromBytes<12> for QueryVersionReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            major_version: u16::from_bytes(
                bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?,
            )?,
            minor_version: u16::from_bytes(
                bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryExtentsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub bounding_shaped: u8,
    pub clip_shaped: u8,
    pub bounding_shape_extents_x: i16,
    pub bounding_shape_extents_y: i16,
    pub bounding_shape_extents_width: u16,
    pub bounding_shape_extents_height: u16,
    pub clip_shape_extents_x: i16,
    pub clip_shape_extents_y: i16,
    pub clip_shape_extents_width: u16,
    pub clip_shape_extents_height: u16,
}
impl FixedLengthFromBytes<28> for QueryExtentsReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            bounding_shaped: u8::from_bytes(
                bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?,
            )?,
            clip_shaped: u8::from_bytes(bytes.get(9..10).ok_or(crate::error::Error::FromBytes)?)?,
            bounding_shape_extents_x: i16::from_bytes(
                bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?,
            )?,
            bounding_shape_extents_y: i16::from_bytes(
                bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            bounding_shape_extents_width: u16::from_bytes(
                bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?,
            )?,
            bounding_shape_extents_height: u16::from_bytes(
                bytes.get(18..20).ok_or(crate::error::Error::FromBytes)?,
            )?,
            clip_shape_extents_x: i16::from_bytes(
                bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?,
            )?,
            clip_shape_extents_y: i16::from_bytes(
                bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?,
            )?,
            clip_shape_extents_width: u16::from_bytes(
                bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?,
            )?,
            clip_shape_extents_height: u16::from_bytes(
                bytes.get(26..28).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct InputSelectedReply {
    pub response_type: u8,
    pub enabled: u8,
    pub sequence: u16,
    pub length: u32,
}
impl FixedLengthFromBytes<8> for InputSelectedReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            enabled: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetRectanglesReply {
    pub response_type: u8,
    pub ordering: crate::proto::xproto::ClipOrderingEnum,
    pub sequence: u16,
    pub length: u32,
    pub rectangles: alloc::vec::Vec<crate::proto::xproto::Rectangle>,
}
impl VariableLengthFromBytes for GetRectanglesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let ordering = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let rectangles_len =
            u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = rectangles_len as usize;
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
                ordering: ordering.into(),
                sequence,
                length,
                rectangles,
            },
            offset,
        ))
    }
}
