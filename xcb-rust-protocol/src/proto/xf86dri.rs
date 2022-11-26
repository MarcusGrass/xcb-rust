#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "XFree86-DRI";
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DrmClipRect {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub x3: i16,
}
impl FixedLengthSerialize<8> for DrmClipRect {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let x1_bytes = self.x1.serialize_fixed();
        let y1_bytes = self.y1.serialize_fixed();
        let x2_bytes = self.x2.serialize_fixed();
        let x3_bytes = self.x3.serialize_fixed();
        [
            x1_bytes[0],
            x1_bytes[1],
            y1_bytes[0],
            y1_bytes[1],
            x2_bytes[0],
            x2_bytes[1],
            x3_bytes[0],
            x3_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<8> for DrmClipRect {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            x1: i16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            y1: i16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            x2: i16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            x3: i16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub dri_major_version: u16,
    pub dri_minor_version: u16,
    pub dri_minor_patch: u32,
}
impl FixedLengthFromBytes<16> for QueryVersionReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            dri_major_version: u16::from_bytes(
                bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?,
            )?,
            dri_minor_version: u16::from_bytes(
                bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            dri_minor_patch: u32::from_bytes(
                bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryDirectRenderingCapableReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub is_capable: u8,
}
impl FixedLengthFromBytes<9> for QueryDirectRenderingCapableReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            is_capable: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct OpenConnectionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub sarea_handle_low: u32,
    pub sarea_handle_high: u32,
    pub bus_id: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for OpenConnectionReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let sarea_handle_low =
            u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let sarea_handle_high =
            u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let bus_id_len = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = bus_id_len as usize;
        let bus_id: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                sarea_handle_low,
                sarea_handle_high,
                bus_id,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetClientDriverNameReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub client_driver_major_version: u32,
    pub client_driver_minor_version: u32,
    pub client_driver_patch_version: u32,
    pub client_driver_name: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetClientDriverNameReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 8 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let client_driver_major_version =
            u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let client_driver_minor_version =
            u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let client_driver_patch_version =
            u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let client_driver_name_len =
            u32::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = client_driver_name_len as usize;
        let client_driver_name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                client_driver_major_version,
                client_driver_minor_version,
                client_driver_patch_version,
                client_driver_name,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CreateContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub hw_context: u32,
}
impl FixedLengthFromBytes<12> for CreateContextReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            hw_context: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CreateDrawableReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub hw_drawable_handle: u32,
}
impl FixedLengthFromBytes<12> for CreateDrawableReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            hw_drawable_handle: u32::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetDrawableInfoReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub drawable_table_index: u32,
    pub drawable_table_stamp: u32,
    pub drawable_origin_x: i16,
    pub drawable_origin_y: i16,
    pub drawable_size_w: i16,
    pub drawable_size_h: i16,
    pub back_x: i16,
    pub back_y: i16,
    pub clip_rects: alloc::vec::Vec<DrmClipRect>,
    pub back_clip_rects: alloc::vec::Vec<DrmClipRect>,
}
impl VariableLengthFromBytes for GetDrawableInfoReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let drawable_table_index =
            u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let drawable_table_stamp =
            u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let drawable_origin_x =
            i16::from_bytes(ptr.get(16..18).ok_or(crate::error::Error::FromBytes)?)?;
        let drawable_origin_y =
            i16::from_bytes(ptr.get(18..20).ok_or(crate::error::Error::FromBytes)?)?;
        let drawable_size_w =
            i16::from_bytes(ptr.get(20..22).ok_or(crate::error::Error::FromBytes)?)?;
        let drawable_size_h =
            i16::from_bytes(ptr.get(22..24).ok_or(crate::error::Error::FromBytes)?)?;
        let num_clip_rects =
            u32::from_bytes(ptr.get(24..28).ok_or(crate::error::Error::FromBytes)?)?;
        let back_x = i16::from_bytes(ptr.get(28..30).ok_or(crate::error::Error::FromBytes)?)?;
        let back_y = i16::from_bytes(ptr.get(30..32).ok_or(crate::error::Error::FromBytes)?)?;
        let num_back_clip_rects =
            u32::from_bytes(ptr.get(32..36).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_clip_rects as usize;
        let clip_rects: alloc::vec::Vec<DrmClipRect> = crate::vec_from_bytes_fixed!(
            ptr.get(36..).ok_or(crate::error::Error::FromBytes)?,
            DrmClipRect,
            length_expr,
            8
        );
        let mut offset = length_expr * 8 + 36;
        let length_expr = num_back_clip_rects as usize;
        let back_clip_rects: alloc::vec::Vec<DrmClipRect> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            DrmClipRect,
            length_expr,
            8
        );
        offset += length_expr * 8;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                drawable_table_index,
                drawable_table_stamp,
                drawable_origin_x,
                drawable_origin_y,
                drawable_size_w,
                drawable_size_h,
                back_x,
                back_y,
                clip_rects,
                back_clip_rects,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetDeviceInfoReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub framebuffer_handle_low: u32,
    pub framebuffer_handle_high: u32,
    pub framebuffer_origin_offset: u32,
    pub framebuffer_size: u32,
    pub framebuffer_stride: u32,
    pub device_private: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for GetDeviceInfoReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let framebuffer_handle_low =
            u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let framebuffer_handle_high =
            u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let framebuffer_origin_offset =
            u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let framebuffer_size =
            u32::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let framebuffer_stride =
            u32::from_bytes(ptr.get(24..28).ok_or(crate::error::Error::FromBytes)?)?;
        let device_private_size =
            u32::from_bytes(ptr.get(28..32).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = device_private_size as usize;
        let device_private: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                framebuffer_handle_low,
                framebuffer_handle_high,
                framebuffer_origin_offset,
                framebuffer_size,
                framebuffer_stride,
                device_private,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct AuthConnectionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub authenticated: u32,
}
impl FixedLengthFromBytes<12> for AuthConnectionReply {
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
