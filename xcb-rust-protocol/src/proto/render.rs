#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "RENDER";
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PictTypeEnum(pub u8);
impl PictTypeEnum {
    pub const INDEXED: Self = Self(0);
    pub const DIRECT: Self = Self(1);
}
impl FixedLengthSerialize<1> for PictTypeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for PictTypeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for PictTypeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for PictTypeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for PictTypeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PictureEnum(pub Picture);
impl PictureEnum {
    pub const NONE: Self = Self(0);
}
impl FixedLengthSerialize<4> for PictureEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for PictureEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(Picture::from_bytes(bytes)?))
    }
}
impl From<u32> for PictureEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(Picture::from(val as u32))
    }
}
impl From<u16> for PictureEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(Picture::from(val as u32))
    }
}
impl From<u8> for PictureEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(Picture::from(val as u32))
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PictOpEnum(pub u8);
impl PictOpEnum {
    pub const CLEAR: Self = Self(0);
    pub const SRC: Self = Self(1);
    pub const DST: Self = Self(2);
    pub const OVER: Self = Self(3);
    pub const OVER_REVERSE: Self = Self(4);
    pub const IN: Self = Self(5);
    pub const IN_REVERSE: Self = Self(6);
    pub const OUT: Self = Self(7);
    pub const OUT_REVERSE: Self = Self(8);
    pub const ATOP: Self = Self(9);
    pub const ATOP_REVERSE: Self = Self(10);
    pub const XOR: Self = Self(11);
    pub const ADD: Self = Self(12);
    pub const SATURATE: Self = Self(13);
    pub const DISJOINT_CLEAR: Self = Self(16);
    pub const DISJOINT_SRC: Self = Self(17);
    pub const DISJOINT_DST: Self = Self(18);
    pub const DISJOINT_OVER: Self = Self(19);
    pub const DISJOINT_OVER_REVERSE: Self = Self(20);
    pub const DISJOINT_IN: Self = Self(21);
    pub const DISJOINT_IN_REVERSE: Self = Self(22);
    pub const DISJOINT_OUT: Self = Self(23);
    pub const DISJOINT_OUT_REVERSE: Self = Self(24);
    pub const DISJOINT_ATOP: Self = Self(25);
    pub const DISJOINT_ATOP_REVERSE: Self = Self(26);
    pub const DISJOINT_XOR: Self = Self(27);
    pub const CONJOINT_CLEAR: Self = Self(32);
    pub const CONJOINT_SRC: Self = Self(33);
    pub const CONJOINT_DST: Self = Self(34);
    pub const CONJOINT_OVER: Self = Self(35);
    pub const CONJOINT_OVER_REVERSE: Self = Self(36);
    pub const CONJOINT_IN: Self = Self(37);
    pub const CONJOINT_IN_REVERSE: Self = Self(38);
    pub const CONJOINT_OUT: Self = Self(39);
    pub const CONJOINT_OUT_REVERSE: Self = Self(40);
    pub const CONJOINT_ATOP: Self = Self(41);
    pub const CONJOINT_ATOP_REVERSE: Self = Self(42);
    pub const CONJOINT_XOR: Self = Self(43);
    pub const MULTIPLY: Self = Self(48);
    pub const SCREEN: Self = Self(49);
    pub const OVERLAY: Self = Self(50);
    pub const DARKEN: Self = Self(51);
    pub const LIGHTEN: Self = Self(52);
    pub const COLOR_DODGE: Self = Self(53);
    pub const COLOR_BURN: Self = Self(54);
    pub const HARD_LIGHT: Self = Self(55);
    pub const SOFT_LIGHT: Self = Self(56);
    pub const DIFFERENCE: Self = Self(57);
    pub const EXCLUSION: Self = Self(58);
    pub const H_S_L_HUE: Self = Self(59);
    pub const H_S_L_SATURATION: Self = Self(60);
    pub const H_S_L_COLOR: Self = Self(61);
    pub const H_S_L_LUMINOSITY: Self = Self(62);
}
impl FixedLengthSerialize<1> for PictOpEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for PictOpEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for PictOpEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for PictOpEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for PictOpEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PolyEdgeEnum(pub u32);
impl PolyEdgeEnum {
    pub const SHARP: Self = Self(0);
    pub const SMOOTH: Self = Self(1);
}
impl FixedLengthSerialize<4> for PolyEdgeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for PolyEdgeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for PolyEdgeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for PolyEdgeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for PolyEdgeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PolyModeEnum(pub u32);
impl PolyModeEnum {
    pub const PRECISE: Self = Self(0);
    pub const IMPRECISE: Self = Self(1);
}
impl FixedLengthSerialize<4> for PolyModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for PolyModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for PolyModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for PolyModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for PolyModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Cp(pub u32);
impl Cp {
    pub const REPEAT: Self = Self(1 << 0);
    pub const ALPHA_MAP: Self = Self(1 << 1);
    pub const ALPHA_X_ORIGIN: Self = Self(1 << 2);
    pub const ALPHA_Y_ORIGIN: Self = Self(1 << 3);
    pub const CLIP_X_ORIGIN: Self = Self(1 << 4);
    pub const CLIP_Y_ORIGIN: Self = Self(1 << 5);
    pub const CLIP_MASK: Self = Self(1 << 6);
    pub const GRAPHICS_EXPOSURE: Self = Self(1 << 7);
    pub const SUBWINDOW_MODE: Self = Self(1 << 8);
    pub const POLY_EDGE: Self = Self(1 << 9);
    pub const POLY_MODE: Self = Self(1 << 10);
    pub const DITHER: Self = Self(1 << 11);
    pub const COMPONENT_ALPHA: Self = Self(1 << 12);
}
impl FixedLengthSerialize<4> for Cp {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for Cp {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for Cp {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for Cp {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for Cp {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(Cp);
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SubPixelEnum(pub u32);
impl SubPixelEnum {
    pub const UNKNOWN: Self = Self(0);
    pub const HORIZONTAL_R_G_B: Self = Self(1);
    pub const HORIZONTAL_B_G_R: Self = Self(2);
    pub const VERTICAL_R_G_B: Self = Self(3);
    pub const VERTICAL_B_G_R: Self = Self(4);
    pub const NONE: Self = Self(5);
}
impl FixedLengthSerialize<4> for SubPixelEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for SubPixelEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for SubPixelEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for SubPixelEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for SubPixelEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct RepeatEnum(pub u32);
impl RepeatEnum {
    pub const NONE: Self = Self(0);
    pub const NORMAL: Self = Self(1);
    pub const PAD: Self = Self(2);
    pub const REFLECT: Self = Self(3);
}
impl FixedLengthSerialize<4> for RepeatEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for RepeatEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for RepeatEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for RepeatEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for RepeatEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
pub type Glyph = u32;
pub type Glyphset = u32;
pub type Picture = u32;
pub type Pictformat = u32;
pub type Fixed = i32;
pub const PICT_FORMAT_ERROR: u8 = 0;
pub const PICTURE_ERROR: u8 = 1;
pub const PICT_OP_ERROR: u8 = 2;
pub const GLYPH_SET_ERROR: u8 = 3;
pub const GLYPH_ERROR: u8 = 4;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Directformat {
    pub red_shift: u16,
    pub red_mask: u16,
    pub green_shift: u16,
    pub green_mask: u16,
    pub blue_shift: u16,
    pub blue_mask: u16,
    pub alpha_shift: u16,
    pub alpha_mask: u16,
}
impl FixedLengthSerialize<16> for Directformat {
    #[inline]
    fn serialize_fixed(self) -> [u8; 16] {
        let red_shift_bytes = self.red_shift.serialize_fixed();
        let red_mask_bytes = self.red_mask.serialize_fixed();
        let green_shift_bytes = self.green_shift.serialize_fixed();
        let green_mask_bytes = self.green_mask.serialize_fixed();
        let blue_shift_bytes = self.blue_shift.serialize_fixed();
        let blue_mask_bytes = self.blue_mask.serialize_fixed();
        let alpha_shift_bytes = self.alpha_shift.serialize_fixed();
        let alpha_mask_bytes = self.alpha_mask.serialize_fixed();
        [
            red_shift_bytes[0],
            red_shift_bytes[1],
            red_mask_bytes[0],
            red_mask_bytes[1],
            green_shift_bytes[0],
            green_shift_bytes[1],
            green_mask_bytes[0],
            green_mask_bytes[1],
            blue_shift_bytes[0],
            blue_shift_bytes[1],
            blue_mask_bytes[0],
            blue_mask_bytes[1],
            alpha_shift_bytes[0],
            alpha_shift_bytes[1],
            alpha_mask_bytes[0],
            alpha_mask_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<16> for Directformat {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            red_shift: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            red_mask: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            green_shift: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            green_mask: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
            blue_shift: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            blue_mask: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
            alpha_shift: u16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
            alpha_mask: u16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Pictforminfo {
    pub id: Pictformat,
    pub r#type: PictTypeEnum,
    pub depth: u8,
    pub direct: Directformat,
    pub colormap: crate::proto::xproto::Colormap,
}
impl FixedLengthSerialize<28> for Pictforminfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 28] {
        let id_bytes = self.id.serialize_fixed();
        let direct_bytes = self.direct.serialize_fixed();
        let colormap_bytes = self.colormap.serialize_fixed();
        [
            id_bytes[0],
            id_bytes[1],
            id_bytes[2],
            id_bytes[3],
            self.r#type.0 as u8,
            self.depth,
            0,
            0,
            direct_bytes[0],
            direct_bytes[1],
            direct_bytes[2],
            direct_bytes[3],
            direct_bytes[4],
            direct_bytes[5],
            direct_bytes[6],
            direct_bytes[7],
            direct_bytes[8],
            direct_bytes[9],
            direct_bytes[10],
            direct_bytes[11],
            direct_bytes[12],
            direct_bytes[13],
            direct_bytes[14],
            direct_bytes[15],
            colormap_bytes[0],
            colormap_bytes[1],
            colormap_bytes[2],
            colormap_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<28> for Pictforminfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            id: Pictformat::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            r#type: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?.into(),
            depth: u8::from_bytes(bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?)?,
            direct: Directformat::from_bytes(
                bytes.get(8..24).ok_or(crate::error::Error::FromBytes)?,
            )?,
            colormap: crate::proto::xproto::Colormap::from_bytes(
                bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Pictvisual {
    pub visual: crate::proto::xproto::Visualid,
    pub format: Pictformat,
}
impl FixedLengthSerialize<8> for Pictvisual {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let visual_bytes = self.visual.serialize_fixed();
        let format_bytes = self.format.serialize_fixed();
        [
            visual_bytes[0],
            visual_bytes[1],
            visual_bytes[2],
            visual_bytes[3],
            format_bytes[0],
            format_bytes[1],
            format_bytes[2],
            format_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for Pictvisual {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            visual: crate::proto::xproto::Visualid::from_bytes(
                bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            format: Pictformat::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Pictdepth {
    pub depth: u8,
    pub visuals: alloc::vec::Vec<Pictvisual>,
}
impl VariableLengthSerialize for Pictdepth {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        // Padding 1 bytes
        let num_visuals =
            u16::try_from(self.visuals.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Padding 4 bytes
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.depth.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_visuals).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.visuals.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            self.visuals,
        )?;
        let offset = list_len + 8;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for Pictdepth {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        let depth = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let num_visuals = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_visuals as usize;
        let visuals: alloc::vec::Vec<Pictvisual> = crate::vec_from_bytes_fixed!(
            ptr.get(8..).ok_or(crate::error::Error::FromBytes)?,
            Pictvisual,
            length_expr,
            8
        );
        let offset = length_expr * 8 + 8;
        Ok((Self { depth, visuals }, offset))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Pictscreen {
    pub fallback: Pictformat,
    pub depths: alloc::vec::Vec<Pictdepth>,
}
impl VariableLengthSerialize for Pictscreen {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let num_depths =
            u32::try_from(self.depths.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(num_depths).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.fallback.serialize_fixed());
        let offset = crate::util::var_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            self.depths,
        )?;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for Pictscreen {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let num_depths = u32::from_bytes(ptr.get(0..4).ok_or(crate::error::Error::FromBytes)?)?;
        let fallback =
            Pictformat::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let depths_length = num_depths as usize;
        let mut offset = 8;
        let depths = crate::vec_from_bytes_var!(ptr, Pictdepth, offset, depths_length,);
        Ok((Self { fallback, depths }, offset))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Indexvalue {
    pub pixel: u32,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub alpha: u16,
}
impl FixedLengthSerialize<12> for Indexvalue {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let pixel_bytes = self.pixel.serialize_fixed();
        let red_bytes = self.red.serialize_fixed();
        let green_bytes = self.green.serialize_fixed();
        let blue_bytes = self.blue.serialize_fixed();
        let alpha_bytes = self.alpha.serialize_fixed();
        [
            pixel_bytes[0],
            pixel_bytes[1],
            pixel_bytes[2],
            pixel_bytes[3],
            red_bytes[0],
            red_bytes[1],
            green_bytes[0],
            green_bytes[1],
            blue_bytes[0],
            blue_bytes[1],
            alpha_bytes[0],
            alpha_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<12> for Indexvalue {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            pixel: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            red: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            green: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
            blue: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            alpha: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Color {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub alpha: u16,
}
impl FixedLengthSerialize<8> for Color {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let red_bytes = self.red.serialize_fixed();
        let green_bytes = self.green.serialize_fixed();
        let blue_bytes = self.blue.serialize_fixed();
        let alpha_bytes = self.alpha.serialize_fixed();
        [
            red_bytes[0],
            red_bytes[1],
            green_bytes[0],
            green_bytes[1],
            blue_bytes[0],
            blue_bytes[1],
            alpha_bytes[0],
            alpha_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<8> for Color {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            red: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            green: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            blue: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            alpha: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Pointfix {
    pub x: Fixed,
    pub y: Fixed,
}
impl FixedLengthSerialize<8> for Pointfix {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let x_bytes = self.x.serialize_fixed();
        let y_bytes = self.y.serialize_fixed();
        [
            x_bytes[0], x_bytes[1], x_bytes[2], x_bytes[3], y_bytes[0], y_bytes[1], y_bytes[2],
            y_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for Pointfix {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            x: Fixed::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            y: Fixed::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Linefix {
    pub p1: Pointfix,
    pub p2: Pointfix,
}
impl FixedLengthSerialize<16> for Linefix {
    #[inline]
    fn serialize_fixed(self) -> [u8; 16] {
        let p1_bytes = self.p1.serialize_fixed();
        let p2_bytes = self.p2.serialize_fixed();
        [
            p1_bytes[0],
            p1_bytes[1],
            p1_bytes[2],
            p1_bytes[3],
            p1_bytes[4],
            p1_bytes[5],
            p1_bytes[6],
            p1_bytes[7],
            p2_bytes[0],
            p2_bytes[1],
            p2_bytes[2],
            p2_bytes[3],
            p2_bytes[4],
            p2_bytes[5],
            p2_bytes[6],
            p2_bytes[7],
        ]
    }
}
impl FixedLengthFromBytes<16> for Linefix {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            p1: Pointfix::from_bytes(bytes.get(0..8).ok_or(crate::error::Error::FromBytes)?)?,
            p2: Pointfix::from_bytes(bytes.get(8..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Triangle {
    pub p1: Pointfix,
    pub p2: Pointfix,
    pub p3: Pointfix,
}
impl FixedLengthSerialize<24> for Triangle {
    #[inline]
    fn serialize_fixed(self) -> [u8; 24] {
        let p1_bytes = self.p1.serialize_fixed();
        let p2_bytes = self.p2.serialize_fixed();
        let p3_bytes = self.p3.serialize_fixed();
        [
            p1_bytes[0],
            p1_bytes[1],
            p1_bytes[2],
            p1_bytes[3],
            p1_bytes[4],
            p1_bytes[5],
            p1_bytes[6],
            p1_bytes[7],
            p2_bytes[0],
            p2_bytes[1],
            p2_bytes[2],
            p2_bytes[3],
            p2_bytes[4],
            p2_bytes[5],
            p2_bytes[6],
            p2_bytes[7],
            p3_bytes[0],
            p3_bytes[1],
            p3_bytes[2],
            p3_bytes[3],
            p3_bytes[4],
            p3_bytes[5],
            p3_bytes[6],
            p3_bytes[7],
        ]
    }
}
impl FixedLengthFromBytes<24> for Triangle {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            p1: Pointfix::from_bytes(bytes.get(0..8).ok_or(crate::error::Error::FromBytes)?)?,
            p2: Pointfix::from_bytes(bytes.get(8..16).ok_or(crate::error::Error::FromBytes)?)?,
            p3: Pointfix::from_bytes(bytes.get(16..24).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Trapezoid {
    pub top: Fixed,
    pub bottom: Fixed,
    pub left: Linefix,
    pub right: Linefix,
}
impl FixedLengthSerialize<40> for Trapezoid {
    #[inline]
    fn serialize_fixed(self) -> [u8; 40] {
        let top_bytes = self.top.serialize_fixed();
        let bottom_bytes = self.bottom.serialize_fixed();
        let left_bytes = self.left.serialize_fixed();
        let right_bytes = self.right.serialize_fixed();
        [
            top_bytes[0],
            top_bytes[1],
            top_bytes[2],
            top_bytes[3],
            bottom_bytes[0],
            bottom_bytes[1],
            bottom_bytes[2],
            bottom_bytes[3],
            left_bytes[0],
            left_bytes[1],
            left_bytes[2],
            left_bytes[3],
            left_bytes[4],
            left_bytes[5],
            left_bytes[6],
            left_bytes[7],
            left_bytes[8],
            left_bytes[9],
            left_bytes[10],
            left_bytes[11],
            left_bytes[12],
            left_bytes[13],
            left_bytes[14],
            left_bytes[15],
            right_bytes[0],
            right_bytes[1],
            right_bytes[2],
            right_bytes[3],
            right_bytes[4],
            right_bytes[5],
            right_bytes[6],
            right_bytes[7],
            right_bytes[8],
            right_bytes[9],
            right_bytes[10],
            right_bytes[11],
            right_bytes[12],
            right_bytes[13],
            right_bytes[14],
            right_bytes[15],
        ]
    }
}
impl FixedLengthFromBytes<40> for Trapezoid {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            top: Fixed::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            bottom: Fixed::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            left: Linefix::from_bytes(bytes.get(8..24).ok_or(crate::error::Error::FromBytes)?)?,
            right: Linefix::from_bytes(bytes.get(24..40).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Glyphinfo {
    pub width: u16,
    pub height: u16,
    pub x: i16,
    pub y: i16,
    pub x_off: i16,
    pub y_off: i16,
}
impl FixedLengthSerialize<12> for Glyphinfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let width_bytes = self.width.serialize_fixed();
        let height_bytes = self.height.serialize_fixed();
        let x_bytes = self.x.serialize_fixed();
        let y_bytes = self.y.serialize_fixed();
        let x_off_bytes = self.x_off.serialize_fixed();
        let y_off_bytes = self.y_off.serialize_fixed();
        [
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            x_bytes[0],
            x_bytes[1],
            y_bytes[0],
            y_bytes[1],
            x_off_bytes[0],
            x_off_bytes[1],
            y_off_bytes[0],
            y_off_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<12> for Glyphinfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            width: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            x: i16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            y: i16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
            x_off: i16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            y_off: i16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
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
pub struct QueryPictFormatsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_depths: u32,
    pub num_visuals: u32,
    pub formats: alloc::vec::Vec<Pictforminfo>,
    pub screens: alloc::vec::Vec<Pictscreen>,
    pub subpixels: alloc::vec::Vec<SubPixelEnum>,
}
impl VariableLengthFromBytes for QueryPictFormatsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_formats = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let num_screens = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let num_depths = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let num_visuals = u32::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let num_subpixel = u32::from_bytes(ptr.get(24..28).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_formats as usize;
        let formats: alloc::vec::Vec<Pictforminfo> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Pictforminfo,
            length_expr,
            28
        );
        let mut offset = length_expr * 28 + 32;
        let screens_length = num_screens as usize;
        let screens = crate::vec_from_bytes_var!(ptr, Pictscreen, offset, screens_length,);
        let length_expr = num_subpixel as usize;
        let subpixels: alloc::vec::Vec<SubPixelEnum> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            SubPixelEnum,
            length_expr,
            4
        );
        offset += length_expr * 4;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                num_depths,
                num_visuals,
                formats,
                screens,
                subpixels,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryPictIndexValuesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub values: alloc::vec::Vec<Indexvalue>,
}
impl VariableLengthFromBytes for QueryPictIndexValuesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_values = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_values as usize;
        let values: alloc::vec::Vec<Indexvalue> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Indexvalue,
            length_expr,
            12
        );
        let offset = length_expr * 12 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                values,
            },
            offset,
        ))
    }
}
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct CreatePictureValueList {
    pub repeat: Option<RepeatEnum>,
    pub alphamap: Option<Picture>,
    pub alphaxorigin: Option<i32>,
    pub alphayorigin: Option<i32>,
    pub clipxorigin: Option<i32>,
    pub clipyorigin: Option<i32>,
    pub clipmask: Option<crate::proto::xproto::Pixmap>,
    pub graphicsexposure: Option<u32>,
    pub subwindowmode: Option<crate::proto::xproto::SubwindowModeEnum>,
    pub polyedge: Option<PolyEdgeEnum>,
    pub polymode: Option<PolyModeEnum>,
    pub dither: Option<u32>,
    pub componentalpha: Option<u32>,
}
impl CreatePictureValueList {
    #[inline]
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let mut offset = 0;
        let buf_ptr = buf;
        if let Some(repeat) = self.repeat {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&repeat.serialize_fixed());
            offset += 4;
        }
        if let Some(alphamap) = self.alphamap {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&alphamap.serialize_fixed());
            offset += 4;
        }
        if let Some(alphaxorigin) = self.alphaxorigin {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&alphaxorigin.serialize_fixed());
            offset += 4;
        }
        if let Some(alphayorigin) = self.alphayorigin {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&alphayorigin.serialize_fixed());
            offset += 4;
        }
        if let Some(clipxorigin) = self.clipxorigin {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&clipxorigin.serialize_fixed());
            offset += 4;
        }
        if let Some(clipyorigin) = self.clipyorigin {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&clipyorigin.serialize_fixed());
            offset += 4;
        }
        if let Some(clipmask) = self.clipmask {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&clipmask.serialize_fixed());
            offset += 4;
        }
        if let Some(graphicsexposure) = self.graphicsexposure {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&graphicsexposure.serialize_fixed());
            offset += 4;
        }
        if let Some(subwindowmode) = self.subwindowmode {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&subwindowmode.serialize_fixed());
            offset += 4;
        }
        if let Some(polyedge) = self.polyedge {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&polyedge.serialize_fixed());
            offset += 4;
        }
        if let Some(polymode) = self.polymode {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&polymode.serialize_fixed());
            offset += 4;
        }
        if let Some(dither) = self.dither {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&dither.serialize_fixed());
            offset += 4;
        }
        if let Some(componentalpha) = self.componentalpha {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&componentalpha.serialize_fixed());
            offset += 4;
        }
        Ok(offset)
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> Cp {
        let mut mask = Cp::default();
        if self.repeat.is_some() {
            mask |= Cp::REPEAT;
        }
        if self.alphamap.is_some() {
            mask |= Cp::ALPHA_MAP;
        }
        if self.alphaxorigin.is_some() {
            mask |= Cp::ALPHA_X_ORIGIN;
        }
        if self.alphayorigin.is_some() {
            mask |= Cp::ALPHA_Y_ORIGIN;
        }
        if self.clipxorigin.is_some() {
            mask |= Cp::CLIP_X_ORIGIN;
        }
        if self.clipyorigin.is_some() {
            mask |= Cp::CLIP_Y_ORIGIN;
        }
        if self.clipmask.is_some() {
            mask |= Cp::CLIP_MASK;
        }
        if self.graphicsexposure.is_some() {
            mask |= Cp::GRAPHICS_EXPOSURE;
        }
        if self.subwindowmode.is_some() {
            mask |= Cp::SUBWINDOW_MODE;
        }
        if self.polyedge.is_some() {
            mask |= Cp::POLY_EDGE;
        }
        if self.polymode.is_some() {
            mask |= Cp::POLY_MODE;
        }
        if self.dither.is_some() {
            mask |= Cp::DITHER;
        }
        if self.componentalpha.is_some() {
            mask |= Cp::COMPONENT_ALPHA;
        }
        mask
    }

    #[inline]
    pub fn repeat(mut self, repeat: RepeatEnum) -> Self {
        self.repeat = Some(repeat);
        self
    }

    #[inline]
    pub fn alphamap(mut self, alphamap: Picture) -> Self {
        self.alphamap = Some(alphamap);
        self
    }

    #[inline]
    pub fn alphaxorigin(mut self, alphaxorigin: i32) -> Self {
        self.alphaxorigin = Some(alphaxorigin);
        self
    }

    #[inline]
    pub fn alphayorigin(mut self, alphayorigin: i32) -> Self {
        self.alphayorigin = Some(alphayorigin);
        self
    }

    #[inline]
    pub fn clipxorigin(mut self, clipxorigin: i32) -> Self {
        self.clipxorigin = Some(clipxorigin);
        self
    }

    #[inline]
    pub fn clipyorigin(mut self, clipyorigin: i32) -> Self {
        self.clipyorigin = Some(clipyorigin);
        self
    }

    #[inline]
    pub fn clipmask(mut self, clipmask: crate::proto::xproto::Pixmap) -> Self {
        self.clipmask = Some(clipmask);
        self
    }

    #[inline]
    pub fn graphicsexposure(mut self, graphicsexposure: u32) -> Self {
        self.graphicsexposure = Some(graphicsexposure);
        self
    }

    #[inline]
    pub fn subwindowmode(mut self, subwindowmode: crate::proto::xproto::SubwindowModeEnum) -> Self {
        self.subwindowmode = Some(subwindowmode);
        self
    }

    #[inline]
    pub fn polyedge(mut self, polyedge: PolyEdgeEnum) -> Self {
        self.polyedge = Some(polyedge);
        self
    }

    #[inline]
    pub fn polymode(mut self, polymode: PolyModeEnum) -> Self {
        self.polymode = Some(polymode);
        self
    }

    #[inline]
    pub fn dither(mut self, dither: u32) -> Self {
        self.dither = Some(dither);
        self
    }

    #[inline]
    pub fn componentalpha(mut self, componentalpha: u32) -> Self {
        self.componentalpha = Some(componentalpha);
        self
    }
}
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct ChangePictureValueList {
    pub repeat: Option<RepeatEnum>,
    pub alphamap: Option<Picture>,
    pub alphaxorigin: Option<i32>,
    pub alphayorigin: Option<i32>,
    pub clipxorigin: Option<i32>,
    pub clipyorigin: Option<i32>,
    pub clipmask: Option<crate::proto::xproto::Pixmap>,
    pub graphicsexposure: Option<u32>,
    pub subwindowmode: Option<crate::proto::xproto::SubwindowModeEnum>,
    pub polyedge: Option<PolyEdgeEnum>,
    pub polymode: Option<PolyModeEnum>,
    pub dither: Option<u32>,
    pub componentalpha: Option<u32>,
}
impl ChangePictureValueList {
    #[inline]
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let mut offset = 0;
        let buf_ptr = buf;
        if let Some(repeat) = self.repeat {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&repeat.serialize_fixed());
            offset += 4;
        }
        if let Some(alphamap) = self.alphamap {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&alphamap.serialize_fixed());
            offset += 4;
        }
        if let Some(alphaxorigin) = self.alphaxorigin {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&alphaxorigin.serialize_fixed());
            offset += 4;
        }
        if let Some(alphayorigin) = self.alphayorigin {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&alphayorigin.serialize_fixed());
            offset += 4;
        }
        if let Some(clipxorigin) = self.clipxorigin {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&clipxorigin.serialize_fixed());
            offset += 4;
        }
        if let Some(clipyorigin) = self.clipyorigin {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&clipyorigin.serialize_fixed());
            offset += 4;
        }
        if let Some(clipmask) = self.clipmask {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&clipmask.serialize_fixed());
            offset += 4;
        }
        if let Some(graphicsexposure) = self.graphicsexposure {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&graphicsexposure.serialize_fixed());
            offset += 4;
        }
        if let Some(subwindowmode) = self.subwindowmode {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&subwindowmode.serialize_fixed());
            offset += 4;
        }
        if let Some(polyedge) = self.polyedge {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&polyedge.serialize_fixed());
            offset += 4;
        }
        if let Some(polymode) = self.polymode {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&polymode.serialize_fixed());
            offset += 4;
        }
        if let Some(dither) = self.dither {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&dither.serialize_fixed());
            offset += 4;
        }
        if let Some(componentalpha) = self.componentalpha {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&componentalpha.serialize_fixed());
            offset += 4;
        }
        Ok(offset)
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> Cp {
        let mut mask = Cp::default();
        if self.repeat.is_some() {
            mask |= Cp::REPEAT;
        }
        if self.alphamap.is_some() {
            mask |= Cp::ALPHA_MAP;
        }
        if self.alphaxorigin.is_some() {
            mask |= Cp::ALPHA_X_ORIGIN;
        }
        if self.alphayorigin.is_some() {
            mask |= Cp::ALPHA_Y_ORIGIN;
        }
        if self.clipxorigin.is_some() {
            mask |= Cp::CLIP_X_ORIGIN;
        }
        if self.clipyorigin.is_some() {
            mask |= Cp::CLIP_Y_ORIGIN;
        }
        if self.clipmask.is_some() {
            mask |= Cp::CLIP_MASK;
        }
        if self.graphicsexposure.is_some() {
            mask |= Cp::GRAPHICS_EXPOSURE;
        }
        if self.subwindowmode.is_some() {
            mask |= Cp::SUBWINDOW_MODE;
        }
        if self.polyedge.is_some() {
            mask |= Cp::POLY_EDGE;
        }
        if self.polymode.is_some() {
            mask |= Cp::POLY_MODE;
        }
        if self.dither.is_some() {
            mask |= Cp::DITHER;
        }
        if self.componentalpha.is_some() {
            mask |= Cp::COMPONENT_ALPHA;
        }
        mask
    }

    #[inline]
    pub fn repeat(mut self, repeat: RepeatEnum) -> Self {
        self.repeat = Some(repeat);
        self
    }

    #[inline]
    pub fn alphamap(mut self, alphamap: Picture) -> Self {
        self.alphamap = Some(alphamap);
        self
    }

    #[inline]
    pub fn alphaxorigin(mut self, alphaxorigin: i32) -> Self {
        self.alphaxorigin = Some(alphaxorigin);
        self
    }

    #[inline]
    pub fn alphayorigin(mut self, alphayorigin: i32) -> Self {
        self.alphayorigin = Some(alphayorigin);
        self
    }

    #[inline]
    pub fn clipxorigin(mut self, clipxorigin: i32) -> Self {
        self.clipxorigin = Some(clipxorigin);
        self
    }

    #[inline]
    pub fn clipyorigin(mut self, clipyorigin: i32) -> Self {
        self.clipyorigin = Some(clipyorigin);
        self
    }

    #[inline]
    pub fn clipmask(mut self, clipmask: crate::proto::xproto::Pixmap) -> Self {
        self.clipmask = Some(clipmask);
        self
    }

    #[inline]
    pub fn graphicsexposure(mut self, graphicsexposure: u32) -> Self {
        self.graphicsexposure = Some(graphicsexposure);
        self
    }

    #[inline]
    pub fn subwindowmode(mut self, subwindowmode: crate::proto::xproto::SubwindowModeEnum) -> Self {
        self.subwindowmode = Some(subwindowmode);
        self
    }

    #[inline]
    pub fn polyedge(mut self, polyedge: PolyEdgeEnum) -> Self {
        self.polyedge = Some(polyedge);
        self
    }

    #[inline]
    pub fn polymode(mut self, polymode: PolyModeEnum) -> Self {
        self.polymode = Some(polymode);
        self
    }

    #[inline]
    pub fn dither(mut self, dither: u32) -> Self {
        self.dither = Some(dither);
        self
    }

    #[inline]
    pub fn componentalpha(mut self, componentalpha: u32) -> Self {
        self.componentalpha = Some(componentalpha);
        self
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Transform {
    pub matrix11: Fixed,
    pub matrix12: Fixed,
    pub matrix13: Fixed,
    pub matrix21: Fixed,
    pub matrix22: Fixed,
    pub matrix23: Fixed,
    pub matrix31: Fixed,
    pub matrix32: Fixed,
    pub matrix33: Fixed,
}
impl FixedLengthSerialize<36> for Transform {
    #[inline]
    fn serialize_fixed(self) -> [u8; 36] {
        let matrix11_bytes = self.matrix11.serialize_fixed();
        let matrix12_bytes = self.matrix12.serialize_fixed();
        let matrix13_bytes = self.matrix13.serialize_fixed();
        let matrix21_bytes = self.matrix21.serialize_fixed();
        let matrix22_bytes = self.matrix22.serialize_fixed();
        let matrix23_bytes = self.matrix23.serialize_fixed();
        let matrix31_bytes = self.matrix31.serialize_fixed();
        let matrix32_bytes = self.matrix32.serialize_fixed();
        let matrix33_bytes = self.matrix33.serialize_fixed();
        [
            matrix11_bytes[0],
            matrix11_bytes[1],
            matrix11_bytes[2],
            matrix11_bytes[3],
            matrix12_bytes[0],
            matrix12_bytes[1],
            matrix12_bytes[2],
            matrix12_bytes[3],
            matrix13_bytes[0],
            matrix13_bytes[1],
            matrix13_bytes[2],
            matrix13_bytes[3],
            matrix21_bytes[0],
            matrix21_bytes[1],
            matrix21_bytes[2],
            matrix21_bytes[3],
            matrix22_bytes[0],
            matrix22_bytes[1],
            matrix22_bytes[2],
            matrix22_bytes[3],
            matrix23_bytes[0],
            matrix23_bytes[1],
            matrix23_bytes[2],
            matrix23_bytes[3],
            matrix31_bytes[0],
            matrix31_bytes[1],
            matrix31_bytes[2],
            matrix31_bytes[3],
            matrix32_bytes[0],
            matrix32_bytes[1],
            matrix32_bytes[2],
            matrix32_bytes[3],
            matrix33_bytes[0],
            matrix33_bytes[1],
            matrix33_bytes[2],
            matrix33_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<36> for Transform {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            matrix11: Fixed::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            matrix12: Fixed::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            matrix13: Fixed::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            matrix21: Fixed::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            matrix22: Fixed::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            matrix23: Fixed::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
            matrix31: Fixed::from_bytes(bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?)?,
            matrix32: Fixed::from_bytes(bytes.get(28..32).ok_or(crate::error::Error::FromBytes)?)?,
            matrix33: Fixed::from_bytes(bytes.get(32..36).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryFiltersReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub aliases: alloc::vec::Vec<u16>,
    pub filters: alloc::vec::Vec<crate::proto::xproto::Str>,
}
impl VariableLengthFromBytes for QueryFiltersReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 16 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_aliases = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let num_filters = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_aliases as usize;
        let aliases: alloc::vec::Vec<u16> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            u16,
            length_expr,
            2
        );
        let mut offset = length_expr * 2 + 32;
        let filters_length = num_filters as usize;
        let filters =
            crate::vec_from_bytes_var!(ptr, crate::proto::xproto::Str, offset, filters_length,);
        Ok((
            Self {
                response_type,
                sequence,
                length,
                aliases,
                filters,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Animcursorelt {
    pub cursor: crate::proto::xproto::Cursor,
    pub delay: u32,
}
impl FixedLengthSerialize<8> for Animcursorelt {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let cursor_bytes = self.cursor.serialize_fixed();
        let delay_bytes = self.delay.serialize_fixed();
        [
            cursor_bytes[0],
            cursor_bytes[1],
            cursor_bytes[2],
            cursor_bytes[3],
            delay_bytes[0],
            delay_bytes[1],
            delay_bytes[2],
            delay_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for Animcursorelt {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            cursor: crate::proto::xproto::Cursor::from_bytes(
                bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            delay: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Spanfix {
    pub l: Fixed,
    pub r: Fixed,
    pub y: Fixed,
}
impl FixedLengthSerialize<12> for Spanfix {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let l_bytes = self.l.serialize_fixed();
        let r_bytes = self.r.serialize_fixed();
        let y_bytes = self.y.serialize_fixed();
        [
            l_bytes[0], l_bytes[1], l_bytes[2], l_bytes[3], r_bytes[0], r_bytes[1], r_bytes[2],
            r_bytes[3], y_bytes[0], y_bytes[1], y_bytes[2], y_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<12> for Spanfix {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            l: Fixed::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            r: Fixed::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            y: Fixed::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Trap {
    pub top: Spanfix,
    pub bot: Spanfix,
}
impl FixedLengthSerialize<24> for Trap {
    #[inline]
    fn serialize_fixed(self) -> [u8; 24] {
        let top_bytes = self.top.serialize_fixed();
        let bot_bytes = self.bot.serialize_fixed();
        [
            top_bytes[0],
            top_bytes[1],
            top_bytes[2],
            top_bytes[3],
            top_bytes[4],
            top_bytes[5],
            top_bytes[6],
            top_bytes[7],
            top_bytes[8],
            top_bytes[9],
            top_bytes[10],
            top_bytes[11],
            bot_bytes[0],
            bot_bytes[1],
            bot_bytes[2],
            bot_bytes[3],
            bot_bytes[4],
            bot_bytes[5],
            bot_bytes[6],
            bot_bytes[7],
            bot_bytes[8],
            bot_bytes[9],
            bot_bytes[10],
            bot_bytes[11],
        ]
    }
}
impl FixedLengthFromBytes<24> for Trap {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            top: Spanfix::from_bytes(bytes.get(0..12).ok_or(crate::error::Error::FromBytes)?)?,
            bot: Spanfix::from_bytes(bytes.get(12..24).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
