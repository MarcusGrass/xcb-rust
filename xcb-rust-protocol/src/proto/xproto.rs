#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Char2b {
    pub byte1: u8,
    pub byte2: u8,
}
impl FixedLengthSerialize<2> for Char2b {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        [self.byte1, self.byte2]
    }
}
impl FixedLengthFromBytes<2> for Char2b {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            byte1: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            byte2: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub type Window = u32;
pub type Pixmap = u32;
pub type Cursor = u32;
pub type Font = u32;
pub type Gcontext = u32;
pub type Colormap = u32;
pub type Atom = u32;
pub type Drawable = u32;
pub type Fontable = u32;
pub type Bool32 = u32;
pub type Visualid = u32;
pub type Timestamp = u32;
pub type Keysym = u32;
pub type Keycode = u8;
pub type Keycode32 = u32;
pub type Button = u8;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}
impl FixedLengthSerialize<4> for Point {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        let x_bytes = self.x.serialize_fixed();
        let y_bytes = self.y.serialize_fixed();
        [x_bytes[0], x_bytes[1], y_bytes[0], y_bytes[1]]
    }
}
impl FixedLengthFromBytes<4> for Point {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            x: i16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            y: i16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Rectangle {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}
impl FixedLengthSerialize<8> for Rectangle {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let x_bytes = self.x.serialize_fixed();
        let y_bytes = self.y.serialize_fixed();
        let width_bytes = self.width.serialize_fixed();
        let height_bytes = self.height.serialize_fixed();
        [
            x_bytes[0],
            x_bytes[1],
            y_bytes[0],
            y_bytes[1],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<8> for Rectangle {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            x: i16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            y: i16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            width: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Arc {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub angle1: i16,
    pub angle2: i16,
}
impl FixedLengthSerialize<12> for Arc {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let x_bytes = self.x.serialize_fixed();
        let y_bytes = self.y.serialize_fixed();
        let width_bytes = self.width.serialize_fixed();
        let height_bytes = self.height.serialize_fixed();
        let angle1_bytes = self.angle1.serialize_fixed();
        let angle2_bytes = self.angle2.serialize_fixed();
        [
            x_bytes[0],
            x_bytes[1],
            y_bytes[0],
            y_bytes[1],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            angle1_bytes[0],
            angle1_bytes[1],
            angle2_bytes[0],
            angle2_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<12> for Arc {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            x: i16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            y: i16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            width: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
            angle1: i16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            angle2: i16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Format {
    pub depth: u8,
    pub bits_per_pixel: u8,
    pub scanline_pad: u8,
}
impl FixedLengthSerialize<8> for Format {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        [
            self.depth,
            self.bits_per_pixel,
            self.scanline_pad,
            0,
            0,
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
            depth: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            bits_per_pixel: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            scanline_pad: u8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct VisualClassEnum(pub u8);
impl VisualClassEnum {
    pub const STATIC_GRAY: Self = Self(0);
    pub const GRAY_SCALE: Self = Self(1);
    pub const STATIC_COLOR: Self = Self(2);
    pub const PSEUDO_COLOR: Self = Self(3);
    pub const TRUE_COLOR: Self = Self(4);
    pub const DIRECT_COLOR: Self = Self(5);
}
impl FixedLengthSerialize<1> for VisualClassEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for VisualClassEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for VisualClassEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for VisualClassEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for VisualClassEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Visualtype {
    pub visual_id: Visualid,
    pub class: VisualClassEnum,
    pub bits_per_rgb_value: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
}
impl FixedLengthSerialize<24> for Visualtype {
    #[inline]
    fn serialize_fixed(self) -> [u8; 24] {
        let visual_id_bytes = self.visual_id.serialize_fixed();
        let colormap_entries_bytes = self.colormap_entries.serialize_fixed();
        let red_mask_bytes = self.red_mask.serialize_fixed();
        let green_mask_bytes = self.green_mask.serialize_fixed();
        let blue_mask_bytes = self.blue_mask.serialize_fixed();
        [
            visual_id_bytes[0],
            visual_id_bytes[1],
            visual_id_bytes[2],
            visual_id_bytes[3],
            self.class.0 as u8,
            self.bits_per_rgb_value,
            colormap_entries_bytes[0],
            colormap_entries_bytes[1],
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
            0,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<24> for Visualtype {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            visual_id: Visualid::from_bytes(
                bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            class: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?.into(),
            bits_per_rgb_value: u8::from_bytes(
                bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?,
            )?,
            colormap_entries: u16::from_bytes(
                bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            red_mask: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            green_mask: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            blue_mask: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Depth {
    pub depth: u8,
    pub visuals: alloc::vec::Vec<Visualtype>,
}
impl VariableLengthSerialize for Depth {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        // Padding 1 bytes
        let visuals_len =
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
                &(u16::try_from(visuals_len).map_err(|_| crate::error::Error::Serialize)?)
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
impl VariableLengthFromBytes for Depth {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        let depth = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let visuals_len = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = visuals_len as usize;
        let visuals: alloc::vec::Vec<Visualtype> = crate::vec_from_bytes_fixed!(
            ptr.get(8..).ok_or(crate::error::Error::FromBytes)?,
            Visualtype,
            length_expr,
            24
        );
        let offset = length_expr * 24 + 8;
        Ok((Self { depth, visuals }, offset))
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct EventMask(pub u32);
impl EventMask {
    pub const NO_EVENT: Self = Self(0);
    pub const KEY_PRESS: Self = Self(1 << 0);
    pub const KEY_RELEASE: Self = Self(1 << 1);
    pub const BUTTON_PRESS: Self = Self(1 << 2);
    pub const BUTTON_RELEASE: Self = Self(1 << 3);
    pub const ENTER_WINDOW: Self = Self(1 << 4);
    pub const LEAVE_WINDOW: Self = Self(1 << 5);
    pub const POINTER_MOTION: Self = Self(1 << 6);
    pub const POINTER_MOTION_HINT: Self = Self(1 << 7);
    pub const BUTTON1_MOTION: Self = Self(1 << 8);
    pub const BUTTON2_MOTION: Self = Self(1 << 9);
    pub const BUTTON3_MOTION: Self = Self(1 << 10);
    pub const BUTTON4_MOTION: Self = Self(1 << 11);
    pub const BUTTON5_MOTION: Self = Self(1 << 12);
    pub const BUTTON_MOTION: Self = Self(1 << 13);
    pub const KEYMAP_STATE: Self = Self(1 << 14);
    pub const EXPOSURE: Self = Self(1 << 15);
    pub const VISIBILITY_CHANGE: Self = Self(1 << 16);
    pub const STRUCTURE_NOTIFY: Self = Self(1 << 17);
    pub const RESIZE_REDIRECT: Self = Self(1 << 18);
    pub const SUBSTRUCTURE_NOTIFY: Self = Self(1 << 19);
    pub const SUBSTRUCTURE_REDIRECT: Self = Self(1 << 20);
    pub const FOCUS_CHANGE: Self = Self(1 << 21);
    pub const PROPERTY_CHANGE: Self = Self(1 << 22);
    pub const COLOR_MAP_CHANGE: Self = Self(1 << 23);
    pub const OWNER_GRAB_BUTTON: Self = Self(1 << 24);
}
impl FixedLengthSerialize<4> for EventMask {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for EventMask {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for EventMask {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for EventMask {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for EventMask {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(EventMask);
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct BackingStoreEnum(pub u32);
impl BackingStoreEnum {
    pub const NOT_USEFUL: Self = Self(0);
    pub const WHEN_MAPPED: Self = Self(1);
    pub const ALWAYS: Self = Self(2);
}
impl FixedLengthSerialize<4> for BackingStoreEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for BackingStoreEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for BackingStoreEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for BackingStoreEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for BackingStoreEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Screen {
    pub root: Window,
    pub default_colormap: Colormap,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: EventMask,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: Visualid,
    pub backing_stores: BackingStoreEnum,
    pub save_unders: u8,
    pub root_depth: u8,
    pub allowed_depths: alloc::vec::Vec<Depth>,
}
impl VariableLengthSerialize for Screen {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let allowed_depths_len =
            u8::try_from(self.allowed_depths.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.root.serialize_fixed());
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.default_colormap.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.white_pixel.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.black_pixel.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.current_input_masks.serialize_fixed());
        buf_ptr
            .get_mut(20..22)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.width_in_pixels.serialize_fixed());
        buf_ptr
            .get_mut(22..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.height_in_pixels.serialize_fixed());
        buf_ptr
            .get_mut(24..26)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.width_in_millimeters.serialize_fixed());
        buf_ptr
            .get_mut(26..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.height_in_millimeters.serialize_fixed());
        buf_ptr
            .get_mut(28..30)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.min_installed_maps.serialize_fixed());
        buf_ptr
            .get_mut(30..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.max_installed_maps.serialize_fixed());
        buf_ptr
            .get_mut(32..36)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.root_visual.serialize_fixed());
        buf_ptr
            .get_mut(36..37)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.backing_stores.serialize_fixed());
        buf_ptr
            .get_mut(37..38)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.save_unders.serialize_fixed());
        buf_ptr
            .get_mut(38..39)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.root_depth.serialize_fixed());
        buf_ptr
            .get_mut(39..40)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(allowed_depths_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let offset = crate::util::var_vec_serialize_into(
            buf_ptr
                .get_mut(40..)
                .ok_or(crate::error::Error::Serialize)?,
            self.allowed_depths,
        )?;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for Screen {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let root = Window::from_bytes(ptr.get(0..4).ok_or(crate::error::Error::FromBytes)?)?;
        let default_colormap =
            Colormap::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let white_pixel = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let black_pixel = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let current_input_masks =
            u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let width_in_pixels =
            u16::from_bytes(ptr.get(20..22).ok_or(crate::error::Error::FromBytes)?)?;
        let height_in_pixels =
            u16::from_bytes(ptr.get(22..24).ok_or(crate::error::Error::FromBytes)?)?;
        let width_in_millimeters =
            u16::from_bytes(ptr.get(24..26).ok_or(crate::error::Error::FromBytes)?)?;
        let height_in_millimeters =
            u16::from_bytes(ptr.get(26..28).ok_or(crate::error::Error::FromBytes)?)?;
        let min_installed_maps =
            u16::from_bytes(ptr.get(28..30).ok_or(crate::error::Error::FromBytes)?)?;
        let max_installed_maps =
            u16::from_bytes(ptr.get(30..32).ok_or(crate::error::Error::FromBytes)?)?;
        let root_visual =
            Visualid::from_bytes(ptr.get(32..36).ok_or(crate::error::Error::FromBytes)?)?;
        let backing_stores =
            u8::from_bytes(ptr.get(36..37).ok_or(crate::error::Error::FromBytes)?)?;
        let save_unders = u8::from_bytes(ptr.get(37..38).ok_or(crate::error::Error::FromBytes)?)?;
        let root_depth = u8::from_bytes(ptr.get(38..39).ok_or(crate::error::Error::FromBytes)?)?;
        let allowed_depths_len =
            u8::from_bytes(ptr.get(39..40).ok_or(crate::error::Error::FromBytes)?)?;
        let allowed_depths_length = allowed_depths_len as usize;
        let mut offset = 40;
        let allowed_depths = crate::vec_from_bytes_var!(ptr, Depth, offset, allowed_depths_length,);
        Ok((
            Self {
                root,
                default_colormap,
                white_pixel,
                black_pixel,
                current_input_masks: current_input_masks.into(),
                width_in_pixels,
                height_in_pixels,
                width_in_millimeters,
                height_in_millimeters,
                min_installed_maps,
                max_installed_maps,
                root_visual,
                backing_stores: backing_stores.into(),
                save_unders,
                root_depth,
                allowed_depths,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetupRequest {
    pub byte_order: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub authorization_protocol_name: alloc::vec::Vec<u8>,
    pub authorization_protocol_data: alloc::vec::Vec<u8>,
}
impl VariableLengthSerialize for SetupRequest {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        // Padding 1 bytes
        let authorization_protocol_name_len = u16::try_from(self.authorization_protocol_name.len())
            .map_err(|_| crate::error::Error::Serialize)?;
        let authorization_protocol_data_len = u16::try_from(self.authorization_protocol_data.len())
            .map_err(|_| crate::error::Error::Serialize)?;
        // Padding 2 bytes
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.byte_order.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.protocol_major_version.serialize_fixed());
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.protocol_minor_version.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(authorization_protocol_name_len)
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(authorization_protocol_data_len)
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = self.authorization_protocol_name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            &self.authorization_protocol_name,
        )?;
        let mut offset = list_len + 12;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let list_len = self.authorization_protocol_data.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            &self.authorization_protocol_data,
        )?;
        offset += list_len;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for SetupRequest {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 2 bytes
        let byte_order = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let protocol_major_version =
            u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let protocol_minor_version =
            u16::from_bytes(ptr.get(4..6).ok_or(crate::error::Error::FromBytes)?)?;
        let authorization_protocol_name_len =
            u16::from_bytes(ptr.get(6..8).ok_or(crate::error::Error::FromBytes)?)?;
        let authorization_protocol_data_len =
            u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = authorization_protocol_name_len as usize;
        let authorization_protocol_name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(12..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr + 12;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let length_expr = authorization_protocol_data_len as usize;
        let authorization_protocol_data: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        offset += length_expr;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok((
            Self {
                byte_order,
                protocol_major_version,
                protocol_minor_version,
                authorization_protocol_name,
                authorization_protocol_data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetupFailed {
    pub status: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
    pub reason: alloc::vec::Vec<u8>,
}
impl VariableLengthSerialize for SetupFailed {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let reason_len =
            u8::try_from(self.reason.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.status.serialize_fixed());
        buf_ptr
            .get_mut(1..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(reason_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.protocol_major_version.serialize_fixed());
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.protocol_minor_version.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.length.serialize_fixed());
        let list_len = self.reason.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            &self.reason,
        )?;
        let offset = list_len + 8;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for SetupFailed {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let status = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let reason_len = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let protocol_major_version =
            u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let protocol_minor_version =
            u16::from_bytes(ptr.get(4..6).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u16::from_bytes(ptr.get(6..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = reason_len as usize;
        let reason: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(8..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 8;
        Ok((
            Self {
                status,
                protocol_major_version,
                protocol_minor_version,
                length,
                reason,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetupAuthenticate {
    pub status: u8,
    pub reason: alloc::vec::Vec<u8>,
}
impl VariableLengthSerialize for SetupAuthenticate {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        // Padding 5 bytes
        let length =
            u16::try_from(self.reason.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.status.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(core::ops::Div::div(length as u16, 4u16 as u16))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = self.reason.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            &self.reason,
        )?;
        let offset = list_len + 8;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for SetupAuthenticate {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 5 bytes
        let status = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u16::from_bytes(ptr.get(6..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(length as u16, 4u16 as u16) as usize;
        let reason: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(8..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 8;
        Ok((Self { status, reason }, offset))
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ImageOrderEnum(pub u8);
impl ImageOrderEnum {
    pub const L_S_B_FIRST: Self = Self(0);
    pub const M_S_B_FIRST: Self = Self(1);
}
impl FixedLengthSerialize<1> for ImageOrderEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ImageOrderEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ImageOrderEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ImageOrderEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ImageOrderEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Setup {
    pub status: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub maximum_request_length: u16,
    pub image_byte_order: ImageOrderEnum,
    pub bitmap_format_bit_order: ImageOrderEnum,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: Keycode,
    pub max_keycode: Keycode,
    pub vendor: alloc::vec::Vec<u8>,
    pub pixmap_formats: alloc::vec::Vec<Format>,
    pub roots: alloc::vec::Vec<Screen>,
}
impl VariableLengthSerialize for Setup {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        // Padding 1 bytes
        let vendor_len =
            u16::try_from(self.vendor.len()).map_err(|_| crate::error::Error::Serialize)?;
        let roots_len =
            u8::try_from(self.roots.len()).map_err(|_| crate::error::Error::Serialize)?;
        let pixmap_formats_len =
            u8::try_from(self.pixmap_formats.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Padding 4 bytes
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.status.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.protocol_major_version.serialize_fixed());
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.protocol_minor_version.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.length.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.release_number.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.resource_id_base.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.resource_id_mask.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.motion_buffer_size.serialize_fixed());
        buf_ptr
            .get_mut(24..26)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(vendor_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(26..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.maximum_request_length.serialize_fixed());
        buf_ptr
            .get_mut(28..29)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(roots_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(29..30)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(pixmap_formats_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(30..31)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.image_byte_order.serialize_fixed());
        buf_ptr
            .get_mut(31..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.bitmap_format_bit_order.serialize_fixed());
        buf_ptr
            .get_mut(32..33)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.bitmap_format_scanline_unit.serialize_fixed());
        buf_ptr
            .get_mut(33..34)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.bitmap_format_scanline_pad.serialize_fixed());
        buf_ptr
            .get_mut(34..35)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.min_keycode.serialize_fixed());
        buf_ptr
            .get_mut(35..36)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.max_keycode.serialize_fixed());
        let list_len = self.vendor.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(40..)
                .ok_or(crate::error::Error::Serialize)?,
            &self.vendor,
        )?;
        let mut offset = list_len + 40;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let list_len = self.pixmap_formats.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            self.pixmap_formats,
        )?;
        offset += list_len;
        offset += crate::util::var_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            self.roots,
        )?;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for Setup {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        let status = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let protocol_major_version =
            u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let protocol_minor_version =
            u16::from_bytes(ptr.get(4..6).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u16::from_bytes(ptr.get(6..8).ok_or(crate::error::Error::FromBytes)?)?;
        let release_number =
            u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let resource_id_base =
            u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let resource_id_mask =
            u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let motion_buffer_size =
            u32::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let vendor_len = u16::from_bytes(ptr.get(24..26).ok_or(crate::error::Error::FromBytes)?)?;
        let maximum_request_length =
            u16::from_bytes(ptr.get(26..28).ok_or(crate::error::Error::FromBytes)?)?;
        let roots_len = u8::from_bytes(ptr.get(28..29).ok_or(crate::error::Error::FromBytes)?)?;
        let pixmap_formats_len =
            u8::from_bytes(ptr.get(29..30).ok_or(crate::error::Error::FromBytes)?)?;
        let image_byte_order =
            u8::from_bytes(ptr.get(30..31).ok_or(crate::error::Error::FromBytes)?)?;
        let bitmap_format_bit_order =
            u8::from_bytes(ptr.get(31..32).ok_or(crate::error::Error::FromBytes)?)?;
        let bitmap_format_scanline_unit =
            u8::from_bytes(ptr.get(32..33).ok_or(crate::error::Error::FromBytes)?)?;
        let bitmap_format_scanline_pad =
            u8::from_bytes(ptr.get(33..34).ok_or(crate::error::Error::FromBytes)?)?;
        let min_keycode =
            Keycode::from_bytes(ptr.get(34..35).ok_or(crate::error::Error::FromBytes)?)?;
        let max_keycode =
            Keycode::from_bytes(ptr.get(35..36).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = vendor_len as usize;
        let vendor: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(40..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr + 40;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let length_expr = pixmap_formats_len as usize;
        let pixmap_formats: alloc::vec::Vec<Format> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Format,
            length_expr,
            8
        );
        offset += length_expr * 8;
        let roots_length = roots_len as usize;
        let roots = crate::vec_from_bytes_var!(ptr, Screen, offset, roots_length,);
        Ok((
            Self {
                status,
                protocol_major_version,
                protocol_minor_version,
                length,
                release_number,
                resource_id_base,
                resource_id_mask,
                motion_buffer_size,
                maximum_request_length,
                image_byte_order: image_byte_order.into(),
                bitmap_format_bit_order: bitmap_format_bit_order.into(),
                bitmap_format_scanline_unit,
                bitmap_format_scanline_pad,
                min_keycode,
                max_keycode,
                vendor,
                pixmap_formats,
                roots,
            },
            offset,
        ))
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ModMask(pub u16);
impl ModMask {
    pub const SHIFT: Self = Self(1 << 0);
    pub const LOCK: Self = Self(1 << 1);
    pub const CONTROL: Self = Self(1 << 2);
    pub const ONE: Self = Self(1 << 3);
    pub const TWO: Self = Self(1 << 4);
    pub const THREE: Self = Self(1 << 5);
    pub const FOUR: Self = Self(1 << 6);
    pub const FIVE: Self = Self(1 << 7);
    pub const ANY: Self = Self(1 << 15);
}
impl FixedLengthSerialize<2> for ModMask {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for ModMask {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for ModMask {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for ModMask {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for ModMask {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
crate::implement_bit_ops!(ModMask);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct KeyButMask(pub u16);
impl KeyButMask {
    pub const SHIFT: Self = Self(1 << 0);
    pub const LOCK: Self = Self(1 << 1);
    pub const CONTROL: Self = Self(1 << 2);
    pub const MOD1: Self = Self(1 << 3);
    pub const MOD2: Self = Self(1 << 4);
    pub const MOD3: Self = Self(1 << 5);
    pub const MOD4: Self = Self(1 << 6);
    pub const MOD5: Self = Self(1 << 7);
    pub const BUTTON1: Self = Self(1 << 8);
    pub const BUTTON2: Self = Self(1 << 9);
    pub const BUTTON3: Self = Self(1 << 10);
    pub const BUTTON4: Self = Self(1 << 11);
    pub const BUTTON5: Self = Self(1 << 12);
}
impl FixedLengthSerialize<2> for KeyButMask {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for KeyButMask {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for KeyButMask {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for KeyButMask {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for KeyButMask {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
crate::implement_bit_ops!(KeyButMask);
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct WindowEnum(pub Window);
impl WindowEnum {
    pub const NONE: Self = Self(0);
}
impl FixedLengthSerialize<4> for WindowEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for WindowEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(Window::from_bytes(bytes)?))
    }
}
impl From<u32> for WindowEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(Window::from(val as u32))
    }
}
impl From<u16> for WindowEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(Window::from(val as u32))
    }
}
impl From<u8> for WindowEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(Window::from(val as u32))
    }
}
pub const KEY_PRESS_EVENT: u8 = 2;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct KeyPressEvent {
    pub opcode: u8,
    pub detail: Keycode,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: WindowEnum,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: KeyButMask,
    pub same_screen: u8,
}
impl FixedLengthFromBytes<32> for KeyPressEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            detail: Keycode::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: Timestamp::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            root: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            event: Window::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            child: Window::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            root_x: i16::from_bytes(bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?)?,
            root_y: i16::from_bytes(bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?)?,
            event_x: i16::from_bytes(bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?)?,
            event_y: i16::from_bytes(bytes.get(26..28).ok_or(crate::error::Error::FromBytes)?)?,
            state: u16::from_bytes(bytes.get(28..30).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            same_screen: u8::from_bytes(bytes.get(30..31).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const KEY_RELEASE_EVENT: u8 = 3;
pub type KeyReleaseEvent = KeyPressEvent;
pub const BUTTON_PRESS_EVENT: u8 = 4;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ButtonPressEvent {
    pub opcode: u8,
    pub detail: Button,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: WindowEnum,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: KeyButMask,
    pub same_screen: u8,
}
impl FixedLengthFromBytes<32> for ButtonPressEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            detail: Button::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: Timestamp::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            root: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            event: Window::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            child: Window::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            root_x: i16::from_bytes(bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?)?,
            root_y: i16::from_bytes(bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?)?,
            event_x: i16::from_bytes(bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?)?,
            event_y: i16::from_bytes(bytes.get(26..28).ok_or(crate::error::Error::FromBytes)?)?,
            state: u16::from_bytes(bytes.get(28..30).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            same_screen: u8::from_bytes(bytes.get(30..31).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const BUTTON_RELEASE_EVENT: u8 = 5;
pub type ButtonReleaseEvent = ButtonPressEvent;
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct MotionEnum(pub u8);
impl MotionEnum {
    pub const NORMAL: Self = Self(0);
    pub const HINT: Self = Self(1);
}
impl FixedLengthSerialize<1> for MotionEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for MotionEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for MotionEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for MotionEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for MotionEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
pub const MOTION_NOTIFY_EVENT: u8 = 6;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct MotionNotifyEvent {
    pub opcode: u8,
    pub detail: MotionEnum,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: WindowEnum,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: KeyButMask,
    pub same_screen: u8,
}
impl FixedLengthFromBytes<32> for MotionNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            detail: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: Timestamp::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            root: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            event: Window::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            child: Window::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            root_x: i16::from_bytes(bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?)?,
            root_y: i16::from_bytes(bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?)?,
            event_x: i16::from_bytes(bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?)?,
            event_y: i16::from_bytes(bytes.get(26..28).ok_or(crate::error::Error::FromBytes)?)?,
            state: u16::from_bytes(bytes.get(28..30).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            same_screen: u8::from_bytes(bytes.get(30..31).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct NotifyDetailEnum(pub u8);
impl NotifyDetailEnum {
    pub const ANCESTOR: Self = Self(0);
    pub const VIRTUAL: Self = Self(1);
    pub const INFERIOR: Self = Self(2);
    pub const NONLINEAR: Self = Self(3);
    pub const NONLINEAR_VIRTUAL: Self = Self(4);
    pub const POINTER: Self = Self(5);
    pub const POINTER_ROOT: Self = Self(6);
    pub const NONE: Self = Self(7);
}
impl FixedLengthSerialize<1> for NotifyDetailEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for NotifyDetailEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for NotifyDetailEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for NotifyDetailEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for NotifyDetailEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct NotifyModeEnum(pub u8);
impl NotifyModeEnum {
    pub const NORMAL: Self = Self(0);
    pub const GRAB: Self = Self(1);
    pub const UNGRAB: Self = Self(2);
    pub const WHILE_GRABBED: Self = Self(3);
}
impl FixedLengthSerialize<1> for NotifyModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for NotifyModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for NotifyModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for NotifyModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for NotifyModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
pub const ENTER_NOTIFY_EVENT: u8 = 7;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct EnterNotifyEvent {
    pub opcode: u8,
    pub detail: NotifyDetailEnum,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: WindowEnum,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: KeyButMask,
    pub mode: NotifyModeEnum,
    pub same_screen_focus: u8,
}
impl FixedLengthFromBytes<32> for EnterNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            detail: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: Timestamp::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            root: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            event: Window::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            child: Window::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            root_x: i16::from_bytes(bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?)?,
            root_y: i16::from_bytes(bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?)?,
            event_x: i16::from_bytes(bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?)?,
            event_y: i16::from_bytes(bytes.get(26..28).ok_or(crate::error::Error::FromBytes)?)?,
            state: u16::from_bytes(bytes.get(28..30).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            mode: u8::from_bytes(bytes.get(30..31).ok_or(crate::error::Error::FromBytes)?)?.into(),
            same_screen_focus: u8::from_bytes(
                bytes.get(31..32).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
pub const LEAVE_NOTIFY_EVENT: u8 = 8;
pub type LeaveNotifyEvent = EnterNotifyEvent;
pub const FOCUS_IN_EVENT: u8 = 9;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct FocusInEvent {
    pub opcode: u8,
    pub detail: NotifyDetailEnum,
    pub sequence: u16,
    pub event: Window,
    pub mode: NotifyModeEnum,
}
impl FixedLengthFromBytes<12> for FocusInEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            detail: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            event: Window::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            mode: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
pub const FOCUS_OUT_EVENT: u8 = 10;
pub type FocusOutEvent = FocusInEvent;
pub const KEYMAP_NOTIFY_EVENT: u8 = 11;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct KeymapNotifyEvent {
    pub opcode: u8,
    pub keys: [u8; 31],
}
impl FixedLengthFromBytes<32> for KeymapNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            // SAFETY: We know we can try into exact size slice
            keys: unsafe {
                bytes
                    .get(1..1 + 31)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}
pub const EXPOSE_EVENT: u8 = 12;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ExposeEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub window: Window,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub count: u16,
}
impl FixedLengthFromBytes<20> for ExposeEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            window: Window::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            x: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            y: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
            width: u16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
            count: u16::from_bytes(bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const GRAPHICS_EXPOSURE_EVENT: u8 = 13;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GraphicsExposureEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub drawable: Drawable,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub minor_opcode: u16,
    pub count: u16,
    pub major_opcode: u8,
}
impl FixedLengthFromBytes<24> for GraphicsExposureEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            drawable: Drawable::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            x: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            y: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
            width: u16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
            minor_opcode: u16::from_bytes(
                bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?,
            )?,
            count: u16::from_bytes(bytes.get(18..20).ok_or(crate::error::Error::FromBytes)?)?,
            major_opcode: u8::from_bytes(bytes.get(20..21).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const NO_EXPOSURE_EVENT: u8 = 14;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct NoExposureEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub drawable: Drawable,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl FixedLengthFromBytes<12> for NoExposureEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            drawable: Drawable::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            minor_opcode: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            major_opcode: u8::from_bytes(bytes.get(10..11).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct VisibilityEnum(pub u8);
impl VisibilityEnum {
    pub const UNOBSCURED: Self = Self(0);
    pub const PARTIALLY_OBSCURED: Self = Self(1);
    pub const FULLY_OBSCURED: Self = Self(2);
}
impl FixedLengthSerialize<1> for VisibilityEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for VisibilityEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for VisibilityEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for VisibilityEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for VisibilityEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
pub const VISIBILITY_NOTIFY_EVENT: u8 = 15;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct VisibilityNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub window: Window,
    pub state: VisibilityEnum,
}
impl FixedLengthFromBytes<12> for VisibilityNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            window: Window::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            state: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
pub const CREATE_NOTIFY_EVENT: u8 = 16;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CreateNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub parent: Window,
    pub window: Window,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: u8,
}
impl FixedLengthFromBytes<24> for CreateNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            parent: Window::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            window: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            x: i16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
            y: i16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
            width: u16::from_bytes(bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(18..20).ok_or(crate::error::Error::FromBytes)?)?,
            border_width: u16::from_bytes(
                bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?,
            )?,
            override_redirect: u8::from_bytes(
                bytes.get(22..23).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
pub const DESTROY_NOTIFY_EVENT: u8 = 17;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DestroyNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
}
impl FixedLengthFromBytes<12> for DestroyNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            event: Window::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            window: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const UNMAP_NOTIFY_EVENT: u8 = 18;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct UnmapNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub from_configure: u8,
}
impl FixedLengthFromBytes<16> for UnmapNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            event: Window::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            window: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            from_configure: u8::from_bytes(
                bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
pub const MAP_NOTIFY_EVENT: u8 = 19;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct MapNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub override_redirect: u8,
}
impl FixedLengthFromBytes<16> for MapNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            event: Window::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            window: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            override_redirect: u8::from_bytes(
                bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
pub const MAP_REQUEST_EVENT: u8 = 20;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct MapRequestEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub parent: Window,
    pub window: Window,
}
impl FixedLengthFromBytes<12> for MapRequestEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            parent: Window::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            window: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const REPARENT_NOTIFY_EVENT: u8 = 21;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ReparentNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: i16,
    pub y: i16,
    pub override_redirect: u8,
}
impl FixedLengthFromBytes<24> for ReparentNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            event: Window::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            window: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            parent: Window::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            x: i16::from_bytes(bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?)?,
            y: i16::from_bytes(bytes.get(18..20).ok_or(crate::error::Error::FromBytes)?)?,
            override_redirect: u8::from_bytes(
                bytes.get(20..21).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
pub const CONFIGURE_NOTIFY_EVENT: u8 = 22;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ConfigureNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub above_sibling: WindowEnum,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: u8,
}
impl FixedLengthFromBytes<28> for ConfigureNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            event: Window::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            window: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            above_sibling: Window::from_bytes(
                bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            x: i16::from_bytes(bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?)?,
            y: i16::from_bytes(bytes.get(18..20).ok_or(crate::error::Error::FromBytes)?)?,
            width: u16::from_bytes(bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?)?,
            border_width: u16::from_bytes(
                bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?,
            )?,
            override_redirect: u8::from_bytes(
                bytes.get(26..27).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
pub const GRAVITY_NOTIFY_EVENT: u8 = 24;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GravityNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub x: i16,
    pub y: i16,
}
impl FixedLengthFromBytes<16> for GravityNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            event: Window::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            window: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            x: i16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
            y: i16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const RESIZE_REQUEST_EVENT: u8 = 25;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ResizeRequestEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub window: Window,
    pub width: u16,
    pub height: u16,
}
impl FixedLengthFromBytes<12> for ResizeRequestEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            window: Window::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            width: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PlaceEnum(pub u8);
impl PlaceEnum {
    pub const ON_TOP: Self = Self(0);
    pub const ON_BOTTOM: Self = Self(1);
}
impl FixedLengthSerialize<1> for PlaceEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for PlaceEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for PlaceEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for PlaceEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for PlaceEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
pub const CIRCULATE_NOTIFY_EVENT: u8 = 26;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CirculateNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub place: PlaceEnum,
}
impl FixedLengthFromBytes<20> for CirculateNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            event: Window::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            window: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            place: u8::from_bytes(bytes.get(16..17).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
pub const CIRCULATE_REQUEST_EVENT: u8 = 27;
pub type CirculateRequestEvent = CirculateNotifyEvent;
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PropertyEnum(pub u8);
impl PropertyEnum {
    pub const NEW_VALUE: Self = Self(0);
    pub const DELETE: Self = Self(1);
}
impl FixedLengthSerialize<1> for PropertyEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for PropertyEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for PropertyEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for PropertyEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for PropertyEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
pub const PROPERTY_NOTIFY_EVENT: u8 = 28;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct PropertyNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub window: Window,
    pub atom: Atom,
    pub time: Timestamp,
    pub state: PropertyEnum,
}
impl FixedLengthFromBytes<20> for PropertyNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            window: Window::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            atom: Atom::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            time: Timestamp::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            state: u8::from_bytes(bytes.get(16..17).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
pub const SELECTION_CLEAR_EVENT: u8 = 29;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SelectionClearEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub time: Timestamp,
    pub owner: Window,
    pub selection: Atom,
}
impl FixedLengthFromBytes<16> for SelectionClearEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: Timestamp::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            owner: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            selection: Atom::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct TimeEnum(pub Timestamp);
impl TimeEnum {
    pub const CURRENT_TIME: Self = Self(0);
}
impl FixedLengthSerialize<4> for TimeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for TimeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(Timestamp::from_bytes(bytes)?))
    }
}
impl From<u32> for TimeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for TimeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for TimeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct AtomEnum(pub Atom);
impl AtomEnum {
    pub const NONE: Self = Self(0);
    pub const ANY: Self = Self(0);
    pub const PRIMARY: Self = Self(1);
    pub const SECONDARY: Self = Self(2);
    pub const ARC: Self = Self(3);
    pub const ATOM: Self = Self(4);
    pub const BITMAP: Self = Self(5);
    pub const CARDINAL: Self = Self(6);
    pub const COLORMAP: Self = Self(7);
    pub const CURSOR: Self = Self(8);
    pub const CUT_BUFFER0: Self = Self(9);
    pub const CUT_BUFFER1: Self = Self(10);
    pub const CUT_BUFFER2: Self = Self(11);
    pub const CUT_BUFFER3: Self = Self(12);
    pub const CUT_BUFFER4: Self = Self(13);
    pub const CUT_BUFFER5: Self = Self(14);
    pub const CUT_BUFFER6: Self = Self(15);
    pub const CUT_BUFFER7: Self = Self(16);
    pub const DRAWABLE: Self = Self(17);
    pub const FONT: Self = Self(18);
    pub const INTEGER: Self = Self(19);
    pub const PIXMAP: Self = Self(20);
    pub const POINT: Self = Self(21);
    pub const RECTANGLE: Self = Self(22);
    pub const RESOURCE_MANAGER: Self = Self(23);
    pub const RGB_COLOR_MAP: Self = Self(24);
    pub const RGB_BEST_MAP: Self = Self(25);
    pub const RGB_BLUE_MAP: Self = Self(26);
    pub const RGB_DEFAULT_MAP: Self = Self(27);
    pub const RGB_GRAY_MAP: Self = Self(28);
    pub const RGB_GREEN_MAP: Self = Self(29);
    pub const RGB_RED_MAP: Self = Self(30);
    pub const STRING: Self = Self(31);
    pub const VISUALID: Self = Self(32);
    pub const WINDOW: Self = Self(33);
    pub const WM_COMMAND: Self = Self(34);
    pub const WM_HINTS: Self = Self(35);
    pub const WM_CLIENT_MACHINE: Self = Self(36);
    pub const WM_ICON_NAME: Self = Self(37);
    pub const WM_ICON_SIZE: Self = Self(38);
    pub const WM_NAME: Self = Self(39);
    pub const WM_NORMAL_HINTS: Self = Self(40);
    pub const WM_SIZE_HINTS: Self = Self(41);
    pub const WM_ZOOM_HINTS: Self = Self(42);
    pub const MIN_SPACE: Self = Self(43);
    pub const NORM_SPACE: Self = Self(44);
    pub const MAX_SPACE: Self = Self(45);
    pub const END_SPACE: Self = Self(46);
    pub const SUPERSCRIPT_X: Self = Self(47);
    pub const SUPERSCRIPT_Y: Self = Self(48);
    pub const SUBSCRIPT_X: Self = Self(49);
    pub const SUBSCRIPT_Y: Self = Self(50);
    pub const UNDERLINE_POSITION: Self = Self(51);
    pub const UNDERLINE_THICKNESS: Self = Self(52);
    pub const STRIKEOUT_ASCENT: Self = Self(53);
    pub const STRIKEOUT_DESCENT: Self = Self(54);
    pub const ITALIC_ANGLE: Self = Self(55);
    pub const X_HEIGHT: Self = Self(56);
    pub const QUAD_WIDTH: Self = Self(57);
    pub const WEIGHT: Self = Self(58);
    pub const POINT_SIZE: Self = Self(59);
    pub const RESOLUTION: Self = Self(60);
    pub const COPYRIGHT: Self = Self(61);
    pub const NOTICE: Self = Self(62);
    pub const FONT_NAME: Self = Self(63);
    pub const FAMILY_NAME: Self = Self(64);
    pub const FULL_NAME: Self = Self(65);
    pub const CAP_HEIGHT: Self = Self(66);
    pub const WM_CLASS: Self = Self(67);
    pub const WM_TRANSIENT_FOR: Self = Self(68);
}
impl FixedLengthSerialize<4> for AtomEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for AtomEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(Atom::from_bytes(bytes)?))
    }
}
impl From<u32> for AtomEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(Atom::from(val as u32))
    }
}
impl From<u16> for AtomEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(Atom::from(val as u32))
    }
}
impl From<u8> for AtomEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(Atom::from(val as u32))
    }
}
pub const SELECTION_REQUEST_EVENT: u8 = 30;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SelectionRequestEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub time: TimeEnum,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: AtomEnum,
}
impl FixedLengthFromBytes<28> for SelectionRequestEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: Timestamp::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            owner: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            requestor: Window::from_bytes(
                bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            selection: Atom::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            target: Atom::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
            property: Atom::from_bytes(bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
pub const SELECTION_NOTIFY_EVENT: u8 = 31;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SelectionNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub time: TimeEnum,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: AtomEnum,
}
impl FixedLengthFromBytes<24> for SelectionNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: Timestamp::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            requestor: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            selection: Atom::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            target: Atom::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            property: Atom::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ColormapStateEnum(pub u8);
impl ColormapStateEnum {
    pub const UNINSTALLED: Self = Self(0);
    pub const INSTALLED: Self = Self(1);
}
impl FixedLengthSerialize<1> for ColormapStateEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ColormapStateEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ColormapStateEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ColormapStateEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ColormapStateEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ColormapEnum(pub Colormap);
impl ColormapEnum {
    pub const NONE: Self = Self(0);
}
impl FixedLengthSerialize<4> for ColormapEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for ColormapEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(Colormap::from_bytes(bytes)?))
    }
}
impl From<u32> for ColormapEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(Colormap::from(val as u32))
    }
}
impl From<u16> for ColormapEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(Colormap::from(val as u32))
    }
}
impl From<u8> for ColormapEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(Colormap::from(val as u32))
    }
}
pub const COLORMAP_NOTIFY_EVENT: u8 = 32;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ColormapNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub window: Window,
    pub colormap: ColormapEnum,
    pub new: u8,
    pub state: ColormapStateEnum,
}
impl FixedLengthFromBytes<16> for ColormapNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            window: Window::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            colormap: Colormap::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            new: u8::from_bytes(bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?)?,
            state: u8::from_bytes(bytes.get(13..14).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ClientMessageData(pub [u8; 20]);
impl FixedLengthFromBytes<20> for ClientMessageData {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        // SAFETY: Checked that the bytes are available
        Ok(Self(unsafe {
            bytes
                .get(..20)
                .ok_or(crate::error::Error::FromBytes)?
                .try_into()
                .unwrap_unchecked()
        }))
    }
}
impl FixedLengthSerialize<20> for ClientMessageData {
    #[inline]
    fn serialize_fixed(self) -> [u8; 20] {
        self.0
    }
}
pub const CLIENT_MESSAGE_EVENT: u8 = 33;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ClientMessageEvent {
    pub opcode: u8,
    pub format: u8,
    pub sequence: u16,
    pub window: Window,
    pub r#type: Atom,
    pub data: ClientMessageData,
}
impl FixedLengthFromBytes<32> for ClientMessageEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            format: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            window: Window::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            r#type: Atom::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            data: ClientMessageData::from_bytes(
                bytes.get(12..32).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct MappingEnum(pub u8);
impl MappingEnum {
    pub const MODIFIER: Self = Self(0);
    pub const KEYBOARD: Self = Self(1);
    pub const POINTER: Self = Self(2);
}
impl FixedLengthSerialize<1> for MappingEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for MappingEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for MappingEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for MappingEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for MappingEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
pub const MAPPING_NOTIFY_EVENT: u8 = 34;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct MappingNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub request: MappingEnum,
    pub first_keycode: Keycode,
    pub count: u8,
}
impl FixedLengthFromBytes<8> for MappingNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            request: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?.into(),
            first_keycode: Keycode::from_bytes(
                bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?,
            )?,
            count: u8::from_bytes(bytes.get(6..7).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const GE_GENERIC_EVENT: u8 = 35;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GeGenericEvent {
    pub opcode: u8,
    pub sequence: u16,
}
impl FixedLengthFromBytes<25> for GeGenericEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(1..3).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const REQUEST_ERROR: u8 = 1;
pub const VALUE_ERROR: u8 = 2;
pub const WINDOW_ERROR: u8 = 3;
pub const PIXMAP_ERROR: u8 = 4;
pub const ATOM_ERROR: u8 = 5;
pub const CURSOR_ERROR: u8 = 6;
pub const FONT_ERROR: u8 = 7;
pub const MATCH_ERROR: u8 = 8;
pub const DRAWABLE_ERROR: u8 = 9;
pub const ACCESS_ERROR: u8 = 10;
pub const ALLOC_ERROR: u8 = 11;
pub const COLORMAP_ERROR: u8 = 12;
pub const G_CONTEXT_ERROR: u8 = 13;
pub const I_D_CHOICE_ERROR: u8 = 14;
pub const NAME_ERROR: u8 = 15;
pub const LENGTH_ERROR: u8 = 16;
pub const IMPLEMENTATION_ERROR: u8 = 17;
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct WindowClassEnum(pub u16);
impl WindowClassEnum {
    pub const COPY_FROM_PARENT: Self = Self(0);
    pub const INPUT_OUTPUT: Self = Self(1);
    pub const INPUT_ONLY: Self = Self(2);
}
impl FixedLengthSerialize<2> for WindowClassEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for WindowClassEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for WindowClassEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for WindowClassEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for WindowClassEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Cw(pub u32);
impl Cw {
    pub const BACK_PIXMAP: Self = Self(1 << 0);
    pub const BACK_PIXEL: Self = Self(1 << 1);
    pub const BORDER_PIXMAP: Self = Self(1 << 2);
    pub const BORDER_PIXEL: Self = Self(1 << 3);
    pub const BIT_GRAVITY: Self = Self(1 << 4);
    pub const WIN_GRAVITY: Self = Self(1 << 5);
    pub const BACKING_STORE: Self = Self(1 << 6);
    pub const BACKING_PLANES: Self = Self(1 << 7);
    pub const BACKING_PIXEL: Self = Self(1 << 8);
    pub const OVERRIDE_REDIRECT: Self = Self(1 << 9);
    pub const SAVE_UNDER: Self = Self(1 << 10);
    pub const EVENT_MASK: Self = Self(1 << 11);
    pub const DONT_PROPAGATE: Self = Self(1 << 12);
    pub const COLORMAP: Self = Self(1 << 13);
    pub const CURSOR: Self = Self(1 << 14);
}
impl FixedLengthSerialize<4> for Cw {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for Cw {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for Cw {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for Cw {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for Cw {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(Cw);
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct BackPixmapEnum(pub Pixmap);
impl BackPixmapEnum {
    pub const NONE: Self = Self(0);
    pub const PARENT_RELATIVE: Self = Self(1);
}
impl FixedLengthSerialize<4> for BackPixmapEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for BackPixmapEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(Pixmap::from_bytes(bytes)?))
    }
}
impl From<u32> for BackPixmapEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(Pixmap::from(val as u32))
    }
}
impl From<u16> for BackPixmapEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(Pixmap::from(val as u32))
    }
}
impl From<u8> for BackPixmapEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(Pixmap::from(val as u32))
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct GravityEnum(pub u32);
impl GravityEnum {
    pub const BIT_FORGET: Self = Self(0);
    pub const WIN_UNMAP: Self = Self(0);
    pub const NORTH_WEST: Self = Self(1);
    pub const NORTH: Self = Self(2);
    pub const NORTH_EAST: Self = Self(3);
    pub const WEST: Self = Self(4);
    pub const CENTER: Self = Self(5);
    pub const EAST: Self = Self(6);
    pub const SOUTH_WEST: Self = Self(7);
    pub const SOUTH: Self = Self(8);
    pub const SOUTH_EAST: Self = Self(9);
    pub const STATIC: Self = Self(10);
}
impl FixedLengthSerialize<4> for GravityEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for GravityEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for GravityEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for GravityEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for GravityEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct MapStateEnum(pub u8);
impl MapStateEnum {
    pub const UNMAPPED: Self = Self(0);
    pub const UNVIEWABLE: Self = Self(1);
    pub const VIEWABLE: Self = Self(2);
}
impl FixedLengthSerialize<1> for MapStateEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for MapStateEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for MapStateEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for MapStateEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for MapStateEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetWindowAttributesReply {
    pub response_type: u8,
    pub backing_store: BackingStoreEnum,
    pub sequence: u16,
    pub length: u32,
    pub visual: Visualid,
    pub class: WindowClassEnum,
    pub bit_gravity: GravityEnum,
    pub win_gravity: GravityEnum,
    pub backing_planes: u32,
    pub backing_pixel: u32,
    pub save_under: u8,
    pub map_is_installed: u8,
    pub map_state: MapStateEnum,
    pub override_redirect: u8,
    pub colormap: ColormapEnum,
    pub all_event_masks: EventMask,
    pub your_event_mask: EventMask,
    pub do_not_propagate_mask: EventMask,
}
impl FixedLengthFromBytes<44> for GetWindowAttributesReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            backing_store: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            visual: Visualid::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            class: u16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            bit_gravity: u8::from_bytes(bytes.get(14..15).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            win_gravity: u8::from_bytes(bytes.get(15..16).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            backing_planes: u32::from_bytes(
                bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?,
            )?,
            backing_pixel: u32::from_bytes(
                bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?,
            )?,
            save_under: u8::from_bytes(bytes.get(24..25).ok_or(crate::error::Error::FromBytes)?)?,
            map_is_installed: u8::from_bytes(
                bytes.get(25..26).ok_or(crate::error::Error::FromBytes)?,
            )?,
            map_state: u8::from_bytes(bytes.get(26..27).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            override_redirect: u8::from_bytes(
                bytes.get(27..28).ok_or(crate::error::Error::FromBytes)?,
            )?,
            colormap: Colormap::from_bytes(
                bytes.get(28..32).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            all_event_masks: u32::from_bytes(
                bytes.get(32..36).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            your_event_mask: u32::from_bytes(
                bytes.get(36..40).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            do_not_propagate_mask: u16::from_bytes(
                bytes.get(40..42).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SetModeEnum(pub u8);
impl SetModeEnum {
    pub const INSERT: Self = Self(0);
    pub const DELETE: Self = Self(1);
}
impl FixedLengthSerialize<1> for SetModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SetModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for SetModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SetModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SetModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ConfigWindow(pub u16);
impl ConfigWindow {
    pub const X: Self = Self(1 << 0);
    pub const Y: Self = Self(1 << 1);
    pub const WIDTH: Self = Self(1 << 2);
    pub const HEIGHT: Self = Self(1 << 3);
    pub const BORDER_WIDTH: Self = Self(1 << 4);
    pub const SIBLING: Self = Self(1 << 5);
    pub const STACK_MODE: Self = Self(1 << 6);
}
impl FixedLengthSerialize<2> for ConfigWindow {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for ConfigWindow {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for ConfigWindow {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for ConfigWindow {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for ConfigWindow {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
crate::implement_bit_ops!(ConfigWindow);
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct StackModeEnum(pub u32);
impl StackModeEnum {
    pub const ABOVE: Self = Self(0);
    pub const BELOW: Self = Self(1);
    pub const TOP_IF: Self = Self(2);
    pub const BOTTOM_IF: Self = Self(3);
    pub const OPPOSITE: Self = Self(4);
}
impl FixedLengthSerialize<4> for StackModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for StackModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for StackModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for StackModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for StackModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct ConfigureWindowValueList {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub border_width: Option<u32>,
    pub sibling: Option<WindowEnum>,
    pub stack_mode: Option<StackModeEnum>,
}
impl ConfigureWindowValueList {
    #[inline]
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let mut offset = 0;
        let buf_ptr = buf;
        if let Some(x) = self.x {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&x.serialize_fixed());
            offset += 4;
        }
        if let Some(y) = self.y {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&y.serialize_fixed());
            offset += 4;
        }
        if let Some(width) = self.width {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&width.serialize_fixed());
            offset += 4;
        }
        if let Some(height) = self.height {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&height.serialize_fixed());
            offset += 4;
        }
        if let Some(border_width) = self.border_width {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&border_width.serialize_fixed());
            offset += 4;
        }
        if let Some(sibling) = self.sibling {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&sibling.serialize_fixed());
            offset += 4;
        }
        if let Some(stack_mode) = self.stack_mode {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&stack_mode.serialize_fixed());
            offset += 4;
        }
        Ok(offset)
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> ConfigWindow {
        let mut mask = ConfigWindow::default();
        if self.x.is_some() {
            mask |= ConfigWindow::X;
        }
        if self.y.is_some() {
            mask |= ConfigWindow::Y;
        }
        if self.width.is_some() {
            mask |= ConfigWindow::WIDTH;
        }
        if self.height.is_some() {
            mask |= ConfigWindow::HEIGHT;
        }
        if self.border_width.is_some() {
            mask |= ConfigWindow::BORDER_WIDTH;
        }
        if self.sibling.is_some() {
            mask |= ConfigWindow::SIBLING;
        }
        if self.stack_mode.is_some() {
            mask |= ConfigWindow::STACK_MODE;
        }
        mask
    }

    #[inline]
    pub fn x(mut self, x: i32) -> Self {
        self.x = Some(x);
        self
    }

    #[inline]
    pub fn y(mut self, y: i32) -> Self {
        self.y = Some(y);
        self
    }

    #[inline]
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    #[inline]
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    #[inline]
    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    #[inline]
    pub fn sibling(mut self, sibling: WindowEnum) -> Self {
        self.sibling = Some(sibling);
        self
    }

    #[inline]
    pub fn stack_mode(mut self, stack_mode: StackModeEnum) -> Self {
        self.stack_mode = Some(stack_mode);
        self
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct CirculateEnum(pub u8);
impl CirculateEnum {
    pub const RAISE_LOWEST: Self = Self(0);
    pub const LOWER_HIGHEST: Self = Self(1);
}
impl FixedLengthSerialize<1> for CirculateEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for CirculateEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for CirculateEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for CirculateEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for CirculateEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetGeometryReply {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: Window,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
}
impl FixedLengthFromBytes<24> for GetGeometryReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            depth: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            root: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            x: i16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
            y: i16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
            width: u16::from_bytes(bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(18..20).ok_or(crate::error::Error::FromBytes)?)?,
            border_width: u16::from_bytes(
                bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryTreeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: Window,
    pub parent: WindowEnum,
    pub children: alloc::vec::Vec<Window>,
}
impl VariableLengthFromBytes for QueryTreeReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 14 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let root = Window::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let parent = Window::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let children_len = u16::from_bytes(ptr.get(16..18).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = children_len as usize;
        let children: alloc::vec::Vec<Window> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Window,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                root,
                parent: parent.into(),
                children,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct InternAtomReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atom: AtomEnum,
}
impl FixedLengthFromBytes<12> for InternAtomReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            atom: Atom::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetAtomNameReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub name: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetAtomNameReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let name_len = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = name_len as usize;
        let name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                name,
            },
            offset,
        ))
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PropModeEnum(pub u8);
impl PropModeEnum {
    pub const REPLACE: Self = Self(0);
    pub const PREPEND: Self = Self(1);
    pub const APPEND: Self = Self(2);
}
impl FixedLengthSerialize<1> for PropModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for PropModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for PropModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for PropModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for PropModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct GetPropertyTypeEnum(pub Atom);
impl GetPropertyTypeEnum {
    pub const ANY: Self = Self(0);
}
impl FixedLengthSerialize<4> for GetPropertyTypeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for GetPropertyTypeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(Atom::from_bytes(bytes)?))
    }
}
impl From<u32> for GetPropertyTypeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(Atom::from(val as u32))
    }
}
impl From<u16> for GetPropertyTypeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(Atom::from(val as u32))
    }
}
impl From<u8> for GetPropertyTypeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(Atom::from(val as u32))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetPropertyReply {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub length: u32,
    pub r#type: Atom,
    pub bytes_after: u32,
    pub value: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetPropertyReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let format = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let r#type = Atom::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let bytes_after = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let value_len = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(
            value_len as u32,
            core::ops::Div::div(format as u32, 8u32 as u32) as u32,
        ) as usize;
        let value: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                format,
                sequence,
                length,
                r#type,
                bytes_after,
                value,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListPropertiesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atoms: alloc::vec::Vec<Atom>,
}
impl VariableLengthFromBytes for ListPropertiesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let atoms_len = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = atoms_len as usize;
        let atoms: alloc::vec::Vec<Atom> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Atom,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                atoms,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetSelectionOwnerReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub owner: WindowEnum,
}
impl FixedLengthFromBytes<12> for GetSelectionOwnerReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            owner: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SendEventDestEnum(pub Window);
impl SendEventDestEnum {
    pub const POINTER_WINDOW: Self = Self(0);
    pub const ITEM_FOCUS: Self = Self(1);
}
impl FixedLengthSerialize<4> for SendEventDestEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for SendEventDestEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(Window::from_bytes(bytes)?))
    }
}
impl From<u32> for SendEventDestEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(Window::from(val as u32))
    }
}
impl From<u16> for SendEventDestEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(Window::from(val as u32))
    }
}
impl From<u8> for SendEventDestEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(Window::from(val as u32))
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct GrabModeEnum(pub u8);
impl GrabModeEnum {
    pub const SYNC: Self = Self(0);
    pub const ASYNC: Self = Self(1);
}
impl FixedLengthSerialize<1> for GrabModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for GrabModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for GrabModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for GrabModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for GrabModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct GrabStatusEnum(pub u8);
impl GrabStatusEnum {
    pub const SUCCESS: Self = Self(0);
    pub const ALREADY_GRABBED: Self = Self(1);
    pub const INVALID_TIME: Self = Self(2);
    pub const NOT_VIEWABLE: Self = Self(3);
    pub const FROZEN: Self = Self(4);
}
impl FixedLengthSerialize<1> for GrabStatusEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for GrabStatusEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for GrabStatusEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for GrabStatusEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for GrabStatusEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct CursorEnum(pub Cursor);
impl CursorEnum {
    pub const NONE: Self = Self(0);
}
impl FixedLengthSerialize<4> for CursorEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for CursorEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(Cursor::from_bytes(bytes)?))
    }
}
impl From<u32> for CursorEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(Cursor::from(val as u32))
    }
}
impl From<u16> for CursorEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(Cursor::from(val as u32))
    }
}
impl From<u8> for CursorEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(Cursor::from(val as u32))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GrabPointerReply {
    pub response_type: u8,
    pub status: GrabStatusEnum,
    pub sequence: u16,
    pub length: u32,
}
impl FixedLengthFromBytes<8> for GrabPointerReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ButtonIndexEnum(pub u8);
impl ButtonIndexEnum {
    pub const ANY: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const TWO: Self = Self(2);
    pub const THREE: Self = Self(3);
    pub const FOUR: Self = Self(4);
    pub const FIVE: Self = Self(5);
}
impl FixedLengthSerialize<1> for ButtonIndexEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ButtonIndexEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ButtonIndexEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ButtonIndexEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ButtonIndexEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GrabKeyboardReply {
    pub response_type: u8,
    pub status: GrabStatusEnum,
    pub sequence: u16,
    pub length: u32,
}
impl FixedLengthFromBytes<8> for GrabKeyboardReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct GrabEnum(pub Keycode);
impl GrabEnum {
    pub const ANY: Self = Self(0);
}
impl FixedLengthSerialize<1> for GrabEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for GrabEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(Keycode::from_bytes(bytes)?))
    }
}
impl From<u32> for GrabEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for GrabEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for GrabEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct AllowEnum(pub u8);
impl AllowEnum {
    pub const ASYNC_POINTER: Self = Self(0);
    pub const SYNC_POINTER: Self = Self(1);
    pub const REPLAY_POINTER: Self = Self(2);
    pub const ASYNC_KEYBOARD: Self = Self(3);
    pub const SYNC_KEYBOARD: Self = Self(4);
    pub const REPLAY_KEYBOARD: Self = Self(5);
    pub const ASYNC_BOTH: Self = Self(6);
    pub const SYNC_BOTH: Self = Self(7);
}
impl FixedLengthSerialize<1> for AllowEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for AllowEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for AllowEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for AllowEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for AllowEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryPointerReply {
    pub response_type: u8,
    pub same_screen: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: Window,
    pub child: WindowEnum,
    pub root_x: i16,
    pub root_y: i16,
    pub win_x: i16,
    pub win_y: i16,
    pub mask: KeyButMask,
}
impl FixedLengthFromBytes<28> for QueryPointerReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            same_screen: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            root: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            child: Window::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            root_x: i16::from_bytes(bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?)?,
            root_y: i16::from_bytes(bytes.get(18..20).ok_or(crate::error::Error::FromBytes)?)?,
            win_x: i16::from_bytes(bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?)?,
            win_y: i16::from_bytes(bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?)?,
            mask: u16::from_bytes(bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Timecoord {
    pub time: Timestamp,
    pub x: i16,
    pub y: i16,
}
impl FixedLengthSerialize<8> for Timecoord {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let time_bytes = self.time.serialize_fixed();
        let x_bytes = self.x.serialize_fixed();
        let y_bytes = self.y.serialize_fixed();
        [
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            x_bytes[0],
            x_bytes[1],
            y_bytes[0],
            y_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<8> for Timecoord {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            time: Timestamp::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            x: i16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            y: i16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetMotionEventsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub events: alloc::vec::Vec<Timecoord>,
}
impl VariableLengthFromBytes for GetMotionEventsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let events_len = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = events_len as usize;
        let events: alloc::vec::Vec<Timecoord> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Timecoord,
            length_expr,
            8
        );
        let offset = length_expr * 8 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                events,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct TranslateCoordinatesReply {
    pub response_type: u8,
    pub same_screen: u8,
    pub sequence: u16,
    pub length: u32,
    pub child: WindowEnum,
    pub dst_x: i16,
    pub dst_y: i16,
}
impl FixedLengthFromBytes<16> for TranslateCoordinatesReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            same_screen: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            child: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            dst_x: i16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
            dst_y: i16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct InputFocusEnum(pub Window);
impl InputFocusEnum {
    pub const NONE: Self = Self(0);
    pub const POINTER_ROOT: Self = Self(1);
    pub const PARENT: Self = Self(2);
    pub const FOLLOW_KEYBOARD: Self = Self(3);
}
impl FixedLengthSerialize<4> for InputFocusEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for InputFocusEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(Window::from_bytes(bytes)?))
    }
}
impl From<u32> for InputFocusEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(Window::from(val as u32))
    }
}
impl From<u16> for InputFocusEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(Window::from(val as u32))
    }
}
impl From<u8> for InputFocusEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(Window::from(val as u32))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetInputFocusReply {
    pub response_type: u8,
    pub revert_to: InputFocusEnum,
    pub sequence: u16,
    pub length: u32,
    pub focus: Window,
}
impl FixedLengthFromBytes<12> for GetInputFocusReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            revert_to: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            focus: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryKeymapReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub keys: [u8; 32],
}
impl FixedLengthFromBytes<40> for QueryKeymapReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            // SAFETY: We know we can try into exact size slice
            keys: unsafe {
                bytes
                    .get(8..8 + 32)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct FontDrawEnum(pub u8);
impl FontDrawEnum {
    pub const LEFT_TO_RIGHT: Self = Self(0);
    pub const RIGHT_TO_LEFT: Self = Self(1);
}
impl FixedLengthSerialize<1> for FontDrawEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for FontDrawEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for FontDrawEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for FontDrawEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for FontDrawEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Fontprop {
    pub name: Atom,
    pub value: u32,
}
impl FixedLengthSerialize<8> for Fontprop {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let name_bytes = self.name.serialize_fixed();
        let value_bytes = self.value.serialize_fixed();
        [
            name_bytes[0],
            name_bytes[1],
            name_bytes[2],
            name_bytes[3],
            value_bytes[0],
            value_bytes[1],
            value_bytes[2],
            value_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for Fontprop {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            name: Atom::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            value: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Charinfo {
    pub left_side_bearing: i16,
    pub right_side_bearing: i16,
    pub character_width: i16,
    pub ascent: i16,
    pub descent: i16,
    pub attributes: u16,
}
impl FixedLengthSerialize<12> for Charinfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let left_side_bearing_bytes = self.left_side_bearing.serialize_fixed();
        let right_side_bearing_bytes = self.right_side_bearing.serialize_fixed();
        let character_width_bytes = self.character_width.serialize_fixed();
        let ascent_bytes = self.ascent.serialize_fixed();
        let descent_bytes = self.descent.serialize_fixed();
        let attributes_bytes = self.attributes.serialize_fixed();
        [
            left_side_bearing_bytes[0],
            left_side_bearing_bytes[1],
            right_side_bearing_bytes[0],
            right_side_bearing_bytes[1],
            character_width_bytes[0],
            character_width_bytes[1],
            ascent_bytes[0],
            ascent_bytes[1],
            descent_bytes[0],
            descent_bytes[1],
            attributes_bytes[0],
            attributes_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<12> for Charinfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            left_side_bearing: i16::from_bytes(
                bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?,
            )?,
            right_side_bearing: i16::from_bytes(
                bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            character_width: i16::from_bytes(
                bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?,
            )?,
            ascent: i16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
            descent: i16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            attributes: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryFontReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_bounds: Charinfo,
    pub max_bounds: Charinfo,
    pub min_char_or_byte2: u16,
    pub max_char_or_byte2: u16,
    pub default_char: u16,
    pub draw_direction: FontDrawEnum,
    pub min_byte1: u8,
    pub max_byte1: u8,
    pub all_chars_exist: u8,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub properties: alloc::vec::Vec<Fontprop>,
    pub char_infos: alloc::vec::Vec<Charinfo>,
}
impl VariableLengthFromBytes for QueryFontReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 4 bytes
        // Padding 4 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let min_bounds =
            Charinfo::from_bytes(ptr.get(8..20).ok_or(crate::error::Error::FromBytes)?)?;
        let max_bounds =
            Charinfo::from_bytes(ptr.get(24..36).ok_or(crate::error::Error::FromBytes)?)?;
        let min_char_or_byte2 =
            u16::from_bytes(ptr.get(40..42).ok_or(crate::error::Error::FromBytes)?)?;
        let max_char_or_byte2 =
            u16::from_bytes(ptr.get(42..44).ok_or(crate::error::Error::FromBytes)?)?;
        let default_char = u16::from_bytes(ptr.get(44..46).ok_or(crate::error::Error::FromBytes)?)?;
        let properties_len =
            u16::from_bytes(ptr.get(46..48).ok_or(crate::error::Error::FromBytes)?)?;
        let draw_direction =
            u8::from_bytes(ptr.get(48..49).ok_or(crate::error::Error::FromBytes)?)?;
        let min_byte1 = u8::from_bytes(ptr.get(49..50).ok_or(crate::error::Error::FromBytes)?)?;
        let max_byte1 = u8::from_bytes(ptr.get(50..51).ok_or(crate::error::Error::FromBytes)?)?;
        let all_chars_exist =
            u8::from_bytes(ptr.get(51..52).ok_or(crate::error::Error::FromBytes)?)?;
        let font_ascent = i16::from_bytes(ptr.get(52..54).ok_or(crate::error::Error::FromBytes)?)?;
        let font_descent = i16::from_bytes(ptr.get(54..56).ok_or(crate::error::Error::FromBytes)?)?;
        let char_infos_len =
            u32::from_bytes(ptr.get(56..60).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = properties_len as usize;
        let properties: alloc::vec::Vec<Fontprop> = crate::vec_from_bytes_fixed!(
            ptr.get(60..).ok_or(crate::error::Error::FromBytes)?,
            Fontprop,
            length_expr,
            8
        );
        let mut offset = length_expr * 8 + 60;
        let length_expr = char_infos_len as usize;
        let char_infos: alloc::vec::Vec<Charinfo> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Charinfo,
            length_expr,
            12
        );
        offset += length_expr * 12;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                min_bounds,
                max_bounds,
                min_char_or_byte2,
                max_char_or_byte2,
                default_char,
                draw_direction: draw_direction.into(),
                min_byte1,
                max_byte1,
                all_chars_exist,
                font_ascent,
                font_descent,
                properties,
                char_infos,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryTextExtentsReply {
    pub response_type: u8,
    pub draw_direction: FontDrawEnum,
    pub sequence: u16,
    pub length: u32,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub overall_ascent: i16,
    pub overall_descent: i16,
    pub overall_width: i32,
    pub overall_left: i32,
    pub overall_right: i32,
}
impl FixedLengthFromBytes<28> for QueryTextExtentsReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            draw_direction: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            font_ascent: i16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            font_descent: i16::from_bytes(
                bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            overall_ascent: i16::from_bytes(
                bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?,
            )?,
            overall_descent: i16::from_bytes(
                bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            overall_width: i32::from_bytes(
                bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?,
            )?,
            overall_left: i32::from_bytes(
                bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?,
            )?,
            overall_right: i32::from_bytes(
                bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Str {
    pub name: alloc::vec::Vec<u8>,
}
impl VariableLengthSerialize for Str {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let name_len = u8::try_from(self.name.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(name_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(1..).ok_or(crate::error::Error::Serialize)?,
            &self.name,
        )?;
        let offset = list_len + 1;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for Str {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let name_len = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = name_len as usize;
        let name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(1..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 1;
        Ok((Self { name }, offset))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListFontsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub names: alloc::vec::Vec<Str>,
}
impl VariableLengthFromBytes for ListFontsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let names_len = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let names_length = names_len as usize;
        let mut offset = 32;
        let names = crate::vec_from_bytes_var!(ptr, Str, offset, names_length,);
        Ok((
            Self {
                response_type,
                sequence,
                length,
                names,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListFontsWithInfoReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_bounds: Charinfo,
    pub max_bounds: Charinfo,
    pub min_char_or_byte2: u16,
    pub max_char_or_byte2: u16,
    pub default_char: u16,
    pub draw_direction: FontDrawEnum,
    pub min_byte1: u8,
    pub max_byte1: u8,
    pub all_chars_exist: u8,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub replies_hint: u32,
    pub properties: alloc::vec::Vec<Fontprop>,
    pub name: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for ListFontsWithInfoReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 4 bytes
        // Padding 4 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let name_len = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let min_bounds =
            Charinfo::from_bytes(ptr.get(8..20).ok_or(crate::error::Error::FromBytes)?)?;
        let max_bounds =
            Charinfo::from_bytes(ptr.get(24..36).ok_or(crate::error::Error::FromBytes)?)?;
        let min_char_or_byte2 =
            u16::from_bytes(ptr.get(40..42).ok_or(crate::error::Error::FromBytes)?)?;
        let max_char_or_byte2 =
            u16::from_bytes(ptr.get(42..44).ok_or(crate::error::Error::FromBytes)?)?;
        let default_char = u16::from_bytes(ptr.get(44..46).ok_or(crate::error::Error::FromBytes)?)?;
        let properties_len =
            u16::from_bytes(ptr.get(46..48).ok_or(crate::error::Error::FromBytes)?)?;
        let draw_direction =
            u8::from_bytes(ptr.get(48..49).ok_or(crate::error::Error::FromBytes)?)?;
        let min_byte1 = u8::from_bytes(ptr.get(49..50).ok_or(crate::error::Error::FromBytes)?)?;
        let max_byte1 = u8::from_bytes(ptr.get(50..51).ok_or(crate::error::Error::FromBytes)?)?;
        let all_chars_exist =
            u8::from_bytes(ptr.get(51..52).ok_or(crate::error::Error::FromBytes)?)?;
        let font_ascent = i16::from_bytes(ptr.get(52..54).ok_or(crate::error::Error::FromBytes)?)?;
        let font_descent = i16::from_bytes(ptr.get(54..56).ok_or(crate::error::Error::FromBytes)?)?;
        let replies_hint = u32::from_bytes(ptr.get(56..60).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = properties_len as usize;
        let properties: alloc::vec::Vec<Fontprop> = crate::vec_from_bytes_fixed!(
            ptr.get(60..).ok_or(crate::error::Error::FromBytes)?,
            Fontprop,
            length_expr,
            8
        );
        let mut offset = length_expr * 8 + 60;
        let length_expr = name_len as usize;
        let name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        offset += length_expr;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                min_bounds,
                max_bounds,
                min_char_or_byte2,
                max_char_or_byte2,
                default_char,
                draw_direction: draw_direction.into(),
                min_byte1,
                max_byte1,
                all_chars_exist,
                font_ascent,
                font_descent,
                replies_hint,
                properties,
                name,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetFontPathReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub path: alloc::vec::Vec<Str>,
}
impl VariableLengthFromBytes for GetFontPathReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let path_len = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let path_length = path_len as usize;
        let mut offset = 32;
        let path = crate::vec_from_bytes_var!(ptr, Str, offset, path_length,);
        Ok((
            Self {
                response_type,
                sequence,
                length,
                path,
            },
            offset,
        ))
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Gc(pub u32);
impl Gc {
    pub const FUNCTION: Self = Self(1 << 0);
    pub const PLANE_MASK: Self = Self(1 << 1);
    pub const FOREGROUND: Self = Self(1 << 2);
    pub const BACKGROUND: Self = Self(1 << 3);
    pub const LINE_WIDTH: Self = Self(1 << 4);
    pub const LINE_STYLE: Self = Self(1 << 5);
    pub const CAP_STYLE: Self = Self(1 << 6);
    pub const JOIN_STYLE: Self = Self(1 << 7);
    pub const FILL_STYLE: Self = Self(1 << 8);
    pub const FILL_RULE: Self = Self(1 << 9);
    pub const TILE: Self = Self(1 << 10);
    pub const STIPPLE: Self = Self(1 << 11);
    pub const TILE_STIPPLE_ORIGIN_X: Self = Self(1 << 12);
    pub const TILE_STIPPLE_ORIGIN_Y: Self = Self(1 << 13);
    pub const FONT: Self = Self(1 << 14);
    pub const SUBWINDOW_MODE: Self = Self(1 << 15);
    pub const GRAPHICS_EXPOSURES: Self = Self(1 << 16);
    pub const CLIP_ORIGIN_X: Self = Self(1 << 17);
    pub const CLIP_ORIGIN_Y: Self = Self(1 << 18);
    pub const CLIP_MASK: Self = Self(1 << 19);
    pub const DASH_OFFSET: Self = Self(1 << 20);
    pub const DASH_LIST: Self = Self(1 << 21);
    pub const ARC_MODE: Self = Self(1 << 22);
}
impl FixedLengthSerialize<4> for Gc {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for Gc {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for Gc {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for Gc {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for Gc {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(Gc);
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct GxEnum(pub u32);
impl GxEnum {
    pub const CLEAR: Self = Self(0);
    pub const AND: Self = Self(1);
    pub const AND_REVERSE: Self = Self(2);
    pub const COPY: Self = Self(3);
    pub const AND_INVERTED: Self = Self(4);
    pub const NOOP: Self = Self(5);
    pub const XOR: Self = Self(6);
    pub const OR: Self = Self(7);
    pub const NOR: Self = Self(8);
    pub const EQUIV: Self = Self(9);
    pub const INVERT: Self = Self(10);
    pub const OR_REVERSE: Self = Self(11);
    pub const COPY_INVERTED: Self = Self(12);
    pub const OR_INVERTED: Self = Self(13);
    pub const NAND: Self = Self(14);
    pub const SET: Self = Self(15);
}
impl FixedLengthSerialize<4> for GxEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for GxEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for GxEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for GxEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for GxEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct LineStyleEnum(pub u32);
impl LineStyleEnum {
    pub const SOLID: Self = Self(0);
    pub const ON_OFF_DASH: Self = Self(1);
    pub const DOUBLE_DASH: Self = Self(2);
}
impl FixedLengthSerialize<4> for LineStyleEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for LineStyleEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for LineStyleEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for LineStyleEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for LineStyleEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct CapStyleEnum(pub u32);
impl CapStyleEnum {
    pub const NOT_LAST: Self = Self(0);
    pub const BUTT: Self = Self(1);
    pub const ROUND: Self = Self(2);
    pub const PROJECTING: Self = Self(3);
}
impl FixedLengthSerialize<4> for CapStyleEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for CapStyleEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for CapStyleEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for CapStyleEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for CapStyleEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct JoinStyleEnum(pub u32);
impl JoinStyleEnum {
    pub const MITER: Self = Self(0);
    pub const ROUND: Self = Self(1);
    pub const BEVEL: Self = Self(2);
}
impl FixedLengthSerialize<4> for JoinStyleEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for JoinStyleEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for JoinStyleEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for JoinStyleEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for JoinStyleEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct FillStyleEnum(pub u32);
impl FillStyleEnum {
    pub const SOLID: Self = Self(0);
    pub const TILED: Self = Self(1);
    pub const STIPPLED: Self = Self(2);
    pub const OPAQUE_STIPPLED: Self = Self(3);
}
impl FixedLengthSerialize<4> for FillStyleEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for FillStyleEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for FillStyleEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for FillStyleEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for FillStyleEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct FillRuleEnum(pub u32);
impl FillRuleEnum {
    pub const EVEN_ODD: Self = Self(0);
    pub const WINDING: Self = Self(1);
}
impl FixedLengthSerialize<4> for FillRuleEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for FillRuleEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for FillRuleEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for FillRuleEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for FillRuleEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SubwindowModeEnum(pub u32);
impl SubwindowModeEnum {
    pub const CLIP_BY_CHILDREN: Self = Self(0);
    pub const INCLUDE_INFERIORS: Self = Self(1);
}
impl FixedLengthSerialize<4> for SubwindowModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for SubwindowModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for SubwindowModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for SubwindowModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for SubwindowModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ArcModeEnum(pub u32);
impl ArcModeEnum {
    pub const CHORD: Self = Self(0);
    pub const PIE_SLICE: Self = Self(1);
}
impl FixedLengthSerialize<4> for ArcModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for ArcModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for ArcModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for ArcModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for ArcModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ClipOrderingEnum(pub u8);
impl ClipOrderingEnum {
    pub const UNSORTED: Self = Self(0);
    pub const Y_SORTED: Self = Self(1);
    pub const Y_X_SORTED: Self = Self(2);
    pub const Y_X_BANDED: Self = Self(3);
}
impl FixedLengthSerialize<1> for ClipOrderingEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ClipOrderingEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ClipOrderingEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ClipOrderingEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ClipOrderingEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct CoordModeEnum(pub u8);
impl CoordModeEnum {
    pub const ORIGIN: Self = Self(0);
    pub const PREVIOUS: Self = Self(1);
}
impl FixedLengthSerialize<1> for CoordModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for CoordModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for CoordModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for CoordModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for CoordModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Segment {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub y2: i16,
}
impl FixedLengthSerialize<8> for Segment {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let x1_bytes = self.x1.serialize_fixed();
        let y1_bytes = self.y1.serialize_fixed();
        let x2_bytes = self.x2.serialize_fixed();
        let y2_bytes = self.y2.serialize_fixed();
        [
            x1_bytes[0],
            x1_bytes[1],
            y1_bytes[0],
            y1_bytes[1],
            x2_bytes[0],
            x2_bytes[1],
            y2_bytes[0],
            y2_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<8> for Segment {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            x1: i16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            y1: i16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            x2: i16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            y2: i16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PolyShapeEnum(pub u8);
impl PolyShapeEnum {
    pub const COMPLEX: Self = Self(0);
    pub const NONCONVEX: Self = Self(1);
    pub const CONVEX: Self = Self(2);
}
impl FixedLengthSerialize<1> for PolyShapeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for PolyShapeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for PolyShapeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for PolyShapeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for PolyShapeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ImageFormatEnum(pub u8);
impl ImageFormatEnum {
    pub const X_Y_BITMAP: Self = Self(0);
    pub const X_Y_PIXMAP: Self = Self(1);
    pub const Z_PIXMAP: Self = Self(2);
}
impl FixedLengthSerialize<1> for ImageFormatEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ImageFormatEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ImageFormatEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ImageFormatEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ImageFormatEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetImageReply {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub visual: Visualid,
    pub data: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetImageReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let depth = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let visual = Visualid::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(length, 4) as usize;
        let data: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                depth,
                sequence,
                length,
                visual,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ColormapAllocEnum(pub u8);
impl ColormapAllocEnum {
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self(1);
}
impl FixedLengthSerialize<1> for ColormapAllocEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ColormapAllocEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ColormapAllocEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ColormapAllocEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ColormapAllocEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListInstalledColormapsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub cmaps: alloc::vec::Vec<Colormap>,
}
impl VariableLengthFromBytes for ListInstalledColormapsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let cmaps_len = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = cmaps_len as usize;
        let cmaps: alloc::vec::Vec<Colormap> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Colormap,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                cmaps,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct AllocColorReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub pixel: u32,
}
impl FixedLengthFromBytes<20> for AllocColorReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            red: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            green: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
            blue: u16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
            pixel: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct AllocNamedColorReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub pixel: u32,
    pub exact_red: u16,
    pub exact_green: u16,
    pub exact_blue: u16,
    pub visual_red: u16,
    pub visual_green: u16,
    pub visual_blue: u16,
}
impl FixedLengthFromBytes<24> for AllocNamedColorReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            pixel: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            exact_red: u16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
            exact_green: u16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
            exact_blue: u16::from_bytes(bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?)?,
            visual_red: u16::from_bytes(bytes.get(18..20).ok_or(crate::error::Error::FromBytes)?)?,
            visual_green: u16::from_bytes(
                bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?,
            )?,
            visual_blue: u16::from_bytes(bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AllocColorCellsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub pixels: alloc::vec::Vec<u32>,
    pub masks: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for AllocColorCellsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let pixels_len = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let masks_len = u16::from_bytes(ptr.get(10..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = pixels_len as usize;
        let pixels: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 32;
        let length_expr = masks_len as usize;
        let masks: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                pixels,
                masks,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AllocColorPlanesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub pixels: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for AllocColorPlanesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 2 bytes
        // Padding 8 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let pixels_len = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let red_mask = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let green_mask = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let blue_mask = u32::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = pixels_len as usize;
        let pixels: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                red_mask,
                green_mask,
                blue_mask,
                pixels,
            },
            offset,
        ))
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ColorFlag(pub u8);
impl ColorFlag {
    pub const RED: Self = Self(1 << 0);
    pub const GREEN: Self = Self(1 << 1);
    pub const BLUE: Self = Self(1 << 2);
}
impl FixedLengthSerialize<1> for ColorFlag {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ColorFlag {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ColorFlag {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ColorFlag {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ColorFlag {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(ColorFlag);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Coloritem {
    pub pixel: u32,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub flags: ColorFlag,
}
impl FixedLengthSerialize<12> for Coloritem {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let pixel_bytes = self.pixel.serialize_fixed();
        let red_bytes = self.red.serialize_fixed();
        let green_bytes = self.green.serialize_fixed();
        let blue_bytes = self.blue.serialize_fixed();
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
            self.flags.0 as u8,
            0,
        ]
    }
}
impl FixedLengthFromBytes<12> for Coloritem {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            pixel: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            red: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            green: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
            blue: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            flags: u8::from_bytes(bytes.get(10..11).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Rgb {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}
impl FixedLengthSerialize<8> for Rgb {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let red_bytes = self.red.serialize_fixed();
        let green_bytes = self.green.serialize_fixed();
        let blue_bytes = self.blue.serialize_fixed();
        [
            red_bytes[0],
            red_bytes[1],
            green_bytes[0],
            green_bytes[1],
            blue_bytes[0],
            blue_bytes[1],
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for Rgb {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            red: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            green: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            blue: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryColorsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub colors: alloc::vec::Vec<Rgb>,
}
impl VariableLengthFromBytes for QueryColorsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let colors_len = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = colors_len as usize;
        let colors: alloc::vec::Vec<Rgb> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Rgb,
            length_expr,
            8
        );
        let offset = length_expr * 8 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                colors,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct LookupColorReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub exact_red: u16,
    pub exact_green: u16,
    pub exact_blue: u16,
    pub visual_red: u16,
    pub visual_green: u16,
    pub visual_blue: u16,
}
impl FixedLengthFromBytes<20> for LookupColorReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            exact_red: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            exact_green: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
            exact_blue: u16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
            visual_red: u16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
            visual_green: u16::from_bytes(
                bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?,
            )?,
            visual_blue: u16::from_bytes(bytes.get(18..20).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PixmapEnum(pub Pixmap);
impl PixmapEnum {
    pub const NONE: Self = Self(0);
}
impl FixedLengthSerialize<4> for PixmapEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for PixmapEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(Pixmap::from_bytes(bytes)?))
    }
}
impl From<u32> for PixmapEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(Pixmap::from(val as u32))
    }
}
impl From<u16> for PixmapEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(Pixmap::from(val as u32))
    }
}
impl From<u8> for PixmapEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(Pixmap::from(val as u32))
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct FontEnum(pub Font);
impl FontEnum {
    pub const NONE: Self = Self(0);
}
impl FixedLengthSerialize<4> for FontEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for FontEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(Font::from_bytes(bytes)?))
    }
}
impl From<u32> for FontEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(Font::from(val as u32))
    }
}
impl From<u16> for FontEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(Font::from(val as u32))
    }
}
impl From<u8> for FontEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(Font::from(val as u32))
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct QueryShapeOfEnum(pub u8);
impl QueryShapeOfEnum {
    pub const LARGEST_CURSOR: Self = Self(0);
    pub const FASTEST_TILE: Self = Self(1);
    pub const FASTEST_STIPPLE: Self = Self(2);
}
impl FixedLengthSerialize<1> for QueryShapeOfEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for QueryShapeOfEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for QueryShapeOfEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for QueryShapeOfEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for QueryShapeOfEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryBestSizeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u16,
    pub height: u16,
}
impl FixedLengthFromBytes<12> for QueryBestSizeReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            width: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryExtensionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub present: u8,
    pub major_opcode: u8,
    pub first_event: u8,
    pub first_error: u8,
}
impl FixedLengthFromBytes<12> for QueryExtensionReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            present: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            major_opcode: u8::from_bytes(bytes.get(9..10).ok_or(crate::error::Error::FromBytes)?)?,
            first_event: u8::from_bytes(bytes.get(10..11).ok_or(crate::error::Error::FromBytes)?)?,
            first_error: u8::from_bytes(bytes.get(11..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListExtensionsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub names: alloc::vec::Vec<Str>,
}
impl VariableLengthFromBytes for ListExtensionsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 24 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let names_len = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let names_length = names_len as usize;
        let mut offset = 32;
        let names = crate::vec_from_bytes_var!(ptr, Str, offset, names_length,);
        Ok((
            Self {
                response_type,
                sequence,
                length,
                names,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetKeyboardMappingReply {
    pub response_type: u8,
    pub keysyms_per_keycode: u8,
    pub sequence: u16,
    pub length: u32,
    pub keysyms: alloc::vec::Vec<Keysym>,
}
impl VariableLengthFromBytes for GetKeyboardMappingReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 24 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let keysyms_per_keycode =
            u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = length as usize;
        let keysyms: alloc::vec::Vec<Keysym> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Keysym,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                keysyms_per_keycode,
                sequence,
                length,
                keysyms,
            },
            offset,
        ))
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Kb(pub u32);
impl Kb {
    pub const KEY_CLICK_PERCENT: Self = Self(1 << 0);
    pub const BELL_PERCENT: Self = Self(1 << 1);
    pub const BELL_PITCH: Self = Self(1 << 2);
    pub const BELL_DURATION: Self = Self(1 << 3);
    pub const LED: Self = Self(1 << 4);
    pub const LED_MODE: Self = Self(1 << 5);
    pub const KEY: Self = Self(1 << 6);
    pub const AUTO_REPEAT_MODE: Self = Self(1 << 7);
}
impl FixedLengthSerialize<4> for Kb {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for Kb {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for Kb {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for Kb {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for Kb {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(Kb);
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct LedModeEnum(pub u32);
impl LedModeEnum {
    pub const OFF: Self = Self(0);
    pub const ON: Self = Self(1);
}
impl FixedLengthSerialize<4> for LedModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for LedModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for LedModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for LedModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for LedModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct AutoRepeatModeEnum(pub u32);
impl AutoRepeatModeEnum {
    pub const OFF: Self = Self(0);
    pub const ON: Self = Self(1);
    pub const DEFAULT: Self = Self(2);
}
impl FixedLengthSerialize<4> for AutoRepeatModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for AutoRepeatModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for AutoRepeatModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for AutoRepeatModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for AutoRepeatModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct ChangeKeyboardControlValueList {
    pub key_click_percent: Option<i32>,
    pub bell_percent: Option<i32>,
    pub bell_pitch: Option<i32>,
    pub bell_duration: Option<i32>,
    pub led: Option<u32>,
    pub led_mode: Option<LedModeEnum>,
    pub key: Option<Keycode32>,
    pub auto_repeat_mode: Option<AutoRepeatModeEnum>,
}
impl ChangeKeyboardControlValueList {
    #[inline]
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let mut offset = 0;
        let buf_ptr = buf;
        if let Some(key_click_percent) = self.key_click_percent {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&key_click_percent.serialize_fixed());
            offset += 4;
        }
        if let Some(bell_percent) = self.bell_percent {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&bell_percent.serialize_fixed());
            offset += 4;
        }
        if let Some(bell_pitch) = self.bell_pitch {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&bell_pitch.serialize_fixed());
            offset += 4;
        }
        if let Some(bell_duration) = self.bell_duration {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&bell_duration.serialize_fixed());
            offset += 4;
        }
        if let Some(led) = self.led {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&led.serialize_fixed());
            offset += 4;
        }
        if let Some(led_mode) = self.led_mode {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&led_mode.serialize_fixed());
            offset += 4;
        }
        if let Some(key) = self.key {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&key.serialize_fixed());
            offset += 4;
        }
        if let Some(auto_repeat_mode) = self.auto_repeat_mode {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&auto_repeat_mode.serialize_fixed());
            offset += 4;
        }
        Ok(offset)
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> Kb {
        let mut mask = Kb::default();
        if self.key_click_percent.is_some() {
            mask |= Kb::KEY_CLICK_PERCENT;
        }
        if self.bell_percent.is_some() {
            mask |= Kb::BELL_PERCENT;
        }
        if self.bell_pitch.is_some() {
            mask |= Kb::BELL_PITCH;
        }
        if self.bell_duration.is_some() {
            mask |= Kb::BELL_DURATION;
        }
        if self.led.is_some() {
            mask |= Kb::LED;
        }
        if self.led_mode.is_some() {
            mask |= Kb::LED_MODE;
        }
        if self.key.is_some() {
            mask |= Kb::KEY;
        }
        if self.auto_repeat_mode.is_some() {
            mask |= Kb::AUTO_REPEAT_MODE;
        }
        mask
    }

    #[inline]
    pub fn key_click_percent(mut self, key_click_percent: i32) -> Self {
        self.key_click_percent = Some(key_click_percent);
        self
    }

    #[inline]
    pub fn bell_percent(mut self, bell_percent: i32) -> Self {
        self.bell_percent = Some(bell_percent);
        self
    }

    #[inline]
    pub fn bell_pitch(mut self, bell_pitch: i32) -> Self {
        self.bell_pitch = Some(bell_pitch);
        self
    }

    #[inline]
    pub fn bell_duration(mut self, bell_duration: i32) -> Self {
        self.bell_duration = Some(bell_duration);
        self
    }

    #[inline]
    pub fn led(mut self, led: u32) -> Self {
        self.led = Some(led);
        self
    }

    #[inline]
    pub fn led_mode(mut self, led_mode: LedModeEnum) -> Self {
        self.led_mode = Some(led_mode);
        self
    }

    #[inline]
    pub fn key(mut self, key: Keycode32) -> Self {
        self.key = Some(key);
        self
    }

    #[inline]
    pub fn auto_repeat_mode(mut self, auto_repeat_mode: AutoRepeatModeEnum) -> Self {
        self.auto_repeat_mode = Some(auto_repeat_mode);
        self
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetKeyboardControlReply {
    pub response_type: u8,
    pub global_auto_repeat: AutoRepeatModeEnum,
    pub sequence: u16,
    pub length: u32,
    pub led_mask: u32,
    pub key_click_percent: u8,
    pub bell_percent: u8,
    pub bell_pitch: u16,
    pub bell_duration: u16,
    pub auto_repeats: [u8; 32],
}
impl FixedLengthFromBytes<52> for GetKeyboardControlReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            global_auto_repeat: u8::from_bytes(
                bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            led_mask: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            key_click_percent: u8::from_bytes(
                bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?,
            )?,
            bell_percent: u8::from_bytes(bytes.get(13..14).ok_or(crate::error::Error::FromBytes)?)?,
            bell_pitch: u16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
            bell_duration: u16::from_bytes(
                bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?,
            )?,
            // SAFETY: We know we can try into exact size slice
            auto_repeats: unsafe {
                bytes
                    .get(20..20 + 32)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetPointerControlReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub acceleration_numerator: u16,
    pub acceleration_denominator: u16,
    pub threshold: u16,
}
impl FixedLengthFromBytes<32> for GetPointerControlReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            acceleration_numerator: u16::from_bytes(
                bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?,
            )?,
            acceleration_denominator: u16::from_bytes(
                bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            threshold: u16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct BlankingEnum(pub u8);
impl BlankingEnum {
    pub const NOT_PREFERRED: Self = Self(0);
    pub const PREFERRED: Self = Self(1);
    pub const DEFAULT: Self = Self(2);
}
impl FixedLengthSerialize<1> for BlankingEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for BlankingEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for BlankingEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for BlankingEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for BlankingEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ExposuresEnum(pub u8);
impl ExposuresEnum {
    pub const NOT_ALLOWED: Self = Self(0);
    pub const ALLOWED: Self = Self(1);
    pub const DEFAULT: Self = Self(2);
}
impl FixedLengthSerialize<1> for ExposuresEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ExposuresEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ExposuresEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ExposuresEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ExposuresEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetScreenSaverReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub timeout: u16,
    pub interval: u16,
    pub prefer_blanking: BlankingEnum,
    pub allow_exposures: ExposuresEnum,
}
impl FixedLengthFromBytes<32> for GetScreenSaverReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            timeout: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            interval: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
            prefer_blanking: u8::from_bytes(
                bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            allow_exposures: u8::from_bytes(
                bytes.get(13..14).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct HostModeEnum(pub u8);
impl HostModeEnum {
    pub const INSERT: Self = Self(0);
    pub const DELETE: Self = Self(1);
}
impl FixedLengthSerialize<1> for HostModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for HostModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for HostModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for HostModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for HostModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct FamilyEnum(pub u8);
impl FamilyEnum {
    pub const INTERNET: Self = Self(0);
    pub const D_E_CNET: Self = Self(1);
    pub const CHAOS: Self = Self(2);
    pub const SERVER_INTERPRETED: Self = Self(5);
    pub const INTERNET6: Self = Self(6);
}
impl FixedLengthSerialize<1> for FamilyEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for FamilyEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for FamilyEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for FamilyEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for FamilyEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Host {
    pub family: FamilyEnum,
    pub address: alloc::vec::Vec<u8>,
}
impl VariableLengthSerialize for Host {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        // Padding 1 bytes
        let address_len =
            u16::try_from(self.address.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.family.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(address_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.address.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?,
            &self.address,
        )?;
        let mut offset = list_len + 4;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for Host {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        let family = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let address_len = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = address_len as usize;
        let address: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr + 4;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok((
            Self {
                family: family.into(),
                address,
            },
            offset,
        ))
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct AccessControlEnum(pub u8);
impl AccessControlEnum {
    pub const DISABLE: Self = Self(0);
    pub const ENABLE: Self = Self(1);
}
impl FixedLengthSerialize<1> for AccessControlEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for AccessControlEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for AccessControlEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for AccessControlEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for AccessControlEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct CloseDownEnum(pub u8);
impl CloseDownEnum {
    pub const DESTROY_ALL: Self = Self(0);
    pub const RETAIN_PERMANENT: Self = Self(1);
    pub const RETAIN_TEMPORARY: Self = Self(2);
}
impl FixedLengthSerialize<1> for CloseDownEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for CloseDownEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for CloseDownEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for CloseDownEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for CloseDownEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct KillEnum(pub u32);
impl KillEnum {
    pub const ALL_TEMPORARY: Self = Self(0);
}
impl FixedLengthSerialize<4> for KillEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for KillEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for KillEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for KillEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for KillEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ScreenSaverEnum(pub u8);
impl ScreenSaverEnum {
    pub const RESET: Self = Self(0);
    pub const ACTIVE: Self = Self(1);
}
impl FixedLengthSerialize<1> for ScreenSaverEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ScreenSaverEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ScreenSaverEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ScreenSaverEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ScreenSaverEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct MappingStatusEnum(pub u8);
impl MappingStatusEnum {
    pub const SUCCESS: Self = Self(0);
    pub const BUSY: Self = Self(1);
    pub const FAILURE: Self = Self(2);
}
impl FixedLengthSerialize<1> for MappingStatusEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for MappingStatusEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for MappingStatusEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for MappingStatusEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for MappingStatusEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SetPointerMappingReply {
    pub response_type: u8,
    pub status: MappingStatusEnum,
    pub sequence: u16,
    pub length: u32,
}
impl FixedLengthFromBytes<8> for SetPointerMappingReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetPointerMappingReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub map: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetPointerMappingReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 24 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let map_len = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = map_len as usize;
        let map: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                map,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SetModifierMappingReply {
    pub response_type: u8,
    pub status: MappingStatusEnum,
    pub sequence: u16,
    pub length: u32,
}
impl FixedLengthFromBytes<8> for SetModifierMappingReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetModifierMappingReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub keycodes: alloc::vec::Vec<Keycode>,
}
impl VariableLengthFromBytes for GetModifierMappingReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 24 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let keycodes_per_modifier =
            u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(keycodes_per_modifier as u8, 8u8 as u8) as usize;
        let keycodes: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                keycodes,
            },
            offset,
        ))
    }
}
pub const CONFIGURE_REQUEST_EVENT: u8 = 23;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ConfigureRequestEvent {
    pub opcode: u8,
    pub stack_mode: StackModeEnum,
    pub sequence: u16,
    pub parent: Window,
    pub window: Window,
    pub sibling: WindowEnum,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub value_mask: ConfigWindow,
}
impl FixedLengthFromBytes<28> for ConfigureRequestEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            stack_mode: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            parent: Window::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            window: Window::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            sibling: Window::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            x: i16::from_bytes(bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?)?,
            y: i16::from_bytes(bytes.get(18..20).ok_or(crate::error::Error::FromBytes)?)?,
            width: u16::from_bytes(bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?)?,
            border_width: u16::from_bytes(
                bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?,
            )?,
            value_mask: u16::from_bytes(bytes.get(26..28).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct CreateWindowValueList {
    pub background_pixmap: Option<BackPixmapEnum>,
    pub background_pixel: Option<u32>,
    pub border_pixmap: Option<PixmapEnum>,
    pub border_pixel: Option<u32>,
    pub bit_gravity: Option<GravityEnum>,
    pub win_gravity: Option<GravityEnum>,
    pub backing_store: Option<BackingStoreEnum>,
    pub backing_planes: Option<u32>,
    pub backing_pixel: Option<u32>,
    pub override_redirect: Option<Bool32>,
    pub save_under: Option<Bool32>,
    pub event_mask: Option<EventMask>,
    pub do_not_propogate_mask: Option<EventMask>,
    pub colormap: Option<ColormapEnum>,
    pub cursor: Option<CursorEnum>,
}
impl CreateWindowValueList {
    #[inline]
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let mut offset = 0;
        let buf_ptr = buf;
        if let Some(background_pixmap) = self.background_pixmap {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&background_pixmap.serialize_fixed());
            offset += 4;
        }
        if let Some(background_pixel) = self.background_pixel {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&background_pixel.serialize_fixed());
            offset += 4;
        }
        if let Some(border_pixmap) = self.border_pixmap {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&border_pixmap.serialize_fixed());
            offset += 4;
        }
        if let Some(border_pixel) = self.border_pixel {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&border_pixel.serialize_fixed());
            offset += 4;
        }
        if let Some(bit_gravity) = self.bit_gravity {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&bit_gravity.serialize_fixed());
            offset += 4;
        }
        if let Some(win_gravity) = self.win_gravity {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&win_gravity.serialize_fixed());
            offset += 4;
        }
        if let Some(backing_store) = self.backing_store {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&backing_store.serialize_fixed());
            offset += 4;
        }
        if let Some(backing_planes) = self.backing_planes {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&backing_planes.serialize_fixed());
            offset += 4;
        }
        if let Some(backing_pixel) = self.backing_pixel {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&backing_pixel.serialize_fixed());
            offset += 4;
        }
        if let Some(override_redirect) = self.override_redirect {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&override_redirect.serialize_fixed());
            offset += 4;
        }
        if let Some(save_under) = self.save_under {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&save_under.serialize_fixed());
            offset += 4;
        }
        if let Some(event_mask) = self.event_mask {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&event_mask.serialize_fixed());
            offset += 4;
        }
        if let Some(do_not_propogate_mask) = self.do_not_propogate_mask {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&do_not_propogate_mask.serialize_fixed());
            offset += 4;
        }
        if let Some(colormap) = self.colormap {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&colormap.serialize_fixed());
            offset += 4;
        }
        if let Some(cursor) = self.cursor {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&cursor.serialize_fixed());
            offset += 4;
        }
        Ok(offset)
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> Cw {
        let mut mask = Cw::default();
        if self.background_pixmap.is_some() {
            mask |= Cw::BACK_PIXMAP;
        }
        if self.background_pixel.is_some() {
            mask |= Cw::BACK_PIXEL;
        }
        if self.border_pixmap.is_some() {
            mask |= Cw::BORDER_PIXMAP;
        }
        if self.border_pixel.is_some() {
            mask |= Cw::BORDER_PIXEL;
        }
        if self.bit_gravity.is_some() {
            mask |= Cw::BIT_GRAVITY;
        }
        if self.win_gravity.is_some() {
            mask |= Cw::WIN_GRAVITY;
        }
        if self.backing_store.is_some() {
            mask |= Cw::BACKING_STORE;
        }
        if self.backing_planes.is_some() {
            mask |= Cw::BACKING_PLANES;
        }
        if self.backing_pixel.is_some() {
            mask |= Cw::BACKING_PIXEL;
        }
        if self.override_redirect.is_some() {
            mask |= Cw::OVERRIDE_REDIRECT;
        }
        if self.save_under.is_some() {
            mask |= Cw::SAVE_UNDER;
        }
        if self.event_mask.is_some() {
            mask |= Cw::EVENT_MASK;
        }
        if self.do_not_propogate_mask.is_some() {
            mask |= Cw::DONT_PROPAGATE;
        }
        if self.colormap.is_some() {
            mask |= Cw::COLORMAP;
        }
        if self.cursor.is_some() {
            mask |= Cw::CURSOR;
        }
        mask
    }

    #[inline]
    pub fn background_pixmap(mut self, background_pixmap: BackPixmapEnum) -> Self {
        self.background_pixmap = Some(background_pixmap);
        self
    }

    #[inline]
    pub fn background_pixel(mut self, background_pixel: u32) -> Self {
        self.background_pixel = Some(background_pixel);
        self
    }

    #[inline]
    pub fn border_pixmap(mut self, border_pixmap: PixmapEnum) -> Self {
        self.border_pixmap = Some(border_pixmap);
        self
    }

    #[inline]
    pub fn border_pixel(mut self, border_pixel: u32) -> Self {
        self.border_pixel = Some(border_pixel);
        self
    }

    #[inline]
    pub fn bit_gravity(mut self, bit_gravity: GravityEnum) -> Self {
        self.bit_gravity = Some(bit_gravity);
        self
    }

    #[inline]
    pub fn win_gravity(mut self, win_gravity: GravityEnum) -> Self {
        self.win_gravity = Some(win_gravity);
        self
    }

    #[inline]
    pub fn backing_store(mut self, backing_store: BackingStoreEnum) -> Self {
        self.backing_store = Some(backing_store);
        self
    }

    #[inline]
    pub fn backing_planes(mut self, backing_planes: u32) -> Self {
        self.backing_planes = Some(backing_planes);
        self
    }

    #[inline]
    pub fn backing_pixel(mut self, backing_pixel: u32) -> Self {
        self.backing_pixel = Some(backing_pixel);
        self
    }

    #[inline]
    pub fn override_redirect(mut self, override_redirect: Bool32) -> Self {
        self.override_redirect = Some(override_redirect);
        self
    }

    #[inline]
    pub fn save_under(mut self, save_under: Bool32) -> Self {
        self.save_under = Some(save_under);
        self
    }

    #[inline]
    pub fn event_mask(mut self, event_mask: EventMask) -> Self {
        self.event_mask = Some(event_mask);
        self
    }

    #[inline]
    pub fn do_not_propogate_mask(mut self, do_not_propogate_mask: EventMask) -> Self {
        self.do_not_propogate_mask = Some(do_not_propogate_mask);
        self
    }

    #[inline]
    pub fn colormap(mut self, colormap: ColormapEnum) -> Self {
        self.colormap = Some(colormap);
        self
    }

    #[inline]
    pub fn cursor(mut self, cursor: CursorEnum) -> Self {
        self.cursor = Some(cursor);
        self
    }
}
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct ChangeWindowAttributesValueList {
    pub background_pixmap: Option<BackPixmapEnum>,
    pub background_pixel: Option<u32>,
    pub border_pixmap: Option<PixmapEnum>,
    pub border_pixel: Option<u32>,
    pub bit_gravity: Option<GravityEnum>,
    pub win_gravity: Option<GravityEnum>,
    pub backing_store: Option<BackingStoreEnum>,
    pub backing_planes: Option<u32>,
    pub backing_pixel: Option<u32>,
    pub override_redirect: Option<Bool32>,
    pub save_under: Option<Bool32>,
    pub event_mask: Option<EventMask>,
    pub do_not_propogate_mask: Option<EventMask>,
    pub colormap: Option<ColormapEnum>,
    pub cursor: Option<CursorEnum>,
}
impl ChangeWindowAttributesValueList {
    #[inline]
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let mut offset = 0;
        let buf_ptr = buf;
        if let Some(background_pixmap) = self.background_pixmap {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&background_pixmap.serialize_fixed());
            offset += 4;
        }
        if let Some(background_pixel) = self.background_pixel {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&background_pixel.serialize_fixed());
            offset += 4;
        }
        if let Some(border_pixmap) = self.border_pixmap {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&border_pixmap.serialize_fixed());
            offset += 4;
        }
        if let Some(border_pixel) = self.border_pixel {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&border_pixel.serialize_fixed());
            offset += 4;
        }
        if let Some(bit_gravity) = self.bit_gravity {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&bit_gravity.serialize_fixed());
            offset += 4;
        }
        if let Some(win_gravity) = self.win_gravity {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&win_gravity.serialize_fixed());
            offset += 4;
        }
        if let Some(backing_store) = self.backing_store {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&backing_store.serialize_fixed());
            offset += 4;
        }
        if let Some(backing_planes) = self.backing_planes {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&backing_planes.serialize_fixed());
            offset += 4;
        }
        if let Some(backing_pixel) = self.backing_pixel {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&backing_pixel.serialize_fixed());
            offset += 4;
        }
        if let Some(override_redirect) = self.override_redirect {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&override_redirect.serialize_fixed());
            offset += 4;
        }
        if let Some(save_under) = self.save_under {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&save_under.serialize_fixed());
            offset += 4;
        }
        if let Some(event_mask) = self.event_mask {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&event_mask.serialize_fixed());
            offset += 4;
        }
        if let Some(do_not_propogate_mask) = self.do_not_propogate_mask {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&do_not_propogate_mask.serialize_fixed());
            offset += 4;
        }
        if let Some(colormap) = self.colormap {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&colormap.serialize_fixed());
            offset += 4;
        }
        if let Some(cursor) = self.cursor {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&cursor.serialize_fixed());
            offset += 4;
        }
        Ok(offset)
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> Cw {
        let mut mask = Cw::default();
        if self.background_pixmap.is_some() {
            mask |= Cw::BACK_PIXMAP;
        }
        if self.background_pixel.is_some() {
            mask |= Cw::BACK_PIXEL;
        }
        if self.border_pixmap.is_some() {
            mask |= Cw::BORDER_PIXMAP;
        }
        if self.border_pixel.is_some() {
            mask |= Cw::BORDER_PIXEL;
        }
        if self.bit_gravity.is_some() {
            mask |= Cw::BIT_GRAVITY;
        }
        if self.win_gravity.is_some() {
            mask |= Cw::WIN_GRAVITY;
        }
        if self.backing_store.is_some() {
            mask |= Cw::BACKING_STORE;
        }
        if self.backing_planes.is_some() {
            mask |= Cw::BACKING_PLANES;
        }
        if self.backing_pixel.is_some() {
            mask |= Cw::BACKING_PIXEL;
        }
        if self.override_redirect.is_some() {
            mask |= Cw::OVERRIDE_REDIRECT;
        }
        if self.save_under.is_some() {
            mask |= Cw::SAVE_UNDER;
        }
        if self.event_mask.is_some() {
            mask |= Cw::EVENT_MASK;
        }
        if self.do_not_propogate_mask.is_some() {
            mask |= Cw::DONT_PROPAGATE;
        }
        if self.colormap.is_some() {
            mask |= Cw::COLORMAP;
        }
        if self.cursor.is_some() {
            mask |= Cw::CURSOR;
        }
        mask
    }

    #[inline]
    pub fn background_pixmap(mut self, background_pixmap: BackPixmapEnum) -> Self {
        self.background_pixmap = Some(background_pixmap);
        self
    }

    #[inline]
    pub fn background_pixel(mut self, background_pixel: u32) -> Self {
        self.background_pixel = Some(background_pixel);
        self
    }

    #[inline]
    pub fn border_pixmap(mut self, border_pixmap: PixmapEnum) -> Self {
        self.border_pixmap = Some(border_pixmap);
        self
    }

    #[inline]
    pub fn border_pixel(mut self, border_pixel: u32) -> Self {
        self.border_pixel = Some(border_pixel);
        self
    }

    #[inline]
    pub fn bit_gravity(mut self, bit_gravity: GravityEnum) -> Self {
        self.bit_gravity = Some(bit_gravity);
        self
    }

    #[inline]
    pub fn win_gravity(mut self, win_gravity: GravityEnum) -> Self {
        self.win_gravity = Some(win_gravity);
        self
    }

    #[inline]
    pub fn backing_store(mut self, backing_store: BackingStoreEnum) -> Self {
        self.backing_store = Some(backing_store);
        self
    }

    #[inline]
    pub fn backing_planes(mut self, backing_planes: u32) -> Self {
        self.backing_planes = Some(backing_planes);
        self
    }

    #[inline]
    pub fn backing_pixel(mut self, backing_pixel: u32) -> Self {
        self.backing_pixel = Some(backing_pixel);
        self
    }

    #[inline]
    pub fn override_redirect(mut self, override_redirect: Bool32) -> Self {
        self.override_redirect = Some(override_redirect);
        self
    }

    #[inline]
    pub fn save_under(mut self, save_under: Bool32) -> Self {
        self.save_under = Some(save_under);
        self
    }

    #[inline]
    pub fn event_mask(mut self, event_mask: EventMask) -> Self {
        self.event_mask = Some(event_mask);
        self
    }

    #[inline]
    pub fn do_not_propogate_mask(mut self, do_not_propogate_mask: EventMask) -> Self {
        self.do_not_propogate_mask = Some(do_not_propogate_mask);
        self
    }

    #[inline]
    pub fn colormap(mut self, colormap: ColormapEnum) -> Self {
        self.colormap = Some(colormap);
        self
    }

    #[inline]
    pub fn cursor(mut self, cursor: CursorEnum) -> Self {
        self.cursor = Some(cursor);
        self
    }
}
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct CreateGCValueList {
    pub function: Option<GxEnum>,
    pub plane_mask: Option<u32>,
    pub foreground: Option<u32>,
    pub background: Option<u32>,
    pub line_width: Option<u32>,
    pub line_style: Option<LineStyleEnum>,
    pub cap_style: Option<CapStyleEnum>,
    pub join_style: Option<JoinStyleEnum>,
    pub fill_style: Option<FillStyleEnum>,
    pub fill_rule: Option<FillRuleEnum>,
    pub tile: Option<PixmapEnum>,
    pub stipple: Option<PixmapEnum>,
    pub tile_stipple_x_origin: Option<i32>,
    pub tile_stipple_y_origin: Option<i32>,
    pub font: Option<FontEnum>,
    pub subwindow_mode: Option<SubwindowModeEnum>,
    pub graphics_exposures: Option<Bool32>,
    pub clip_x_origin: Option<i32>,
    pub clip_y_origin: Option<i32>,
    pub clip_mask: Option<PixmapEnum>,
    pub dash_offset: Option<u32>,
    pub dashes: Option<u32>,
    pub arc_mode: Option<ArcModeEnum>,
}
impl CreateGCValueList {
    #[inline]
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let mut offset = 0;
        let buf_ptr = buf;
        if let Some(function) = self.function {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&function.serialize_fixed());
            offset += 4;
        }
        if let Some(plane_mask) = self.plane_mask {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&plane_mask.serialize_fixed());
            offset += 4;
        }
        if let Some(foreground) = self.foreground {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&foreground.serialize_fixed());
            offset += 4;
        }
        if let Some(background) = self.background {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&background.serialize_fixed());
            offset += 4;
        }
        if let Some(line_width) = self.line_width {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&line_width.serialize_fixed());
            offset += 4;
        }
        if let Some(line_style) = self.line_style {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&line_style.serialize_fixed());
            offset += 4;
        }
        if let Some(cap_style) = self.cap_style {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&cap_style.serialize_fixed());
            offset += 4;
        }
        if let Some(join_style) = self.join_style {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&join_style.serialize_fixed());
            offset += 4;
        }
        if let Some(fill_style) = self.fill_style {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&fill_style.serialize_fixed());
            offset += 4;
        }
        if let Some(fill_rule) = self.fill_rule {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&fill_rule.serialize_fixed());
            offset += 4;
        }
        if let Some(tile) = self.tile {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&tile.serialize_fixed());
            offset += 4;
        }
        if let Some(stipple) = self.stipple {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&stipple.serialize_fixed());
            offset += 4;
        }
        if let Some(tile_stipple_x_origin) = self.tile_stipple_x_origin {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&tile_stipple_x_origin.serialize_fixed());
            offset += 4;
        }
        if let Some(tile_stipple_y_origin) = self.tile_stipple_y_origin {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&tile_stipple_y_origin.serialize_fixed());
            offset += 4;
        }
        if let Some(font) = self.font {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&font.serialize_fixed());
            offset += 4;
        }
        if let Some(subwindow_mode) = self.subwindow_mode {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&subwindow_mode.serialize_fixed());
            offset += 4;
        }
        if let Some(graphics_exposures) = self.graphics_exposures {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&graphics_exposures.serialize_fixed());
            offset += 4;
        }
        if let Some(clip_x_origin) = self.clip_x_origin {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&clip_x_origin.serialize_fixed());
            offset += 4;
        }
        if let Some(clip_y_origin) = self.clip_y_origin {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&clip_y_origin.serialize_fixed());
            offset += 4;
        }
        if let Some(clip_mask) = self.clip_mask {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&clip_mask.serialize_fixed());
            offset += 4;
        }
        if let Some(dash_offset) = self.dash_offset {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&dash_offset.serialize_fixed());
            offset += 4;
        }
        if let Some(dashes) = self.dashes {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&dashes.serialize_fixed());
            offset += 4;
        }
        if let Some(arc_mode) = self.arc_mode {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&arc_mode.serialize_fixed());
            offset += 4;
        }
        Ok(offset)
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> Gc {
        let mut mask = Gc::default();
        if self.function.is_some() {
            mask |= Gc::FUNCTION;
        }
        if self.plane_mask.is_some() {
            mask |= Gc::PLANE_MASK;
        }
        if self.foreground.is_some() {
            mask |= Gc::FOREGROUND;
        }
        if self.background.is_some() {
            mask |= Gc::BACKGROUND;
        }
        if self.line_width.is_some() {
            mask |= Gc::LINE_WIDTH;
        }
        if self.line_style.is_some() {
            mask |= Gc::LINE_STYLE;
        }
        if self.cap_style.is_some() {
            mask |= Gc::CAP_STYLE;
        }
        if self.join_style.is_some() {
            mask |= Gc::JOIN_STYLE;
        }
        if self.fill_style.is_some() {
            mask |= Gc::FILL_STYLE;
        }
        if self.fill_rule.is_some() {
            mask |= Gc::FILL_RULE;
        }
        if self.tile.is_some() {
            mask |= Gc::TILE;
        }
        if self.stipple.is_some() {
            mask |= Gc::STIPPLE;
        }
        if self.tile_stipple_x_origin.is_some() {
            mask |= Gc::TILE_STIPPLE_ORIGIN_X;
        }
        if self.tile_stipple_y_origin.is_some() {
            mask |= Gc::TILE_STIPPLE_ORIGIN_Y;
        }
        if self.font.is_some() {
            mask |= Gc::FONT;
        }
        if self.subwindow_mode.is_some() {
            mask |= Gc::SUBWINDOW_MODE;
        }
        if self.graphics_exposures.is_some() {
            mask |= Gc::GRAPHICS_EXPOSURES;
        }
        if self.clip_x_origin.is_some() {
            mask |= Gc::CLIP_ORIGIN_X;
        }
        if self.clip_y_origin.is_some() {
            mask |= Gc::CLIP_ORIGIN_Y;
        }
        if self.clip_mask.is_some() {
            mask |= Gc::CLIP_MASK;
        }
        if self.dash_offset.is_some() {
            mask |= Gc::DASH_OFFSET;
        }
        if self.dashes.is_some() {
            mask |= Gc::DASH_LIST;
        }
        if self.arc_mode.is_some() {
            mask |= Gc::ARC_MODE;
        }
        mask
    }

    #[inline]
    pub fn function(mut self, function: GxEnum) -> Self {
        self.function = Some(function);
        self
    }

    #[inline]
    pub fn plane_mask(mut self, plane_mask: u32) -> Self {
        self.plane_mask = Some(plane_mask);
        self
    }

    #[inline]
    pub fn foreground(mut self, foreground: u32) -> Self {
        self.foreground = Some(foreground);
        self
    }

    #[inline]
    pub fn background(mut self, background: u32) -> Self {
        self.background = Some(background);
        self
    }

    #[inline]
    pub fn line_width(mut self, line_width: u32) -> Self {
        self.line_width = Some(line_width);
        self
    }

    #[inline]
    pub fn line_style(mut self, line_style: LineStyleEnum) -> Self {
        self.line_style = Some(line_style);
        self
    }

    #[inline]
    pub fn cap_style(mut self, cap_style: CapStyleEnum) -> Self {
        self.cap_style = Some(cap_style);
        self
    }

    #[inline]
    pub fn join_style(mut self, join_style: JoinStyleEnum) -> Self {
        self.join_style = Some(join_style);
        self
    }

    #[inline]
    pub fn fill_style(mut self, fill_style: FillStyleEnum) -> Self {
        self.fill_style = Some(fill_style);
        self
    }

    #[inline]
    pub fn fill_rule(mut self, fill_rule: FillRuleEnum) -> Self {
        self.fill_rule = Some(fill_rule);
        self
    }

    #[inline]
    pub fn tile(mut self, tile: PixmapEnum) -> Self {
        self.tile = Some(tile);
        self
    }

    #[inline]
    pub fn stipple(mut self, stipple: PixmapEnum) -> Self {
        self.stipple = Some(stipple);
        self
    }

    #[inline]
    pub fn tile_stipple_x_origin(mut self, tile_stipple_x_origin: i32) -> Self {
        self.tile_stipple_x_origin = Some(tile_stipple_x_origin);
        self
    }

    #[inline]
    pub fn tile_stipple_y_origin(mut self, tile_stipple_y_origin: i32) -> Self {
        self.tile_stipple_y_origin = Some(tile_stipple_y_origin);
        self
    }

    #[inline]
    pub fn font(mut self, font: FontEnum) -> Self {
        self.font = Some(font);
        self
    }

    #[inline]
    pub fn subwindow_mode(mut self, subwindow_mode: SubwindowModeEnum) -> Self {
        self.subwindow_mode = Some(subwindow_mode);
        self
    }

    #[inline]
    pub fn graphics_exposures(mut self, graphics_exposures: Bool32) -> Self {
        self.graphics_exposures = Some(graphics_exposures);
        self
    }

    #[inline]
    pub fn clip_x_origin(mut self, clip_x_origin: i32) -> Self {
        self.clip_x_origin = Some(clip_x_origin);
        self
    }

    #[inline]
    pub fn clip_y_origin(mut self, clip_y_origin: i32) -> Self {
        self.clip_y_origin = Some(clip_y_origin);
        self
    }

    #[inline]
    pub fn clip_mask(mut self, clip_mask: PixmapEnum) -> Self {
        self.clip_mask = Some(clip_mask);
        self
    }

    #[inline]
    pub fn dash_offset(mut self, dash_offset: u32) -> Self {
        self.dash_offset = Some(dash_offset);
        self
    }

    #[inline]
    pub fn dashes(mut self, dashes: u32) -> Self {
        self.dashes = Some(dashes);
        self
    }

    #[inline]
    pub fn arc_mode(mut self, arc_mode: ArcModeEnum) -> Self {
        self.arc_mode = Some(arc_mode);
        self
    }
}
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct ChangeGCValueList {
    pub function: Option<GxEnum>,
    pub plane_mask: Option<u32>,
    pub foreground: Option<u32>,
    pub background: Option<u32>,
    pub line_width: Option<u32>,
    pub line_style: Option<LineStyleEnum>,
    pub cap_style: Option<CapStyleEnum>,
    pub join_style: Option<JoinStyleEnum>,
    pub fill_style: Option<FillStyleEnum>,
    pub fill_rule: Option<FillRuleEnum>,
    pub tile: Option<PixmapEnum>,
    pub stipple: Option<PixmapEnum>,
    pub tile_stipple_x_origin: Option<i32>,
    pub tile_stipple_y_origin: Option<i32>,
    pub font: Option<FontEnum>,
    pub subwindow_mode: Option<SubwindowModeEnum>,
    pub graphics_exposures: Option<Bool32>,
    pub clip_x_origin: Option<i32>,
    pub clip_y_origin: Option<i32>,
    pub clip_mask: Option<PixmapEnum>,
    pub dash_offset: Option<u32>,
    pub dashes: Option<u32>,
    pub arc_mode: Option<ArcModeEnum>,
}
impl ChangeGCValueList {
    #[inline]
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let mut offset = 0;
        let buf_ptr = buf;
        if let Some(function) = self.function {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&function.serialize_fixed());
            offset += 4;
        }
        if let Some(plane_mask) = self.plane_mask {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&plane_mask.serialize_fixed());
            offset += 4;
        }
        if let Some(foreground) = self.foreground {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&foreground.serialize_fixed());
            offset += 4;
        }
        if let Some(background) = self.background {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&background.serialize_fixed());
            offset += 4;
        }
        if let Some(line_width) = self.line_width {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&line_width.serialize_fixed());
            offset += 4;
        }
        if let Some(line_style) = self.line_style {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&line_style.serialize_fixed());
            offset += 4;
        }
        if let Some(cap_style) = self.cap_style {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&cap_style.serialize_fixed());
            offset += 4;
        }
        if let Some(join_style) = self.join_style {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&join_style.serialize_fixed());
            offset += 4;
        }
        if let Some(fill_style) = self.fill_style {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&fill_style.serialize_fixed());
            offset += 4;
        }
        if let Some(fill_rule) = self.fill_rule {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&fill_rule.serialize_fixed());
            offset += 4;
        }
        if let Some(tile) = self.tile {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&tile.serialize_fixed());
            offset += 4;
        }
        if let Some(stipple) = self.stipple {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&stipple.serialize_fixed());
            offset += 4;
        }
        if let Some(tile_stipple_x_origin) = self.tile_stipple_x_origin {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&tile_stipple_x_origin.serialize_fixed());
            offset += 4;
        }
        if let Some(tile_stipple_y_origin) = self.tile_stipple_y_origin {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&tile_stipple_y_origin.serialize_fixed());
            offset += 4;
        }
        if let Some(font) = self.font {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&font.serialize_fixed());
            offset += 4;
        }
        if let Some(subwindow_mode) = self.subwindow_mode {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&subwindow_mode.serialize_fixed());
            offset += 4;
        }
        if let Some(graphics_exposures) = self.graphics_exposures {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&graphics_exposures.serialize_fixed());
            offset += 4;
        }
        if let Some(clip_x_origin) = self.clip_x_origin {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&clip_x_origin.serialize_fixed());
            offset += 4;
        }
        if let Some(clip_y_origin) = self.clip_y_origin {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&clip_y_origin.serialize_fixed());
            offset += 4;
        }
        if let Some(clip_mask) = self.clip_mask {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&clip_mask.serialize_fixed());
            offset += 4;
        }
        if let Some(dash_offset) = self.dash_offset {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&dash_offset.serialize_fixed());
            offset += 4;
        }
        if let Some(dashes) = self.dashes {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&dashes.serialize_fixed());
            offset += 4;
        }
        if let Some(arc_mode) = self.arc_mode {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&arc_mode.serialize_fixed());
            offset += 4;
        }
        Ok(offset)
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> Gc {
        let mut mask = Gc::default();
        if self.function.is_some() {
            mask |= Gc::FUNCTION;
        }
        if self.plane_mask.is_some() {
            mask |= Gc::PLANE_MASK;
        }
        if self.foreground.is_some() {
            mask |= Gc::FOREGROUND;
        }
        if self.background.is_some() {
            mask |= Gc::BACKGROUND;
        }
        if self.line_width.is_some() {
            mask |= Gc::LINE_WIDTH;
        }
        if self.line_style.is_some() {
            mask |= Gc::LINE_STYLE;
        }
        if self.cap_style.is_some() {
            mask |= Gc::CAP_STYLE;
        }
        if self.join_style.is_some() {
            mask |= Gc::JOIN_STYLE;
        }
        if self.fill_style.is_some() {
            mask |= Gc::FILL_STYLE;
        }
        if self.fill_rule.is_some() {
            mask |= Gc::FILL_RULE;
        }
        if self.tile.is_some() {
            mask |= Gc::TILE;
        }
        if self.stipple.is_some() {
            mask |= Gc::STIPPLE;
        }
        if self.tile_stipple_x_origin.is_some() {
            mask |= Gc::TILE_STIPPLE_ORIGIN_X;
        }
        if self.tile_stipple_y_origin.is_some() {
            mask |= Gc::TILE_STIPPLE_ORIGIN_Y;
        }
        if self.font.is_some() {
            mask |= Gc::FONT;
        }
        if self.subwindow_mode.is_some() {
            mask |= Gc::SUBWINDOW_MODE;
        }
        if self.graphics_exposures.is_some() {
            mask |= Gc::GRAPHICS_EXPOSURES;
        }
        if self.clip_x_origin.is_some() {
            mask |= Gc::CLIP_ORIGIN_X;
        }
        if self.clip_y_origin.is_some() {
            mask |= Gc::CLIP_ORIGIN_Y;
        }
        if self.clip_mask.is_some() {
            mask |= Gc::CLIP_MASK;
        }
        if self.dash_offset.is_some() {
            mask |= Gc::DASH_OFFSET;
        }
        if self.dashes.is_some() {
            mask |= Gc::DASH_LIST;
        }
        if self.arc_mode.is_some() {
            mask |= Gc::ARC_MODE;
        }
        mask
    }

    #[inline]
    pub fn function(mut self, function: GxEnum) -> Self {
        self.function = Some(function);
        self
    }

    #[inline]
    pub fn plane_mask(mut self, plane_mask: u32) -> Self {
        self.plane_mask = Some(plane_mask);
        self
    }

    #[inline]
    pub fn foreground(mut self, foreground: u32) -> Self {
        self.foreground = Some(foreground);
        self
    }

    #[inline]
    pub fn background(mut self, background: u32) -> Self {
        self.background = Some(background);
        self
    }

    #[inline]
    pub fn line_width(mut self, line_width: u32) -> Self {
        self.line_width = Some(line_width);
        self
    }

    #[inline]
    pub fn line_style(mut self, line_style: LineStyleEnum) -> Self {
        self.line_style = Some(line_style);
        self
    }

    #[inline]
    pub fn cap_style(mut self, cap_style: CapStyleEnum) -> Self {
        self.cap_style = Some(cap_style);
        self
    }

    #[inline]
    pub fn join_style(mut self, join_style: JoinStyleEnum) -> Self {
        self.join_style = Some(join_style);
        self
    }

    #[inline]
    pub fn fill_style(mut self, fill_style: FillStyleEnum) -> Self {
        self.fill_style = Some(fill_style);
        self
    }

    #[inline]
    pub fn fill_rule(mut self, fill_rule: FillRuleEnum) -> Self {
        self.fill_rule = Some(fill_rule);
        self
    }

    #[inline]
    pub fn tile(mut self, tile: PixmapEnum) -> Self {
        self.tile = Some(tile);
        self
    }

    #[inline]
    pub fn stipple(mut self, stipple: PixmapEnum) -> Self {
        self.stipple = Some(stipple);
        self
    }

    #[inline]
    pub fn tile_stipple_x_origin(mut self, tile_stipple_x_origin: i32) -> Self {
        self.tile_stipple_x_origin = Some(tile_stipple_x_origin);
        self
    }

    #[inline]
    pub fn tile_stipple_y_origin(mut self, tile_stipple_y_origin: i32) -> Self {
        self.tile_stipple_y_origin = Some(tile_stipple_y_origin);
        self
    }

    #[inline]
    pub fn font(mut self, font: FontEnum) -> Self {
        self.font = Some(font);
        self
    }

    #[inline]
    pub fn subwindow_mode(mut self, subwindow_mode: SubwindowModeEnum) -> Self {
        self.subwindow_mode = Some(subwindow_mode);
        self
    }

    #[inline]
    pub fn graphics_exposures(mut self, graphics_exposures: Bool32) -> Self {
        self.graphics_exposures = Some(graphics_exposures);
        self
    }

    #[inline]
    pub fn clip_x_origin(mut self, clip_x_origin: i32) -> Self {
        self.clip_x_origin = Some(clip_x_origin);
        self
    }

    #[inline]
    pub fn clip_y_origin(mut self, clip_y_origin: i32) -> Self {
        self.clip_y_origin = Some(clip_y_origin);
        self
    }

    #[inline]
    pub fn clip_mask(mut self, clip_mask: PixmapEnum) -> Self {
        self.clip_mask = Some(clip_mask);
        self
    }

    #[inline]
    pub fn dash_offset(mut self, dash_offset: u32) -> Self {
        self.dash_offset = Some(dash_offset);
        self
    }

    #[inline]
    pub fn dashes(mut self, dashes: u32) -> Self {
        self.dashes = Some(dashes);
        self
    }

    #[inline]
    pub fn arc_mode(mut self, arc_mode: ArcModeEnum) -> Self {
        self.arc_mode = Some(arc_mode);
        self
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListHostsReply {
    pub response_type: u8,
    pub mode: AccessControlEnum,
    pub sequence: u16,
    pub length: u32,
    pub hosts: alloc::vec::Vec<Host>,
}
impl VariableLengthFromBytes for ListHostsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let mode = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let hosts_len = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let hosts_length = hosts_len as usize;
        let mut offset = 32;
        let hosts = crate::vec_from_bytes_var!(ptr, Host, offset, hosts_length,);
        Ok((
            Self {
                response_type,
                mode: mode.into(),
                sequence,
                length,
                hosts,
            },
            offset,
        ))
    }
}
