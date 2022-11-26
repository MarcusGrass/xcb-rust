#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "GLX";
pub type Pixmap = u32;
pub type Context = u32;
pub type Pbuffer = u32;
pub type Window = u32;
pub type Fbconfig = u32;
pub type Drawable = u32;
pub type Float32 = f32;
pub type Float64 = f64;
pub type Bool32 = u32;
pub type ContextTag = u32;
pub const GENERIC_ERROR: u8 = 255;
pub const BAD_CONTEXT_ERROR: u8 = 0;
pub const BAD_CONTEXT_STATE_ERROR: u8 = 1;
pub const BAD_DRAWABLE_ERROR: u8 = 2;
pub const BAD_PIXMAP_ERROR: u8 = 3;
pub const BAD_CONTEXT_TAG_ERROR: u8 = 4;
pub const BAD_CURRENT_WINDOW_ERROR: u8 = 5;
pub const BAD_RENDER_REQUEST_ERROR: u8 = 6;
pub const BAD_LARGE_REQUEST_ERROR: u8 = 7;
pub const UNSUPPORTED_PRIVATE_REQUEST_ERROR: u8 = 8;
pub const BAD_F_B_CONFIG_ERROR: u8 = 9;
pub const BAD_PBUFFER_ERROR: u8 = 10;
pub const BAD_CURRENT_DRAWABLE_ERROR: u8 = 11;
pub const BAD_WINDOW_ERROR: u8 = 12;
pub const G_L_X_BAD_PROFILE_A_R_B_ERROR: u8 = 13;
pub const PBUFFER_CLOBBER_EVENT: u8 = 0;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct PbufferClobberEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub event_type: u16,
    pub draw_type: u16,
    pub drawable: Drawable,
    pub b_mask: u32,
    pub aux_buffer: u16,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub count: u16,
}
impl FixedLengthFromBytes<32> for PbufferClobberEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            event_type: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            draw_type: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
            drawable: Drawable::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            b_mask: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            aux_buffer: u16::from_bytes(bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?)?,
            x: u16::from_bytes(bytes.get(18..20).ok_or(crate::error::Error::FromBytes)?)?,
            y: u16::from_bytes(bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?)?,
            width: u16::from_bytes(bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?)?,
            count: u16::from_bytes(bytes.get(26..28).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const BUFFER_SWAP_COMPLETE_EVENT: u8 = 1;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct BufferSwapCompleteEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub event_type: u16,
    pub drawable: Drawable,
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
            event_type: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            drawable: Drawable::from_bytes(
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
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct MakeCurrentReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_tag: ContextTag,
}
impl FixedLengthFromBytes<32> for MakeCurrentReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            context_tag: ContextTag::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct IsDirectReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub is_direct: u8,
}
impl FixedLengthFromBytes<32> for IsDirectReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            is_direct: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
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
#[derive(Debug, Clone, PartialEq)]
pub struct GetVisualConfigsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_visuals: u32,
    pub num_properties: u32,
    pub property_list: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for GetVisualConfigsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 16 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_visuals = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let num_properties =
            u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = length as usize;
        let property_list: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                num_visuals,
                num_properties,
                property_list,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct VendorPrivateWithReplyReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub retval: u32,
    pub data1: [u8; 24],
    pub data2: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for VendorPrivateWithReplyReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let retval = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let data1 = <[u8; 24]>::from_bytes(ptr.get(12..36).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(length, 4) as usize;
        let data2: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(36..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 36;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                retval,
                data1,
                data2,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryExtensionsStringReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub n: u32,
}
impl FixedLengthFromBytes<32> for QueryExtensionsStringReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            n: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryServerStringReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub string: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for QueryServerStringReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 16 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let str_len = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = str_len as usize;
        let string: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                string,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetFBConfigsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_fb_configs: u32,
    pub num_properties: u32,
    pub property_list: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for GetFBConfigsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 16 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_fb_configs =
            u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let num_properties =
            u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = length as usize;
        let property_list: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                num_fb_configs,
                num_properties,
                property_list,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub attribs: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for QueryContextReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_attribs = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(num_attribs as u32, 2u32 as u32) as usize;
        let attribs: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                attribs,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct MakeContextCurrentReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_tag: ContextTag,
}
impl FixedLengthFromBytes<32> for MakeContextCurrentReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            context_tag: ContextTag::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetDrawableAttributesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub attribs: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for GetDrawableAttributesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_attribs = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(num_attribs as u32, 2u32 as u32) as usize;
        let attribs: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                attribs,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GenListsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: u32,
}
impl FixedLengthFromBytes<12> for GenListsReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            ret_val: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct RenderModeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: u32,
    pub new_mode: u32,
    pub data: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for RenderModeReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let ret_val = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let new_mode = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                ret_val,
                new_mode,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct FinishReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
}
impl FixedLengthFromBytes<8> for FinishReply {
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
pub struct ReadPixelsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub data: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for ReadPixelsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 24 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(length, 4) as usize;
        let data: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetBooleanvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: u8,
    pub data: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetBooleanvReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 15 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = u8::from_bytes(ptr.get(16..17).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetClipPlaneReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub data: alloc::vec::Vec<Float64>,
}
impl VariableLengthFromBytes for GetClipPlaneReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Start align 8 None
        // Padding 1 bytes
        // Padding 24 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(1..3).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(3..7).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Div::div(length, 2) as usize;
        let data: alloc::vec::Vec<Float64> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Float64,
            length_expr,
            8
        );
        let offset = length_expr * 8 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetDoublevReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float64,
    pub data: alloc::vec::Vec<Float64>,
}
impl VariableLengthFromBytes for GetDoublevReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Start align 8 None
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 8 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(1..3).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(3..7).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = Float64::from_bytes(ptr.get(16..24).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<Float64> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Float64,
            length_expr,
            8
        );
        let offset = length_expr * 8 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetErrorReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub error: i32,
}
impl FixedLengthFromBytes<12> for GetErrorReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            error: i32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetFloatvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: alloc::vec::Vec<Float32>,
}
impl VariableLengthFromBytes for GetFloatvReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = Float32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<Float32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Float32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetIntegervReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: alloc::vec::Vec<i32>,
}
impl VariableLengthFromBytes for GetIntegervReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            i32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetLightfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: alloc::vec::Vec<Float32>,
}
impl VariableLengthFromBytes for GetLightfvReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = Float32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<Float32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Float32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetLightivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: alloc::vec::Vec<i32>,
}
impl VariableLengthFromBytes for GetLightivReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            i32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetMapdvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float64,
    pub data: alloc::vec::Vec<Float64>,
}
impl VariableLengthFromBytes for GetMapdvReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Start align 8 None
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 8 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(1..3).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(3..7).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = Float64::from_bytes(ptr.get(16..24).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<Float64> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Float64,
            length_expr,
            8
        );
        let offset = length_expr * 8 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetMapfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: alloc::vec::Vec<Float32>,
}
impl VariableLengthFromBytes for GetMapfvReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = Float32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<Float32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Float32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetMapivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: alloc::vec::Vec<i32>,
}
impl VariableLengthFromBytes for GetMapivReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            i32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetMaterialfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: alloc::vec::Vec<Float32>,
}
impl VariableLengthFromBytes for GetMaterialfvReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = Float32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<Float32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Float32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetMaterialivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: alloc::vec::Vec<i32>,
}
impl VariableLengthFromBytes for GetMaterialivReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            i32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetPixelMapfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: alloc::vec::Vec<Float32>,
}
impl VariableLengthFromBytes for GetPixelMapfvReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = Float32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<Float32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Float32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetPixelMapuivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: u32,
    pub data: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for GetPixelMapuivReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetPixelMapusvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: u16,
    pub data: alloc::vec::Vec<u16>,
}
impl VariableLengthFromBytes for GetPixelMapusvReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 16 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = u16::from_bytes(ptr.get(16..18).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<u16> = crate::vec_from_bytes_fixed!(
            ptr.get(34..).ok_or(crate::error::Error::FromBytes)?,
            u16,
            length_expr,
            2
        );
        let offset = length_expr * 2 + 34;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetPolygonStippleReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub data: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetPolygonStippleReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 24 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(length, 4) as usize;
        let data: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetStringReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub string: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetStringReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 16 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let string: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                string,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetTexEnvfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: alloc::vec::Vec<Float32>,
}
impl VariableLengthFromBytes for GetTexEnvfvReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = Float32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<Float32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Float32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetTexEnvivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: alloc::vec::Vec<i32>,
}
impl VariableLengthFromBytes for GetTexEnvivReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            i32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetTexGendvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float64,
    pub data: alloc::vec::Vec<Float64>,
}
impl VariableLengthFromBytes for GetTexGendvReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Start align 8 None
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 8 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(1..3).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(3..7).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = Float64::from_bytes(ptr.get(16..24).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<Float64> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Float64,
            length_expr,
            8
        );
        let offset = length_expr * 8 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetTexGenfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: alloc::vec::Vec<Float32>,
}
impl VariableLengthFromBytes for GetTexGenfvReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = Float32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<Float32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Float32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetTexGenivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: alloc::vec::Vec<i32>,
}
impl VariableLengthFromBytes for GetTexGenivReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            i32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetTexImageReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub data: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetTexImageReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 8 bytes
        // Padding 4 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let width = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let height = i32::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let depth = i32::from_bytes(ptr.get(24..28).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(length, 4) as usize;
        let data: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                width,
                height,
                depth,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetTexParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: alloc::vec::Vec<Float32>,
}
impl VariableLengthFromBytes for GetTexParameterfvReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = Float32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<Float32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Float32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetTexParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: alloc::vec::Vec<i32>,
}
impl VariableLengthFromBytes for GetTexParameterivReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            i32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetTexLevelParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: alloc::vec::Vec<Float32>,
}
impl VariableLengthFromBytes for GetTexLevelParameterfvReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = Float32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<Float32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Float32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetTexLevelParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: alloc::vec::Vec<i32>,
}
impl VariableLengthFromBytes for GetTexLevelParameterivReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            i32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct IsEnabledReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl FixedLengthFromBytes<12> for IsEnabledReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            ret_val: Bool32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct IsListReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl FixedLengthFromBytes<12> for IsListReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            ret_val: Bool32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AreTexturesResidentReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
    pub data: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for AreTexturesResidentReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let ret_val = Bool32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(length, 4) as usize;
        let data: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                ret_val,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GenTexturesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub data: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for GenTexturesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 24 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = length as usize;
        let data: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct IsTextureReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl FixedLengthFromBytes<12> for IsTextureReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            ret_val: Bool32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetColorTableReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: i32,
    pub data: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetColorTableReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 8 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let width = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(length, 4) as usize;
        let data: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                width,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetColorTableParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: alloc::vec::Vec<Float32>,
}
impl VariableLengthFromBytes for GetColorTableParameterfvReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = Float32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<Float32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Float32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetColorTableParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: alloc::vec::Vec<i32>,
}
impl VariableLengthFromBytes for GetColorTableParameterivReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            i32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetConvolutionFilterReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: i32,
    pub height: i32,
    pub data: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetConvolutionFilterReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 8 bytes
        // Padding 8 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let width = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let height = i32::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(length, 4) as usize;
        let data: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                width,
                height,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetConvolutionParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: alloc::vec::Vec<Float32>,
}
impl VariableLengthFromBytes for GetConvolutionParameterfvReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = Float32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<Float32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Float32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetConvolutionParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: alloc::vec::Vec<i32>,
}
impl VariableLengthFromBytes for GetConvolutionParameterivReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            i32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetSeparableFilterReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub row_w: i32,
    pub col_h: i32,
    pub rows_and_cols: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetSeparableFilterReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 8 bytes
        // Padding 8 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let row_w = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let col_h = i32::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(length, 4) as usize;
        let rows_and_cols: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                row_w,
                col_h,
                rows_and_cols,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetHistogramReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: i32,
    pub data: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetHistogramReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 8 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let width = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(length, 4) as usize;
        let data: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                width,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetHistogramParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: alloc::vec::Vec<Float32>,
}
impl VariableLengthFromBytes for GetHistogramParameterfvReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = Float32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<Float32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Float32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetHistogramParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: alloc::vec::Vec<i32>,
}
impl VariableLengthFromBytes for GetHistogramParameterivReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            i32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetMinmaxReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub data: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetMinmaxReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 24 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(length, 4) as usize;
        let data: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetMinmaxParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: alloc::vec::Vec<Float32>,
}
impl VariableLengthFromBytes for GetMinmaxParameterfvReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = Float32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<Float32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Float32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetMinmaxParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: alloc::vec::Vec<i32>,
}
impl VariableLengthFromBytes for GetMinmaxParameterivReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            i32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetCompressedTexImageARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: i32,
    pub data: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetCompressedTexImageARBReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 8 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let size = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(length, 4) as usize;
        let data: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                size,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GenQueriesARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub data: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for GenQueriesARBReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 24 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = length as usize;
        let data: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct IsQueryARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl FixedLengthFromBytes<12> for IsQueryARBReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            ret_val: Bool32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetQueryivARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: alloc::vec::Vec<i32>,
}
impl VariableLengthFromBytes for GetQueryivARBReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            i32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetQueryObjectivARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: alloc::vec::Vec<i32>,
}
impl VariableLengthFromBytes for GetQueryObjectivARBReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = i32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            i32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                datum,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetQueryObjectuivARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: u32,
    pub data: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for GetQueryObjectuivARBReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let datum = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n as usize;
        let data: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                datum,
                data,
            },
            offset,
        ))
    }
}
