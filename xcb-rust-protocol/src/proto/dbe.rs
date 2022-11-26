#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "DOUBLE-BUFFER";
pub type BackBuffer = crate::proto::xproto::Drawable;
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SwapActionEnum(pub u8);
impl SwapActionEnum {
    pub const UNDEFINED: Self = Self(0);
    pub const BACKGROUND: Self = Self(1);
    pub const UNTOUCHED: Self = Self(2);
    pub const COPIED: Self = Self(3);
}
impl FixedLengthSerialize<1> for SwapActionEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SwapActionEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for SwapActionEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SwapActionEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SwapActionEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SwapInfo {
    pub window: crate::proto::xproto::Window,
    pub swap_action: SwapActionEnum,
}
impl FixedLengthSerialize<8> for SwapInfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let window_bytes = self.window.serialize_fixed();
        [
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            self.swap_action.0 as u8,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for SwapInfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            swap_action: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct BufferAttributes {
    pub window: crate::proto::xproto::Window,
}
impl FixedLengthSerialize<4> for BufferAttributes {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        let window_bytes = self.window.serialize_fixed();
        [
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<4> for BufferAttributes {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct VisualInfo {
    pub visual_id: crate::proto::xproto::Visualid,
    pub depth: u8,
    pub perf_level: u8,
}
impl FixedLengthSerialize<8> for VisualInfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let visual_id_bytes = self.visual_id.serialize_fixed();
        [
            visual_id_bytes[0],
            visual_id_bytes[1],
            visual_id_bytes[2],
            visual_id_bytes[3],
            self.depth,
            self.perf_level,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for VisualInfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            visual_id: crate::proto::xproto::Visualid::from_bytes(
                bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            depth: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?,
            perf_level: u8::from_bytes(bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct VisualInfos {
    pub infos: alloc::vec::Vec<VisualInfo>,
}
impl VariableLengthSerialize for VisualInfos {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let n_infos =
            u32::try_from(self.infos.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(n_infos).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.infos.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?,
            self.infos,
        )?;
        let offset = list_len + 4;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for VisualInfos {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let n_infos = u32::from_bytes(ptr.get(0..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n_infos as usize;
        let infos: alloc::vec::Vec<VisualInfo> = crate::vec_from_bytes_fixed!(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            VisualInfo,
            length_expr,
            8
        );
        let offset = length_expr * 8 + 4;
        Ok((Self { infos }, offset))
    }
}
pub const BAD_BUFFER_ERROR: u8 = 0;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u8,
    pub minor_version: u8,
}
impl FixedLengthFromBytes<32> for QueryVersionReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            major_version: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            minor_version: u8::from_bytes(bytes.get(9..10).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetVisualInfoReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub supported_visuals: alloc::vec::Vec<VisualInfos>,
}
impl VariableLengthFromBytes for GetVisualInfoReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n_supported_visuals =
            u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let supported_visuals_length = n_supported_visuals as usize;
        let mut offset = 32;
        let supported_visuals =
            crate::vec_from_bytes_var!(ptr, VisualInfos, offset, supported_visuals_length,);
        Ok((
            Self {
                response_type,
                sequence,
                length,
                supported_visuals,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetBackBufferAttributesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub attributes: BufferAttributes,
}
impl FixedLengthFromBytes<32> for GetBackBufferAttributesReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            attributes: BufferAttributes::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
