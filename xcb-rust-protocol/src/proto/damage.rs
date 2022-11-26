#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "DAMAGE";
pub type Damage = u32;
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ReportLevelEnum(pub u8);
impl ReportLevelEnum {
    pub const RAW_RECTANGLES: Self = Self(0);
    pub const DELTA_RECTANGLES: Self = Self(1);
    pub const BOUNDING_BOX: Self = Self(2);
    pub const NON_EMPTY: Self = Self(3);
}
impl FixedLengthSerialize<1> for ReportLevelEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ReportLevelEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ReportLevelEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ReportLevelEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ReportLevelEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
pub const BAD_DAMAGE_ERROR: u8 = 0;
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
pub const NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct NotifyEvent {
    pub opcode: u8,
    pub level: ReportLevelEnum,
    pub sequence: u16,
    pub drawable: crate::proto::xproto::Drawable,
    pub damage: Damage,
    pub timestamp: crate::proto::xproto::Timestamp,
    pub area: crate::proto::xproto::Rectangle,
    pub geometry: crate::proto::xproto::Rectangle,
}
impl FixedLengthFromBytes<32> for NotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            level: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            drawable: crate::proto::xproto::Drawable::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            damage: Damage::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            area: crate::proto::xproto::Rectangle::from_bytes(
                bytes.get(16..24).ok_or(crate::error::Error::FromBytes)?,
            )?,
            geometry: crate::proto::xproto::Rectangle::from_bytes(
                bytes.get(24..32).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
