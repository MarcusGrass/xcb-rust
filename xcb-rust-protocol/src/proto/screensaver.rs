#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "MIT-SCREEN-SAVER";
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct KindEnum(pub u8);
impl KindEnum {
    pub const BLANKED: Self = Self(0);
    pub const INTERNAL: Self = Self(1);
    pub const EXTERNAL: Self = Self(2);
}
impl FixedLengthSerialize<1> for KindEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for KindEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for KindEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for KindEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for KindEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Event(pub u32);
impl Event {
    pub const NOTIFY_MASK: Self = Self(1 << 0);
    pub const CYCLE_MASK: Self = Self(1 << 1);
}
impl FixedLengthSerialize<4> for Event {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for Event {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for Event {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for Event {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for Event {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(Event);
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct StateEnum(pub u8);
impl StateEnum {
    pub const OFF: Self = Self(0);
    pub const ON: Self = Self(1);
    pub const CYCLE: Self = Self(2);
    pub const DISABLED: Self = Self(3);
}
impl FixedLengthSerialize<1> for StateEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for StateEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for StateEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for StateEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for StateEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: u16,
    pub server_minor_version: u16,
}
impl FixedLengthFromBytes<32> for QueryVersionReply {
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
pub struct QueryInfoReply {
    pub response_type: u8,
    pub state: u8,
    pub sequence: u16,
    pub length: u32,
    pub saver_window: crate::proto::xproto::Window,
    pub ms_until_server: u32,
    pub ms_since_user_input: u32,
    pub event_mask: u32,
    pub kind: KindEnum,
}
impl FixedLengthFromBytes<32> for QueryInfoReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            state: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            saver_window: crate::proto::xproto::Window::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            ms_until_server: u32::from_bytes(
                bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            ms_since_user_input: u32::from_bytes(
                bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?,
            )?,
            event_mask: u32::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
            kind: u8::from_bytes(bytes.get(24..25).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct SetAttributesValueList {
    pub background_pixmap: Option<crate::proto::xproto::BackPixmapEnum>,
    pub background_pixel: Option<u32>,
    pub border_pixmap: Option<crate::proto::xproto::PixmapEnum>,
    pub border_pixel: Option<u32>,
    pub bit_gravity: Option<crate::proto::xproto::GravityEnum>,
    pub win_gravity: Option<crate::proto::xproto::GravityEnum>,
    pub backing_store: Option<crate::proto::xproto::BackingStoreEnum>,
    pub backing_planes: Option<u32>,
    pub backing_pixel: Option<u32>,
    pub override_redirect: Option<crate::proto::xproto::Bool32>,
    pub save_under: Option<crate::proto::xproto::Bool32>,
    pub event_mask: Option<crate::proto::xproto::EventMask>,
    pub do_not_propogate_mask: Option<crate::proto::xproto::EventMask>,
    pub colormap: Option<crate::proto::xproto::ColormapEnum>,
    pub cursor: Option<crate::proto::xproto::CursorEnum>,
}
impl SetAttributesValueList {
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
    pub fn switch_expr(&self) -> crate::proto::xproto::Cw {
        let mut mask = crate::proto::xproto::Cw::default();
        if self.background_pixmap.is_some() {
            mask |= crate::proto::xproto::Cw::BACK_PIXMAP;
        }
        if self.background_pixel.is_some() {
            mask |= crate::proto::xproto::Cw::BACK_PIXEL;
        }
        if self.border_pixmap.is_some() {
            mask |= crate::proto::xproto::Cw::BORDER_PIXMAP;
        }
        if self.border_pixel.is_some() {
            mask |= crate::proto::xproto::Cw::BORDER_PIXEL;
        }
        if self.bit_gravity.is_some() {
            mask |= crate::proto::xproto::Cw::BIT_GRAVITY;
        }
        if self.win_gravity.is_some() {
            mask |= crate::proto::xproto::Cw::WIN_GRAVITY;
        }
        if self.backing_store.is_some() {
            mask |= crate::proto::xproto::Cw::BACKING_STORE;
        }
        if self.backing_planes.is_some() {
            mask |= crate::proto::xproto::Cw::BACKING_PLANES;
        }
        if self.backing_pixel.is_some() {
            mask |= crate::proto::xproto::Cw::BACKING_PIXEL;
        }
        if self.override_redirect.is_some() {
            mask |= crate::proto::xproto::Cw::OVERRIDE_REDIRECT;
        }
        if self.save_under.is_some() {
            mask |= crate::proto::xproto::Cw::SAVE_UNDER;
        }
        if self.event_mask.is_some() {
            mask |= crate::proto::xproto::Cw::EVENT_MASK;
        }
        if self.do_not_propogate_mask.is_some() {
            mask |= crate::proto::xproto::Cw::DONT_PROPAGATE;
        }
        if self.colormap.is_some() {
            mask |= crate::proto::xproto::Cw::COLORMAP;
        }
        if self.cursor.is_some() {
            mask |= crate::proto::xproto::Cw::CURSOR;
        }
        mask
    }

    #[inline]
    pub fn background_pixmap(
        mut self,
        background_pixmap: crate::proto::xproto::BackPixmapEnum,
    ) -> Self {
        self.background_pixmap = Some(background_pixmap);
        self
    }

    #[inline]
    pub fn background_pixel(mut self, background_pixel: u32) -> Self {
        self.background_pixel = Some(background_pixel);
        self
    }

    #[inline]
    pub fn border_pixmap(mut self, border_pixmap: crate::proto::xproto::PixmapEnum) -> Self {
        self.border_pixmap = Some(border_pixmap);
        self
    }

    #[inline]
    pub fn border_pixel(mut self, border_pixel: u32) -> Self {
        self.border_pixel = Some(border_pixel);
        self
    }

    #[inline]
    pub fn bit_gravity(mut self, bit_gravity: crate::proto::xproto::GravityEnum) -> Self {
        self.bit_gravity = Some(bit_gravity);
        self
    }

    #[inline]
    pub fn win_gravity(mut self, win_gravity: crate::proto::xproto::GravityEnum) -> Self {
        self.win_gravity = Some(win_gravity);
        self
    }

    #[inline]
    pub fn backing_store(mut self, backing_store: crate::proto::xproto::BackingStoreEnum) -> Self {
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
    pub fn override_redirect(mut self, override_redirect: crate::proto::xproto::Bool32) -> Self {
        self.override_redirect = Some(override_redirect);
        self
    }

    #[inline]
    pub fn save_under(mut self, save_under: crate::proto::xproto::Bool32) -> Self {
        self.save_under = Some(save_under);
        self
    }

    #[inline]
    pub fn event_mask(mut self, event_mask: crate::proto::xproto::EventMask) -> Self {
        self.event_mask = Some(event_mask);
        self
    }

    #[inline]
    pub fn do_not_propogate_mask(
        mut self,
        do_not_propogate_mask: crate::proto::xproto::EventMask,
    ) -> Self {
        self.do_not_propogate_mask = Some(do_not_propogate_mask);
        self
    }

    #[inline]
    pub fn colormap(mut self, colormap: crate::proto::xproto::ColormapEnum) -> Self {
        self.colormap = Some(colormap);
        self
    }

    #[inline]
    pub fn cursor(mut self, cursor: crate::proto::xproto::CursorEnum) -> Self {
        self.cursor = Some(cursor);
        self
    }
}
pub const NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct NotifyEvent {
    pub opcode: u8,
    pub state: StateEnum,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub root: crate::proto::xproto::Window,
    pub window: crate::proto::xproto::Window,
    pub kind: KindEnum,
    pub forced: u8,
}
impl FixedLengthFromBytes<32> for NotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            state: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            root: crate::proto::xproto::Window::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            kind: u8::from_bytes(bytes.get(16..17).ok_or(crate::error::Error::FromBytes)?)?.into(),
            forced: u8::from_bytes(bytes.get(17..18).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
