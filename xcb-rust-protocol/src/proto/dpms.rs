#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "DPMS";
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: u16,
    pub server_minor_version: u16,
}
impl FixedLengthFromBytes<12> for GetVersionReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            server_major_version: u16::from_bytes(
                bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?,
            )?,
            server_minor_version: u16::from_bytes(
                bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CapableReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub capable: u8,
}
impl FixedLengthFromBytes<32> for CapableReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            capable: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetTimeoutsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub standby_timeout: u16,
    pub suspend_timeout: u16,
    pub off_timeout: u16,
}
impl FixedLengthFromBytes<32> for GetTimeoutsReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            standby_timeout: u16::from_bytes(
                bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?,
            )?,
            suspend_timeout: u16::from_bytes(
                bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            off_timeout: u16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct DPMSModeEnum(pub u16);
impl DPMSModeEnum {
    pub const ON: Self = Self(0);
    pub const STANDBY: Self = Self(1);
    pub const SUSPEND: Self = Self(2);
    pub const OFF: Self = Self(3);
}
impl FixedLengthSerialize<2> for DPMSModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for DPMSModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for DPMSModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for DPMSModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for DPMSModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct InfoReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub power_level: DPMSModeEnum,
    pub state: u8,
}
impl FixedLengthFromBytes<32> for InfoReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            power_level: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            state: u8::from_bytes(bytes.get(10..11).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
