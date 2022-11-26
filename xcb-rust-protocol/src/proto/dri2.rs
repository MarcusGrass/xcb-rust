#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "DRI2";
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct AttachmentEnum(pub u32);
impl AttachmentEnum {
    pub const BUFFER_FRONT_LEFT: Self = Self(0);
    pub const BUFFER_BACK_LEFT: Self = Self(1);
    pub const BUFFER_FRONT_RIGHT: Self = Self(2);
    pub const BUFFER_BACK_RIGHT: Self = Self(3);
    pub const BUFFER_DEPTH: Self = Self(4);
    pub const BUFFER_STENCIL: Self = Self(5);
    pub const BUFFER_ACCUM: Self = Self(6);
    pub const BUFFER_FAKE_FRONT_LEFT: Self = Self(7);
    pub const BUFFER_FAKE_FRONT_RIGHT: Self = Self(8);
    pub const BUFFER_DEPTH_STENCIL: Self = Self(9);
    pub const BUFFER_HIZ: Self = Self(10);
}
impl FixedLengthSerialize<4> for AttachmentEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for AttachmentEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for AttachmentEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for AttachmentEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for AttachmentEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct DriverTypeEnum(pub u32);
impl DriverTypeEnum {
    pub const DRI: Self = Self(0);
    pub const VDPAU: Self = Self(1);
}
impl FixedLengthSerialize<4> for DriverTypeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for DriverTypeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for DriverTypeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for DriverTypeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for DriverTypeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct EventTypeEnum(pub u16);
impl EventTypeEnum {
    pub const EXCHANGE_COMPLETE: Self = Self(1);
    pub const BLIT_COMPLETE: Self = Self(2);
    pub const FLIP_COMPLETE: Self = Self(3);
}
impl FixedLengthSerialize<2> for EventTypeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for EventTypeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for EventTypeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for EventTypeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for EventTypeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DRI2Buffer {
    pub attachment: AttachmentEnum,
    pub name: u32,
    pub pitch: u32,
    pub cpp: u32,
    pub flags: u32,
}
impl FixedLengthSerialize<20> for DRI2Buffer {
    #[inline]
    fn serialize_fixed(self) -> [u8; 20] {
        let attachment_bytes = self.attachment.serialize_fixed();
        let name_bytes = self.name.serialize_fixed();
        let pitch_bytes = self.pitch.serialize_fixed();
        let cpp_bytes = self.cpp.serialize_fixed();
        let flags_bytes = self.flags.serialize_fixed();
        [
            attachment_bytes[0],
            attachment_bytes[1],
            attachment_bytes[2],
            attachment_bytes[3],
            name_bytes[0],
            name_bytes[1],
            name_bytes[2],
            name_bytes[3],
            pitch_bytes[0],
            pitch_bytes[1],
            pitch_bytes[2],
            pitch_bytes[3],
            cpp_bytes[0],
            cpp_bytes[1],
            cpp_bytes[2],
            cpp_bytes[3],
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<20> for DRI2Buffer {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            attachment: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            name: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            pitch: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            cpp: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            flags: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct AttachFormat {
    pub attachment: AttachmentEnum,
    pub format: u32,
}
impl FixedLengthSerialize<8> for AttachFormat {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let attachment_bytes = self.attachment.serialize_fixed();
        let format_bytes = self.format.serialize_fixed();
        [
            attachment_bytes[0],
            attachment_bytes[1],
            attachment_bytes[2],
            attachment_bytes[3],
            format_bytes[0],
            format_bytes[1],
            format_bytes[2],
            format_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for AttachFormat {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            attachment: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            format: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
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
#[derive(Debug, Clone, PartialEq)]
pub struct ConnectReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub driver_name: alloc::vec::Vec<u8>,
    pub alignment_pad: alloc::vec::Vec<u8>,
    pub device_name: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for ConnectReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 16 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let driver_name_length =
            u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let device_name_length =
            u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = driver_name_length as usize;
        let driver_name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr + 32;
        let length_expr = core::ops::Sub::sub(
            core::ops::BitAnd::bitand(
                core::ops::Add::add(driver_name_length as u32, 3u32 as u32) as u32,
                core::ops::Not::not(3u32) as u32,
            ) as u32,
            driver_name_length as u32,
        ) as usize;
        let alignment_pad: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        offset += length_expr;
        let length_expr = device_name_length as usize;
        let device_name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        offset += length_expr;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                driver_name,
                alignment_pad,
                device_name,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct AuthenticateReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub authenticated: u32,
}
impl FixedLengthFromBytes<12> for AuthenticateReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            authenticated: u32::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetBuffersReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u32,
    pub height: u32,
    pub buffers: alloc::vec::Vec<DRI2Buffer>,
}
impl VariableLengthFromBytes for GetBuffersReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let width = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let height = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let count = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = count as usize;
        let buffers: alloc::vec::Vec<DRI2Buffer> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            DRI2Buffer,
            length_expr,
            20
        );
        let offset = length_expr * 20 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                width,
                height,
                buffers,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CopyRegionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
}
impl FixedLengthFromBytes<8> for CopyRegionReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetBuffersWithFormatReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u32,
    pub height: u32,
    pub buffers: alloc::vec::Vec<DRI2Buffer>,
}
impl VariableLengthFromBytes for GetBuffersWithFormatReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let width = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let height = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let count = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = count as usize;
        let buffers: alloc::vec::Vec<DRI2Buffer> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            DRI2Buffer,
            length_expr,
            20
        );
        let offset = length_expr * 20 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                width,
                height,
                buffers,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SwapBuffersReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub swap_hi: u32,
    pub swap_lo: u32,
}
impl FixedLengthFromBytes<16> for SwapBuffersReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            swap_hi: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            swap_lo: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetMSCReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc_hi: u32,
    pub sbc_lo: u32,
}
impl FixedLengthFromBytes<32> for GetMSCReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            ust_hi: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            ust_lo: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            msc_hi: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            msc_lo: u32::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
            sbc_hi: u32::from_bytes(bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?)?,
            sbc_lo: u32::from_bytes(bytes.get(28..32).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct WaitMSCReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc_hi: u32,
    pub sbc_lo: u32,
}
impl FixedLengthFromBytes<32> for WaitMSCReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            ust_hi: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            ust_lo: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            msc_hi: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            msc_lo: u32::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
            sbc_hi: u32::from_bytes(bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?)?,
            sbc_lo: u32::from_bytes(bytes.get(28..32).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct WaitSBCReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc_hi: u32,
    pub sbc_lo: u32,
}
impl FixedLengthFromBytes<32> for WaitSBCReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            ust_hi: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            ust_lo: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            msc_hi: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            msc_lo: u32::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
            sbc_hi: u32::from_bytes(bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?)?,
            sbc_lo: u32::from_bytes(bytes.get(28..32).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetParamReply {
    pub response_type: u8,
    pub is_param_recognized: u8,
    pub sequence: u16,
    pub length: u32,
    pub value_hi: u32,
    pub value_lo: u32,
}
impl FixedLengthFromBytes<16> for GetParamReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            is_param_recognized: u8::from_bytes(
                bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?,
            )?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            value_hi: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            value_lo: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const BUFFER_SWAP_COMPLETE_EVENT: u8 = 0;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct BufferSwapCompleteEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub event_type: EventTypeEnum,
    pub drawable: crate::proto::xproto::Drawable,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc: u32,
}
impl FixedLengthFromBytes<32> for BufferSwapCompleteEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            event_type: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            drawable: crate::proto::xproto::Drawable::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            ust_hi: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            ust_lo: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            msc_hi: u32::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
            msc_lo: u32::from_bytes(bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?)?,
            sbc: u32::from_bytes(bytes.get(28..32).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const INVALIDATE_BUFFERS_EVENT: u8 = 1;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct InvalidateBuffersEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub drawable: crate::proto::xproto::Drawable,
}
impl FixedLengthFromBytes<8> for InvalidateBuffersEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            drawable: crate::proto::xproto::Drawable::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
