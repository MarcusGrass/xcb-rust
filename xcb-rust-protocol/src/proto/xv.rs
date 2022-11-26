#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "XVideo";
pub type Port = u32;
pub type Encoding = u32;
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Type(pub u8);
impl Type {
    pub const INPUT_MASK: Self = Self(1 << 0);
    pub const OUTPUT_MASK: Self = Self(1 << 1);
    pub const VIDEO_MASK: Self = Self(1 << 2);
    pub const STILL_MASK: Self = Self(1 << 3);
    pub const IMAGE_MASK: Self = Self(1 << 4);
}
impl FixedLengthSerialize<1> for Type {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for Type {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for Type {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for Type {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for Type {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(Type);
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ImageFormatInfoTypeEnum(pub u8);
impl ImageFormatInfoTypeEnum {
    pub const RGB: Self = Self(0);
    pub const YUV: Self = Self(1);
}
impl FixedLengthSerialize<1> for ImageFormatInfoTypeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ImageFormatInfoTypeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ImageFormatInfoTypeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ImageFormatInfoTypeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ImageFormatInfoTypeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ImageFormatInfoFormatEnum(pub u8);
impl ImageFormatInfoFormatEnum {
    pub const PACKED: Self = Self(0);
    pub const PLANAR: Self = Self(1);
}
impl FixedLengthSerialize<1> for ImageFormatInfoFormatEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ImageFormatInfoFormatEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ImageFormatInfoFormatEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ImageFormatInfoFormatEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ImageFormatInfoFormatEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct AttributeFlag(pub u32);
impl AttributeFlag {
    pub const GETTABLE: Self = Self(1 << 0);
    pub const SETTABLE: Self = Self(1 << 1);
}
impl FixedLengthSerialize<4> for AttributeFlag {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for AttributeFlag {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for AttributeFlag {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for AttributeFlag {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for AttributeFlag {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(AttributeFlag);
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct VideoNotifyReasonEnum(pub u8);
impl VideoNotifyReasonEnum {
    pub const STARTED: Self = Self(0);
    pub const STOPPED: Self = Self(1);
    pub const BUSY: Self = Self(2);
    pub const PREEMPTED: Self = Self(3);
    pub const HARD_ERROR: Self = Self(4);
}
impl FixedLengthSerialize<1> for VideoNotifyReasonEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for VideoNotifyReasonEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for VideoNotifyReasonEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for VideoNotifyReasonEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for VideoNotifyReasonEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ScanlineOrderEnum(pub u8);
impl ScanlineOrderEnum {
    pub const TOP_TO_BOTTOM: Self = Self(0);
    pub const BOTTOM_TO_TOP: Self = Self(1);
}
impl FixedLengthSerialize<1> for ScanlineOrderEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ScanlineOrderEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ScanlineOrderEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ScanlineOrderEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ScanlineOrderEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct GrabPortStatusEnum(pub u8);
impl GrabPortStatusEnum {
    pub const SUCCESS: Self = Self(0);
    pub const BAD_EXTENSION: Self = Self(1);
    pub const ALREADY_GRABBED: Self = Self(2);
    pub const INVALID_TIME: Self = Self(3);
    pub const BAD_REPLY: Self = Self(4);
    pub const BAD_ALLOC: Self = Self(5);
}
impl FixedLengthSerialize<1> for GrabPortStatusEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for GrabPortStatusEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for GrabPortStatusEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for GrabPortStatusEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for GrabPortStatusEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Rational {
    pub numerator: i32,
    pub denominator: i32,
}
impl FixedLengthSerialize<8> for Rational {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let numerator_bytes = self.numerator.serialize_fixed();
        let denominator_bytes = self.denominator.serialize_fixed();
        [
            numerator_bytes[0],
            numerator_bytes[1],
            numerator_bytes[2],
            numerator_bytes[3],
            denominator_bytes[0],
            denominator_bytes[1],
            denominator_bytes[2],
            denominator_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for Rational {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            numerator: i32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            denominator: i32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Format {
    pub visual: crate::proto::xproto::Visualid,
    pub depth: u8,
}
impl FixedLengthSerialize<8> for Format {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let visual_bytes = self.visual.serialize_fixed();
        [
            visual_bytes[0],
            visual_bytes[1],
            visual_bytes[2],
            visual_bytes[3],
            self.depth,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for Format {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            visual: crate::proto::xproto::Visualid::from_bytes(
                bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            depth: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AdaptorInfo {
    pub base_id: Port,
    pub num_ports: u16,
    pub r#type: Type,
    pub name: alloc::vec::Vec<u8>,
    pub formats: alloc::vec::Vec<Format>,
}
impl VariableLengthSerialize for AdaptorInfo {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let name_size =
            u16::try_from(self.name.len()).map_err(|_| crate::error::Error::Serialize)?;
        let num_formats =
            u16::try_from(self.formats.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Padding 1 bytes
        buf_ptr
            .get_mut(0..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.base_id.serialize_fixed());
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(name_size).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.num_ports.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_formats).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(10..11)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.r#type.serialize_fixed());
        let list_len = self.name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            &self.name,
        )?;
        let mut offset = list_len + 12;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let list_len = self.formats.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            self.formats,
        )?;
        offset += list_len;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for AdaptorInfo {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        let base_id = Port::from_bytes(ptr.get(0..4).ok_or(crate::error::Error::FromBytes)?)?;
        let name_size = u16::from_bytes(ptr.get(4..6).ok_or(crate::error::Error::FromBytes)?)?;
        let num_ports = u16::from_bytes(ptr.get(6..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_formats = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let r#type = u8::from_bytes(ptr.get(10..11).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = name_size as usize;
        let name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(12..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr + 12;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let length_expr = num_formats as usize;
        let formats: alloc::vec::Vec<Format> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Format,
            length_expr,
            8
        );
        offset += length_expr * 8;
        Ok((
            Self {
                base_id,
                num_ports,
                r#type: r#type.into(),
                name,
                formats,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct EncodingInfo {
    pub encoding: Encoding,
    pub width: u16,
    pub height: u16,
    pub rate: Rational,
    pub name: alloc::vec::Vec<u8>,
}
impl VariableLengthSerialize for EncodingInfo {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let name_size =
            u16::try_from(self.name.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Padding 2 bytes
        buf_ptr
            .get_mut(0..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.encoding.serialize_fixed());
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(name_size).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.width.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.height.serialize_fixed());
        buf_ptr
            .get_mut(12..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.rate.serialize_fixed());
        let list_len = self.name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(20..)
                .ok_or(crate::error::Error::Serialize)?,
            &self.name,
        )?;
        let mut offset = list_len + 20;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for EncodingInfo {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 2 bytes
        let encoding = Encoding::from_bytes(ptr.get(0..4).ok_or(crate::error::Error::FromBytes)?)?;
        let name_size = u16::from_bytes(ptr.get(4..6).ok_or(crate::error::Error::FromBytes)?)?;
        let width = u16::from_bytes(ptr.get(6..8).ok_or(crate::error::Error::FromBytes)?)?;
        let height = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let rate = Rational::from_bytes(ptr.get(12..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = name_size as usize;
        let name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(20..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr + 20;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok((
            Self {
                encoding,
                width,
                height,
                rate,
                name,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub id: u32,
    pub width: u16,
    pub height: u16,
    pub pitches: alloc::vec::Vec<u32>,
    pub offsets: alloc::vec::Vec<u32>,
    pub data: alloc::vec::Vec<u8>,
}
impl VariableLengthSerialize for Image {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let data_size =
            u32::try_from(self.data.len()).map_err(|_| crate::error::Error::Serialize)?;
        let num_planes =
            u32::try_from(self.offsets.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.id.serialize_fixed());
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.width.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.height.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(data_size).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(num_planes).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.pitches.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            self.pitches,
        )?;
        let mut offset = list_len + 16;
        let list_len = self.offsets.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            self.offsets,
        )?;
        offset += list_len;
        let list_len = self.data.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            &self.data,
        )?;
        offset += list_len;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for Image {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let id = u32::from_bytes(ptr.get(0..4).ok_or(crate::error::Error::FromBytes)?)?;
        let width = u16::from_bytes(ptr.get(4..6).ok_or(crate::error::Error::FromBytes)?)?;
        let height = u16::from_bytes(ptr.get(6..8).ok_or(crate::error::Error::FromBytes)?)?;
        let data_size = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let num_planes = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_planes as usize;
        let pitches: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(16..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 16;
        let length_expr = num_planes as usize;
        let offsets: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        offset += length_expr * 4;
        let length_expr = data_size as usize;
        let data: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        offset += length_expr;
        Ok((
            Self {
                id,
                width,
                height,
                pitches,
                offsets,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeInfo {
    pub flags: AttributeFlag,
    pub min: i32,
    pub max: i32,
    pub name: alloc::vec::Vec<u8>,
}
impl VariableLengthSerialize for AttributeInfo {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let size = u32::try_from(self.name.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.flags.serialize_fixed());
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.min.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.max.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(size).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            &self.name,
        )?;
        let mut offset = list_len + 16;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for AttributeInfo {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let flags = u32::from_bytes(ptr.get(0..4).ok_or(crate::error::Error::FromBytes)?)?;
        let min = i32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let max = i32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let size = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = size as usize;
        let name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(16..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr + 16;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok((
            Self {
                flags: flags.into(),
                min,
                max,
                name,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ImageFormatInfo {
    pub id: u32,
    pub r#type: ImageFormatInfoTypeEnum,
    pub byte_order: crate::proto::xproto::ImageOrderEnum,
    pub guid: [u8; 16],
    pub bpp: u8,
    pub num_planes: u8,
    pub depth: u8,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub format: ImageFormatInfoFormatEnum,
    pub y_sample_bits: u32,
    pub u_sample_bits: u32,
    pub v_sample_bits: u32,
    pub vhorz_y_period: u32,
    pub vhorz_u_period: u32,
    pub vhorz_v_period: u32,
    pub vvert_y_period: u32,
    pub vvert_u_period: u32,
    pub vvert_v_period: u32,
    pub vcomp_order: [u8; 32],
    pub vscanline_order: ScanlineOrderEnum,
}
impl FixedLengthSerialize<128> for ImageFormatInfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 128] {
        let id_bytes = self.id.serialize_fixed();
        let red_mask_bytes = self.red_mask.serialize_fixed();
        let green_mask_bytes = self.green_mask.serialize_fixed();
        let blue_mask_bytes = self.blue_mask.serialize_fixed();
        let y_sample_bits_bytes = self.y_sample_bits.serialize_fixed();
        let u_sample_bits_bytes = self.u_sample_bits.serialize_fixed();
        let v_sample_bits_bytes = self.v_sample_bits.serialize_fixed();
        let vhorz_y_period_bytes = self.vhorz_y_period.serialize_fixed();
        let vhorz_u_period_bytes = self.vhorz_u_period.serialize_fixed();
        let vhorz_v_period_bytes = self.vhorz_v_period.serialize_fixed();
        let vvert_y_period_bytes = self.vvert_y_period.serialize_fixed();
        let vvert_u_period_bytes = self.vvert_u_period.serialize_fixed();
        let vvert_v_period_bytes = self.vvert_v_period.serialize_fixed();
        [
            id_bytes[0],
            id_bytes[1],
            id_bytes[2],
            id_bytes[3],
            self.r#type.0 as u8,
            self.byte_order.0 as u8,
            0,
            0,
            self.guid[0],
            self.guid[1],
            self.guid[2],
            self.guid[3],
            self.guid[4],
            self.guid[5],
            self.guid[6],
            self.guid[7],
            self.guid[8],
            self.guid[9],
            self.guid[10],
            self.guid[11],
            self.guid[12],
            self.guid[13],
            self.guid[14],
            self.guid[15],
            self.bpp,
            self.num_planes,
            0,
            0,
            self.depth,
            0,
            0,
            0,
            red_mask_bytes[0],
            red_mask_bytes[1],
            red_mask_bytes[2],
            red_mask_bytes[3],
            green_mask_bytes[0],
            green_mask_bytes[1],
            green_mask_bytes[2],
            green_mask_bytes[3],
            blue_mask_bytes[0],
            blue_mask_bytes[1],
            blue_mask_bytes[2],
            blue_mask_bytes[3],
            self.format.0 as u8,
            0,
            0,
            0,
            y_sample_bits_bytes[0],
            y_sample_bits_bytes[1],
            y_sample_bits_bytes[2],
            y_sample_bits_bytes[3],
            u_sample_bits_bytes[0],
            u_sample_bits_bytes[1],
            u_sample_bits_bytes[2],
            u_sample_bits_bytes[3],
            v_sample_bits_bytes[0],
            v_sample_bits_bytes[1],
            v_sample_bits_bytes[2],
            v_sample_bits_bytes[3],
            vhorz_y_period_bytes[0],
            vhorz_y_period_bytes[1],
            vhorz_y_period_bytes[2],
            vhorz_y_period_bytes[3],
            vhorz_u_period_bytes[0],
            vhorz_u_period_bytes[1],
            vhorz_u_period_bytes[2],
            vhorz_u_period_bytes[3],
            vhorz_v_period_bytes[0],
            vhorz_v_period_bytes[1],
            vhorz_v_period_bytes[2],
            vhorz_v_period_bytes[3],
            vvert_y_period_bytes[0],
            vvert_y_period_bytes[1],
            vvert_y_period_bytes[2],
            vvert_y_period_bytes[3],
            vvert_u_period_bytes[0],
            vvert_u_period_bytes[1],
            vvert_u_period_bytes[2],
            vvert_u_period_bytes[3],
            vvert_v_period_bytes[0],
            vvert_v_period_bytes[1],
            vvert_v_period_bytes[2],
            vvert_v_period_bytes[3],
            self.vcomp_order[0],
            self.vcomp_order[1],
            self.vcomp_order[2],
            self.vcomp_order[3],
            self.vcomp_order[4],
            self.vcomp_order[5],
            self.vcomp_order[6],
            self.vcomp_order[7],
            self.vcomp_order[8],
            self.vcomp_order[9],
            self.vcomp_order[10],
            self.vcomp_order[11],
            self.vcomp_order[12],
            self.vcomp_order[13],
            self.vcomp_order[14],
            self.vcomp_order[15],
            self.vcomp_order[16],
            self.vcomp_order[17],
            self.vcomp_order[18],
            self.vcomp_order[19],
            self.vcomp_order[20],
            self.vcomp_order[21],
            self.vcomp_order[22],
            self.vcomp_order[23],
            self.vcomp_order[24],
            self.vcomp_order[25],
            self.vcomp_order[26],
            self.vcomp_order[27],
            self.vcomp_order[28],
            self.vcomp_order[29],
            self.vcomp_order[30],
            self.vcomp_order[31],
            self.vscanline_order.0 as u8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<128> for ImageFormatInfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            id: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            r#type: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?.into(),
            byte_order: u8::from_bytes(bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            // SAFETY: We know we can try into exact size slice
            guid: unsafe {
                bytes
                    .get(8..8 + 16)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
            bpp: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            num_planes: u8::from_bytes(bytes.get(9..10).ok_or(crate::error::Error::FromBytes)?)?,
            depth: u8::from_bytes(bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?)?,
            red_mask: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            green_mask: u32::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
            blue_mask: u32::from_bytes(bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?)?,
            format: u8::from_bytes(bytes.get(28..29).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            y_sample_bits: u32::from_bytes(
                bytes.get(32..36).ok_or(crate::error::Error::FromBytes)?,
            )?,
            u_sample_bits: u32::from_bytes(
                bytes.get(36..40).ok_or(crate::error::Error::FromBytes)?,
            )?,
            v_sample_bits: u32::from_bytes(
                bytes.get(40..44).ok_or(crate::error::Error::FromBytes)?,
            )?,
            vhorz_y_period: u32::from_bytes(
                bytes.get(44..48).ok_or(crate::error::Error::FromBytes)?,
            )?,
            vhorz_u_period: u32::from_bytes(
                bytes.get(48..52).ok_or(crate::error::Error::FromBytes)?,
            )?,
            vhorz_v_period: u32::from_bytes(
                bytes.get(52..56).ok_or(crate::error::Error::FromBytes)?,
            )?,
            vvert_y_period: u32::from_bytes(
                bytes.get(56..60).ok_or(crate::error::Error::FromBytes)?,
            )?,
            vvert_u_period: u32::from_bytes(
                bytes.get(60..64).ok_or(crate::error::Error::FromBytes)?,
            )?,
            vvert_v_period: u32::from_bytes(
                bytes.get(64..68).ok_or(crate::error::Error::FromBytes)?,
            )?,
            // SAFETY: We know we can try into exact size slice
            vcomp_order: unsafe {
                bytes
                    .get(68..68 + 32)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
            vscanline_order: u8::from_bytes(
                bytes.get(68..69).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
        })
    }
}
pub const BAD_PORT_ERROR: u8 = 0;
pub const BAD_ENCODING_ERROR: u8 = 1;
pub const BAD_CONTROL_ERROR: u8 = 2;
pub const VIDEO_NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct VideoNotifyEvent {
    pub opcode: u8,
    pub reason: VideoNotifyReasonEnum,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub drawable: crate::proto::xproto::Drawable,
    pub port: Port,
}
impl FixedLengthFromBytes<16> for VideoNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            reason: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            drawable: crate::proto::xproto::Drawable::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            port: Port::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const PORT_NOTIFY_EVENT: u8 = 1;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct PortNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub port: Port,
    pub attribute: u32,
    pub value: i32,
}
impl FixedLengthFromBytes<20> for PortNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            port: Port::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            attribute: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            value: i32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryExtensionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major: u16,
    pub minor: u16,
}
impl FixedLengthFromBytes<12> for QueryExtensionReply {
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
#[derive(Debug, Clone, PartialEq)]
pub struct QueryAdaptorsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub info: alloc::vec::Vec<AdaptorInfo>,
}
impl VariableLengthFromBytes for QueryAdaptorsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_adaptors = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let info_length = num_adaptors as usize;
        let mut offset = 32;
        let info = crate::vec_from_bytes_var!(ptr, AdaptorInfo, offset, info_length,);
        Ok((
            Self {
                response_type,
                sequence,
                length,
                info,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryEncodingsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub info: alloc::vec::Vec<EncodingInfo>,
}
impl VariableLengthFromBytes for QueryEncodingsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_encodings = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let info_length = num_encodings as usize;
        let mut offset = 32;
        let info = crate::vec_from_bytes_var!(ptr, EncodingInfo, offset, info_length,);
        Ok((
            Self {
                response_type,
                sequence,
                length,
                info,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GrabPortReply {
    pub response_type: u8,
    pub result: GrabPortStatusEnum,
    pub sequence: u16,
    pub length: u32,
}
impl FixedLengthFromBytes<8> for GrabPortReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            result: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryBestSizeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub actual_width: u16,
    pub actual_height: u16,
}
impl FixedLengthFromBytes<12> for QueryBestSizeReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            actual_width: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            actual_height: u16::from_bytes(
                bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetPortAttributeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub value: i32,
}
impl FixedLengthFromBytes<12> for GetPortAttributeReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            value: i32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryPortAttributesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub text_size: u32,
    pub attributes: alloc::vec::Vec<AttributeInfo>,
}
impl VariableLengthFromBytes for QueryPortAttributesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 16 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_attributes =
            u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let text_size = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let attributes_length = num_attributes as usize;
        let mut offset = 32;
        let attributes = crate::vec_from_bytes_var!(ptr, AttributeInfo, offset, attributes_length,);
        Ok((
            Self {
                response_type,
                sequence,
                length,
                text_size,
                attributes,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListImageFormatsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub format: alloc::vec::Vec<ImageFormatInfo>,
}
impl VariableLengthFromBytes for ListImageFormatsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_formats = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_formats as usize;
        let format: alloc::vec::Vec<ImageFormatInfo> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            ImageFormatInfo,
            length_expr,
            128
        );
        let offset = length_expr * 128 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                format,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryImageAttributesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub data_size: u32,
    pub width: u16,
    pub height: u16,
    pub pitches: alloc::vec::Vec<u32>,
    pub offsets: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for QueryImageAttributesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_planes = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let data_size = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let width = u16::from_bytes(ptr.get(16..18).ok_or(crate::error::Error::FromBytes)?)?;
        let height = u16::from_bytes(ptr.get(18..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_planes as usize;
        let pitches: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 32;
        let length_expr = num_planes as usize;
        let offsets: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        offset += length_expr * 4;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                data_size,
                width,
                height,
                pitches,
                offsets,
            },
            offset,
        ))
    }
}
