#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "Present";
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct EventMask(pub u32);
impl EventMask {
    pub const NO_EVENT: Self = Self(0);
    pub const CONFIGURE_NOTIFY: Self = Self(1 << 0);
    pub const COMPLETE_NOTIFY: Self = Self(1 << 1);
    pub const IDLE_NOTIFY: Self = Self(1 << 2);
    pub const REDIRECT_NOTIFY: Self = Self(1 << 3);
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
pub struct CompleteKindEnum(pub u8);
impl CompleteKindEnum {
    pub const PIXMAP: Self = Self(0);
    pub const NOTIFY_M_S_C: Self = Self(1);
}
impl FixedLengthSerialize<1> for CompleteKindEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for CompleteKindEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for CompleteKindEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for CompleteKindEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for CompleteKindEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct CompleteModeEnum(pub u8);
impl CompleteModeEnum {
    pub const COPY: Self = Self(0);
    pub const FLIP: Self = Self(1);
    pub const SKIP: Self = Self(2);
    pub const SUBOPTIMAL_COPY: Self = Self(3);
}
impl FixedLengthSerialize<1> for CompleteModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for CompleteModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for CompleteModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for CompleteModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for CompleteModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Notify {
    pub window: crate::proto::xproto::Window,
    pub serial: u32,
}
impl FixedLengthSerialize<8> for Notify {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let window_bytes = self.window.serialize_fixed();
        let serial_bytes = self.serial.serialize_fixed();
        [
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            serial_bytes[0],
            serial_bytes[1],
            serial_bytes[2],
            serial_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for Notify {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            serial: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
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
pub type Event = u32;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryCapabilitiesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub capabilities: u32,
}
impl FixedLengthFromBytes<12> for QueryCapabilitiesReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            capabilities: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const GENERIC_EVENT: u8 = 0;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GenericEvent {
    pub opcode: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub evtype: u16,
    pub event: Event,
}
impl FixedLengthFromBytes<16> for GenericEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            extension: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            evtype: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            event: Event::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const CONFIGURE_NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ConfigureNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub event: Event,
    pub window: crate::proto::xproto::Window,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub off_x: i16,
    pub off_y: i16,
    pub pixmap_width: u16,
    pub pixmap_height: u16,
    pub pixmap_flags: u32,
}
impl FixedLengthFromBytes<33> for ConfigureNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(1..3).ok_or(crate::error::Error::FromBytes)?)?,
            event: Event::from_bytes(bytes.get(5..9).ok_or(crate::error::Error::FromBytes)?)?,
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(9..13).ok_or(crate::error::Error::FromBytes)?,
            )?,
            x: i16::from_bytes(bytes.get(13..15).ok_or(crate::error::Error::FromBytes)?)?,
            y: i16::from_bytes(bytes.get(15..17).ok_or(crate::error::Error::FromBytes)?)?,
            width: u16::from_bytes(bytes.get(17..19).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(19..21).ok_or(crate::error::Error::FromBytes)?)?,
            off_x: i16::from_bytes(bytes.get(21..23).ok_or(crate::error::Error::FromBytes)?)?,
            off_y: i16::from_bytes(bytes.get(23..25).ok_or(crate::error::Error::FromBytes)?)?,
            pixmap_width: u16::from_bytes(
                bytes.get(25..27).ok_or(crate::error::Error::FromBytes)?,
            )?,
            pixmap_height: u16::from_bytes(
                bytes.get(27..29).ok_or(crate::error::Error::FromBytes)?,
            )?,
            pixmap_flags: u32::from_bytes(
                bytes.get(29..33).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
pub const COMPLETE_NOTIFY_EVENT: u8 = 1;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CompleteNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub kind: CompleteKindEnum,
    pub mode: CompleteModeEnum,
    pub event: Event,
    pub window: crate::proto::xproto::Window,
    pub serial: u32,
    pub ust: u64,
    pub msc: u64,
}
impl FixedLengthFromBytes<33> for CompleteNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(1..3).ok_or(crate::error::Error::FromBytes)?)?,
            kind: u8::from_bytes(bytes.get(3..4).ok_or(crate::error::Error::FromBytes)?)?.into(),
            mode: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?.into(),
            event: Event::from_bytes(bytes.get(5..9).ok_or(crate::error::Error::FromBytes)?)?,
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(9..13).ok_or(crate::error::Error::FromBytes)?,
            )?,
            serial: u32::from_bytes(bytes.get(13..17).ok_or(crate::error::Error::FromBytes)?)?,
            ust: u64::from_bytes(bytes.get(17..25).ok_or(crate::error::Error::FromBytes)?)?,
            msc: u64::from_bytes(bytes.get(25..33).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const IDLE_NOTIFY_EVENT: u8 = 2;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct IdleNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub event: Event,
    pub window: crate::proto::xproto::Window,
    pub serial: u32,
    pub pixmap: crate::proto::xproto::Pixmap,
    pub idle_fence: crate::proto::sync::Fence,
}
impl FixedLengthFromBytes<25> for IdleNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(1..3).ok_or(crate::error::Error::FromBytes)?)?,
            event: Event::from_bytes(bytes.get(5..9).ok_or(crate::error::Error::FromBytes)?)?,
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(9..13).ok_or(crate::error::Error::FromBytes)?,
            )?,
            serial: u32::from_bytes(bytes.get(13..17).ok_or(crate::error::Error::FromBytes)?)?,
            pixmap: crate::proto::xproto::Pixmap::from_bytes(
                bytes.get(17..21).ok_or(crate::error::Error::FromBytes)?,
            )?,
            idle_fence: crate::proto::sync::Fence::from_bytes(
                bytes.get(21..25).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
pub const REDIRECT_NOTIFY_EVENT: u8 = 3;
#[derive(Debug, Clone, PartialEq)]
pub struct RedirectNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub update_window: u8,
    pub event: Event,
    pub event_window: crate::proto::xproto::Window,
    pub window: crate::proto::xproto::Window,
    pub pixmap: crate::proto::xproto::Pixmap,
    pub serial: u32,
    pub valid_region: crate::proto::xfixes::Region,
    pub update_region: crate::proto::xfixes::Region,
    pub valid_rect: crate::proto::xproto::Rectangle,
    pub update_rect: crate::proto::xproto::Rectangle,
    pub x_off: i16,
    pub y_off: i16,
    pub target_crtc: crate::proto::randr::Crtc,
    pub wait_fence: crate::proto::sync::Fence,
    pub idle_fence: crate::proto::sync::Fence,
    pub options: u32,
    pub target_msc: u64,
    pub divisor: u64,
    pub remainder: u64,
    pub notifies: alloc::vec::Vec<Notify>,
}
impl VariableLengthFromBytes for RedirectNotifyEvent {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Start align 8 None
        // Padding 1 bytes
        // Padding 4 bytes
        let opcode = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(1..3).ok_or(crate::error::Error::FromBytes)?)?;
        let update_window = u8::from_bytes(ptr.get(3..4).ok_or(crate::error::Error::FromBytes)?)?;
        let event = Event::from_bytes(ptr.get(5..9).ok_or(crate::error::Error::FromBytes)?)?;
        let event_window = crate::proto::xproto::Window::from_bytes(
            ptr.get(9..13).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let window = crate::proto::xproto::Window::from_bytes(
            ptr.get(13..17).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let pixmap = crate::proto::xproto::Pixmap::from_bytes(
            ptr.get(17..21).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let serial = u32::from_bytes(ptr.get(21..25).ok_or(crate::error::Error::FromBytes)?)?;
        let valid_region = crate::proto::xfixes::Region::from_bytes(
            ptr.get(25..29).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let update_region = crate::proto::xfixes::Region::from_bytes(
            ptr.get(29..33).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let valid_rect = crate::proto::xproto::Rectangle::from_bytes(
            ptr.get(33..41).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let update_rect = crate::proto::xproto::Rectangle::from_bytes(
            ptr.get(41..49).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let x_off = i16::from_bytes(ptr.get(49..51).ok_or(crate::error::Error::FromBytes)?)?;
        let y_off = i16::from_bytes(ptr.get(51..53).ok_or(crate::error::Error::FromBytes)?)?;
        let target_crtc = crate::proto::randr::Crtc::from_bytes(
            ptr.get(53..57).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let wait_fence = crate::proto::sync::Fence::from_bytes(
            ptr.get(57..61).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let idle_fence = crate::proto::sync::Fence::from_bytes(
            ptr.get(61..65).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let options = u32::from_bytes(ptr.get(65..69).ok_or(crate::error::Error::FromBytes)?)?;
        let target_msc = u64::from_bytes(ptr.get(73..81).ok_or(crate::error::Error::FromBytes)?)?;
        let divisor = u64::from_bytes(ptr.get(81..89).ok_or(crate::error::Error::FromBytes)?)?;
        let remainder = u64::from_bytes(ptr.get(89..97).ok_or(crate::error::Error::FromBytes)?)?;
        let mut offset = 97;
        let mut notifies = alloc::vec::Vec::new();
        while let Some(buf) = ptr.get(offset..).filter(|buf| !buf.is_empty()) {
            notifies.push(Notify::from_bytes(buf)?);
            offset += 8;
        }
        Ok((
            Self {
                opcode,
                sequence,
                update_window,
                event,
                event_window,
                window,
                pixmap,
                serial,
                valid_region,
                update_region,
                valid_rect,
                update_rect,
                x_off,
                y_off,
                target_crtc,
                wait_fence,
                idle_fence,
                options,
                target_msc,
                divisor,
                remainder,
                notifies,
            },
            offset,
        ))
    }
}
