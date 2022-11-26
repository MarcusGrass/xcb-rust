#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "XINERAMA";
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ScreenInfo {
    pub x_org: i16,
    pub y_org: i16,
    pub width: u16,
    pub height: u16,
}
impl FixedLengthSerialize<8> for ScreenInfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let x_org_bytes = self.x_org.serialize_fixed();
        let y_org_bytes = self.y_org.serialize_fixed();
        let width_bytes = self.width.serialize_fixed();
        let height_bytes = self.height.serialize_fixed();
        [
            x_org_bytes[0],
            x_org_bytes[1],
            y_org_bytes[0],
            y_org_bytes[1],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<8> for ScreenInfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            x_org: i16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            y_org: i16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            width: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major: u16,
    pub minor: u16,
}
impl FixedLengthFromBytes<12> for QueryVersionReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            major: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            minor: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetStateReply {
    pub response_type: u8,
    pub state: u8,
    pub sequence: u16,
    pub length: u32,
    pub window: crate::proto::xproto::Window,
}
impl FixedLengthFromBytes<12> for GetStateReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            state: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetScreenCountReply {
    pub response_type: u8,
    pub screen_count: u8,
    pub sequence: u16,
    pub length: u32,
    pub window: crate::proto::xproto::Window,
}
impl FixedLengthFromBytes<12> for GetScreenCountReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            screen_count: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetScreenSizeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u32,
    pub height: u32,
    pub window: crate::proto::xproto::Window,
    pub screen: u32,
}
impl FixedLengthFromBytes<24> for GetScreenSizeReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            width: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            height: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?,
            )?,
            screen: u32::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct IsActiveReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub state: u32,
}
impl FixedLengthFromBytes<12> for IsActiveReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            state: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryScreensReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub screen_info: alloc::vec::Vec<ScreenInfo>,
}
impl VariableLengthFromBytes for QueryScreensReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let number = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = number as usize;
        let screen_info: alloc::vec::Vec<ScreenInfo> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            ScreenInfo,
            length_expr,
            8
        );
        let offset = length_expr * 8 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                screen_info,
            },
            offset,
        ))
    }
}
