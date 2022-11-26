#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "XFree86-VidModeExtension";
pub type Syncrange = u32;
pub type Dotclock = u32;
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ModeFlag(pub u32);
impl ModeFlag {
    pub const POSITIVE_HSYNC: Self = Self(1 << 0);
    pub const NEGATIVE_HSYNC: Self = Self(1 << 1);
    pub const POSITIVE_VSYNC: Self = Self(1 << 2);
    pub const NEGATIVE_VSYNC: Self = Self(1 << 3);
    pub const INTERLACE: Self = Self(1 << 4);
    pub const COMPOSITE_SYNC: Self = Self(1 << 5);
    pub const POSITIVE_CSYNC: Self = Self(1 << 6);
    pub const NEGATIVE_CSYNC: Self = Self(1 << 7);
    pub const H_SKEW: Self = Self(1 << 8);
    pub const BROADCAST: Self = Self(1 << 9);
    pub const PIXMUX: Self = Self(1 << 10);
    pub const DOUBLE_CLOCK: Self = Self(1 << 11);
    pub const HALF_CLOCK: Self = Self(1 << 12);
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
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ClockFlag(pub u32);
impl ClockFlag {
    pub const PROGRAMABLE: Self = Self(1 << 0);
}
impl FixedLengthSerialize<4> for ClockFlag {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for ClockFlag {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for ClockFlag {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for ClockFlag {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for ClockFlag {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(ClockFlag);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Permission(pub u32);
impl Permission {
    pub const READ: Self = Self(1 << 0);
    pub const WRITE: Self = Self(1 << 1);
}
impl FixedLengthSerialize<4> for Permission {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for Permission {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for Permission {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for Permission {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for Permission {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(Permission);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ModeInfo {
    pub dotclock: Dotclock,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u32,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub flags: ModeFlag,
    pub privsize: u32,
}
impl FixedLengthSerialize<48> for ModeInfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 48] {
        let dotclock_bytes = self.dotclock.serialize_fixed();
        let hdisplay_bytes = self.hdisplay.serialize_fixed();
        let hsyncstart_bytes = self.hsyncstart.serialize_fixed();
        let hsyncend_bytes = self.hsyncend.serialize_fixed();
        let htotal_bytes = self.htotal.serialize_fixed();
        let hskew_bytes = self.hskew.serialize_fixed();
        let vdisplay_bytes = self.vdisplay.serialize_fixed();
        let vsyncstart_bytes = self.vsyncstart.serialize_fixed();
        let vsyncend_bytes = self.vsyncend.serialize_fixed();
        let vtotal_bytes = self.vtotal.serialize_fixed();
        let flags_bytes = self.flags.serialize_fixed();
        let privsize_bytes = self.privsize.serialize_fixed();
        [
            dotclock_bytes[0],
            dotclock_bytes[1],
            dotclock_bytes[2],
            dotclock_bytes[3],
            hdisplay_bytes[0],
            hdisplay_bytes[1],
            hsyncstart_bytes[0],
            hsyncstart_bytes[1],
            hsyncend_bytes[0],
            hsyncend_bytes[1],
            htotal_bytes[0],
            htotal_bytes[1],
            hskew_bytes[0],
            hskew_bytes[1],
            hskew_bytes[2],
            hskew_bytes[3],
            vdisplay_bytes[0],
            vdisplay_bytes[1],
            vsyncstart_bytes[0],
            vsyncstart_bytes[1],
            vsyncend_bytes[0],
            vsyncend_bytes[1],
            vtotal_bytes[0],
            vtotal_bytes[1],
            0,
            0,
            0,
            0,
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
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
            privsize_bytes[0],
            privsize_bytes[1],
            privsize_bytes[2],
            privsize_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<48> for ModeInfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            dotclock: Dotclock::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            hdisplay: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            hsyncstart: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
            hsyncend: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            htotal: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
            hskew: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            vdisplay: u16::from_bytes(bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?)?,
            vsyncstart: u16::from_bytes(bytes.get(18..20).ok_or(crate::error::Error::FromBytes)?)?,
            vsyncend: u16::from_bytes(bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?)?,
            vtotal: u16::from_bytes(bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?)?,
            flags: u32::from_bytes(bytes.get(28..32).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            privsize: u32::from_bytes(bytes.get(44..48).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
}
impl FixedLengthFromBytes<12> for QueryVersionReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            major_version: u16::from_bytes(
                bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?,
            )?,
            minor_version: u16::from_bytes(
                bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetModeLineReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub dotclock: Dotclock,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub flags: ModeFlag,
    pub private: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetModeLineReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 2 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let dotclock = Dotclock::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let hdisplay = u16::from_bytes(ptr.get(12..14).ok_or(crate::error::Error::FromBytes)?)?;
        let hsyncstart = u16::from_bytes(ptr.get(14..16).ok_or(crate::error::Error::FromBytes)?)?;
        let hsyncend = u16::from_bytes(ptr.get(16..18).ok_or(crate::error::Error::FromBytes)?)?;
        let htotal = u16::from_bytes(ptr.get(18..20).ok_or(crate::error::Error::FromBytes)?)?;
        let hskew = u16::from_bytes(ptr.get(20..22).ok_or(crate::error::Error::FromBytes)?)?;
        let vdisplay = u16::from_bytes(ptr.get(22..24).ok_or(crate::error::Error::FromBytes)?)?;
        let vsyncstart = u16::from_bytes(ptr.get(24..26).ok_or(crate::error::Error::FromBytes)?)?;
        let vsyncend = u16::from_bytes(ptr.get(26..28).ok_or(crate::error::Error::FromBytes)?)?;
        let vtotal = u16::from_bytes(ptr.get(28..30).ok_or(crate::error::Error::FromBytes)?)?;
        let flags = u32::from_bytes(ptr.get(32..36).ok_or(crate::error::Error::FromBytes)?)?;
        let privsize = u32::from_bytes(ptr.get(48..52).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = privsize as usize;
        let private: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(52..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 52;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                dotclock,
                hdisplay,
                hsyncstart,
                hsyncend,
                htotal,
                hskew,
                vdisplay,
                vsyncstart,
                vsyncend,
                vtotal,
                flags: flags.into(),
                private,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetMonitorReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub hsync: alloc::vec::Vec<Syncrange>,
    pub vsync: alloc::vec::Vec<Syncrange>,
    pub vendor: alloc::vec::Vec<u8>,
    pub alignment_pad: alloc::vec::Vec<u8>,
    pub model: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetMonitorReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let vendor_length = u8::from_bytes(ptr.get(8..9).ok_or(crate::error::Error::FromBytes)?)?;
        let model_length = u8::from_bytes(ptr.get(9..10).ok_or(crate::error::Error::FromBytes)?)?;
        let num_hsync = u8::from_bytes(ptr.get(10..11).ok_or(crate::error::Error::FromBytes)?)?;
        let num_vsync = u8::from_bytes(ptr.get(11..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_hsync as usize;
        let hsync: alloc::vec::Vec<Syncrange> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Syncrange,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 32;
        let length_expr = num_vsync as usize;
        let vsync: alloc::vec::Vec<Syncrange> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Syncrange,
            length_expr,
            4
        );
        offset += length_expr * 4;
        let length_expr = vendor_length as usize;
        let vendor: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        offset += length_expr;
        let length_expr = core::ops::Sub::sub(
            core::ops::BitAnd::bitand(
                core::ops::Add::add(vendor_length as u8, 3u8 as u8) as u8,
                core::ops::Not::not(3u8) as u8,
            ) as u8,
            vendor_length as u8,
        ) as usize;
        let alignment_pad: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        offset += length_expr;
        let length_expr = model_length as usize;
        let model: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        offset += length_expr;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                hsync,
                vsync,
                vendor,
                alignment_pad,
                model,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetAllModeLinesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub modeinfo: alloc::vec::Vec<ModeInfo>,
}
impl VariableLengthFromBytes for GetAllModeLinesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let modecount = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = modecount as usize;
        let modeinfo: alloc::vec::Vec<ModeInfo> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            ModeInfo,
            length_expr,
            48
        );
        let offset = length_expr * 48 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                modeinfo,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ValidateModeLineReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u32,
}
impl FixedLengthFromBytes<32> for ValidateModeLineReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            status: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetViewPortReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub x: u32,
    pub y: u32,
}
impl FixedLengthFromBytes<32> for GetViewPortReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            x: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            y: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetDotClocksReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub flags: ClockFlag,
    pub clocks: u32,
    pub maxclocks: u32,
    pub clock: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for GetDotClocksReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 12 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let flags = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let clocks = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let maxclocks = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(
            core::ops::Sub::sub(
                1u32 as u32,
                core::ops::BitAnd::bitand(flags as u32, 1u32 as u32) as u32,
            ) as u32,
            clocks as u32,
        ) as usize;
        let clock: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                flags: flags.into(),
                clocks,
                maxclocks,
                clock,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetGammaReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}
impl FixedLengthFromBytes<32> for GetGammaReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            red: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            green: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            blue: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetGammaRampReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub red: alloc::vec::Vec<u16>,
    pub green: alloc::vec::Vec<u16>,
    pub blue: alloc::vec::Vec<u16>,
}
impl VariableLengthFromBytes for GetGammaRampReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let size = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::BitAnd::bitand(
            core::ops::Add::add(size as u16, 1u16 as u16) as u16,
            core::ops::Not::not(1u16) as u16,
        ) as usize;
        let red: alloc::vec::Vec<u16> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            u16,
            length_expr,
            2
        );
        let mut offset = length_expr * 2 + 32;
        let length_expr = core::ops::BitAnd::bitand(
            core::ops::Add::add(size as u16, 1u16 as u16) as u16,
            core::ops::Not::not(1u16) as u16,
        ) as usize;
        let green: alloc::vec::Vec<u16> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u16,
            length_expr,
            2
        );
        offset += length_expr * 2;
        let length_expr = core::ops::BitAnd::bitand(
            core::ops::Add::add(size as u16, 1u16 as u16) as u16,
            core::ops::Not::not(1u16) as u16,
        ) as usize;
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
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetGammaRampSizeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: u16,
}
impl FixedLengthFromBytes<32> for GetGammaRampSizeReply {
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
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetPermissionsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub permissions: Permission,
}
impl FixedLengthFromBytes<32> for GetPermissionsReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            permissions: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
pub const BAD_CLOCK_ERROR: u8 = 0;
pub const BAD_H_TIMINGS_ERROR: u8 = 1;
pub const BAD_V_TIMINGS_ERROR: u8 = 2;
pub const MODE_UNSUITABLE_ERROR: u8 = 3;
pub const EXTENSION_DISABLED_ERROR: u8 = 4;
pub const CLIENT_NOT_LOCAL_ERROR: u8 = 5;
pub const ZOOM_LOCKED_ERROR: u8 = 6;
