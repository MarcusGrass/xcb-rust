#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "XVideo-MotionCompensation";
pub type Context = u32;
pub type Surface = u32;
pub type Subpicture = u32;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SurfaceInfo {
    pub id: Surface,
    pub chroma_format: u16,
    pub pad0: u16,
    pub max_width: u16,
    pub max_height: u16,
    pub subpicture_max_width: u16,
    pub subpicture_max_height: u16,
    pub mc_type: u32,
    pub flags: u32,
}
impl FixedLengthSerialize<24> for SurfaceInfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 24] {
        let id_bytes = self.id.serialize_fixed();
        let chroma_format_bytes = self.chroma_format.serialize_fixed();
        let pad0_bytes = self.pad0.serialize_fixed();
        let max_width_bytes = self.max_width.serialize_fixed();
        let max_height_bytes = self.max_height.serialize_fixed();
        let subpicture_max_width_bytes = self.subpicture_max_width.serialize_fixed();
        let subpicture_max_height_bytes = self.subpicture_max_height.serialize_fixed();
        let mc_type_bytes = self.mc_type.serialize_fixed();
        let flags_bytes = self.flags.serialize_fixed();
        [
            id_bytes[0],
            id_bytes[1],
            id_bytes[2],
            id_bytes[3],
            chroma_format_bytes[0],
            chroma_format_bytes[1],
            pad0_bytes[0],
            pad0_bytes[1],
            max_width_bytes[0],
            max_width_bytes[1],
            max_height_bytes[0],
            max_height_bytes[1],
            subpicture_max_width_bytes[0],
            subpicture_max_width_bytes[1],
            subpicture_max_height_bytes[0],
            subpicture_max_height_bytes[1],
            mc_type_bytes[0],
            mc_type_bytes[1],
            mc_type_bytes[2],
            mc_type_bytes[3],
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<24> for SurfaceInfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            id: Surface::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            chroma_format: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            pad0: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
            max_width: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            max_height: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
            subpicture_max_width: u16::from_bytes(
                bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?,
            )?,
            subpicture_max_height: u16::from_bytes(
                bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            mc_type: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            flags: u32::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major: u32,
    pub minor: u32,
}
impl FixedLengthFromBytes<16> for QueryVersionReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            major: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            minor: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListSurfaceTypesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub surfaces: alloc::vec::Vec<SurfaceInfo>,
}
impl VariableLengthFromBytes for ListSurfaceTypesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num as usize;
        let surfaces: alloc::vec::Vec<SurfaceInfo> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            SurfaceInfo,
            length_expr,
            24
        );
        let offset = length_expr * 24 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                surfaces,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CreateContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width_actual: u16,
    pub height_actual: u16,
    pub flags_return: u32,
    pub priv_data: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for CreateContextReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let width_actual = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let height_actual =
            u16::from_bytes(ptr.get(10..12).ok_or(crate::error::Error::FromBytes)?)?;
        let flags_return = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = length as usize;
        let priv_data: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(36..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 36;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                width_actual,
                height_actual,
                flags_return,
                priv_data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CreateSurfaceReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub priv_data: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for CreateSurfaceReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 24 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = length as usize;
        let priv_data: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                priv_data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CreateSubpictureReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width_actual: u16,
    pub height_actual: u16,
    pub num_palette_entries: u16,
    pub entry_bytes: u16,
    pub component_order: [u8; 4],
    pub priv_data: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for CreateSubpictureReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let width_actual = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let height_actual =
            u16::from_bytes(ptr.get(10..12).ok_or(crate::error::Error::FromBytes)?)?;
        let num_palette_entries =
            u16::from_bytes(ptr.get(12..14).ok_or(crate::error::Error::FromBytes)?)?;
        let entry_bytes = u16::from_bytes(ptr.get(14..16).ok_or(crate::error::Error::FromBytes)?)?;
        let component_order =
            <[u8; 4]>::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = length as usize;
        let priv_data: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                width_actual,
                height_actual,
                num_palette_entries,
                entry_bytes,
                component_order,
                priv_data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListSubpictureTypesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub types: alloc::vec::Vec<crate::proto::xv::ImageFormatInfo>,
}
impl VariableLengthFromBytes for ListSubpictureTypesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num as usize;
        let types: alloc::vec::Vec<crate::proto::xv::ImageFormatInfo> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            crate::proto::xv::ImageFormatInfo,
            length_expr,
            128
        );
        let offset = length_expr * 128 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                types,
            },
            offset,
        ))
    }
}
