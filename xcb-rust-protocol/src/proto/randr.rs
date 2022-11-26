#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "RANDR";
pub type Mode = u32;
pub type Crtc = u32;
pub type Output = u32;
pub type Provider = u32;
pub type Lease = u32;
pub const BAD_OUTPUT_ERROR: u8 = 0;
pub const BAD_CRTC_ERROR: u8 = 1;
pub const BAD_MODE_ERROR: u8 = 2;
pub const BAD_PROVIDER_ERROR: u8 = 3;
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Rotation(pub u16);
impl Rotation {
    pub const ROTATE_0: Self = Self(1 << 0);
    pub const ROTATE_90: Self = Self(1 << 1);
    pub const ROTATE_180: Self = Self(1 << 2);
    pub const ROTATE_270: Self = Self(1 << 3);
    pub const REFLECT_X: Self = Self(1 << 4);
    pub const REFLECT_Y: Self = Self(1 << 5);
}
impl FixedLengthSerialize<2> for Rotation {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for Rotation {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for Rotation {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for Rotation {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for Rotation {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
crate::implement_bit_ops!(Rotation);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ScreenSize {
    pub width: u16,
    pub height: u16,
    pub mwidth: u16,
    pub mheight: u16,
}
impl FixedLengthSerialize<8> for ScreenSize {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let width_bytes = self.width.serialize_fixed();
        let height_bytes = self.height.serialize_fixed();
        let mwidth_bytes = self.mwidth.serialize_fixed();
        let mheight_bytes = self.mheight.serialize_fixed();
        [
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            mwidth_bytes[0],
            mwidth_bytes[1],
            mheight_bytes[0],
            mheight_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<8> for ScreenSize {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            width: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            mwidth: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            mheight: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct RefreshRates {
    pub rates: alloc::vec::Vec<u16>,
}
impl VariableLengthSerialize for RefreshRates {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let n_rates =
            u16::try_from(self.rates.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(n_rates).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.rates.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(2..).ok_or(crate::error::Error::Serialize)?,
            self.rates,
        )?;
        let offset = list_len + 2;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for RefreshRates {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let n_rates = u16::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n_rates as usize;
        let rates: alloc::vec::Vec<u16> = crate::vec_from_bytes_fixed!(
            ptr.get(2..).ok_or(crate::error::Error::FromBytes)?,
            u16,
            length_expr,
            2
        );
        let offset = length_expr * 2 + 2;
        Ok((Self { rates }, offset))
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
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SetConfigEnum(pub u8);
impl SetConfigEnum {
    pub const SUCCESS: Self = Self(0);
    pub const INVALID_CONFIG_TIME: Self = Self(1);
    pub const INVALID_TIME: Self = Self(2);
    pub const FAILED: Self = Self(3);
}
impl FixedLengthSerialize<1> for SetConfigEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SetConfigEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for SetConfigEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SetConfigEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SetConfigEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SetScreenConfigReply {
    pub response_type: u8,
    pub status: SetConfigEnum,
    pub sequence: u16,
    pub length: u32,
    pub new_timestamp: crate::proto::xproto::Timestamp,
    pub config_timestamp: crate::proto::xproto::Timestamp,
    pub root: crate::proto::xproto::Window,
    pub subpixel_order: crate::proto::render::SubPixelEnum,
}
impl FixedLengthFromBytes<32> for SetScreenConfigReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            new_timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            config_timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            root: crate::proto::xproto::Window::from_bytes(
                bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?,
            )?,
            subpixel_order: u16::from_bytes(
                bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
        })
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct NotifyMask(pub u16);
impl NotifyMask {
    pub const SCREEN_CHANGE: Self = Self(1 << 0);
    pub const CRTC_CHANGE: Self = Self(1 << 1);
    pub const OUTPUT_CHANGE: Self = Self(1 << 2);
    pub const OUTPUT_PROPERTY: Self = Self(1 << 3);
    pub const PROVIDER_CHANGE: Self = Self(1 << 4);
    pub const PROVIDER_PROPERTY: Self = Self(1 << 5);
    pub const RESOURCE_CHANGE: Self = Self(1 << 6);
    pub const LEASE: Self = Self(1 << 7);
}
impl FixedLengthSerialize<2> for NotifyMask {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for NotifyMask {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for NotifyMask {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for NotifyMask {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for NotifyMask {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
crate::implement_bit_ops!(NotifyMask);
#[derive(Debug, Clone, PartialEq)]
pub struct GetScreenInfoReply {
    pub response_type: u8,
    pub rotations: Rotation,
    pub sequence: u16,
    pub length: u32,
    pub root: crate::proto::xproto::Window,
    pub timestamp: crate::proto::xproto::Timestamp,
    pub config_timestamp: crate::proto::xproto::Timestamp,
    pub size_i_d: u16,
    pub rotation: Rotation,
    pub rate: u16,
    pub n_info: u16,
    pub sizes: alloc::vec::Vec<ScreenSize>,
    pub rates: alloc::vec::Vec<RefreshRates>,
}
impl VariableLengthFromBytes for GetScreenInfoReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 2 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let rotations = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let root = crate::proto::xproto::Window::from_bytes(
            ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let timestamp = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let config_timestamp = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let n_sizes = u16::from_bytes(ptr.get(20..22).ok_or(crate::error::Error::FromBytes)?)?;
        let size_i_d = u16::from_bytes(ptr.get(22..24).ok_or(crate::error::Error::FromBytes)?)?;
        let rotation = u16::from_bytes(ptr.get(24..26).ok_or(crate::error::Error::FromBytes)?)?;
        let rate = u16::from_bytes(ptr.get(26..28).ok_or(crate::error::Error::FromBytes)?)?;
        let n_info = u16::from_bytes(ptr.get(28..30).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n_sizes as usize;
        let sizes: alloc::vec::Vec<ScreenSize> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            ScreenSize,
            length_expr,
            8
        );
        let mut offset = length_expr * 8 + 32;
        let rates_length = core::ops::Sub::sub(n_info as u16, n_sizes as u16) as usize;
        let rates = crate::vec_from_bytes_var!(ptr, RefreshRates, offset, rates_length,);
        Ok((
            Self {
                response_type,
                rotations: rotations.into(),
                sequence,
                length,
                root,
                timestamp,
                config_timestamp,
                size_i_d,
                rotation: rotation.into(),
                rate,
                n_info,
                sizes,
                rates,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetScreenSizeRangeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_width: u16,
    pub min_height: u16,
    pub max_width: u16,
    pub max_height: u16,
}
impl FixedLengthFromBytes<32> for GetScreenSizeRangeReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            min_width: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            min_height: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
            max_width: u16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
            max_height: u16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ModeFlag(pub u32);
impl ModeFlag {
    pub const HSYNC_POSITIVE: Self = Self(1 << 0);
    pub const HSYNC_NEGATIVE: Self = Self(1 << 1);
    pub const VSYNC_POSITIVE: Self = Self(1 << 2);
    pub const VSYNC_NEGATIVE: Self = Self(1 << 3);
    pub const INTERLACE: Self = Self(1 << 4);
    pub const DOUBLE_SCAN: Self = Self(1 << 5);
    pub const CSYNC: Self = Self(1 << 6);
    pub const CSYNC_POSITIVE: Self = Self(1 << 7);
    pub const CSYNC_NEGATIVE: Self = Self(1 << 8);
    pub const HSKEW_PRESENT: Self = Self(1 << 9);
    pub const BCAST: Self = Self(1 << 10);
    pub const PIXEL_MULTIPLEX: Self = Self(1 << 11);
    pub const DOUBLE_CLOCK: Self = Self(1 << 12);
    pub const HALVE_CLOCK: Self = Self(1 << 13);
}
impl FixedLengthSerialize<4> for ModeFlag {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for ModeFlag {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for ModeFlag {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for ModeFlag {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for ModeFlag {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(ModeFlag);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ModeInfo {
    pub id: u32,
    pub width: u16,
    pub height: u16,
    pub dot_clock: u32,
    pub hsync_start: u16,
    pub hsync_end: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vsync_start: u16,
    pub vsync_end: u16,
    pub vtotal: u16,
    pub name_len: u16,
    pub mode_flags: ModeFlag,
}
impl FixedLengthSerialize<32> for ModeInfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 32] {
        let id_bytes = self.id.serialize_fixed();
        let width_bytes = self.width.serialize_fixed();
        let height_bytes = self.height.serialize_fixed();
        let dot_clock_bytes = self.dot_clock.serialize_fixed();
        let hsync_start_bytes = self.hsync_start.serialize_fixed();
        let hsync_end_bytes = self.hsync_end.serialize_fixed();
        let htotal_bytes = self.htotal.serialize_fixed();
        let hskew_bytes = self.hskew.serialize_fixed();
        let vsync_start_bytes = self.vsync_start.serialize_fixed();
        let vsync_end_bytes = self.vsync_end.serialize_fixed();
        let vtotal_bytes = self.vtotal.serialize_fixed();
        let name_len_bytes = self.name_len.serialize_fixed();
        let mode_flags_bytes = self.mode_flags.serialize_fixed();
        [
            id_bytes[0],
            id_bytes[1],
            id_bytes[2],
            id_bytes[3],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            dot_clock_bytes[0],
            dot_clock_bytes[1],
            dot_clock_bytes[2],
            dot_clock_bytes[3],
            hsync_start_bytes[0],
            hsync_start_bytes[1],
            hsync_end_bytes[0],
            hsync_end_bytes[1],
            htotal_bytes[0],
            htotal_bytes[1],
            hskew_bytes[0],
            hskew_bytes[1],
            vsync_start_bytes[0],
            vsync_start_bytes[1],
            vsync_end_bytes[0],
            vsync_end_bytes[1],
            vtotal_bytes[0],
            vtotal_bytes[1],
            name_len_bytes[0],
            name_len_bytes[1],
            mode_flags_bytes[0],
            mode_flags_bytes[1],
            mode_flags_bytes[2],
            mode_flags_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<32> for ModeInfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            id: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            width: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
            dot_clock: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            hsync_start: u16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
            hsync_end: u16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
            htotal: u16::from_bytes(bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?)?,
            hskew: u16::from_bytes(bytes.get(18..20).ok_or(crate::error::Error::FromBytes)?)?,
            vsync_start: u16::from_bytes(bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?)?,
            vsync_end: u16::from_bytes(bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?)?,
            vtotal: u16::from_bytes(bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?)?,
            name_len: u16::from_bytes(bytes.get(26..28).ok_or(crate::error::Error::FromBytes)?)?,
            mode_flags: u32::from_bytes(bytes.get(28..32).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetScreenResourcesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: crate::proto::xproto::Timestamp,
    pub config_timestamp: crate::proto::xproto::Timestamp,
    pub crtcs: alloc::vec::Vec<Crtc>,
    pub outputs: alloc::vec::Vec<Output>,
    pub modes: alloc::vec::Vec<ModeInfo>,
    pub names: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetScreenResourcesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 8 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let timestamp = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let config_timestamp = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let num_crtcs = u16::from_bytes(ptr.get(16..18).ok_or(crate::error::Error::FromBytes)?)?;
        let num_outputs = u16::from_bytes(ptr.get(18..20).ok_or(crate::error::Error::FromBytes)?)?;
        let num_modes = u16::from_bytes(ptr.get(20..22).ok_or(crate::error::Error::FromBytes)?)?;
        let names_len = u16::from_bytes(ptr.get(22..24).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_crtcs as usize;
        let crtcs: alloc::vec::Vec<Crtc> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Crtc,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 32;
        let length_expr = num_outputs as usize;
        let outputs: alloc::vec::Vec<Output> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Output,
            length_expr,
            4
        );
        offset += length_expr * 4;
        let length_expr = num_modes as usize;
        let modes: alloc::vec::Vec<ModeInfo> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            ModeInfo,
            length_expr,
            32
        );
        offset += length_expr * 32;
        let length_expr = names_len as usize;
        let names: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        offset += length_expr;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                timestamp,
                config_timestamp,
                crtcs,
                outputs,
                modes,
                names,
            },
            offset,
        ))
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ConnectionEnum(pub u8);
impl ConnectionEnum {
    pub const CONNECTED: Self = Self(0);
    pub const DISCONNECTED: Self = Self(1);
    pub const UNKNOWN: Self = Self(2);
}
impl FixedLengthSerialize<1> for ConnectionEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ConnectionEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ConnectionEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ConnectionEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ConnectionEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetOutputInfoReply {
    pub response_type: u8,
    pub status: SetConfigEnum,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: crate::proto::xproto::Timestamp,
    pub crtc: Crtc,
    pub mm_width: u32,
    pub mm_height: u32,
    pub connection: ConnectionEnum,
    pub subpixel_order: crate::proto::render::SubPixelEnum,
    pub num_preferred: u16,
    pub crtcs: alloc::vec::Vec<Crtc>,
    pub modes: alloc::vec::Vec<Mode>,
    pub clones: alloc::vec::Vec<Output>,
    pub name: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetOutputInfoReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let status = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let timestamp = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let crtc = Crtc::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let mm_width = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let mm_height = u32::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let connection = u8::from_bytes(ptr.get(24..25).ok_or(crate::error::Error::FromBytes)?)?;
        let subpixel_order =
            u8::from_bytes(ptr.get(25..26).ok_or(crate::error::Error::FromBytes)?)?;
        let num_crtcs = u16::from_bytes(ptr.get(26..28).ok_or(crate::error::Error::FromBytes)?)?;
        let num_modes = u16::from_bytes(ptr.get(28..30).ok_or(crate::error::Error::FromBytes)?)?;
        let num_preferred =
            u16::from_bytes(ptr.get(30..32).ok_or(crate::error::Error::FromBytes)?)?;
        let num_clones = u16::from_bytes(ptr.get(32..34).ok_or(crate::error::Error::FromBytes)?)?;
        let name_len = u16::from_bytes(ptr.get(34..36).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_crtcs as usize;
        let crtcs: alloc::vec::Vec<Crtc> = crate::vec_from_bytes_fixed!(
            ptr.get(36..).ok_or(crate::error::Error::FromBytes)?,
            Crtc,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 36;
        let length_expr = num_modes as usize;
        let modes: alloc::vec::Vec<Mode> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Mode,
            length_expr,
            4
        );
        offset += length_expr * 4;
        let length_expr = num_clones as usize;
        let clones: alloc::vec::Vec<Output> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Output,
            length_expr,
            4
        );
        offset += length_expr * 4;
        let length_expr = name_len as usize;
        let name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        offset += length_expr;
        Ok((
            Self {
                response_type,
                status: status.into(),
                sequence,
                length,
                timestamp,
                crtc,
                mm_width,
                mm_height,
                connection: connection.into(),
                subpixel_order: subpixel_order.into(),
                num_preferred,
                crtcs,
                modes,
                clones,
                name,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListOutputPropertiesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atoms: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for ListOutputPropertiesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_atoms = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_atoms as usize;
        let atoms: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                atoms,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryOutputPropertyReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub pending: u8,
    pub range: u8,
    pub immutable: u8,
    pub valid_values: alloc::vec::Vec<i32>,
}
impl VariableLengthFromBytes for QueryOutputPropertyReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 21 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let pending = u8::from_bytes(ptr.get(8..9).ok_or(crate::error::Error::FromBytes)?)?;
        let range = u8::from_bytes(ptr.get(9..10).ok_or(crate::error::Error::FromBytes)?)?;
        let immutable = u8::from_bytes(ptr.get(10..11).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = length as usize;
        let valid_values: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
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
                pending,
                range,
                immutable,
                valid_values,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetOutputPropertyReply {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub length: u32,
    pub r#type: crate::proto::xproto::AtomEnum,
    pub bytes_after: u32,
    pub num_items: u32,
    pub data: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetOutputPropertyReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let format = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let r#type = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let bytes_after = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let num_items = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(
            num_items as u32,
            core::ops::Div::div(format as u32, 8u32 as u32) as u32,
        ) as usize;
        let data: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
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
                r#type: r#type.into(),
                bytes_after,
                num_items,
                data,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CreateModeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub mode: Mode,
}
impl FixedLengthFromBytes<32> for CreateModeReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            mode: Mode::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetCrtcInfoReply {
    pub response_type: u8,
    pub status: SetConfigEnum,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: crate::proto::xproto::Timestamp,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub mode: Mode,
    pub rotation: Rotation,
    pub rotations: Rotation,
    pub outputs: alloc::vec::Vec<Output>,
    pub possible: alloc::vec::Vec<Output>,
}
impl VariableLengthFromBytes for GetCrtcInfoReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let status = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let timestamp = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let x = i16::from_bytes(ptr.get(12..14).ok_or(crate::error::Error::FromBytes)?)?;
        let y = i16::from_bytes(ptr.get(14..16).ok_or(crate::error::Error::FromBytes)?)?;
        let width = u16::from_bytes(ptr.get(16..18).ok_or(crate::error::Error::FromBytes)?)?;
        let height = u16::from_bytes(ptr.get(18..20).ok_or(crate::error::Error::FromBytes)?)?;
        let mode = Mode::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let rotation = u16::from_bytes(ptr.get(24..26).ok_or(crate::error::Error::FromBytes)?)?;
        let rotations = u16::from_bytes(ptr.get(26..28).ok_or(crate::error::Error::FromBytes)?)?;
        let num_outputs = u16::from_bytes(ptr.get(28..30).ok_or(crate::error::Error::FromBytes)?)?;
        let num_possible_outputs =
            u16::from_bytes(ptr.get(30..32).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_outputs as usize;
        let outputs: alloc::vec::Vec<Output> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Output,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 32;
        let length_expr = num_possible_outputs as usize;
        let possible: alloc::vec::Vec<Output> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Output,
            length_expr,
            4
        );
        offset += length_expr * 4;
        Ok((
            Self {
                response_type,
                status: status.into(),
                sequence,
                length,
                timestamp,
                x,
                y,
                width,
                height,
                mode,
                rotation: rotation.into(),
                rotations: rotations.into(),
                outputs,
                possible,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SetCrtcConfigReply {
    pub response_type: u8,
    pub status: SetConfigEnum,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: crate::proto::xproto::Timestamp,
}
impl FixedLengthFromBytes<32> for SetCrtcConfigReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetCrtcGammaSizeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: u16,
}
impl FixedLengthFromBytes<32> for GetCrtcGammaSizeReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            size: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetCrtcGammaReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub red: alloc::vec::Vec<u16>,
    pub green: alloc::vec::Vec<u16>,
    pub blue: alloc::vec::Vec<u16>,
}
impl VariableLengthFromBytes for GetCrtcGammaReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let size = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = size as usize;
        let red: alloc::vec::Vec<u16> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            u16,
            length_expr,
            2
        );
        let mut offset = length_expr * 2 + 32;
        let length_expr = size as usize;
        let green: alloc::vec::Vec<u16> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u16,
            length_expr,
            2
        );
        offset += length_expr * 2;
        let length_expr = size as usize;
        let blue: alloc::vec::Vec<u16> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u16,
            length_expr,
            2
        );
        offset += length_expr * 2;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                red,
                green,
                blue,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetScreenResourcesCurrentReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: crate::proto::xproto::Timestamp,
    pub config_timestamp: crate::proto::xproto::Timestamp,
    pub crtcs: alloc::vec::Vec<Crtc>,
    pub outputs: alloc::vec::Vec<Output>,
    pub modes: alloc::vec::Vec<ModeInfo>,
    pub names: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetScreenResourcesCurrentReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 8 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let timestamp = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let config_timestamp = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let num_crtcs = u16::from_bytes(ptr.get(16..18).ok_or(crate::error::Error::FromBytes)?)?;
        let num_outputs = u16::from_bytes(ptr.get(18..20).ok_or(crate::error::Error::FromBytes)?)?;
        let num_modes = u16::from_bytes(ptr.get(20..22).ok_or(crate::error::Error::FromBytes)?)?;
        let names_len = u16::from_bytes(ptr.get(22..24).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_crtcs as usize;
        let crtcs: alloc::vec::Vec<Crtc> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Crtc,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 32;
        let length_expr = num_outputs as usize;
        let outputs: alloc::vec::Vec<Output> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Output,
            length_expr,
            4
        );
        offset += length_expr * 4;
        let length_expr = num_modes as usize;
        let modes: alloc::vec::Vec<ModeInfo> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            ModeInfo,
            length_expr,
            32
        );
        offset += length_expr * 32;
        let length_expr = names_len as usize;
        let names: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        offset += length_expr;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                timestamp,
                config_timestamp,
                crtcs,
                outputs,
                modes,
                names,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetCrtcTransformReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub pending_transform: crate::proto::render::Transform,
    pub has_transforms: u8,
    pub current_transform: crate::proto::render::Transform,
    pub pending_filter_name: alloc::vec::Vec<u8>,
    pub pending_params: alloc::vec::Vec<crate::proto::render::Fixed>,
    pub current_filter_name: alloc::vec::Vec<u8>,
    pub current_params: alloc::vec::Vec<crate::proto::render::Fixed>,
}
impl VariableLengthFromBytes for GetCrtcTransformReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 3 bytes
        // Padding 4 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let pending_transform = crate::proto::render::Transform::from_bytes(
            ptr.get(8..44).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let has_transforms =
            u8::from_bytes(ptr.get(44..45).ok_or(crate::error::Error::FromBytes)?)?;
        let current_transform = crate::proto::render::Transform::from_bytes(
            ptr.get(48..84).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let pending_len = u16::from_bytes(ptr.get(88..90).ok_or(crate::error::Error::FromBytes)?)?;
        let pending_nparams =
            u16::from_bytes(ptr.get(90..92).ok_or(crate::error::Error::FromBytes)?)?;
        let current_len = u16::from_bytes(ptr.get(92..94).ok_or(crate::error::Error::FromBytes)?)?;
        let current_nparams =
            u16::from_bytes(ptr.get(94..96).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = pending_len as usize;
        let pending_filter_name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(96..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr + 96;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let length_expr = pending_nparams as usize;
        let pending_params: alloc::vec::Vec<crate::proto::render::Fixed> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            crate::proto::render::Fixed,
            length_expr,
            4
        );
        offset += length_expr * 4;
        let length_expr = current_len as usize;
        let current_filter_name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        offset += length_expr;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let length_expr = current_nparams as usize;
        let current_params: alloc::vec::Vec<crate::proto::render::Fixed> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            crate::proto::render::Fixed,
            length_expr,
            4
        );
        offset += length_expr * 4;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                pending_transform,
                has_transforms,
                current_transform,
                pending_filter_name,
                pending_params,
                current_filter_name,
                current_params,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetPanningReply {
    pub response_type: u8,
    pub status: SetConfigEnum,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: crate::proto::xproto::Timestamp,
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
    pub track_left: u16,
    pub track_top: u16,
    pub track_width: u16,
    pub track_height: u16,
    pub border_left: i16,
    pub border_top: i16,
    pub border_right: i16,
    pub border_bottom: i16,
}
impl FixedLengthFromBytes<36> for GetPanningReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            left: u16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
            top: u16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
            width: u16::from_bytes(bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(18..20).ok_or(crate::error::Error::FromBytes)?)?,
            track_left: u16::from_bytes(bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?)?,
            track_top: u16::from_bytes(bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?)?,
            track_width: u16::from_bytes(bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?)?,
            track_height: u16::from_bytes(
                bytes.get(26..28).ok_or(crate::error::Error::FromBytes)?,
            )?,
            border_left: i16::from_bytes(bytes.get(28..30).ok_or(crate::error::Error::FromBytes)?)?,
            border_top: i16::from_bytes(bytes.get(30..32).ok_or(crate::error::Error::FromBytes)?)?,
            border_right: i16::from_bytes(
                bytes.get(32..34).ok_or(crate::error::Error::FromBytes)?,
            )?,
            border_bottom: i16::from_bytes(
                bytes.get(34..36).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SetPanningReply {
    pub response_type: u8,
    pub status: SetConfigEnum,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: crate::proto::xproto::Timestamp,
}
impl FixedLengthFromBytes<12> for SetPanningReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetOutputPrimaryReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub output: Output,
}
impl FixedLengthFromBytes<12> for GetOutputPrimaryReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            output: Output::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetProvidersReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: crate::proto::xproto::Timestamp,
    pub providers: alloc::vec::Vec<Provider>,
}
impl VariableLengthFromBytes for GetProvidersReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 18 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let timestamp = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let num_providers =
            u16::from_bytes(ptr.get(12..14).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_providers as usize;
        let providers: alloc::vec::Vec<Provider> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Provider,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                timestamp,
                providers,
            },
            offset,
        ))
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ProviderCapability(pub u32);
impl ProviderCapability {
    pub const SOURCE_OUTPUT: Self = Self(1 << 0);
    pub const SINK_OUTPUT: Self = Self(1 << 1);
    pub const SOURCE_OFFLOAD: Self = Self(1 << 2);
    pub const SINK_OFFLOAD: Self = Self(1 << 3);
}
impl FixedLengthSerialize<4> for ProviderCapability {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for ProviderCapability {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for ProviderCapability {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for ProviderCapability {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for ProviderCapability {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(ProviderCapability);
#[derive(Debug, Clone, PartialEq)]
pub struct GetProviderInfoReply {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: crate::proto::xproto::Timestamp,
    pub capabilities: ProviderCapability,
    pub crtcs: alloc::vec::Vec<Crtc>,
    pub outputs: alloc::vec::Vec<Output>,
    pub associated_providers: alloc::vec::Vec<Provider>,
    pub associated_capability: alloc::vec::Vec<u32>,
    pub name: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetProviderInfoReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 8 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let status = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let timestamp = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let capabilities = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let num_crtcs = u16::from_bytes(ptr.get(16..18).ok_or(crate::error::Error::FromBytes)?)?;
        let num_outputs = u16::from_bytes(ptr.get(18..20).ok_or(crate::error::Error::FromBytes)?)?;
        let num_associated_providers =
            u16::from_bytes(ptr.get(20..22).ok_or(crate::error::Error::FromBytes)?)?;
        let name_len = u16::from_bytes(ptr.get(22..24).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_crtcs as usize;
        let crtcs: alloc::vec::Vec<Crtc> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Crtc,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 32;
        let length_expr = num_outputs as usize;
        let outputs: alloc::vec::Vec<Output> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Output,
            length_expr,
            4
        );
        offset += length_expr * 4;
        let length_expr = num_associated_providers as usize;
        let associated_providers: alloc::vec::Vec<Provider> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Provider,
            length_expr,
            4
        );
        offset += length_expr * 4;
        let length_expr = num_associated_providers as usize;
        let associated_capability: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        offset += length_expr * 4;
        let length_expr = name_len as usize;
        let name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        offset += length_expr;
        Ok((
            Self {
                response_type,
                status,
                sequence,
                length,
                timestamp,
                capabilities: capabilities.into(),
                crtcs,
                outputs,
                associated_providers,
                associated_capability,
                name,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListProviderPropertiesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atoms: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for ListProviderPropertiesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_atoms = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_atoms as usize;
        let atoms: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                atoms,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryProviderPropertyReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub pending: u8,
    pub range: u8,
    pub immutable: u8,
    pub valid_values: alloc::vec::Vec<i32>,
}
impl VariableLengthFromBytes for QueryProviderPropertyReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 21 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let pending = u8::from_bytes(ptr.get(8..9).ok_or(crate::error::Error::FromBytes)?)?;
        let range = u8::from_bytes(ptr.get(9..10).ok_or(crate::error::Error::FromBytes)?)?;
        let immutable = u8::from_bytes(ptr.get(10..11).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = length as usize;
        let valid_values: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
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
                pending,
                range,
                immutable,
                valid_values,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetProviderPropertyReply {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub length: u32,
    pub r#type: u32,
    pub bytes_after: u32,
    pub num_items: u32,
    pub data: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetProviderPropertyReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let format = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let r#type = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let bytes_after = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let num_items = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(
            num_items as u32,
            core::ops::Div::div(format as u32, 8u32 as u32) as u32,
        ) as usize;
        let data: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
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
                num_items,
                data,
            },
            offset,
        ))
    }
}
pub const SCREEN_CHANGE_NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ScreenChangeNotifyEvent {
    pub opcode: u8,
    pub rotation: Rotation,
    pub sequence: u16,
    pub timestamp: crate::proto::xproto::Timestamp,
    pub config_timestamp: crate::proto::xproto::Timestamp,
    pub root: crate::proto::xproto::Window,
    pub request_window: crate::proto::xproto::Window,
    pub size_i_d: u16,
    pub subpixel_order: crate::proto::render::SubPixelEnum,
    pub width: u16,
    pub height: u16,
    pub mwidth: u16,
    pub mheight: u16,
}
impl FixedLengthFromBytes<32> for ScreenChangeNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            rotation: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            config_timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            root: crate::proto::xproto::Window::from_bytes(
                bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            request_window: crate::proto::xproto::Window::from_bytes(
                bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?,
            )?,
            size_i_d: u16::from_bytes(bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?)?,
            subpixel_order: u16::from_bytes(
                bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            width: u16::from_bytes(bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(26..28).ok_or(crate::error::Error::FromBytes)?)?,
            mwidth: u16::from_bytes(bytes.get(28..30).ok_or(crate::error::Error::FromBytes)?)?,
            mheight: u16::from_bytes(bytes.get(30..32).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct NotifyEnum(pub u8);
impl NotifyEnum {
    pub const CRTC_CHANGE: Self = Self(0);
    pub const OUTPUT_CHANGE: Self = Self(1);
    pub const OUTPUT_PROPERTY: Self = Self(2);
    pub const PROVIDER_CHANGE: Self = Self(3);
    pub const PROVIDER_PROPERTY: Self = Self(4);
    pub const RESOURCE_CHANGE: Self = Self(5);
    pub const LEASE: Self = Self(6);
}
impl FixedLengthSerialize<1> for NotifyEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for NotifyEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for NotifyEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for NotifyEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for NotifyEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CrtcChange {
    pub timestamp: crate::proto::xproto::Timestamp,
    pub window: crate::proto::xproto::Window,
    pub crtc: Crtc,
    pub mode: Mode,
    pub rotation: Rotation,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}
impl FixedLengthSerialize<28> for CrtcChange {
    #[inline]
    fn serialize_fixed(self) -> [u8; 28] {
        let timestamp_bytes = self.timestamp.serialize_fixed();
        let window_bytes = self.window.serialize_fixed();
        let crtc_bytes = self.crtc.serialize_fixed();
        let mode_bytes = self.mode.serialize_fixed();
        let rotation_bytes = self.rotation.serialize_fixed();
        let x_bytes = self.x.serialize_fixed();
        let y_bytes = self.y.serialize_fixed();
        let width_bytes = self.width.serialize_fixed();
        let height_bytes = self.height.serialize_fixed();
        [
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            crtc_bytes[0],
            crtc_bytes[1],
            crtc_bytes[2],
            crtc_bytes[3],
            mode_bytes[0],
            mode_bytes[1],
            mode_bytes[2],
            mode_bytes[3],
            rotation_bytes[0],
            rotation_bytes[1],
            0,
            0,
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
impl FixedLengthFromBytes<28> for CrtcChange {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            crtc: Crtc::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            mode: Mode::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            rotation: u16::from_bytes(bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            x: i16::from_bytes(bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?)?,
            y: i16::from_bytes(bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?)?,
            width: u16::from_bytes(bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?)?,
            height: u16::from_bytes(bytes.get(26..28).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct OutputChange {
    pub timestamp: crate::proto::xproto::Timestamp,
    pub config_timestamp: crate::proto::xproto::Timestamp,
    pub window: crate::proto::xproto::Window,
    pub output: Output,
    pub crtc: Crtc,
    pub mode: Mode,
    pub rotation: Rotation,
    pub connection: ConnectionEnum,
    pub subpixel_order: crate::proto::render::SubPixelEnum,
}
impl FixedLengthSerialize<28> for OutputChange {
    #[inline]
    fn serialize_fixed(self) -> [u8; 28] {
        let timestamp_bytes = self.timestamp.serialize_fixed();
        let config_timestamp_bytes = self.config_timestamp.serialize_fixed();
        let window_bytes = self.window.serialize_fixed();
        let output_bytes = self.output.serialize_fixed();
        let crtc_bytes = self.crtc.serialize_fixed();
        let mode_bytes = self.mode.serialize_fixed();
        let rotation_bytes = self.rotation.serialize_fixed();
        [
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            config_timestamp_bytes[0],
            config_timestamp_bytes[1],
            config_timestamp_bytes[2],
            config_timestamp_bytes[3],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            output_bytes[0],
            output_bytes[1],
            output_bytes[2],
            output_bytes[3],
            crtc_bytes[0],
            crtc_bytes[1],
            crtc_bytes[2],
            crtc_bytes[3],
            mode_bytes[0],
            mode_bytes[1],
            mode_bytes[2],
            mode_bytes[3],
            rotation_bytes[0],
            rotation_bytes[1],
            self.connection.0 as u8,
            self.subpixel_order.0 as u8,
        ]
    }
}
impl FixedLengthFromBytes<28> for OutputChange {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            config_timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            output: Output::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            crtc: Crtc::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            mode: Mode::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
            rotation: u16::from_bytes(bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            connection: u8::from_bytes(bytes.get(26..27).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            subpixel_order: u8::from_bytes(
                bytes.get(27..28).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct OutputProperty {
    pub window: crate::proto::xproto::Window,
    pub output: Output,
    pub atom: u32,
    pub timestamp: crate::proto::xproto::Timestamp,
    pub status: crate::proto::xproto::PropertyEnum,
}
impl FixedLengthSerialize<28> for OutputProperty {
    #[inline]
    fn serialize_fixed(self) -> [u8; 28] {
        let window_bytes = self.window.serialize_fixed();
        let output_bytes = self.output.serialize_fixed();
        let atom_bytes = self.atom.serialize_fixed();
        let timestamp_bytes = self.timestamp.serialize_fixed();
        [
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            output_bytes[0],
            output_bytes[1],
            output_bytes[2],
            output_bytes[3],
            atom_bytes[0],
            atom_bytes[1],
            atom_bytes[2],
            atom_bytes[3],
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            self.status.0 as u8,
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
impl FixedLengthFromBytes<28> for OutputProperty {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            output: Output::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            atom: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            status: u8::from_bytes(bytes.get(16..17).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ProviderChange {
    pub timestamp: crate::proto::xproto::Timestamp,
    pub window: crate::proto::xproto::Window,
    pub provider: Provider,
}
impl FixedLengthSerialize<28> for ProviderChange {
    #[inline]
    fn serialize_fixed(self) -> [u8; 28] {
        let timestamp_bytes = self.timestamp.serialize_fixed();
        let window_bytes = self.window.serialize_fixed();
        let provider_bytes = self.provider.serialize_fixed();
        [
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            provider_bytes[0],
            provider_bytes[1],
            provider_bytes[2],
            provider_bytes[3],
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
            0,
            0,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<28> for ProviderChange {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            provider: Provider::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ProviderProperty {
    pub window: crate::proto::xproto::Window,
    pub provider: Provider,
    pub atom: u32,
    pub timestamp: crate::proto::xproto::Timestamp,
    pub state: u8,
}
impl FixedLengthSerialize<28> for ProviderProperty {
    #[inline]
    fn serialize_fixed(self) -> [u8; 28] {
        let window_bytes = self.window.serialize_fixed();
        let provider_bytes = self.provider.serialize_fixed();
        let atom_bytes = self.atom.serialize_fixed();
        let timestamp_bytes = self.timestamp.serialize_fixed();
        [
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            provider_bytes[0],
            provider_bytes[1],
            provider_bytes[2],
            provider_bytes[3],
            atom_bytes[0],
            atom_bytes[1],
            atom_bytes[2],
            atom_bytes[3],
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            self.state,
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
impl FixedLengthFromBytes<28> for ProviderProperty {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            provider: Provider::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            atom: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            state: u8::from_bytes(bytes.get(16..17).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ResourceChange {
    pub timestamp: crate::proto::xproto::Timestamp,
    pub window: crate::proto::xproto::Window,
}
impl FixedLengthSerialize<28> for ResourceChange {
    #[inline]
    fn serialize_fixed(self) -> [u8; 28] {
        let timestamp_bytes = self.timestamp.serialize_fixed();
        let window_bytes = self.window.serialize_fixed();
        [
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
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
impl FixedLengthFromBytes<28> for ResourceChange {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct MonitorInfo {
    pub name: u32,
    pub primary: u8,
    pub automatic: u8,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub width_in_millimeters: u32,
    pub height_in_millimeters: u32,
    pub outputs: alloc::vec::Vec<Output>,
}
impl VariableLengthSerialize for MonitorInfo {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let n_output =
            u16::try_from(self.outputs.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.name.serialize_fixed());
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.primary.serialize_fixed());
        buf_ptr
            .get_mut(5..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.automatic.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(n_output).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.x.serialize_fixed());
        buf_ptr
            .get_mut(10..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.y.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.width.serialize_fixed());
        buf_ptr
            .get_mut(14..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.height.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.width_in_millimeters.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.height_in_millimeters.serialize_fixed());
        let list_len = self.outputs.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            self.outputs,
        )?;
        let offset = list_len + 24;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for MonitorInfo {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let name = u32::from_bytes(ptr.get(0..4).ok_or(crate::error::Error::FromBytes)?)?;
        let primary = u8::from_bytes(ptr.get(4..5).ok_or(crate::error::Error::FromBytes)?)?;
        let automatic = u8::from_bytes(ptr.get(5..6).ok_or(crate::error::Error::FromBytes)?)?;
        let n_output = u16::from_bytes(ptr.get(6..8).ok_or(crate::error::Error::FromBytes)?)?;
        let x = i16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let y = i16::from_bytes(ptr.get(10..12).ok_or(crate::error::Error::FromBytes)?)?;
        let width = u16::from_bytes(ptr.get(12..14).ok_or(crate::error::Error::FromBytes)?)?;
        let height = u16::from_bytes(ptr.get(14..16).ok_or(crate::error::Error::FromBytes)?)?;
        let width_in_millimeters =
            u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let height_in_millimeters =
            u32::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n_output as usize;
        let outputs: alloc::vec::Vec<Output> = crate::vec_from_bytes_fixed!(
            ptr.get(24..).ok_or(crate::error::Error::FromBytes)?,
            Output,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 24;
        Ok((
            Self {
                name,
                primary,
                automatic,
                x,
                y,
                width,
                height,
                width_in_millimeters,
                height_in_millimeters,
                outputs,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetMonitorsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: crate::proto::xproto::Timestamp,
    pub n_outputs: u32,
    pub monitors: alloc::vec::Vec<MonitorInfo>,
}
impl VariableLengthFromBytes for GetMonitorsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let timestamp = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let n_monitors = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let n_outputs = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let monitors_length = n_monitors as usize;
        let mut offset = 32;
        let monitors = crate::vec_from_bytes_var!(ptr, MonitorInfo, offset, monitors_length,);
        Ok((
            Self {
                response_type,
                sequence,
                length,
                timestamp,
                n_outputs,
                monitors,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CreateLeaseReply {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub master_fd: (),
}
impl FixedLengthFromBytes<32> for CreateLeaseReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            nfd: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            master_fd: (),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct LeaseNotify {
    pub timestamp: crate::proto::xproto::Timestamp,
    pub window: crate::proto::xproto::Window,
    pub lease: Lease,
    pub created: u8,
}
impl FixedLengthSerialize<28> for LeaseNotify {
    #[inline]
    fn serialize_fixed(self) -> [u8; 28] {
        let timestamp_bytes = self.timestamp.serialize_fixed();
        let window_bytes = self.window.serialize_fixed();
        let lease_bytes = self.lease.serialize_fixed();
        [
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            lease_bytes[0],
            lease_bytes[1],
            lease_bytes[2],
            lease_bytes[3],
            self.created,
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
            0,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<28> for LeaseNotify {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            lease: Lease::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            created: u8::from_bytes(bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct NotifyData(pub [u8; 28]);
impl FixedLengthFromBytes<28> for NotifyData {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        // SAFETY: Checked that the bytes are available
        Ok(Self(unsafe {
            bytes
                .get(..28)
                .ok_or(crate::error::Error::FromBytes)?
                .try_into()
                .unwrap_unchecked()
        }))
    }
}
impl FixedLengthSerialize<28> for NotifyData {
    #[inline]
    fn serialize_fixed(self) -> [u8; 28] {
        self.0
    }
}
pub const NOTIFY_EVENT: u8 = 1;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct NotifyEvent {
    pub opcode: u8,
    pub sub_code: NotifyEnum,
    pub sequence: u16,
    pub u: NotifyData,
}
impl FixedLengthFromBytes<32> for NotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sub_code: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            u: NotifyData::from_bytes(bytes.get(4..32).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
