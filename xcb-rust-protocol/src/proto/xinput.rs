#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "XInputExtension";
pub type EventClass = u32;
pub type KeyCode = u8;
pub type DeviceId = u16;
pub type Fp1616 = i32;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Fp3232 {
    pub integral: i32,
    pub frac: u32,
}
impl FixedLengthSerialize<8> for Fp3232 {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let integral_bytes = self.integral.serialize_fixed();
        let frac_bytes = self.frac.serialize_fixed();
        [
            integral_bytes[0],
            integral_bytes[1],
            integral_bytes[2],
            integral_bytes[3],
            frac_bytes[0],
            frac_bytes[1],
            frac_bytes[2],
            frac_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for Fp3232 {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            integral: i32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            frac: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetExtensionVersionReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
    pub present: u8,
}
impl FixedLengthFromBytes<32> for GetExtensionVersionReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xi_reply_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            server_major: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            server_minor: u16::from_bytes(
                bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            present: u8::from_bytes(bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct DeviceUseEnum(pub u8);
impl DeviceUseEnum {
    pub const IS_X_POINTER: Self = Self(0);
    pub const IS_X_KEYBOARD: Self = Self(1);
    pub const IS_X_EXTENSION_DEVICE: Self = Self(2);
    pub const IS_X_EXTENSION_KEYBOARD: Self = Self(3);
    pub const IS_X_EXTENSION_POINTER: Self = Self(4);
}
impl FixedLengthSerialize<1> for DeviceUseEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for DeviceUseEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for DeviceUseEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for DeviceUseEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for DeviceUseEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct InputClassEnum(pub u8);
impl InputClassEnum {
    pub const KEY: Self = Self(0);
    pub const BUTTON: Self = Self(1);
    pub const VALUATOR: Self = Self(2);
    pub const FEEDBACK: Self = Self(3);
    pub const PROXIMITY: Self = Self(4);
    pub const FOCUS: Self = Self(5);
    pub const OTHER: Self = Self(6);
}
impl FixedLengthSerialize<1> for InputClassEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for InputClassEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for InputClassEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for InputClassEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for InputClassEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ValuatorModeEnum(pub u8);
impl ValuatorModeEnum {
    pub const RELATIVE: Self = Self(0);
    pub const ABSOLUTE: Self = Self(1);
}
impl FixedLengthSerialize<1> for ValuatorModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ValuatorModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ValuatorModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ValuatorModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ValuatorModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceInfo {
    pub device_type: u32,
    pub device_id: u8,
    pub num_class_info: u8,
    pub device_use: DeviceUseEnum,
}
impl FixedLengthSerialize<8> for DeviceInfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let device_type_bytes = self.device_type.serialize_fixed();
        [
            device_type_bytes[0],
            device_type_bytes[1],
            device_type_bytes[2],
            device_type_bytes[3],
            self.device_id,
            self.num_class_info,
            self.device_use.0 as u8,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for DeviceInfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            device_type: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            device_id: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?,
            num_class_info: u8::from_bytes(bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?)?,
            device_use: u8::from_bytes(bytes.get(6..7).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct KeyInfo {
    pub class_id: InputClassEnum,
    pub len: u8,
    pub min_keycode: KeyCode,
    pub max_keycode: KeyCode,
    pub num_keys: u16,
}
impl FixedLengthSerialize<8> for KeyInfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let min_keycode_bytes = self.min_keycode.serialize_fixed();
        let max_keycode_bytes = self.max_keycode.serialize_fixed();
        let num_keys_bytes = self.num_keys.serialize_fixed();
        [
            self.class_id.0 as u8,
            self.len,
            min_keycode_bytes[0],
            max_keycode_bytes[0],
            num_keys_bytes[0],
            num_keys_bytes[1],
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for KeyInfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            class_id: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            len: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            min_keycode: KeyCode::from_bytes(
                bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?,
            )?,
            max_keycode: KeyCode::from_bytes(
                bytes.get(3..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            num_keys: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ButtonInfo {
    pub class_id: InputClassEnum,
    pub len: u8,
    pub num_buttons: u16,
}
impl FixedLengthSerialize<4> for ButtonInfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        let num_buttons_bytes = self.num_buttons.serialize_fixed();
        [
            self.class_id.0 as u8,
            self.len,
            num_buttons_bytes[0],
            num_buttons_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<4> for ButtonInfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            class_id: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            len: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            num_buttons: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct AxisInfo {
    pub resolution: u32,
    pub minimum: i32,
    pub maximum: i32,
}
impl FixedLengthSerialize<12> for AxisInfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let resolution_bytes = self.resolution.serialize_fixed();
        let minimum_bytes = self.minimum.serialize_fixed();
        let maximum_bytes = self.maximum.serialize_fixed();
        [
            resolution_bytes[0],
            resolution_bytes[1],
            resolution_bytes[2],
            resolution_bytes[3],
            minimum_bytes[0],
            minimum_bytes[1],
            minimum_bytes[2],
            minimum_bytes[3],
            maximum_bytes[0],
            maximum_bytes[1],
            maximum_bytes[2],
            maximum_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<12> for AxisInfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            resolution: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            minimum: i32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            maximum: i32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ValuatorInfo {
    pub class_id: InputClassEnum,
    pub len: u8,
    pub mode: ValuatorModeEnum,
    pub motion_size: u32,
    pub axes: alloc::vec::Vec<AxisInfo>,
}
impl VariableLengthSerialize for ValuatorInfo {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let axes_len = u8::try_from(self.axes.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.class_id.serialize_fixed());
        buf_ptr
            .get_mut(1..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.len.serialize_fixed());
        buf_ptr
            .get_mut(2..3)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(axes_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(3..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.mode.serialize_fixed());
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.motion_size.serialize_fixed());
        let list_len = self.axes.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            self.axes,
        )?;
        let offset = list_len + 8;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for ValuatorInfo {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let class_id = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let len = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let axes_len = u8::from_bytes(ptr.get(2..3).ok_or(crate::error::Error::FromBytes)?)?;
        let mode = u8::from_bytes(ptr.get(3..4).ok_or(crate::error::Error::FromBytes)?)?;
        let motion_size = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = axes_len as usize;
        let axes: alloc::vec::Vec<AxisInfo> = crate::vec_from_bytes_fixed!(
            ptr.get(8..).ok_or(crate::error::Error::FromBytes)?,
            AxisInfo,
            length_expr,
            12
        );
        let offset = length_expr * 12 + 8;
        Ok((
            Self {
                class_id: class_id.into(),
                len,
                mode: mode.into(),
                motion_size,
                axes,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct InputInfo {
    pub class_id: InputClassEnum,
    pub len: u8,
    pub input_info_switch_enum: InputInfoSwitchEnum,
}
impl VariableLengthSerialize for InputInfo {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.class_id.serialize_fixed());
        buf_ptr
            .get_mut(1..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.len.serialize_fixed());
        let offset = self
            .input_info_switch_enum
            .serialize_into(buf_ptr.get_mut(2..).ok_or(crate::error::Error::Serialize)?)?;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for InputInfo {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let class_id = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let len = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let (input_info_switch_enum, offset) = InputInfoSwitchEnum::from_bytes(
            ptr.get(2..).ok_or(crate::error::Error::FromBytes)?,
            class_id,
        )?;
        Ok((
            Self {
                class_id: class_id.into(),
                len,
                input_info_switch_enum,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct InputInfoSwitchEnumKey {
    pub min_keycode: KeyCode,
    pub max_keycode: KeyCode,
    pub num_keys: u16,
}
impl FixedLengthSerialize<6> for InputInfoSwitchEnumKey {
    #[inline]
    fn serialize_fixed(self) -> [u8; 6] {
        let min_keycode_bytes = self.min_keycode.serialize_fixed();
        let max_keycode_bytes = self.max_keycode.serialize_fixed();
        let num_keys_bytes = self.num_keys.serialize_fixed();
        [
            min_keycode_bytes[0],
            max_keycode_bytes[0],
            num_keys_bytes[0],
            num_keys_bytes[1],
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<6> for InputInfoSwitchEnumKey {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            min_keycode: KeyCode::from_bytes(
                bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?,
            )?,
            max_keycode: KeyCode::from_bytes(
                bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?,
            )?,
            num_keys: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct InputInfoSwitchEnumButton {
    pub num_buttons: u16,
}
impl FixedLengthSerialize<2> for InputInfoSwitchEnumButton {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        let num_buttons_bytes = self.num_buttons.serialize_fixed();
        [num_buttons_bytes[0], num_buttons_bytes[1]]
    }
}
impl FixedLengthFromBytes<2> for InputInfoSwitchEnumButton {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            num_buttons: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct InputInfoSwitchEnumValuator {
    pub mode: ValuatorModeEnum,
    pub motion_size: u32,
    pub axes: alloc::vec::Vec<AxisInfo>,
}
impl VariableLengthSerialize for InputInfoSwitchEnumValuator {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        // Start align 4 Some(2)
        let axes_len = u8::try_from(self.axes.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(axes_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(1..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.mode.serialize_fixed());
        buf_ptr
            .get_mut(2..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.motion_size.serialize_fixed());
        let list_len = self.axes.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(6..).ok_or(crate::error::Error::Serialize)?,
            self.axes,
        )?;
        let offset = list_len + 6;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for InputInfoSwitchEnumValuator {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Start align 4 Some(2)
        let axes_len = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let mode = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let motion_size = u32::from_bytes(ptr.get(2..6).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = axes_len as usize;
        let axes: alloc::vec::Vec<AxisInfo> = crate::vec_from_bytes_fixed!(
            ptr.get(6..).ok_or(crate::error::Error::FromBytes)?,
            AxisInfo,
            length_expr,
            12
        );
        let offset = length_expr * 12 + 6;
        Ok((
            Self {
                mode: mode.into(),
                motion_size,
                axes,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum InputInfoSwitchEnum {
    InputInfoSwitchEnumKey(InputInfoSwitchEnumKey),

    InputInfoSwitchEnumButton(InputInfoSwitchEnumButton),

    InputInfoSwitchEnumValuator(InputInfoSwitchEnumValuator),

    BadValue,
}
impl InputInfoSwitchEnum {
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        match self {
            Self::InputInfoSwitchEnumKey(case) => {
                buf_ptr
                    .get_mut(..6)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(6)
            }
            Self::InputInfoSwitchEnumButton(case) => {
                buf_ptr
                    .get_mut(..2)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(2)
            }
            Self::InputInfoSwitchEnumValuator(case) => case.serialize_into(buf_ptr),
            Self::BadValue => Ok(0),
        }
    }
}
impl InputInfoSwitchEnum {
    pub fn from_bytes(bytes: &[u8], class_id: u8) -> crate::error::Result<(Self, usize)> {
        let mask = class_id;
        if mask & InputClassEnum::KEY.0 == 0 {
            return Ok((
                Self::InputInfoSwitchEnumKey(InputInfoSwitchEnumKey::from_bytes(bytes)?),
                6,
            ));
        }
        if mask & InputClassEnum::BUTTON.0 == 0 {
            return Ok((
                Self::InputInfoSwitchEnumButton(InputInfoSwitchEnumButton::from_bytes(bytes)?),
                2,
            ));
        }
        if mask & InputClassEnum::VALUATOR.0 == 0 {
            let (parsed, offset) = InputInfoSwitchEnumValuator::from_bytes(bytes)?;
            return Ok((Self::InputInfoSwitchEnumValuator(parsed), offset));
        }
        Ok((Self::BadValue, 0))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceName {
    pub string: alloc::vec::Vec<u8>,
}
impl VariableLengthSerialize for DeviceName {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let len = u8::try_from(self.string.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(len).map_err(|_| crate::error::Error::Serialize)?).serialize_fixed(),
            );
        let list_len = self.string.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(1..).ok_or(crate::error::Error::Serialize)?,
            &self.string,
        )?;
        let offset = list_len + 1;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for DeviceName {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let len = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = len as usize;
        let string: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(1..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 1;
        Ok((Self { string }, offset))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListInputDevicesReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub devices: alloc::vec::Vec<DeviceInfo>,
    pub infos: alloc::vec::Vec<InputInfo>,
    pub names: alloc::vec::Vec<crate::proto::xproto::Str>,
}
impl VariableLengthFromBytes for ListInputDevicesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 23 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let xi_reply_type = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let devices_len = u8::from_bytes(ptr.get(8..9).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = devices_len as usize;
        let devices: alloc::vec::Vec<DeviceInfo> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            DeviceInfo,
            length_expr,
            8
        );
        let mut offset = length_expr * 8 + 32;
        let infos_length = devices.iter().try_fold(0u32, |start, val| {
            start
                .checked_add(
                    u32::try_from(val.num_class_info as usize)
                        .map_err(|_| crate::error::Error::TryFromInt)?,
                )
                .ok_or(crate::error::Error::TryFromInt)
        })? as usize;
        let infos = crate::vec_from_bytes_var!(ptr, InputInfo, offset, infos_length,);
        let names_length = devices_len as usize;
        let names =
            crate::vec_from_bytes_var!(ptr, crate::proto::xproto::Str, offset, names_length,);
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok((
            Self {
                response_type,
                xi_reply_type,
                sequence,
                length,
                devices,
                infos,
                names,
            },
            offset,
        ))
    }
}
pub type EventTypeBase = u8;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct InputClassInfo {
    pub class_id: InputClassEnum,
    pub event_type_base: EventTypeBase,
}
impl FixedLengthSerialize<2> for InputClassInfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        let event_type_base_bytes = self.event_type_base.serialize_fixed();
        [self.class_id.0 as u8, event_type_base_bytes[0]]
    }
}
impl FixedLengthFromBytes<2> for InputClassInfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            class_id: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            event_type_base: EventTypeBase::from_bytes(
                bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct OpenDeviceReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub class_info: alloc::vec::Vec<InputClassInfo>,
}
impl VariableLengthFromBytes for OpenDeviceReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 23 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let xi_reply_type = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_classes = u8::from_bytes(ptr.get(8..9).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_classes as usize;
        let class_info: alloc::vec::Vec<InputClassInfo> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            InputClassInfo,
            length_expr,
            2
        );
        let mut offset = length_expr * 2 + 32;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok((
            Self {
                response_type,
                xi_reply_type,
                sequence,
                length,
                class_info,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SetDeviceModeReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: crate::proto::xproto::GrabStatusEnum,
}
impl FixedLengthFromBytes<32> for SetDeviceModeReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xi_reply_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetSelectedExtensionEventsReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub this_classes: alloc::vec::Vec<EventClass>,
    pub all_classes: alloc::vec::Vec<EventClass>,
}
impl VariableLengthFromBytes for GetSelectedExtensionEventsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let xi_reply_type = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_this_classes =
            u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let num_all_classes =
            u16::from_bytes(ptr.get(10..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_this_classes as usize;
        let this_classes: alloc::vec::Vec<EventClass> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            EventClass,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 32;
        let length_expr = num_all_classes as usize;
        let all_classes: alloc::vec::Vec<EventClass> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            EventClass,
            length_expr,
            4
        );
        offset += length_expr * 4;
        Ok((
            Self {
                response_type,
                xi_reply_type,
                sequence,
                length,
                this_classes,
                all_classes,
            },
            offset,
        ))
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PropagateModeEnum(pub u8);
impl PropagateModeEnum {
    pub const ADD_TO_LIST: Self = Self(0);
    pub const DELETE_FROM_LIST: Self = Self(1);
}
impl FixedLengthSerialize<1> for PropagateModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for PropagateModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for PropagateModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for PropagateModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for PropagateModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetDeviceDontPropagateListReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub classes: alloc::vec::Vec<EventClass>,
}
impl VariableLengthFromBytes for GetDeviceDontPropagateListReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let xi_reply_type = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_classes = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_classes as usize;
        let classes: alloc::vec::Vec<EventClass> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            EventClass,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                xi_reply_type,
                sequence,
                length,
                classes,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceTimeCoord {
    pub time: crate::proto::xproto::Timestamp,
    pub axisvalues: alloc::vec::Vec<i32>,
}
impl VariableLengthSerialize for DeviceTimeCoord {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        buf_ptr
            .get_mut(0..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.time.serialize_fixed());
        let list_len = self.axisvalues.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?,
            self.axisvalues,
        )?;
        let offset = list_len + 4;
        Ok(offset)
    }
}
impl DeviceTimeCoord {
    fn from_bytes(bytes: &[u8], num_axes: u8) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let time = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(0..4).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let length_expr = num_axes as usize;
        let axisvalues: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            i32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 4;
        Ok((Self { time, axisvalues }, offset))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetDeviceMotionEventsReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_axes: u8,
    pub device_mode: ValuatorModeEnum,
    pub events: alloc::vec::Vec<DeviceTimeCoord>,
}
impl VariableLengthFromBytes for GetDeviceMotionEventsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 18 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let xi_reply_type = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_events = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let num_axes = u8::from_bytes(ptr.get(12..13).ok_or(crate::error::Error::FromBytes)?)?;
        let device_mode = u8::from_bytes(ptr.get(13..14).ok_or(crate::error::Error::FromBytes)?)?;
        let events_length = num_events as usize;
        let mut offset = 32;
        let events =
            crate::vec_from_bytes_var!(ptr, DeviceTimeCoord, offset, events_length, num_axes);
        Ok((
            Self {
                response_type,
                xi_reply_type,
                sequence,
                length,
                num_axes,
                device_mode: device_mode.into(),
                events,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ChangeKeyboardDeviceReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: crate::proto::xproto::GrabStatusEnum,
}
impl FixedLengthFromBytes<32> for ChangeKeyboardDeviceReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xi_reply_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ChangePointerDeviceReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: crate::proto::xproto::GrabStatusEnum,
}
impl FixedLengthFromBytes<32> for ChangePointerDeviceReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xi_reply_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GrabDeviceReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: crate::proto::xproto::GrabStatusEnum,
}
impl FixedLengthFromBytes<32> for GrabDeviceReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xi_reply_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ModifierDeviceEnum(pub u8);
impl ModifierDeviceEnum {
    pub const USE_X_KEYBOARD: Self = Self(255);
}
impl FixedLengthSerialize<1> for ModifierDeviceEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ModifierDeviceEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ModifierDeviceEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ModifierDeviceEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ModifierDeviceEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct DeviceInputModeEnum(pub u8);
impl DeviceInputModeEnum {
    pub const ASYNC_THIS_DEVICE: Self = Self(0);
    pub const SYNC_THIS_DEVICE: Self = Self(1);
    pub const REPLAY_THIS_DEVICE: Self = Self(2);
    pub const ASYNC_OTHER_DEVICES: Self = Self(3);
    pub const ASYNC_ALL: Self = Self(4);
    pub const SYNC_ALL: Self = Self(5);
}
impl FixedLengthSerialize<1> for DeviceInputModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for DeviceInputModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for DeviceInputModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for DeviceInputModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for DeviceInputModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetDeviceFocusReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub focus: crate::proto::xproto::Window,
    pub time: crate::proto::xproto::Timestamp,
    pub revert_to: crate::proto::xproto::InputFocusEnum,
}
impl FixedLengthFromBytes<32> for GetDeviceFocusReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xi_reply_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            focus: crate::proto::xproto::Window::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            revert_to: u8::from_bytes(bytes.get(16..17).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct FeedbackClassEnum(pub u8);
impl FeedbackClassEnum {
    pub const KEYBOARD: Self = Self(0);
    pub const POINTER: Self = Self(1);
    pub const STRING: Self = Self(2);
    pub const INTEGER: Self = Self(3);
    pub const LED: Self = Self(4);
    pub const BELL: Self = Self(5);
}
impl FixedLengthSerialize<1> for FeedbackClassEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for FeedbackClassEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for FeedbackClassEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for FeedbackClassEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for FeedbackClassEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct KbdFeedbackState {
    pub class_id: FeedbackClassEnum,
    pub feedback_id: u8,
    pub len: u16,
    pub pitch: u16,
    pub duration: u16,
    pub led_mask: u32,
    pub led_values: u32,
    pub global_auto_repeat: u8,
    pub click: u8,
    pub percent: u8,
    pub auto_repeats: [u8; 32],
}
impl FixedLengthSerialize<52> for KbdFeedbackState {
    #[inline]
    fn serialize_fixed(self) -> [u8; 52] {
        let len_bytes = self.len.serialize_fixed();
        let pitch_bytes = self.pitch.serialize_fixed();
        let duration_bytes = self.duration.serialize_fixed();
        let led_mask_bytes = self.led_mask.serialize_fixed();
        let led_values_bytes = self.led_values.serialize_fixed();
        [
            self.class_id.0 as u8,
            self.feedback_id,
            len_bytes[0],
            len_bytes[1],
            pitch_bytes[0],
            pitch_bytes[1],
            duration_bytes[0],
            duration_bytes[1],
            led_mask_bytes[0],
            led_mask_bytes[1],
            led_mask_bytes[2],
            led_mask_bytes[3],
            led_values_bytes[0],
            led_values_bytes[1],
            led_values_bytes[2],
            led_values_bytes[3],
            self.global_auto_repeat,
            self.click,
            self.percent,
            0,
            self.auto_repeats[0],
            self.auto_repeats[1],
            self.auto_repeats[2],
            self.auto_repeats[3],
            self.auto_repeats[4],
            self.auto_repeats[5],
            self.auto_repeats[6],
            self.auto_repeats[7],
            self.auto_repeats[8],
            self.auto_repeats[9],
            self.auto_repeats[10],
            self.auto_repeats[11],
            self.auto_repeats[12],
            self.auto_repeats[13],
            self.auto_repeats[14],
            self.auto_repeats[15],
            self.auto_repeats[16],
            self.auto_repeats[17],
            self.auto_repeats[18],
            self.auto_repeats[19],
            self.auto_repeats[20],
            self.auto_repeats[21],
            self.auto_repeats[22],
            self.auto_repeats[23],
            self.auto_repeats[24],
            self.auto_repeats[25],
            self.auto_repeats[26],
            self.auto_repeats[27],
            self.auto_repeats[28],
            self.auto_repeats[29],
            self.auto_repeats[30],
            self.auto_repeats[31],
        ]
    }
}
impl FixedLengthFromBytes<52> for KbdFeedbackState {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            class_id: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            feedback_id: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            pitch: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            duration: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
            led_mask: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            led_values: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            global_auto_repeat: u8::from_bytes(
                bytes.get(16..17).ok_or(crate::error::Error::FromBytes)?,
            )?,
            click: u8::from_bytes(bytes.get(17..18).ok_or(crate::error::Error::FromBytes)?)?,
            percent: u8::from_bytes(bytes.get(18..19).ok_or(crate::error::Error::FromBytes)?)?,
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
pub struct PtrFeedbackState {
    pub class_id: FeedbackClassEnum,
    pub feedback_id: u8,
    pub len: u16,
    pub accel_num: u16,
    pub accel_denom: u16,
    pub threshold: u16,
}
impl FixedLengthSerialize<12> for PtrFeedbackState {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let len_bytes = self.len.serialize_fixed();
        let accel_num_bytes = self.accel_num.serialize_fixed();
        let accel_denom_bytes = self.accel_denom.serialize_fixed();
        let threshold_bytes = self.threshold.serialize_fixed();
        [
            self.class_id.0 as u8,
            self.feedback_id,
            len_bytes[0],
            len_bytes[1],
            0,
            0,
            accel_num_bytes[0],
            accel_num_bytes[1],
            accel_denom_bytes[0],
            accel_denom_bytes[1],
            threshold_bytes[0],
            threshold_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<12> for PtrFeedbackState {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            class_id: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            feedback_id: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            accel_num: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
            accel_denom: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            threshold: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct IntegerFeedbackState {
    pub class_id: FeedbackClassEnum,
    pub feedback_id: u8,
    pub len: u16,
    pub resolution: u32,
    pub min_value: i32,
    pub max_value: i32,
}
impl FixedLengthSerialize<16> for IntegerFeedbackState {
    #[inline]
    fn serialize_fixed(self) -> [u8; 16] {
        let len_bytes = self.len.serialize_fixed();
        let resolution_bytes = self.resolution.serialize_fixed();
        let min_value_bytes = self.min_value.serialize_fixed();
        let max_value_bytes = self.max_value.serialize_fixed();
        [
            self.class_id.0 as u8,
            self.feedback_id,
            len_bytes[0],
            len_bytes[1],
            resolution_bytes[0],
            resolution_bytes[1],
            resolution_bytes[2],
            resolution_bytes[3],
            min_value_bytes[0],
            min_value_bytes[1],
            min_value_bytes[2],
            min_value_bytes[3],
            max_value_bytes[0],
            max_value_bytes[1],
            max_value_bytes[2],
            max_value_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<16> for IntegerFeedbackState {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            class_id: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            feedback_id: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            resolution: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            min_value: i32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            max_value: i32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct StringFeedbackState {
    pub class_id: FeedbackClassEnum,
    pub feedback_id: u8,
    pub len: u16,
    pub max_symbols: u16,
    pub keysyms: alloc::vec::Vec<crate::proto::xproto::Keysym>,
}
impl VariableLengthSerialize for StringFeedbackState {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let num_keysyms =
            u16::try_from(self.keysyms.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.class_id.serialize_fixed());
        buf_ptr
            .get_mut(1..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.feedback_id.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.len.serialize_fixed());
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.max_symbols.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_keysyms).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.keysyms.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            self.keysyms,
        )?;
        let offset = list_len + 8;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for StringFeedbackState {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let class_id = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let feedback_id = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let len = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let max_symbols = u16::from_bytes(ptr.get(4..6).ok_or(crate::error::Error::FromBytes)?)?;
        let num_keysyms = u16::from_bytes(ptr.get(6..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_keysyms as usize;
        let keysyms: alloc::vec::Vec<crate::proto::xproto::Keysym> = crate::vec_from_bytes_fixed!(
            ptr.get(8..).ok_or(crate::error::Error::FromBytes)?,
            crate::proto::xproto::Keysym,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 8;
        Ok((
            Self {
                class_id: class_id.into(),
                feedback_id,
                len,
                max_symbols,
                keysyms,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct BellFeedbackState {
    pub class_id: FeedbackClassEnum,
    pub feedback_id: u8,
    pub len: u16,
    pub percent: u8,
    pub pitch: u16,
    pub duration: u16,
}
impl FixedLengthSerialize<12> for BellFeedbackState {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let len_bytes = self.len.serialize_fixed();
        let pitch_bytes = self.pitch.serialize_fixed();
        let duration_bytes = self.duration.serialize_fixed();
        [
            self.class_id.0 as u8,
            self.feedback_id,
            len_bytes[0],
            len_bytes[1],
            self.percent,
            0,
            0,
            0,
            pitch_bytes[0],
            pitch_bytes[1],
            duration_bytes[0],
            duration_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<12> for BellFeedbackState {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            class_id: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            feedback_id: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            percent: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?,
            pitch: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            duration: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct LedFeedbackState {
    pub class_id: FeedbackClassEnum,
    pub feedback_id: u8,
    pub len: u16,
    pub led_mask: u32,
    pub led_values: u32,
}
impl FixedLengthSerialize<12> for LedFeedbackState {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let len_bytes = self.len.serialize_fixed();
        let led_mask_bytes = self.led_mask.serialize_fixed();
        let led_values_bytes = self.led_values.serialize_fixed();
        [
            self.class_id.0 as u8,
            self.feedback_id,
            len_bytes[0],
            len_bytes[1],
            led_mask_bytes[0],
            led_mask_bytes[1],
            led_mask_bytes[2],
            led_mask_bytes[3],
            led_values_bytes[0],
            led_values_bytes[1],
            led_values_bytes[2],
            led_values_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<12> for LedFeedbackState {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            class_id: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            feedback_id: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            led_mask: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            led_values: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct FeedbackState {
    pub class_id: FeedbackClassEnum,
    pub feedback_id: u8,
    pub len: u16,
    pub feedback_state_switch_enum: FeedbackStateSwitchEnum,
}
impl VariableLengthSerialize for FeedbackState {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.class_id.serialize_fixed());
        buf_ptr
            .get_mut(1..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.feedback_id.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.len.serialize_fixed());
        let offset = self
            .feedback_state_switch_enum
            .serialize_into(buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?)?;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for FeedbackState {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let class_id = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let feedback_id = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let len = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let (feedback_state_switch_enum, offset) = FeedbackStateSwitchEnum::from_bytes(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            class_id,
        )?;
        Ok((
            Self {
                class_id: class_id.into(),
                feedback_id,
                len,
                feedback_state_switch_enum,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct FeedbackStateSwitchEnumKeyboard {
    pub pitch: u16,
    pub duration: u16,
    pub led_mask: u32,
    pub led_values: u32,
    pub global_auto_repeat: u8,
    pub click: u8,
    pub percent: u8,
    pub auto_repeats: [u8; 32],
}
impl FixedLengthSerialize<48> for FeedbackStateSwitchEnumKeyboard {
    #[inline]
    fn serialize_fixed(self) -> [u8; 48] {
        let pitch_bytes = self.pitch.serialize_fixed();
        let duration_bytes = self.duration.serialize_fixed();
        let led_mask_bytes = self.led_mask.serialize_fixed();
        let led_values_bytes = self.led_values.serialize_fixed();
        [
            pitch_bytes[0],
            pitch_bytes[1],
            duration_bytes[0],
            duration_bytes[1],
            led_mask_bytes[0],
            led_mask_bytes[1],
            led_mask_bytes[2],
            led_mask_bytes[3],
            led_values_bytes[0],
            led_values_bytes[1],
            led_values_bytes[2],
            led_values_bytes[3],
            self.global_auto_repeat,
            self.click,
            self.percent,
            0,
            self.auto_repeats[0],
            self.auto_repeats[1],
            self.auto_repeats[2],
            self.auto_repeats[3],
            self.auto_repeats[4],
            self.auto_repeats[5],
            self.auto_repeats[6],
            self.auto_repeats[7],
            self.auto_repeats[8],
            self.auto_repeats[9],
            self.auto_repeats[10],
            self.auto_repeats[11],
            self.auto_repeats[12],
            self.auto_repeats[13],
            self.auto_repeats[14],
            self.auto_repeats[15],
            self.auto_repeats[16],
            self.auto_repeats[17],
            self.auto_repeats[18],
            self.auto_repeats[19],
            self.auto_repeats[20],
            self.auto_repeats[21],
            self.auto_repeats[22],
            self.auto_repeats[23],
            self.auto_repeats[24],
            self.auto_repeats[25],
            self.auto_repeats[26],
            self.auto_repeats[27],
            self.auto_repeats[28],
            self.auto_repeats[29],
            self.auto_repeats[30],
            self.auto_repeats[31],
        ]
    }
}
impl FixedLengthFromBytes<48> for FeedbackStateSwitchEnumKeyboard {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            pitch: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            duration: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            led_mask: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            led_values: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            global_auto_repeat: u8::from_bytes(
                bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?,
            )?,
            click: u8::from_bytes(bytes.get(13..14).ok_or(crate::error::Error::FromBytes)?)?,
            percent: u8::from_bytes(bytes.get(14..15).ok_or(crate::error::Error::FromBytes)?)?,
            // SAFETY: We know we can try into exact size slice
            auto_repeats: unsafe {
                bytes
                    .get(16..16 + 32)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct FeedbackStateSwitchEnumPointer {
    pub accel_num: u16,
    pub accel_denom: u16,
    pub threshold: u16,
}
impl FixedLengthSerialize<8> for FeedbackStateSwitchEnumPointer {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let accel_num_bytes = self.accel_num.serialize_fixed();
        let accel_denom_bytes = self.accel_denom.serialize_fixed();
        let threshold_bytes = self.threshold.serialize_fixed();
        [
            0,
            0,
            accel_num_bytes[0],
            accel_num_bytes[1],
            accel_denom_bytes[0],
            accel_denom_bytes[1],
            threshold_bytes[0],
            threshold_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<8> for FeedbackStateSwitchEnumPointer {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            accel_num: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            accel_denom: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            threshold: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct FeedbackStateSwitchEnumString {
    pub max_symbols: u16,
    pub keysyms: alloc::vec::Vec<crate::proto::xproto::Keysym>,
}
impl VariableLengthSerialize for FeedbackStateSwitchEnumString {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let num_keysyms =
            u16::try_from(self.keysyms.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.max_symbols.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_keysyms).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.keysyms.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?,
            self.keysyms,
        )?;
        let offset = list_len + 4;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for FeedbackStateSwitchEnumString {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let max_symbols = u16::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let num_keysyms = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_keysyms as usize;
        let keysyms: alloc::vec::Vec<crate::proto::xproto::Keysym> = crate::vec_from_bytes_fixed!(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            crate::proto::xproto::Keysym,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 4;
        Ok((
            Self {
                max_symbols,
                keysyms,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct FeedbackStateSwitchEnumInteger {
    pub resolution: u32,
    pub min_value: i32,
    pub max_value: i32,
}
impl FixedLengthSerialize<12> for FeedbackStateSwitchEnumInteger {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let resolution_bytes = self.resolution.serialize_fixed();
        let min_value_bytes = self.min_value.serialize_fixed();
        let max_value_bytes = self.max_value.serialize_fixed();
        [
            resolution_bytes[0],
            resolution_bytes[1],
            resolution_bytes[2],
            resolution_bytes[3],
            min_value_bytes[0],
            min_value_bytes[1],
            min_value_bytes[2],
            min_value_bytes[3],
            max_value_bytes[0],
            max_value_bytes[1],
            max_value_bytes[2],
            max_value_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<12> for FeedbackStateSwitchEnumInteger {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            resolution: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            min_value: i32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            max_value: i32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct FeedbackStateSwitchEnumLed {
    pub led_mask: u32,
    pub led_values: u32,
}
impl FixedLengthSerialize<8> for FeedbackStateSwitchEnumLed {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let led_mask_bytes = self.led_mask.serialize_fixed();
        let led_values_bytes = self.led_values.serialize_fixed();
        [
            led_mask_bytes[0],
            led_mask_bytes[1],
            led_mask_bytes[2],
            led_mask_bytes[3],
            led_values_bytes[0],
            led_values_bytes[1],
            led_values_bytes[2],
            led_values_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for FeedbackStateSwitchEnumLed {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            led_mask: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            led_values: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct FeedbackStateSwitchEnumBell {
    pub percent: u8,
    pub pitch: u16,
    pub duration: u16,
}
impl FixedLengthSerialize<8> for FeedbackStateSwitchEnumBell {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let pitch_bytes = self.pitch.serialize_fixed();
        let duration_bytes = self.duration.serialize_fixed();
        [
            self.percent,
            0,
            0,
            0,
            pitch_bytes[0],
            pitch_bytes[1],
            duration_bytes[0],
            duration_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<8> for FeedbackStateSwitchEnumBell {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            percent: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            pitch: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            duration: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum FeedbackStateSwitchEnum {
    FeedbackStateSwitchEnumKeyboard(FeedbackStateSwitchEnumKeyboard),

    FeedbackStateSwitchEnumPointer(FeedbackStateSwitchEnumPointer),

    FeedbackStateSwitchEnumString(FeedbackStateSwitchEnumString),

    FeedbackStateSwitchEnumInteger(FeedbackStateSwitchEnumInteger),

    FeedbackStateSwitchEnumLed(FeedbackStateSwitchEnumLed),

    FeedbackStateSwitchEnumBell(FeedbackStateSwitchEnumBell),

    BadValue,
}
impl FeedbackStateSwitchEnum {
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        match self {
            Self::FeedbackStateSwitchEnumKeyboard(case) => {
                buf_ptr
                    .get_mut(..48)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(48)
            }
            Self::FeedbackStateSwitchEnumPointer(case) => {
                buf_ptr
                    .get_mut(..8)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(8)
            }
            Self::FeedbackStateSwitchEnumString(case) => case.serialize_into(buf_ptr),
            Self::FeedbackStateSwitchEnumInteger(case) => {
                buf_ptr
                    .get_mut(..12)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(12)
            }
            Self::FeedbackStateSwitchEnumLed(case) => {
                buf_ptr
                    .get_mut(..8)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(8)
            }
            Self::FeedbackStateSwitchEnumBell(case) => {
                buf_ptr
                    .get_mut(..8)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(8)
            }
            Self::BadValue => Ok(0),
        }
    }
}
impl FeedbackStateSwitchEnum {
    pub fn from_bytes(bytes: &[u8], class_id: u8) -> crate::error::Result<(Self, usize)> {
        let mask = class_id;
        if mask & FeedbackClassEnum::KEYBOARD.0 == 0 {
            return Ok((
                Self::FeedbackStateSwitchEnumKeyboard(FeedbackStateSwitchEnumKeyboard::from_bytes(
                    bytes,
                )?),
                48,
            ));
        }
        if mask & FeedbackClassEnum::POINTER.0 == 0 {
            return Ok((
                Self::FeedbackStateSwitchEnumPointer(FeedbackStateSwitchEnumPointer::from_bytes(
                    bytes,
                )?),
                8,
            ));
        }
        if mask & FeedbackClassEnum::STRING.0 == 0 {
            let (parsed, offset) = FeedbackStateSwitchEnumString::from_bytes(bytes)?;
            return Ok((Self::FeedbackStateSwitchEnumString(parsed), offset));
        }
        if mask & FeedbackClassEnum::INTEGER.0 == 0 {
            return Ok((
                Self::FeedbackStateSwitchEnumInteger(FeedbackStateSwitchEnumInteger::from_bytes(
                    bytes,
                )?),
                12,
            ));
        }
        if mask & FeedbackClassEnum::LED.0 == 0 {
            return Ok((
                Self::FeedbackStateSwitchEnumLed(FeedbackStateSwitchEnumLed::from_bytes(bytes)?),
                8,
            ));
        }
        if mask & FeedbackClassEnum::BELL.0 == 0 {
            return Ok((
                Self::FeedbackStateSwitchEnumBell(FeedbackStateSwitchEnumBell::from_bytes(bytes)?),
                8,
            ));
        }
        Ok((Self::BadValue, 0))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetFeedbackControlReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub feedbacks: alloc::vec::Vec<FeedbackState>,
}
impl VariableLengthFromBytes for GetFeedbackControlReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let xi_reply_type = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_feedbacks = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let feedbacks_length = num_feedbacks as usize;
        let mut offset = 32;
        let feedbacks = crate::vec_from_bytes_var!(ptr, FeedbackState, offset, feedbacks_length,);
        Ok((
            Self {
                response_type,
                xi_reply_type,
                sequence,
                length,
                feedbacks,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct KbdFeedbackCtl {
    pub class_id: FeedbackClassEnum,
    pub feedback_id: u8,
    pub len: u16,
    pub key: KeyCode,
    pub auto_repeat_mode: u8,
    pub key_click_percent: i8,
    pub bell_percent: i8,
    pub bell_pitch: i16,
    pub bell_duration: i16,
    pub led_mask: u32,
    pub led_values: u32,
}
impl FixedLengthSerialize<20> for KbdFeedbackCtl {
    #[inline]
    fn serialize_fixed(self) -> [u8; 20] {
        let len_bytes = self.len.serialize_fixed();
        let key_bytes = self.key.serialize_fixed();
        let key_click_percent_bytes = self.key_click_percent.serialize_fixed();
        let bell_percent_bytes = self.bell_percent.serialize_fixed();
        let bell_pitch_bytes = self.bell_pitch.serialize_fixed();
        let bell_duration_bytes = self.bell_duration.serialize_fixed();
        let led_mask_bytes = self.led_mask.serialize_fixed();
        let led_values_bytes = self.led_values.serialize_fixed();
        [
            self.class_id.0 as u8,
            self.feedback_id,
            len_bytes[0],
            len_bytes[1],
            key_bytes[0],
            self.auto_repeat_mode,
            key_click_percent_bytes[0],
            bell_percent_bytes[0],
            bell_pitch_bytes[0],
            bell_pitch_bytes[1],
            bell_duration_bytes[0],
            bell_duration_bytes[1],
            led_mask_bytes[0],
            led_mask_bytes[1],
            led_mask_bytes[2],
            led_mask_bytes[3],
            led_values_bytes[0],
            led_values_bytes[1],
            led_values_bytes[2],
            led_values_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<20> for KbdFeedbackCtl {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            class_id: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            feedback_id: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            key: KeyCode::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?,
            auto_repeat_mode: u8::from_bytes(
                bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?,
            )?,
            key_click_percent: i8::from_bytes(
                bytes.get(6..7).ok_or(crate::error::Error::FromBytes)?,
            )?,
            bell_percent: i8::from_bytes(bytes.get(7..8).ok_or(crate::error::Error::FromBytes)?)?,
            bell_pitch: i16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            bell_duration: i16::from_bytes(
                bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            led_mask: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            led_values: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct PtrFeedbackCtl {
    pub class_id: FeedbackClassEnum,
    pub feedback_id: u8,
    pub len: u16,
    pub num: i16,
    pub denom: i16,
    pub threshold: i16,
}
impl FixedLengthSerialize<12> for PtrFeedbackCtl {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let len_bytes = self.len.serialize_fixed();
        let num_bytes = self.num.serialize_fixed();
        let denom_bytes = self.denom.serialize_fixed();
        let threshold_bytes = self.threshold.serialize_fixed();
        [
            self.class_id.0 as u8,
            self.feedback_id,
            len_bytes[0],
            len_bytes[1],
            0,
            0,
            num_bytes[0],
            num_bytes[1],
            denom_bytes[0],
            denom_bytes[1],
            threshold_bytes[0],
            threshold_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<12> for PtrFeedbackCtl {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            class_id: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            feedback_id: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            num: i16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
            denom: i16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            threshold: i16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct IntegerFeedbackCtl {
    pub class_id: FeedbackClassEnum,
    pub feedback_id: u8,
    pub len: u16,
    pub int_to_display: i32,
}
impl FixedLengthSerialize<8> for IntegerFeedbackCtl {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let len_bytes = self.len.serialize_fixed();
        let int_to_display_bytes = self.int_to_display.serialize_fixed();
        [
            self.class_id.0 as u8,
            self.feedback_id,
            len_bytes[0],
            len_bytes[1],
            int_to_display_bytes[0],
            int_to_display_bytes[1],
            int_to_display_bytes[2],
            int_to_display_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for IntegerFeedbackCtl {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            class_id: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            feedback_id: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            int_to_display: i32::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct StringFeedbackCtl {
    pub class_id: FeedbackClassEnum,
    pub feedback_id: u8,
    pub len: u16,
    pub keysyms: alloc::vec::Vec<crate::proto::xproto::Keysym>,
}
impl VariableLengthSerialize for StringFeedbackCtl {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        // Padding 2 bytes
        let num_keysyms =
            u16::try_from(self.keysyms.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.class_id.serialize_fixed());
        buf_ptr
            .get_mut(1..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.feedback_id.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.len.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_keysyms).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.keysyms.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            self.keysyms,
        )?;
        let offset = list_len + 8;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for StringFeedbackCtl {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 2 bytes
        let class_id = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let feedback_id = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let len = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let num_keysyms = u16::from_bytes(ptr.get(6..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_keysyms as usize;
        let keysyms: alloc::vec::Vec<crate::proto::xproto::Keysym> = crate::vec_from_bytes_fixed!(
            ptr.get(8..).ok_or(crate::error::Error::FromBytes)?,
            crate::proto::xproto::Keysym,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 8;
        Ok((
            Self {
                class_id: class_id.into(),
                feedback_id,
                len,
                keysyms,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct BellFeedbackCtl {
    pub class_id: FeedbackClassEnum,
    pub feedback_id: u8,
    pub len: u16,
    pub percent: i8,
    pub pitch: i16,
    pub duration: i16,
}
impl FixedLengthSerialize<12> for BellFeedbackCtl {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let len_bytes = self.len.serialize_fixed();
        let percent_bytes = self.percent.serialize_fixed();
        let pitch_bytes = self.pitch.serialize_fixed();
        let duration_bytes = self.duration.serialize_fixed();
        [
            self.class_id.0 as u8,
            self.feedback_id,
            len_bytes[0],
            len_bytes[1],
            percent_bytes[0],
            0,
            0,
            0,
            pitch_bytes[0],
            pitch_bytes[1],
            duration_bytes[0],
            duration_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<12> for BellFeedbackCtl {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            class_id: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            feedback_id: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            percent: i8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?,
            pitch: i16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            duration: i16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct LedFeedbackCtl {
    pub class_id: FeedbackClassEnum,
    pub feedback_id: u8,
    pub len: u16,
    pub led_mask: u32,
    pub led_values: u32,
}
impl FixedLengthSerialize<12> for LedFeedbackCtl {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let len_bytes = self.len.serialize_fixed();
        let led_mask_bytes = self.led_mask.serialize_fixed();
        let led_values_bytes = self.led_values.serialize_fixed();
        [
            self.class_id.0 as u8,
            self.feedback_id,
            len_bytes[0],
            len_bytes[1],
            led_mask_bytes[0],
            led_mask_bytes[1],
            led_mask_bytes[2],
            led_mask_bytes[3],
            led_values_bytes[0],
            led_values_bytes[1],
            led_values_bytes[2],
            led_values_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<12> for LedFeedbackCtl {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            class_id: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            feedback_id: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            led_mask: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            led_values: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct FeedbackCtl {
    pub class_id: FeedbackClassEnum,
    pub feedback_id: u8,
    pub len: u16,
    pub feedback_ctl_switch_enum: FeedbackCtlSwitchEnum,
}
impl VariableLengthSerialize for FeedbackCtl {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.class_id.serialize_fixed());
        buf_ptr
            .get_mut(1..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.feedback_id.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.len.serialize_fixed());
        let offset = self
            .feedback_ctl_switch_enum
            .serialize_into(buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?)?;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for FeedbackCtl {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let class_id = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let feedback_id = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let len = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let (feedback_ctl_switch_enum, offset) = FeedbackCtlSwitchEnum::from_bytes(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            class_id,
        )?;
        Ok((
            Self {
                class_id: class_id.into(),
                feedback_id,
                len,
                feedback_ctl_switch_enum,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct FeedbackCtlSwitchEnumKeyboard {
    pub key: KeyCode,
    pub auto_repeat_mode: u8,
    pub key_click_percent: i8,
    pub bell_percent: i8,
    pub bell_pitch: i16,
    pub bell_duration: i16,
    pub led_mask: u32,
    pub led_values: u32,
}
impl FixedLengthSerialize<16> for FeedbackCtlSwitchEnumKeyboard {
    #[inline]
    fn serialize_fixed(self) -> [u8; 16] {
        let key_bytes = self.key.serialize_fixed();
        let key_click_percent_bytes = self.key_click_percent.serialize_fixed();
        let bell_percent_bytes = self.bell_percent.serialize_fixed();
        let bell_pitch_bytes = self.bell_pitch.serialize_fixed();
        let bell_duration_bytes = self.bell_duration.serialize_fixed();
        let led_mask_bytes = self.led_mask.serialize_fixed();
        let led_values_bytes = self.led_values.serialize_fixed();
        [
            key_bytes[0],
            self.auto_repeat_mode,
            key_click_percent_bytes[0],
            bell_percent_bytes[0],
            bell_pitch_bytes[0],
            bell_pitch_bytes[1],
            bell_duration_bytes[0],
            bell_duration_bytes[1],
            led_mask_bytes[0],
            led_mask_bytes[1],
            led_mask_bytes[2],
            led_mask_bytes[3],
            led_values_bytes[0],
            led_values_bytes[1],
            led_values_bytes[2],
            led_values_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<16> for FeedbackCtlSwitchEnumKeyboard {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            key: KeyCode::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            auto_repeat_mode: u8::from_bytes(
                bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?,
            )?,
            key_click_percent: i8::from_bytes(
                bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?,
            )?,
            bell_percent: i8::from_bytes(bytes.get(3..4).ok_or(crate::error::Error::FromBytes)?)?,
            bell_pitch: i16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            bell_duration: i16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
            led_mask: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            led_values: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct FeedbackCtlSwitchEnumPointer {
    pub num: i16,
    pub denom: i16,
    pub threshold: i16,
}
impl FixedLengthSerialize<8> for FeedbackCtlSwitchEnumPointer {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let num_bytes = self.num.serialize_fixed();
        let denom_bytes = self.denom.serialize_fixed();
        let threshold_bytes = self.threshold.serialize_fixed();
        [
            0,
            0,
            num_bytes[0],
            num_bytes[1],
            denom_bytes[0],
            denom_bytes[1],
            threshold_bytes[0],
            threshold_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<8> for FeedbackCtlSwitchEnumPointer {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            num: i16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            denom: i16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            threshold: i16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct FeedbackCtlSwitchEnumString {
    pub keysyms: alloc::vec::Vec<crate::proto::xproto::Keysym>,
}
impl VariableLengthSerialize for FeedbackCtlSwitchEnumString {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        // Padding 2 bytes
        let num_keysyms =
            u16::try_from(self.keysyms.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_keysyms).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.keysyms.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?,
            self.keysyms,
        )?;
        let offset = list_len + 4;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for FeedbackCtlSwitchEnumString {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 2 bytes
        let num_keysyms = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_keysyms as usize;
        let keysyms: alloc::vec::Vec<crate::proto::xproto::Keysym> = crate::vec_from_bytes_fixed!(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            crate::proto::xproto::Keysym,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 4;
        Ok((Self { keysyms }, offset))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct FeedbackCtlSwitchEnumInteger {
    pub int_to_display: i32,
}
impl FixedLengthSerialize<4> for FeedbackCtlSwitchEnumInteger {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        let int_to_display_bytes = self.int_to_display.serialize_fixed();
        [
            int_to_display_bytes[0],
            int_to_display_bytes[1],
            int_to_display_bytes[2],
            int_to_display_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<4> for FeedbackCtlSwitchEnumInteger {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            int_to_display: i32::from_bytes(
                bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct FeedbackCtlSwitchEnumLed {
    pub led_mask: u32,
    pub led_values: u32,
}
impl FixedLengthSerialize<8> for FeedbackCtlSwitchEnumLed {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let led_mask_bytes = self.led_mask.serialize_fixed();
        let led_values_bytes = self.led_values.serialize_fixed();
        [
            led_mask_bytes[0],
            led_mask_bytes[1],
            led_mask_bytes[2],
            led_mask_bytes[3],
            led_values_bytes[0],
            led_values_bytes[1],
            led_values_bytes[2],
            led_values_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for FeedbackCtlSwitchEnumLed {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            led_mask: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            led_values: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct FeedbackCtlSwitchEnumBell {
    pub percent: i8,
    pub pitch: i16,
    pub duration: i16,
}
impl FixedLengthSerialize<8> for FeedbackCtlSwitchEnumBell {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let percent_bytes = self.percent.serialize_fixed();
        let pitch_bytes = self.pitch.serialize_fixed();
        let duration_bytes = self.duration.serialize_fixed();
        [
            percent_bytes[0],
            0,
            0,
            0,
            pitch_bytes[0],
            pitch_bytes[1],
            duration_bytes[0],
            duration_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<8> for FeedbackCtlSwitchEnumBell {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            percent: i8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            pitch: i16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            duration: i16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum FeedbackCtlSwitchEnum {
    FeedbackCtlSwitchEnumKeyboard(FeedbackCtlSwitchEnumKeyboard),

    FeedbackCtlSwitchEnumPointer(FeedbackCtlSwitchEnumPointer),

    FeedbackCtlSwitchEnumString(FeedbackCtlSwitchEnumString),

    FeedbackCtlSwitchEnumInteger(FeedbackCtlSwitchEnumInteger),

    FeedbackCtlSwitchEnumLed(FeedbackCtlSwitchEnumLed),

    FeedbackCtlSwitchEnumBell(FeedbackCtlSwitchEnumBell),

    BadValue,
}
impl FeedbackCtlSwitchEnum {
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        match self {
            Self::FeedbackCtlSwitchEnumKeyboard(case) => {
                buf_ptr
                    .get_mut(..16)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(16)
            }
            Self::FeedbackCtlSwitchEnumPointer(case) => {
                buf_ptr
                    .get_mut(..8)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(8)
            }
            Self::FeedbackCtlSwitchEnumString(case) => case.serialize_into(buf_ptr),
            Self::FeedbackCtlSwitchEnumInteger(case) => {
                buf_ptr
                    .get_mut(..4)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(4)
            }
            Self::FeedbackCtlSwitchEnumLed(case) => {
                buf_ptr
                    .get_mut(..8)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(8)
            }
            Self::FeedbackCtlSwitchEnumBell(case) => {
                buf_ptr
                    .get_mut(..8)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(8)
            }
            Self::BadValue => Ok(0),
        }
    }
}
impl FeedbackCtlSwitchEnum {
    pub fn from_bytes(bytes: &[u8], class_id: u8) -> crate::error::Result<(Self, usize)> {
        let mask = class_id;
        if mask & FeedbackClassEnum::KEYBOARD.0 == 0 {
            return Ok((
                Self::FeedbackCtlSwitchEnumKeyboard(FeedbackCtlSwitchEnumKeyboard::from_bytes(
                    bytes,
                )?),
                16,
            ));
        }
        if mask & FeedbackClassEnum::POINTER.0 == 0 {
            return Ok((
                Self::FeedbackCtlSwitchEnumPointer(FeedbackCtlSwitchEnumPointer::from_bytes(
                    bytes,
                )?),
                8,
            ));
        }
        if mask & FeedbackClassEnum::STRING.0 == 0 {
            let (parsed, offset) = FeedbackCtlSwitchEnumString::from_bytes(bytes)?;
            return Ok((Self::FeedbackCtlSwitchEnumString(parsed), offset));
        }
        if mask & FeedbackClassEnum::INTEGER.0 == 0 {
            return Ok((
                Self::FeedbackCtlSwitchEnumInteger(FeedbackCtlSwitchEnumInteger::from_bytes(
                    bytes,
                )?),
                4,
            ));
        }
        if mask & FeedbackClassEnum::LED.0 == 0 {
            return Ok((
                Self::FeedbackCtlSwitchEnumLed(FeedbackCtlSwitchEnumLed::from_bytes(bytes)?),
                8,
            ));
        }
        if mask & FeedbackClassEnum::BELL.0 == 0 {
            return Ok((
                Self::FeedbackCtlSwitchEnumBell(FeedbackCtlSwitchEnumBell::from_bytes(bytes)?),
                8,
            ));
        }
        Ok((Self::BadValue, 0))
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ChangeFeedbackControlMask(pub u32);
impl ChangeFeedbackControlMask {
    pub const KEY_CLICK_PERCENT: Self = Self(1 << 0);
    pub const PERCENT: Self = Self(1 << 1);
    pub const PITCH: Self = Self(1 << 2);
    pub const DURATION: Self = Self(1 << 3);
    pub const LED: Self = Self(1 << 4);
    pub const LED_MODE: Self = Self(1 << 5);
    pub const KEY: Self = Self(1 << 6);
    pub const AUTO_REPEAT_MODE: Self = Self(1 << 7);
    pub const STRING: Self = Self(1 << 0);
    pub const INTEGER: Self = Self(1 << 0);
    pub const ACCEL_NUM: Self = Self(1 << 0);
    pub const ACCEL_DENOM: Self = Self(1 << 1);
    pub const THRESHOLD: Self = Self(1 << 2);
}
impl FixedLengthSerialize<4> for ChangeFeedbackControlMask {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for ChangeFeedbackControlMask {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for ChangeFeedbackControlMask {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for ChangeFeedbackControlMask {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for ChangeFeedbackControlMask {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(ChangeFeedbackControlMask);
#[derive(Debug, Clone, PartialEq)]
pub struct GetDeviceKeyMappingReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub keysyms_per_keycode: u8,
    pub keysyms: alloc::vec::Vec<crate::proto::xproto::Keysym>,
}
impl VariableLengthFromBytes for GetDeviceKeyMappingReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 23 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let xi_reply_type = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let keysyms_per_keycode =
            u8::from_bytes(ptr.get(8..9).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = length as usize;
        let keysyms: alloc::vec::Vec<crate::proto::xproto::Keysym> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            crate::proto::xproto::Keysym,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 32;
        Ok((
            Self {
                response_type,
                xi_reply_type,
                sequence,
                length,
                keysyms_per_keycode,
                keysyms,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetDeviceModifierMappingReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub keymaps: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetDeviceModifierMappingReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 23 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let xi_reply_type = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let keycodes_per_modifier =
            u8::from_bytes(ptr.get(8..9).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(keycodes_per_modifier as u8, 8u8 as u8) as usize;
        let keymaps: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                xi_reply_type,
                sequence,
                length,
                keymaps,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SetDeviceModifierMappingReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: crate::proto::xproto::MappingStatusEnum,
}
impl FixedLengthFromBytes<32> for SetDeviceModifierMappingReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xi_reply_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetDeviceButtonMappingReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub map: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetDeviceButtonMappingReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 23 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let xi_reply_type = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let map_size = u8::from_bytes(ptr.get(8..9).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = map_size as usize;
        let map: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr + 32;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok((
            Self {
                response_type,
                xi_reply_type,
                sequence,
                length,
                map,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SetDeviceButtonMappingReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: crate::proto::xproto::MappingStatusEnum,
}
impl FixedLengthFromBytes<32> for SetDeviceButtonMappingReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xi_reply_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct KeyState {
    pub class_id: InputClassEnum,
    pub len: u8,
    pub num_keys: u8,
    pub keys: [u8; 32],
}
impl FixedLengthSerialize<36> for KeyState {
    #[inline]
    fn serialize_fixed(self) -> [u8; 36] {
        [
            self.class_id.0 as u8,
            self.len,
            self.num_keys,
            0,
            self.keys[0],
            self.keys[1],
            self.keys[2],
            self.keys[3],
            self.keys[4],
            self.keys[5],
            self.keys[6],
            self.keys[7],
            self.keys[8],
            self.keys[9],
            self.keys[10],
            self.keys[11],
            self.keys[12],
            self.keys[13],
            self.keys[14],
            self.keys[15],
            self.keys[16],
            self.keys[17],
            self.keys[18],
            self.keys[19],
            self.keys[20],
            self.keys[21],
            self.keys[22],
            self.keys[23],
            self.keys[24],
            self.keys[25],
            self.keys[26],
            self.keys[27],
            self.keys[28],
            self.keys[29],
            self.keys[30],
            self.keys[31],
        ]
    }
}
impl FixedLengthFromBytes<36> for KeyState {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            class_id: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            len: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            num_keys: u8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?,
            // SAFETY: We know we can try into exact size slice
            keys: unsafe {
                bytes
                    .get(4..4 + 32)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ButtonState {
    pub class_id: InputClassEnum,
    pub len: u8,
    pub num_buttons: u8,
    pub buttons: [u8; 32],
}
impl FixedLengthSerialize<36> for ButtonState {
    #[inline]
    fn serialize_fixed(self) -> [u8; 36] {
        [
            self.class_id.0 as u8,
            self.len,
            self.num_buttons,
            0,
            self.buttons[0],
            self.buttons[1],
            self.buttons[2],
            self.buttons[3],
            self.buttons[4],
            self.buttons[5],
            self.buttons[6],
            self.buttons[7],
            self.buttons[8],
            self.buttons[9],
            self.buttons[10],
            self.buttons[11],
            self.buttons[12],
            self.buttons[13],
            self.buttons[14],
            self.buttons[15],
            self.buttons[16],
            self.buttons[17],
            self.buttons[18],
            self.buttons[19],
            self.buttons[20],
            self.buttons[21],
            self.buttons[22],
            self.buttons[23],
            self.buttons[24],
            self.buttons[25],
            self.buttons[26],
            self.buttons[27],
            self.buttons[28],
            self.buttons[29],
            self.buttons[30],
            self.buttons[31],
        ]
    }
}
impl FixedLengthFromBytes<36> for ButtonState {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            class_id: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            len: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            num_buttons: u8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?,
            // SAFETY: We know we can try into exact size slice
            buttons: unsafe {
                bytes
                    .get(4..4 + 32)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ValuatorStateModeMask(pub u8);
impl ValuatorStateModeMask {
    pub const DEVICE_MODE_ABSOLUTE: Self = Self(1 << 0);
    pub const OUT_OF_PROXIMITY: Self = Self(1 << 1);
}
impl FixedLengthSerialize<1> for ValuatorStateModeMask {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ValuatorStateModeMask {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ValuatorStateModeMask {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ValuatorStateModeMask {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ValuatorStateModeMask {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(ValuatorStateModeMask);
#[derive(Debug, Clone, PartialEq)]
pub struct ValuatorState {
    pub class_id: InputClassEnum,
    pub len: u8,
    pub mode: ValuatorStateModeMask,
    pub valuators: alloc::vec::Vec<i32>,
}
impl VariableLengthSerialize for ValuatorState {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let num_valuators =
            u8::try_from(self.valuators.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.class_id.serialize_fixed());
        buf_ptr
            .get_mut(1..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.len.serialize_fixed());
        buf_ptr
            .get_mut(2..3)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(num_valuators).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(3..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.mode.serialize_fixed());
        let list_len = self.valuators.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?,
            self.valuators,
        )?;
        let offset = list_len + 4;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for ValuatorState {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let class_id = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let len = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let num_valuators = u8::from_bytes(ptr.get(2..3).ok_or(crate::error::Error::FromBytes)?)?;
        let mode = u8::from_bytes(ptr.get(3..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_valuators as usize;
        let valuators: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            i32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 4;
        Ok((
            Self {
                class_id: class_id.into(),
                len,
                mode: mode.into(),
                valuators,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct InputState {
    pub class_id: InputClassEnum,
    pub len: u8,
    pub input_state_switch_enum: InputStateSwitchEnum,
}
impl VariableLengthSerialize for InputState {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.class_id.serialize_fixed());
        buf_ptr
            .get_mut(1..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.len.serialize_fixed());
        let offset = self
            .input_state_switch_enum
            .serialize_into(buf_ptr.get_mut(2..).ok_or(crate::error::Error::Serialize)?)?;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for InputState {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let class_id = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let len = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let (input_state_switch_enum, offset) = InputStateSwitchEnum::from_bytes(
            ptr.get(2..).ok_or(crate::error::Error::FromBytes)?,
            class_id,
        )?;
        Ok((
            Self {
                class_id: class_id.into(),
                len,
                input_state_switch_enum,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct InputStateSwitchEnumKey {
    pub num_keys: u8,
    pub keys: [u8; 32],
}
impl FixedLengthSerialize<34> for InputStateSwitchEnumKey {
    #[inline]
    fn serialize_fixed(self) -> [u8; 34] {
        [
            self.num_keys,
            0,
            self.keys[0],
            self.keys[1],
            self.keys[2],
            self.keys[3],
            self.keys[4],
            self.keys[5],
            self.keys[6],
            self.keys[7],
            self.keys[8],
            self.keys[9],
            self.keys[10],
            self.keys[11],
            self.keys[12],
            self.keys[13],
            self.keys[14],
            self.keys[15],
            self.keys[16],
            self.keys[17],
            self.keys[18],
            self.keys[19],
            self.keys[20],
            self.keys[21],
            self.keys[22],
            self.keys[23],
            self.keys[24],
            self.keys[25],
            self.keys[26],
            self.keys[27],
            self.keys[28],
            self.keys[29],
            self.keys[30],
            self.keys[31],
        ]
    }
}
impl FixedLengthFromBytes<34> for InputStateSwitchEnumKey {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            num_keys: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            // SAFETY: We know we can try into exact size slice
            keys: unsafe {
                bytes
                    .get(2..2 + 32)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct InputStateSwitchEnumButton {
    pub num_buttons: u8,
    pub buttons: [u8; 32],
}
impl FixedLengthSerialize<34> for InputStateSwitchEnumButton {
    #[inline]
    fn serialize_fixed(self) -> [u8; 34] {
        [
            self.num_buttons,
            0,
            self.buttons[0],
            self.buttons[1],
            self.buttons[2],
            self.buttons[3],
            self.buttons[4],
            self.buttons[5],
            self.buttons[6],
            self.buttons[7],
            self.buttons[8],
            self.buttons[9],
            self.buttons[10],
            self.buttons[11],
            self.buttons[12],
            self.buttons[13],
            self.buttons[14],
            self.buttons[15],
            self.buttons[16],
            self.buttons[17],
            self.buttons[18],
            self.buttons[19],
            self.buttons[20],
            self.buttons[21],
            self.buttons[22],
            self.buttons[23],
            self.buttons[24],
            self.buttons[25],
            self.buttons[26],
            self.buttons[27],
            self.buttons[28],
            self.buttons[29],
            self.buttons[30],
            self.buttons[31],
        ]
    }
}
impl FixedLengthFromBytes<34> for InputStateSwitchEnumButton {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            num_buttons: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            // SAFETY: We know we can try into exact size slice
            buttons: unsafe {
                bytes
                    .get(2..2 + 32)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct InputStateSwitchEnumValuator {
    pub mode: ValuatorStateModeMask,
    pub valuators: alloc::vec::Vec<i32>,
}
impl VariableLengthSerialize for InputStateSwitchEnumValuator {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        // Start align 4 Some(2)
        let num_valuators =
            u8::try_from(self.valuators.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(num_valuators).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(1..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.mode.serialize_fixed());
        let list_len = self.valuators.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(2..).ok_or(crate::error::Error::Serialize)?,
            self.valuators,
        )?;
        let offset = list_len + 2;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for InputStateSwitchEnumValuator {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Start align 4 Some(2)
        let num_valuators = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let mode = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_valuators as usize;
        let valuators: alloc::vec::Vec<i32> = crate::vec_from_bytes_fixed!(
            ptr.get(2..).ok_or(crate::error::Error::FromBytes)?,
            i32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 2;
        Ok((
            Self {
                mode: mode.into(),
                valuators,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum InputStateSwitchEnum {
    InputStateSwitchEnumKey(InputStateSwitchEnumKey),

    InputStateSwitchEnumButton(InputStateSwitchEnumButton),

    InputStateSwitchEnumValuator(InputStateSwitchEnumValuator),

    BadValue,
}
impl InputStateSwitchEnum {
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        match self {
            Self::InputStateSwitchEnumKey(case) => {
                buf_ptr
                    .get_mut(..34)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(34)
            }
            Self::InputStateSwitchEnumButton(case) => {
                buf_ptr
                    .get_mut(..34)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(34)
            }
            Self::InputStateSwitchEnumValuator(case) => case.serialize_into(buf_ptr),
            Self::BadValue => Ok(0),
        }
    }
}
impl InputStateSwitchEnum {
    pub fn from_bytes(bytes: &[u8], class_id: u8) -> crate::error::Result<(Self, usize)> {
        let mask = class_id;
        if mask & InputClassEnum::KEY.0 == 0 {
            return Ok((
                Self::InputStateSwitchEnumKey(InputStateSwitchEnumKey::from_bytes(bytes)?),
                34,
            ));
        }
        if mask & InputClassEnum::BUTTON.0 == 0 {
            return Ok((
                Self::InputStateSwitchEnumButton(InputStateSwitchEnumButton::from_bytes(bytes)?),
                34,
            ));
        }
        if mask & InputClassEnum::VALUATOR.0 == 0 {
            let (parsed, offset) = InputStateSwitchEnumValuator::from_bytes(bytes)?;
            return Ok((Self::InputStateSwitchEnumValuator(parsed), offset));
        }
        Ok((Self::BadValue, 0))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryDeviceStateReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub classes: alloc::vec::Vec<InputState>,
}
impl VariableLengthFromBytes for QueryDeviceStateReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 23 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let xi_reply_type = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_classes = u8::from_bytes(ptr.get(8..9).ok_or(crate::error::Error::FromBytes)?)?;
        let classes_length = num_classes as usize;
        let mut offset = 32;
        let classes = crate::vec_from_bytes_var!(ptr, InputState, offset, classes_length,);
        Ok((
            Self {
                response_type,
                xi_reply_type,
                sequence,
                length,
                classes,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SetDeviceValuatorsReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: crate::proto::xproto::GrabStatusEnum,
}
impl FixedLengthFromBytes<32> for SetDeviceValuatorsReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xi_reply_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct DeviceControlEnum(pub u16);
impl DeviceControlEnum {
    pub const RESOLUTION: Self = Self(1);
    pub const ABS_CALIB: Self = Self(2);
    pub const CORE: Self = Self(3);
    pub const ENABLE: Self = Self(4);
    pub const ABS_AREA: Self = Self(5);
}
impl FixedLengthSerialize<2> for DeviceControlEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for DeviceControlEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for DeviceControlEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for DeviceControlEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for DeviceControlEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceResolutionState {
    pub control_id: DeviceControlEnum,
    pub len: u16,
    pub resolution_values: alloc::vec::Vec<u32>,
    pub resolution_min: alloc::vec::Vec<u32>,
    pub resolution_max: alloc::vec::Vec<u32>,
}
impl VariableLengthSerialize for DeviceResolutionState {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let num_valuators =
            u32::try_from(self.resolution_max.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.control_id.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.len.serialize_fixed());
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(num_valuators).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.resolution_values.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            self.resolution_values,
        )?;
        let mut offset = list_len + 8;
        let list_len = self.resolution_min.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            self.resolution_min,
        )?;
        offset += list_len;
        let list_len = self.resolution_max.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            self.resolution_max,
        )?;
        offset += list_len;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for DeviceResolutionState {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let control_id = u16::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let len = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let num_valuators = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_valuators as usize;
        let resolution_values: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(8..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 8;
        let length_expr = num_valuators as usize;
        let resolution_min: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        offset += length_expr * 4;
        let length_expr = num_valuators as usize;
        let resolution_max: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        offset += length_expr * 4;
        Ok((
            Self {
                control_id: control_id.into(),
                len,
                resolution_values,
                resolution_min,
                resolution_max,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceAbsCalibState {
    pub control_id: DeviceControlEnum,
    pub len: u16,
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub flip_x: u32,
    pub flip_y: u32,
    pub rotation: u32,
    pub button_threshold: u32,
}
impl FixedLengthSerialize<36> for DeviceAbsCalibState {
    #[inline]
    fn serialize_fixed(self) -> [u8; 36] {
        let control_id_bytes = self.control_id.serialize_fixed();
        let len_bytes = self.len.serialize_fixed();
        let min_x_bytes = self.min_x.serialize_fixed();
        let max_x_bytes = self.max_x.serialize_fixed();
        let min_y_bytes = self.min_y.serialize_fixed();
        let max_y_bytes = self.max_y.serialize_fixed();
        let flip_x_bytes = self.flip_x.serialize_fixed();
        let flip_y_bytes = self.flip_y.serialize_fixed();
        let rotation_bytes = self.rotation.serialize_fixed();
        let button_threshold_bytes = self.button_threshold.serialize_fixed();
        [
            control_id_bytes[0],
            control_id_bytes[1],
            len_bytes[0],
            len_bytes[1],
            min_x_bytes[0],
            min_x_bytes[1],
            min_x_bytes[2],
            min_x_bytes[3],
            max_x_bytes[0],
            max_x_bytes[1],
            max_x_bytes[2],
            max_x_bytes[3],
            min_y_bytes[0],
            min_y_bytes[1],
            min_y_bytes[2],
            min_y_bytes[3],
            max_y_bytes[0],
            max_y_bytes[1],
            max_y_bytes[2],
            max_y_bytes[3],
            flip_x_bytes[0],
            flip_x_bytes[1],
            flip_x_bytes[2],
            flip_x_bytes[3],
            flip_y_bytes[0],
            flip_y_bytes[1],
            flip_y_bytes[2],
            flip_y_bytes[3],
            rotation_bytes[0],
            rotation_bytes[1],
            rotation_bytes[2],
            rotation_bytes[3],
            button_threshold_bytes[0],
            button_threshold_bytes[1],
            button_threshold_bytes[2],
            button_threshold_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<36> for DeviceAbsCalibState {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            control_id: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            min_x: i32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            max_x: i32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            min_y: i32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            max_y: i32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            flip_x: u32::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
            flip_y: u32::from_bytes(bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?)?,
            rotation: u32::from_bytes(bytes.get(28..32).ok_or(crate::error::Error::FromBytes)?)?,
            button_threshold: u32::from_bytes(
                bytes.get(32..36).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceAbsAreaState {
    pub control_id: DeviceControlEnum,
    pub len: u16,
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: u32,
    pub height: u32,
    pub screen: u32,
    pub following: u32,
}
impl FixedLengthSerialize<28> for DeviceAbsAreaState {
    #[inline]
    fn serialize_fixed(self) -> [u8; 28] {
        let control_id_bytes = self.control_id.serialize_fixed();
        let len_bytes = self.len.serialize_fixed();
        let offset_x_bytes = self.offset_x.serialize_fixed();
        let offset_y_bytes = self.offset_y.serialize_fixed();
        let width_bytes = self.width.serialize_fixed();
        let height_bytes = self.height.serialize_fixed();
        let screen_bytes = self.screen.serialize_fixed();
        let following_bytes = self.following.serialize_fixed();
        [
            control_id_bytes[0],
            control_id_bytes[1],
            len_bytes[0],
            len_bytes[1],
            offset_x_bytes[0],
            offset_x_bytes[1],
            offset_x_bytes[2],
            offset_x_bytes[3],
            offset_y_bytes[0],
            offset_y_bytes[1],
            offset_y_bytes[2],
            offset_y_bytes[3],
            width_bytes[0],
            width_bytes[1],
            width_bytes[2],
            width_bytes[3],
            height_bytes[0],
            height_bytes[1],
            height_bytes[2],
            height_bytes[3],
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            following_bytes[0],
            following_bytes[1],
            following_bytes[2],
            following_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<28> for DeviceAbsAreaState {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            control_id: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            offset_x: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            offset_y: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            width: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            height: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            screen: u32::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
            following: u32::from_bytes(bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceCoreState {
    pub control_id: DeviceControlEnum,
    pub len: u16,
    pub status: u8,
    pub iscore: u8,
}
impl FixedLengthSerialize<8> for DeviceCoreState {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let control_id_bytes = self.control_id.serialize_fixed();
        let len_bytes = self.len.serialize_fixed();
        [
            control_id_bytes[0],
            control_id_bytes[1],
            len_bytes[0],
            len_bytes[1],
            self.status,
            self.iscore,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for DeviceCoreState {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            control_id: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?,
            iscore: u8::from_bytes(bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceEnableState {
    pub control_id: DeviceControlEnum,
    pub len: u16,
    pub enable: u8,
}
impl FixedLengthSerialize<8> for DeviceEnableState {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let control_id_bytes = self.control_id.serialize_fixed();
        let len_bytes = self.len.serialize_fixed();
        [
            control_id_bytes[0],
            control_id_bytes[1],
            len_bytes[0],
            len_bytes[1],
            self.enable,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for DeviceEnableState {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            control_id: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            enable: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceState {
    pub control_id: DeviceControlEnum,
    pub len: u16,
    pub device_state_switch_enum: DeviceStateSwitchEnum,
}
impl VariableLengthSerialize for DeviceState {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.control_id.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.len.serialize_fixed());
        let offset = self
            .device_state_switch_enum
            .serialize_into(buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?)?;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for DeviceState {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let control_id = u16::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let len = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let (device_state_switch_enum, offset) = DeviceStateSwitchEnum::from_bytes(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            control_id,
        )?;
        Ok((
            Self {
                control_id: control_id.into(),
                len,
                device_state_switch_enum,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceStateSwitchEnumResolution {
    pub resolution_values: alloc::vec::Vec<u32>,
    pub resolution_min: alloc::vec::Vec<u32>,
    pub resolution_max: alloc::vec::Vec<u32>,
}
impl VariableLengthSerialize for DeviceStateSwitchEnumResolution {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let num_valuators =
            u32::try_from(self.resolution_max.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(num_valuators).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.resolution_values.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?,
            self.resolution_values,
        )?;
        let mut offset = list_len + 4;
        let list_len = self.resolution_min.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            self.resolution_min,
        )?;
        offset += list_len;
        let list_len = self.resolution_max.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            self.resolution_max,
        )?;
        offset += list_len;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for DeviceStateSwitchEnumResolution {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let num_valuators = u32::from_bytes(ptr.get(0..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_valuators as usize;
        let resolution_values: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 4;
        let length_expr = num_valuators as usize;
        let resolution_min: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        offset += length_expr * 4;
        let length_expr = num_valuators as usize;
        let resolution_max: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        offset += length_expr * 4;
        Ok((
            Self {
                resolution_values,
                resolution_min,
                resolution_max,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceStateSwitchEnumAbsCalib {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub flip_x: u32,
    pub flip_y: u32,
    pub rotation: u32,
    pub button_threshold: u32,
}
impl FixedLengthSerialize<32> for DeviceStateSwitchEnumAbsCalib {
    #[inline]
    fn serialize_fixed(self) -> [u8; 32] {
        let min_x_bytes = self.min_x.serialize_fixed();
        let max_x_bytes = self.max_x.serialize_fixed();
        let min_y_bytes = self.min_y.serialize_fixed();
        let max_y_bytes = self.max_y.serialize_fixed();
        let flip_x_bytes = self.flip_x.serialize_fixed();
        let flip_y_bytes = self.flip_y.serialize_fixed();
        let rotation_bytes = self.rotation.serialize_fixed();
        let button_threshold_bytes = self.button_threshold.serialize_fixed();
        [
            min_x_bytes[0],
            min_x_bytes[1],
            min_x_bytes[2],
            min_x_bytes[3],
            max_x_bytes[0],
            max_x_bytes[1],
            max_x_bytes[2],
            max_x_bytes[3],
            min_y_bytes[0],
            min_y_bytes[1],
            min_y_bytes[2],
            min_y_bytes[3],
            max_y_bytes[0],
            max_y_bytes[1],
            max_y_bytes[2],
            max_y_bytes[3],
            flip_x_bytes[0],
            flip_x_bytes[1],
            flip_x_bytes[2],
            flip_x_bytes[3],
            flip_y_bytes[0],
            flip_y_bytes[1],
            flip_y_bytes[2],
            flip_y_bytes[3],
            rotation_bytes[0],
            rotation_bytes[1],
            rotation_bytes[2],
            rotation_bytes[3],
            button_threshold_bytes[0],
            button_threshold_bytes[1],
            button_threshold_bytes[2],
            button_threshold_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<32> for DeviceStateSwitchEnumAbsCalib {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            min_x: i32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            max_x: i32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            min_y: i32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            max_y: i32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            flip_x: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            flip_y: u32::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
            rotation: u32::from_bytes(bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?)?,
            button_threshold: u32::from_bytes(
                bytes.get(28..32).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceStateSwitchEnumCore {
    pub status: u8,
    pub iscore: u8,
}
impl FixedLengthSerialize<4> for DeviceStateSwitchEnumCore {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        [self.status, self.iscore, 0, 0]
    }
}
impl FixedLengthFromBytes<4> for DeviceStateSwitchEnumCore {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            status: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            iscore: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceStateSwitchEnumEnable {
    pub enable: u8,
}
impl FixedLengthSerialize<4> for DeviceStateSwitchEnumEnable {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        [self.enable, 0, 0, 0]
    }
}
impl FixedLengthFromBytes<4> for DeviceStateSwitchEnumEnable {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            enable: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceStateSwitchEnumAbsArea {
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: u32,
    pub height: u32,
    pub screen: u32,
    pub following: u32,
}
impl FixedLengthSerialize<24> for DeviceStateSwitchEnumAbsArea {
    #[inline]
    fn serialize_fixed(self) -> [u8; 24] {
        let offset_x_bytes = self.offset_x.serialize_fixed();
        let offset_y_bytes = self.offset_y.serialize_fixed();
        let width_bytes = self.width.serialize_fixed();
        let height_bytes = self.height.serialize_fixed();
        let screen_bytes = self.screen.serialize_fixed();
        let following_bytes = self.following.serialize_fixed();
        [
            offset_x_bytes[0],
            offset_x_bytes[1],
            offset_x_bytes[2],
            offset_x_bytes[3],
            offset_y_bytes[0],
            offset_y_bytes[1],
            offset_y_bytes[2],
            offset_y_bytes[3],
            width_bytes[0],
            width_bytes[1],
            width_bytes[2],
            width_bytes[3],
            height_bytes[0],
            height_bytes[1],
            height_bytes[2],
            height_bytes[3],
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            following_bytes[0],
            following_bytes[1],
            following_bytes[2],
            following_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<24> for DeviceStateSwitchEnumAbsArea {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            offset_x: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            offset_y: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            width: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            height: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            screen: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            following: u32::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceStateSwitchEnum {
    DeviceStateSwitchEnumResolution(DeviceStateSwitchEnumResolution),

    DeviceStateSwitchEnumAbsCalib(DeviceStateSwitchEnumAbsCalib),

    DeviceStateSwitchEnumCore(DeviceStateSwitchEnumCore),

    DeviceStateSwitchEnumEnable(DeviceStateSwitchEnumEnable),

    DeviceStateSwitchEnumAbsArea(DeviceStateSwitchEnumAbsArea),

    BadValue,
}
impl DeviceStateSwitchEnum {
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        match self {
            Self::DeviceStateSwitchEnumResolution(case) => case.serialize_into(buf_ptr),
            Self::DeviceStateSwitchEnumAbsCalib(case) => {
                buf_ptr
                    .get_mut(..32)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(32)
            }
            Self::DeviceStateSwitchEnumCore(case) => {
                buf_ptr
                    .get_mut(..4)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(4)
            }
            Self::DeviceStateSwitchEnumEnable(case) => {
                buf_ptr
                    .get_mut(..4)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(4)
            }
            Self::DeviceStateSwitchEnumAbsArea(case) => {
                buf_ptr
                    .get_mut(..24)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(24)
            }
            Self::BadValue => Ok(0),
        }
    }
}
impl DeviceStateSwitchEnum {
    pub fn from_bytes(bytes: &[u8], control_id: u16) -> crate::error::Result<(Self, usize)> {
        let mask = control_id;
        if mask & DeviceControlEnum::RESOLUTION.0 == 0 {
            let (parsed, offset) = DeviceStateSwitchEnumResolution::from_bytes(bytes)?;
            return Ok((Self::DeviceStateSwitchEnumResolution(parsed), offset));
        }
        if mask & DeviceControlEnum::ABS_CALIB.0 == 0 {
            return Ok((
                Self::DeviceStateSwitchEnumAbsCalib(DeviceStateSwitchEnumAbsCalib::from_bytes(
                    bytes,
                )?),
                32,
            ));
        }
        if mask & DeviceControlEnum::CORE.0 == 0 {
            return Ok((
                Self::DeviceStateSwitchEnumCore(DeviceStateSwitchEnumCore::from_bytes(bytes)?),
                4,
            ));
        }
        if mask & DeviceControlEnum::ENABLE.0 == 0 {
            return Ok((
                Self::DeviceStateSwitchEnumEnable(DeviceStateSwitchEnumEnable::from_bytes(bytes)?),
                4,
            ));
        }
        if mask & DeviceControlEnum::ABS_AREA.0 == 0 {
            return Ok((
                Self::DeviceStateSwitchEnumAbsArea(DeviceStateSwitchEnumAbsArea::from_bytes(
                    bytes,
                )?),
                24,
            ));
        }
        Ok((Self::BadValue, 0))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetDeviceControlReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: crate::proto::xproto::GrabStatusEnum,
    pub control: DeviceState,
}
impl VariableLengthFromBytes for GetDeviceControlReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 23 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let xi_reply_type = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let status = u8::from_bytes(ptr.get(8..9).ok_or(crate::error::Error::FromBytes)?)?;
        let (control, offset) =
            DeviceState::from_bytes(ptr.get(32..).ok_or(crate::error::Error::FromBytes)?)?;
        Ok((
            Self {
                response_type,
                xi_reply_type,
                sequence,
                length,
                status: status.into(),
                control,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceResolutionCtl {
    pub control_id: DeviceControlEnum,
    pub len: u16,
    pub first_valuator: u8,
    pub resolution_values: alloc::vec::Vec<u32>,
}
impl VariableLengthSerialize for DeviceResolutionCtl {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let num_valuators = u8::try_from(self.resolution_values.len())
            .map_err(|_| crate::error::Error::Serialize)?;
        // Padding 2 bytes
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.control_id.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.len.serialize_fixed());
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.first_valuator.serialize_fixed());
        buf_ptr
            .get_mut(5..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(num_valuators).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.resolution_values.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            self.resolution_values,
        )?;
        let offset = list_len + 8;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for DeviceResolutionCtl {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 2 bytes
        let control_id = u16::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let len = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let first_valuator = u8::from_bytes(ptr.get(4..5).ok_or(crate::error::Error::FromBytes)?)?;
        let num_valuators = u8::from_bytes(ptr.get(5..6).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_valuators as usize;
        let resolution_values: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(8..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 8;
        Ok((
            Self {
                control_id: control_id.into(),
                len,
                first_valuator,
                resolution_values,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceAbsCalibCtl {
    pub control_id: DeviceControlEnum,
    pub len: u16,
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub flip_x: u32,
    pub flip_y: u32,
    pub rotation: u32,
    pub button_threshold: u32,
}
impl FixedLengthSerialize<36> for DeviceAbsCalibCtl {
    #[inline]
    fn serialize_fixed(self) -> [u8; 36] {
        let control_id_bytes = self.control_id.serialize_fixed();
        let len_bytes = self.len.serialize_fixed();
        let min_x_bytes = self.min_x.serialize_fixed();
        let max_x_bytes = self.max_x.serialize_fixed();
        let min_y_bytes = self.min_y.serialize_fixed();
        let max_y_bytes = self.max_y.serialize_fixed();
        let flip_x_bytes = self.flip_x.serialize_fixed();
        let flip_y_bytes = self.flip_y.serialize_fixed();
        let rotation_bytes = self.rotation.serialize_fixed();
        let button_threshold_bytes = self.button_threshold.serialize_fixed();
        [
            control_id_bytes[0],
            control_id_bytes[1],
            len_bytes[0],
            len_bytes[1],
            min_x_bytes[0],
            min_x_bytes[1],
            min_x_bytes[2],
            min_x_bytes[3],
            max_x_bytes[0],
            max_x_bytes[1],
            max_x_bytes[2],
            max_x_bytes[3],
            min_y_bytes[0],
            min_y_bytes[1],
            min_y_bytes[2],
            min_y_bytes[3],
            max_y_bytes[0],
            max_y_bytes[1],
            max_y_bytes[2],
            max_y_bytes[3],
            flip_x_bytes[0],
            flip_x_bytes[1],
            flip_x_bytes[2],
            flip_x_bytes[3],
            flip_y_bytes[0],
            flip_y_bytes[1],
            flip_y_bytes[2],
            flip_y_bytes[3],
            rotation_bytes[0],
            rotation_bytes[1],
            rotation_bytes[2],
            rotation_bytes[3],
            button_threshold_bytes[0],
            button_threshold_bytes[1],
            button_threshold_bytes[2],
            button_threshold_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<36> for DeviceAbsCalibCtl {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            control_id: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            min_x: i32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            max_x: i32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            min_y: i32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            max_y: i32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            flip_x: u32::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
            flip_y: u32::from_bytes(bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?)?,
            rotation: u32::from_bytes(bytes.get(28..32).ok_or(crate::error::Error::FromBytes)?)?,
            button_threshold: u32::from_bytes(
                bytes.get(32..36).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceAbsAreaCtrl {
    pub control_id: DeviceControlEnum,
    pub len: u16,
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: i32,
    pub height: i32,
    pub screen: i32,
    pub following: u32,
}
impl FixedLengthSerialize<28> for DeviceAbsAreaCtrl {
    #[inline]
    fn serialize_fixed(self) -> [u8; 28] {
        let control_id_bytes = self.control_id.serialize_fixed();
        let len_bytes = self.len.serialize_fixed();
        let offset_x_bytes = self.offset_x.serialize_fixed();
        let offset_y_bytes = self.offset_y.serialize_fixed();
        let width_bytes = self.width.serialize_fixed();
        let height_bytes = self.height.serialize_fixed();
        let screen_bytes = self.screen.serialize_fixed();
        let following_bytes = self.following.serialize_fixed();
        [
            control_id_bytes[0],
            control_id_bytes[1],
            len_bytes[0],
            len_bytes[1],
            offset_x_bytes[0],
            offset_x_bytes[1],
            offset_x_bytes[2],
            offset_x_bytes[3],
            offset_y_bytes[0],
            offset_y_bytes[1],
            offset_y_bytes[2],
            offset_y_bytes[3],
            width_bytes[0],
            width_bytes[1],
            width_bytes[2],
            width_bytes[3],
            height_bytes[0],
            height_bytes[1],
            height_bytes[2],
            height_bytes[3],
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            following_bytes[0],
            following_bytes[1],
            following_bytes[2],
            following_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<28> for DeviceAbsAreaCtrl {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            control_id: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            offset_x: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            offset_y: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            width: i32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            height: i32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            screen: i32::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
            following: u32::from_bytes(bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceCoreCtrl {
    pub control_id: DeviceControlEnum,
    pub len: u16,
    pub status: u8,
}
impl FixedLengthSerialize<8> for DeviceCoreCtrl {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let control_id_bytes = self.control_id.serialize_fixed();
        let len_bytes = self.len.serialize_fixed();
        [
            control_id_bytes[0],
            control_id_bytes[1],
            len_bytes[0],
            len_bytes[1],
            self.status,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for DeviceCoreCtrl {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            control_id: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceEnableCtrl {
    pub control_id: DeviceControlEnum,
    pub len: u16,
    pub enable: u8,
}
impl FixedLengthSerialize<8> for DeviceEnableCtrl {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let control_id_bytes = self.control_id.serialize_fixed();
        let len_bytes = self.len.serialize_fixed();
        [
            control_id_bytes[0],
            control_id_bytes[1],
            len_bytes[0],
            len_bytes[1],
            self.enable,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for DeviceEnableCtrl {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            control_id: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            enable: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceCtl {
    pub control_id: DeviceControlEnum,
    pub len: u16,
    pub device_ctl_switch_enum: DeviceCtlSwitchEnum,
}
impl VariableLengthSerialize for DeviceCtl {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.control_id.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.len.serialize_fixed());
        let offset = self
            .device_ctl_switch_enum
            .serialize_into(buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?)?;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for DeviceCtl {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let control_id = u16::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let len = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let (device_ctl_switch_enum, offset) = DeviceCtlSwitchEnum::from_bytes(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            control_id,
        )?;
        Ok((
            Self {
                control_id: control_id.into(),
                len,
                device_ctl_switch_enum,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceCtlSwitchEnumResolution {
    pub first_valuator: u8,
    pub resolution_values: alloc::vec::Vec<u32>,
}
impl VariableLengthSerialize for DeviceCtlSwitchEnumResolution {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let num_valuators = u8::try_from(self.resolution_values.len())
            .map_err(|_| crate::error::Error::Serialize)?;
        // Padding 2 bytes
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.first_valuator.serialize_fixed());
        buf_ptr
            .get_mut(1..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(num_valuators).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.resolution_values.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?,
            self.resolution_values,
        )?;
        let offset = list_len + 4;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for DeviceCtlSwitchEnumResolution {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 2 bytes
        let first_valuator = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let num_valuators = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_valuators as usize;
        let resolution_values: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 4;
        Ok((
            Self {
                first_valuator,
                resolution_values,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceCtlSwitchEnumAbsCalib {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub flip_x: u32,
    pub flip_y: u32,
    pub rotation: u32,
    pub button_threshold: u32,
}
impl FixedLengthSerialize<32> for DeviceCtlSwitchEnumAbsCalib {
    #[inline]
    fn serialize_fixed(self) -> [u8; 32] {
        let min_x_bytes = self.min_x.serialize_fixed();
        let max_x_bytes = self.max_x.serialize_fixed();
        let min_y_bytes = self.min_y.serialize_fixed();
        let max_y_bytes = self.max_y.serialize_fixed();
        let flip_x_bytes = self.flip_x.serialize_fixed();
        let flip_y_bytes = self.flip_y.serialize_fixed();
        let rotation_bytes = self.rotation.serialize_fixed();
        let button_threshold_bytes = self.button_threshold.serialize_fixed();
        [
            min_x_bytes[0],
            min_x_bytes[1],
            min_x_bytes[2],
            min_x_bytes[3],
            max_x_bytes[0],
            max_x_bytes[1],
            max_x_bytes[2],
            max_x_bytes[3],
            min_y_bytes[0],
            min_y_bytes[1],
            min_y_bytes[2],
            min_y_bytes[3],
            max_y_bytes[0],
            max_y_bytes[1],
            max_y_bytes[2],
            max_y_bytes[3],
            flip_x_bytes[0],
            flip_x_bytes[1],
            flip_x_bytes[2],
            flip_x_bytes[3],
            flip_y_bytes[0],
            flip_y_bytes[1],
            flip_y_bytes[2],
            flip_y_bytes[3],
            rotation_bytes[0],
            rotation_bytes[1],
            rotation_bytes[2],
            rotation_bytes[3],
            button_threshold_bytes[0],
            button_threshold_bytes[1],
            button_threshold_bytes[2],
            button_threshold_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<32> for DeviceCtlSwitchEnumAbsCalib {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            min_x: i32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            max_x: i32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            min_y: i32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            max_y: i32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            flip_x: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            flip_y: u32::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
            rotation: u32::from_bytes(bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?)?,
            button_threshold: u32::from_bytes(
                bytes.get(28..32).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceCtlSwitchEnumCore {
    pub status: u8,
}
impl FixedLengthSerialize<4> for DeviceCtlSwitchEnumCore {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        [self.status, 0, 0, 0]
    }
}
impl FixedLengthFromBytes<4> for DeviceCtlSwitchEnumCore {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            status: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceCtlSwitchEnumEnable {
    pub enable: u8,
}
impl FixedLengthSerialize<4> for DeviceCtlSwitchEnumEnable {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        [self.enable, 0, 0, 0]
    }
}
impl FixedLengthFromBytes<4> for DeviceCtlSwitchEnumEnable {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            enable: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceCtlSwitchEnumAbsArea {
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: i32,
    pub height: i32,
    pub screen: i32,
    pub following: u32,
}
impl FixedLengthSerialize<24> for DeviceCtlSwitchEnumAbsArea {
    #[inline]
    fn serialize_fixed(self) -> [u8; 24] {
        let offset_x_bytes = self.offset_x.serialize_fixed();
        let offset_y_bytes = self.offset_y.serialize_fixed();
        let width_bytes = self.width.serialize_fixed();
        let height_bytes = self.height.serialize_fixed();
        let screen_bytes = self.screen.serialize_fixed();
        let following_bytes = self.following.serialize_fixed();
        [
            offset_x_bytes[0],
            offset_x_bytes[1],
            offset_x_bytes[2],
            offset_x_bytes[3],
            offset_y_bytes[0],
            offset_y_bytes[1],
            offset_y_bytes[2],
            offset_y_bytes[3],
            width_bytes[0],
            width_bytes[1],
            width_bytes[2],
            width_bytes[3],
            height_bytes[0],
            height_bytes[1],
            height_bytes[2],
            height_bytes[3],
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            following_bytes[0],
            following_bytes[1],
            following_bytes[2],
            following_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<24> for DeviceCtlSwitchEnumAbsArea {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            offset_x: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            offset_y: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            width: i32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            height: i32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            screen: i32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            following: u32::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceCtlSwitchEnum {
    DeviceCtlSwitchEnumResolution(DeviceCtlSwitchEnumResolution),

    DeviceCtlSwitchEnumAbsCalib(DeviceCtlSwitchEnumAbsCalib),

    DeviceCtlSwitchEnumCore(DeviceCtlSwitchEnumCore),

    DeviceCtlSwitchEnumEnable(DeviceCtlSwitchEnumEnable),

    DeviceCtlSwitchEnumAbsArea(DeviceCtlSwitchEnumAbsArea),

    BadValue,
}
impl DeviceCtlSwitchEnum {
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        match self {
            Self::DeviceCtlSwitchEnumResolution(case) => case.serialize_into(buf_ptr),
            Self::DeviceCtlSwitchEnumAbsCalib(case) => {
                buf_ptr
                    .get_mut(..32)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(32)
            }
            Self::DeviceCtlSwitchEnumCore(case) => {
                buf_ptr
                    .get_mut(..4)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(4)
            }
            Self::DeviceCtlSwitchEnumEnable(case) => {
                buf_ptr
                    .get_mut(..4)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(4)
            }
            Self::DeviceCtlSwitchEnumAbsArea(case) => {
                buf_ptr
                    .get_mut(..24)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(24)
            }
            Self::BadValue => Ok(0),
        }
    }
}
impl DeviceCtlSwitchEnum {
    pub fn from_bytes(bytes: &[u8], control_id: u16) -> crate::error::Result<(Self, usize)> {
        let mask = control_id;
        if mask & DeviceControlEnum::RESOLUTION.0 == 0 {
            let (parsed, offset) = DeviceCtlSwitchEnumResolution::from_bytes(bytes)?;
            return Ok((Self::DeviceCtlSwitchEnumResolution(parsed), offset));
        }
        if mask & DeviceControlEnum::ABS_CALIB.0 == 0 {
            return Ok((
                Self::DeviceCtlSwitchEnumAbsCalib(DeviceCtlSwitchEnumAbsCalib::from_bytes(bytes)?),
                32,
            ));
        }
        if mask & DeviceControlEnum::CORE.0 == 0 {
            return Ok((
                Self::DeviceCtlSwitchEnumCore(DeviceCtlSwitchEnumCore::from_bytes(bytes)?),
                4,
            ));
        }
        if mask & DeviceControlEnum::ENABLE.0 == 0 {
            return Ok((
                Self::DeviceCtlSwitchEnumEnable(DeviceCtlSwitchEnumEnable::from_bytes(bytes)?),
                4,
            ));
        }
        if mask & DeviceControlEnum::ABS_AREA.0 == 0 {
            return Ok((
                Self::DeviceCtlSwitchEnumAbsArea(DeviceCtlSwitchEnumAbsArea::from_bytes(bytes)?),
                24,
            ));
        }
        Ok((Self::BadValue, 0))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ChangeDeviceControlReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: crate::proto::xproto::GrabStatusEnum,
}
impl FixedLengthFromBytes<32> for ChangeDeviceControlReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xi_reply_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListDevicePropertiesReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atoms: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for ListDevicePropertiesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let xi_reply_type = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
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
                xi_reply_type,
                sequence,
                length,
                atoms,
            },
            offset,
        ))
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PropertyFormatEnum(pub u8);
impl PropertyFormatEnum {
    pub const EIGHT_BITS: Self = Self(8);
    pub const ONE6_BITS: Self = Self(16);
    pub const THREE2_BITS: Self = Self(32);
}
impl FixedLengthSerialize<1> for PropertyFormatEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for PropertyFormatEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for PropertyFormatEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for PropertyFormatEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for PropertyFormatEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ChangeDevicePropertySwitchEnumAnonCase0 {
    pub data8: alloc::vec::Vec<u8>,
}
impl VariableLengthSerialize for ChangeDevicePropertySwitchEnumAnonCase0 {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let list_len = self.data8.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(0..).ok_or(crate::error::Error::Serialize)?,
            &self.data8,
        )?;
        let mut offset = list_len;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok(offset)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ChangeDevicePropertySwitchEnumAnonCase1 {
    pub data16: alloc::vec::Vec<u16>,
}
impl VariableLengthSerialize for ChangeDevicePropertySwitchEnumAnonCase1 {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let list_len = self.data16.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(0..).ok_or(crate::error::Error::Serialize)?,
            self.data16,
        )?;
        let mut offset = list_len;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok(offset)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ChangeDevicePropertySwitchEnumAnonCase2 {
    pub data32: alloc::vec::Vec<u32>,
}
impl VariableLengthSerialize for ChangeDevicePropertySwitchEnumAnonCase2 {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let list_len = self.data32.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(0..).ok_or(crate::error::Error::Serialize)?,
            self.data32,
        )?;
        let offset = list_len;
        Ok(offset)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeDevicePropertySwitchEnum {
    ChangeDevicePropertySwitchEnumAnonCase0(ChangeDevicePropertySwitchEnumAnonCase0),

    ChangeDevicePropertySwitchEnumAnonCase1(ChangeDevicePropertySwitchEnumAnonCase1),

    ChangeDevicePropertySwitchEnumAnonCase2(ChangeDevicePropertySwitchEnumAnonCase2),

    BadValue,
}
impl ChangeDevicePropertySwitchEnum {
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        match self {
            Self::ChangeDevicePropertySwitchEnumAnonCase0(case) => case.serialize_into(buf_ptr),
            Self::ChangeDevicePropertySwitchEnumAnonCase1(case) => case.serialize_into(buf_ptr),
            Self::ChangeDevicePropertySwitchEnumAnonCase2(case) => case.serialize_into(buf_ptr),
            Self::BadValue => Ok(0),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetDevicePropertyReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub r#type: u32,
    pub bytes_after: u32,
    pub num_items: u32,
    pub format: PropertyFormatEnum,
    pub device_id: u8,
    pub get_device_property_switch_enum: GetDevicePropertySwitchEnum,
}
impl VariableLengthFromBytes for GetDevicePropertyReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 10 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let xi_reply_type = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let r#type = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let bytes_after = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let num_items = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let format = u8::from_bytes(ptr.get(20..21).ok_or(crate::error::Error::FromBytes)?)?;
        let device_id = u8::from_bytes(ptr.get(21..22).ok_or(crate::error::Error::FromBytes)?)?;
        let (get_device_property_switch_enum, offset) = GetDevicePropertySwitchEnum::from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            format,
            num_items,
        )?;
        Ok((
            Self {
                response_type,
                xi_reply_type,
                sequence,
                length,
                r#type,
                bytes_after,
                num_items,
                format: format.into(),
                device_id,
                get_device_property_switch_enum,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetDevicePropertySwitchEnumAnonCase0 {
    pub data8: alloc::vec::Vec<u8>,
}
impl GetDevicePropertySwitchEnumAnonCase0 {
    fn from_bytes(bytes: &[u8], num_items: u32) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let length_expr = num_items as usize;
        let data8: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(0..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok((Self { data8 }, offset))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetDevicePropertySwitchEnumAnonCase1 {
    pub data16: alloc::vec::Vec<u16>,
}
impl GetDevicePropertySwitchEnumAnonCase1 {
    fn from_bytes(bytes: &[u8], num_items: u32) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let length_expr = num_items as usize;
        let data16: alloc::vec::Vec<u16> = crate::vec_from_bytes_fixed!(
            ptr.get(0..).ok_or(crate::error::Error::FromBytes)?,
            u16,
            length_expr,
            2
        );
        let mut offset = length_expr * 2;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok((Self { data16 }, offset))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetDevicePropertySwitchEnumAnonCase2 {
    pub data32: alloc::vec::Vec<u32>,
}
impl GetDevicePropertySwitchEnumAnonCase2 {
    fn from_bytes(bytes: &[u8], num_items: u32) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let length_expr = num_items as usize;
        let data32: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(0..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let offset = length_expr * 4;
        Ok((Self { data32 }, offset))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum GetDevicePropertySwitchEnum {
    GetDevicePropertySwitchEnumAnonCase0(GetDevicePropertySwitchEnumAnonCase0),

    GetDevicePropertySwitchEnumAnonCase1(GetDevicePropertySwitchEnumAnonCase1),

    GetDevicePropertySwitchEnumAnonCase2(GetDevicePropertySwitchEnumAnonCase2),

    BadValue,
}
impl GetDevicePropertySwitchEnum {
    pub fn from_bytes(
        bytes: &[u8],
        format: u8,
        num_items: u32,
    ) -> crate::error::Result<(Self, usize)> {
        let mask = format;
        if mask & PropertyFormatEnum::EIGHT_BITS.0 == 0 {
            let (parsed, offset) =
                GetDevicePropertySwitchEnumAnonCase0::from_bytes(bytes, num_items)?;
            return Ok((Self::GetDevicePropertySwitchEnumAnonCase0(parsed), offset));
        }
        if mask & PropertyFormatEnum::ONE6_BITS.0 == 0 {
            let (parsed, offset) =
                GetDevicePropertySwitchEnumAnonCase1::from_bytes(bytes, num_items)?;
            return Ok((Self::GetDevicePropertySwitchEnumAnonCase1(parsed), offset));
        }
        if mask & PropertyFormatEnum::THREE2_BITS.0 == 0 {
            let (parsed, offset) =
                GetDevicePropertySwitchEnumAnonCase2::from_bytes(bytes, num_items)?;
            return Ok((Self::GetDevicePropertySwitchEnumAnonCase2(parsed), offset));
        }
        Ok((Self::BadValue, 0))
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct DeviceEnum(pub DeviceId);
impl DeviceEnum {
    pub const ALL: Self = Self(0);
    pub const ALL_MASTER: Self = Self(1);
}
impl FixedLengthSerialize<2> for DeviceEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for DeviceEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(DeviceId::from_bytes(bytes)?))
    }
}
impl From<u32> for DeviceEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for DeviceEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for DeviceEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GroupInfo {
    pub base: u8,
    pub latched: u8,
    pub locked: u8,
    pub effective: u8,
}
impl FixedLengthSerialize<4> for GroupInfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        [self.base, self.latched, self.locked, self.effective]
    }
}
impl FixedLengthFromBytes<4> for GroupInfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            base: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            latched: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            locked: u8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?,
            effective: u8::from_bytes(bytes.get(3..4).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ModifierInfo {
    pub base: u32,
    pub latched: u32,
    pub locked: u32,
    pub effective: u32,
}
impl FixedLengthSerialize<16> for ModifierInfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 16] {
        let base_bytes = self.base.serialize_fixed();
        let latched_bytes = self.latched.serialize_fixed();
        let locked_bytes = self.locked.serialize_fixed();
        let effective_bytes = self.effective.serialize_fixed();
        [
            base_bytes[0],
            base_bytes[1],
            base_bytes[2],
            base_bytes[3],
            latched_bytes[0],
            latched_bytes[1],
            latched_bytes[2],
            latched_bytes[3],
            locked_bytes[0],
            locked_bytes[1],
            locked_bytes[2],
            locked_bytes[3],
            effective_bytes[0],
            effective_bytes[1],
            effective_bytes[2],
            effective_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<16> for ModifierInfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            base: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            latched: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            locked: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            effective: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct XIQueryPointerReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: crate::proto::xproto::Window,
    pub child: crate::proto::xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub win_x: Fp1616,
    pub win_y: Fp1616,
    pub same_screen: u8,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub buttons: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for XIQueryPointerReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 1 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let root = crate::proto::xproto::Window::from_bytes(
            ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let child = crate::proto::xproto::Window::from_bytes(
            ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let root_x = Fp1616::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let root_y = Fp1616::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let win_x = Fp1616::from_bytes(ptr.get(24..28).ok_or(crate::error::Error::FromBytes)?)?;
        let win_y = Fp1616::from_bytes(ptr.get(28..32).ok_or(crate::error::Error::FromBytes)?)?;
        let same_screen = u8::from_bytes(ptr.get(32..33).ok_or(crate::error::Error::FromBytes)?)?;
        let buttons_len = u16::from_bytes(ptr.get(34..36).ok_or(crate::error::Error::FromBytes)?)?;
        let mods =
            ModifierInfo::from_bytes(ptr.get(36..52).ok_or(crate::error::Error::FromBytes)?)?;
        let group = GroupInfo::from_bytes(ptr.get(52..56).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = buttons_len as usize;
        let buttons: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(56..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 56;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                root,
                child,
                root_x,
                root_y,
                win_x,
                win_y,
                same_screen,
                mods,
                group,
                buttons,
            },
            offset,
        ))
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct HierarchyChangeTypeEnum(pub u16);
impl HierarchyChangeTypeEnum {
    pub const ADD_MASTER: Self = Self(1);
    pub const REMOVE_MASTER: Self = Self(2);
    pub const ATTACH_SLAVE: Self = Self(3);
    pub const DETACH_SLAVE: Self = Self(4);
}
impl FixedLengthSerialize<2> for HierarchyChangeTypeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for HierarchyChangeTypeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for HierarchyChangeTypeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for HierarchyChangeTypeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for HierarchyChangeTypeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ChangeModeEnum(pub u8);
impl ChangeModeEnum {
    pub const ATTACH: Self = Self(1);
    pub const FLOAT: Self = Self(2);
}
impl FixedLengthSerialize<1> for ChangeModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ChangeModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ChangeModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ChangeModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ChangeModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AddMaster {
    pub r#type: HierarchyChangeTypeEnum,
    pub len: u16,
    pub send_core: u8,
    pub enable: u8,
    pub name: alloc::vec::Vec<u8>,
}
impl VariableLengthSerialize for AddMaster {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let name_len =
            u16::try_from(self.name.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.r#type.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.len.serialize_fixed());
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(name_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(6..7)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.send_core.serialize_fixed());
        buf_ptr
            .get_mut(7..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.enable.serialize_fixed());
        let list_len = self.name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            &self.name,
        )?;
        let mut offset = list_len + 8;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for AddMaster {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let r#type = u16::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let len = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let name_len = u16::from_bytes(ptr.get(4..6).ok_or(crate::error::Error::FromBytes)?)?;
        let send_core = u8::from_bytes(ptr.get(6..7).ok_or(crate::error::Error::FromBytes)?)?;
        let enable = u8::from_bytes(ptr.get(7..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = name_len as usize;
        let name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(8..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr + 8;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok((
            Self {
                r#type: r#type.into(),
                len,
                send_core,
                enable,
                name,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct RemoveMaster {
    pub r#type: HierarchyChangeTypeEnum,
    pub len: u16,
    pub deviceid: DeviceEnum,
    pub return_mode: ChangeModeEnum,
    pub return_pointer: DeviceEnum,
    pub return_keyboard: DeviceEnum,
}
impl FixedLengthSerialize<12> for RemoveMaster {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let r#type_bytes = self.r#type.serialize_fixed();
        let len_bytes = self.len.serialize_fixed();
        let deviceid_bytes = self.deviceid.serialize_fixed();
        let return_pointer_bytes = self.return_pointer.serialize_fixed();
        let return_keyboard_bytes = self.return_keyboard.serialize_fixed();
        [
            r#type_bytes[0],
            r#type_bytes[1],
            len_bytes[0],
            len_bytes[1],
            deviceid_bytes[0],
            deviceid_bytes[1],
            self.return_mode.0 as u8,
            0,
            return_pointer_bytes[0],
            return_pointer_bytes[1],
            return_keyboard_bytes[0],
            return_keyboard_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<12> for RemoveMaster {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            deviceid: DeviceId::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            return_mode: u8::from_bytes(bytes.get(6..7).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            return_pointer: DeviceId::from_bytes(
                bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            return_keyboard: DeviceId::from_bytes(
                bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct AttachSlave {
    pub r#type: HierarchyChangeTypeEnum,
    pub len: u16,
    pub deviceid: DeviceEnum,
    pub master: DeviceEnum,
}
impl FixedLengthSerialize<8> for AttachSlave {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let r#type_bytes = self.r#type.serialize_fixed();
        let len_bytes = self.len.serialize_fixed();
        let deviceid_bytes = self.deviceid.serialize_fixed();
        let master_bytes = self.master.serialize_fixed();
        [
            r#type_bytes[0],
            r#type_bytes[1],
            len_bytes[0],
            len_bytes[1],
            deviceid_bytes[0],
            deviceid_bytes[1],
            master_bytes[0],
            master_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<8> for AttachSlave {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            deviceid: DeviceId::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            master: DeviceId::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DetachSlave {
    pub r#type: HierarchyChangeTypeEnum,
    pub len: u16,
    pub deviceid: DeviceEnum,
}
impl FixedLengthSerialize<8> for DetachSlave {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let r#type_bytes = self.r#type.serialize_fixed();
        let len_bytes = self.len.serialize_fixed();
        let deviceid_bytes = self.deviceid.serialize_fixed();
        [
            r#type_bytes[0],
            r#type_bytes[1],
            len_bytes[0],
            len_bytes[1],
            deviceid_bytes[0],
            deviceid_bytes[1],
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for DetachSlave {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            deviceid: DeviceId::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct HierarchyChange {
    pub r#type: HierarchyChangeTypeEnum,
    pub len: u16,
    pub hierarchy_change_switch_enum: HierarchyChangeSwitchEnum,
}
impl VariableLengthSerialize for HierarchyChange {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.r#type.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.len.serialize_fixed());
        let offset = self
            .hierarchy_change_switch_enum
            .serialize_into(buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?)?;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for HierarchyChange {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let r#type = u16::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let len = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let (hierarchy_change_switch_enum, offset) = HierarchyChangeSwitchEnum::from_bytes(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            r#type,
        )?;
        Ok((
            Self {
                r#type: r#type.into(),
                len,
                hierarchy_change_switch_enum,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct HierarchyChangeSwitchEnumAddMaster {
    pub send_core: u8,
    pub enable: u8,
    pub name: alloc::vec::Vec<u8>,
}
impl VariableLengthSerialize for HierarchyChangeSwitchEnumAddMaster {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let name_len =
            u16::try_from(self.name.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(name_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(2..3)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.send_core.serialize_fixed());
        buf_ptr
            .get_mut(3..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.enable.serialize_fixed());
        let list_len = self.name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?,
            &self.name,
        )?;
        let mut offset = list_len + 4;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for HierarchyChangeSwitchEnumAddMaster {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let name_len = u16::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let send_core = u8::from_bytes(ptr.get(2..3).ok_or(crate::error::Error::FromBytes)?)?;
        let enable = u8::from_bytes(ptr.get(3..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = name_len as usize;
        let name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr + 4;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok((
            Self {
                send_core,
                enable,
                name,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct HierarchyChangeSwitchEnumRemoveMaster {
    pub deviceid: DeviceEnum,
    pub return_mode: ChangeModeEnum,
    pub return_pointer: DeviceEnum,
    pub return_keyboard: DeviceEnum,
}
impl FixedLengthSerialize<8> for HierarchyChangeSwitchEnumRemoveMaster {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let deviceid_bytes = self.deviceid.serialize_fixed();
        let return_pointer_bytes = self.return_pointer.serialize_fixed();
        let return_keyboard_bytes = self.return_keyboard.serialize_fixed();
        [
            deviceid_bytes[0],
            deviceid_bytes[1],
            self.return_mode.0 as u8,
            0,
            return_pointer_bytes[0],
            return_pointer_bytes[1],
            return_keyboard_bytes[0],
            return_keyboard_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<8> for HierarchyChangeSwitchEnumRemoveMaster {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            deviceid: DeviceId::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            return_mode: u8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            return_pointer: DeviceId::from_bytes(
                bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            return_keyboard: DeviceId::from_bytes(
                bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct HierarchyChangeSwitchEnumAttachSlave {
    pub deviceid: DeviceEnum,
    pub master: DeviceEnum,
}
impl FixedLengthSerialize<4> for HierarchyChangeSwitchEnumAttachSlave {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        let deviceid_bytes = self.deviceid.serialize_fixed();
        let master_bytes = self.master.serialize_fixed();
        [
            deviceid_bytes[0],
            deviceid_bytes[1],
            master_bytes[0],
            master_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<4> for HierarchyChangeSwitchEnumAttachSlave {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            deviceid: DeviceId::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            master: DeviceId::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct HierarchyChangeSwitchEnumDetachSlave {
    pub deviceid: DeviceEnum,
}
impl FixedLengthSerialize<4> for HierarchyChangeSwitchEnumDetachSlave {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        let deviceid_bytes = self.deviceid.serialize_fixed();
        [deviceid_bytes[0], deviceid_bytes[1], 0, 0]
    }
}
impl FixedLengthFromBytes<4> for HierarchyChangeSwitchEnumDetachSlave {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            deviceid: DeviceId::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum HierarchyChangeSwitchEnum {
    HierarchyChangeSwitchEnumAddMaster(HierarchyChangeSwitchEnumAddMaster),

    HierarchyChangeSwitchEnumRemoveMaster(HierarchyChangeSwitchEnumRemoveMaster),

    HierarchyChangeSwitchEnumAttachSlave(HierarchyChangeSwitchEnumAttachSlave),

    HierarchyChangeSwitchEnumDetachSlave(HierarchyChangeSwitchEnumDetachSlave),

    BadValue,
}
impl HierarchyChangeSwitchEnum {
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        match self {
            Self::HierarchyChangeSwitchEnumAddMaster(case) => case.serialize_into(buf_ptr),
            Self::HierarchyChangeSwitchEnumRemoveMaster(case) => {
                buf_ptr
                    .get_mut(..8)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(8)
            }
            Self::HierarchyChangeSwitchEnumAttachSlave(case) => {
                buf_ptr
                    .get_mut(..4)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(4)
            }
            Self::HierarchyChangeSwitchEnumDetachSlave(case) => {
                buf_ptr
                    .get_mut(..4)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(4)
            }
            Self::BadValue => Ok(0),
        }
    }
}
impl HierarchyChangeSwitchEnum {
    pub fn from_bytes(bytes: &[u8], r#type: u16) -> crate::error::Result<(Self, usize)> {
        let mask = r#type;
        if mask & HierarchyChangeTypeEnum::ADD_MASTER.0 == 0 {
            let (parsed, offset) = HierarchyChangeSwitchEnumAddMaster::from_bytes(bytes)?;
            return Ok((Self::HierarchyChangeSwitchEnumAddMaster(parsed), offset));
        }
        if mask & HierarchyChangeTypeEnum::REMOVE_MASTER.0 == 0 {
            return Ok((
                Self::HierarchyChangeSwitchEnumRemoveMaster(
                    HierarchyChangeSwitchEnumRemoveMaster::from_bytes(bytes)?,
                ),
                8,
            ));
        }
        if mask & HierarchyChangeTypeEnum::ATTACH_SLAVE.0 == 0 {
            return Ok((
                Self::HierarchyChangeSwitchEnumAttachSlave(
                    HierarchyChangeSwitchEnumAttachSlave::from_bytes(bytes)?,
                ),
                4,
            ));
        }
        if mask & HierarchyChangeTypeEnum::DETACH_SLAVE.0 == 0 {
            return Ok((
                Self::HierarchyChangeSwitchEnumDetachSlave(
                    HierarchyChangeSwitchEnumDetachSlave::from_bytes(bytes)?,
                ),
                4,
            ));
        }
        Ok((Self::BadValue, 0))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct XIGetClientPointerReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub set: u8,
    pub deviceid: DeviceEnum,
}
impl FixedLengthFromBytes<32> for XIGetClientPointerReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            set: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            deviceid: DeviceId::from_bytes(
                bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
        })
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct XIEventMask(pub u32);
impl XIEventMask {
    pub const DEVICE_CHANGED: Self = Self(1 << 1);
    pub const KEY_PRESS: Self = Self(1 << 2);
    pub const KEY_RELEASE: Self = Self(1 << 3);
    pub const BUTTON_PRESS: Self = Self(1 << 4);
    pub const BUTTON_RELEASE: Self = Self(1 << 5);
    pub const MOTION: Self = Self(1 << 6);
    pub const ENTER: Self = Self(1 << 7);
    pub const LEAVE: Self = Self(1 << 8);
    pub const FOCUS_IN: Self = Self(1 << 9);
    pub const FOCUS_OUT: Self = Self(1 << 10);
    pub const HIERARCHY: Self = Self(1 << 11);
    pub const PROPERTY: Self = Self(1 << 12);
    pub const RAW_KEY_PRESS: Self = Self(1 << 13);
    pub const RAW_KEY_RELEASE: Self = Self(1 << 14);
    pub const RAW_BUTTON_PRESS: Self = Self(1 << 15);
    pub const RAW_BUTTON_RELEASE: Self = Self(1 << 16);
    pub const RAW_MOTION: Self = Self(1 << 17);
    pub const TOUCH_BEGIN: Self = Self(1 << 18);
    pub const TOUCH_UPDATE: Self = Self(1 << 19);
    pub const TOUCH_END: Self = Self(1 << 20);
    pub const TOUCH_OWNERSHIP: Self = Self(1 << 21);
    pub const RAW_TOUCH_BEGIN: Self = Self(1 << 22);
    pub const RAW_TOUCH_UPDATE: Self = Self(1 << 23);
    pub const RAW_TOUCH_END: Self = Self(1 << 24);
    pub const BARRIER_HIT: Self = Self(1 << 25);
    pub const BARRIER_LEAVE: Self = Self(1 << 26);
}
impl FixedLengthSerialize<4> for XIEventMask {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for XIEventMask {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for XIEventMask {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for XIEventMask {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for XIEventMask {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(XIEventMask);
#[derive(Debug, Clone, PartialEq)]
pub struct EventMask {
    pub deviceid: DeviceEnum,
    pub mask: alloc::vec::Vec<XIEventMask>,
}
impl VariableLengthSerialize for EventMask {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let mask_len =
            u16::try_from(self.mask.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.deviceid.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(mask_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.mask.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?,
            self.mask,
        )?;
        let offset = list_len + 4;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for EventMask {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let deviceid = DeviceId::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let mask_len = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = mask_len as usize;
        let mask: alloc::vec::Vec<XIEventMask> = crate::vec_from_bytes_fixed!(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            XIEventMask,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 4;
        Ok((
            Self {
                deviceid: deviceid.into(),
                mask,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct XIQueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
}
impl FixedLengthFromBytes<32> for XIQueryVersionReply {
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
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct DeviceClassTypeEnum(pub u16);
impl DeviceClassTypeEnum {
    pub const KEY: Self = Self(0);
    pub const BUTTON: Self = Self(1);
    pub const VALUATOR: Self = Self(2);
    pub const SCROLL: Self = Self(3);
    pub const TOUCH: Self = Self(8);
    pub const GESTURE: Self = Self(9);
}
impl FixedLengthSerialize<2> for DeviceClassTypeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for DeviceClassTypeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for DeviceClassTypeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for DeviceClassTypeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for DeviceClassTypeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct DeviceTypeEnum(pub u16);
impl DeviceTypeEnum {
    pub const MASTER_POINTER: Self = Self(1);
    pub const MASTER_KEYBOARD: Self = Self(2);
    pub const SLAVE_POINTER: Self = Self(3);
    pub const SLAVE_KEYBOARD: Self = Self(4);
    pub const FLOATING_SLAVE: Self = Self(5);
}
impl FixedLengthSerialize<2> for DeviceTypeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for DeviceTypeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for DeviceTypeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for DeviceTypeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for DeviceTypeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ScrollFlags(pub u32);
impl ScrollFlags {
    pub const NO_EMULATION: Self = Self(1 << 0);
    pub const PREFERRED: Self = Self(1 << 1);
}
impl FixedLengthSerialize<4> for ScrollFlags {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for ScrollFlags {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for ScrollFlags {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for ScrollFlags {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for ScrollFlags {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(ScrollFlags);
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ScrollTypeEnum(pub u16);
impl ScrollTypeEnum {
    pub const VERTICAL: Self = Self(1);
    pub const HORIZONTAL: Self = Self(2);
}
impl FixedLengthSerialize<2> for ScrollTypeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for ScrollTypeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for ScrollTypeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for ScrollTypeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for ScrollTypeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct TouchModeEnum(pub u8);
impl TouchModeEnum {
    pub const DIRECT: Self = Self(1);
    pub const DEPENDENT: Self = Self(2);
}
impl FixedLengthSerialize<1> for TouchModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for TouchModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for TouchModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for TouchModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for TouchModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ButtonClass {
    pub r#type: DeviceClassTypeEnum,
    pub len: u16,
    pub sourceid: DeviceId,
    pub state: alloc::vec::Vec<u32>,
    pub labels: alloc::vec::Vec<u32>,
}
impl VariableLengthSerialize for ButtonClass {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let num_buttons =
            u16::try_from(self.labels.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.r#type.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.len.serialize_fixed());
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.sourceid.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_buttons).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.state.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            self.state,
        )?;
        let mut offset = list_len + 8;
        let list_len = self.labels.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            self.labels,
        )?;
        offset += list_len;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for ButtonClass {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let r#type = u16::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let len = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let sourceid = DeviceId::from_bytes(ptr.get(4..6).ok_or(crate::error::Error::FromBytes)?)?;
        let num_buttons = u16::from_bytes(ptr.get(6..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Div::div(
            core::ops::Add::add(num_buttons as u16, 31u16 as u16) as u16,
            32u16 as u16,
        ) as usize;
        let state: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(8..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 8;
        let length_expr = num_buttons as usize;
        let labels: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        offset += length_expr * 4;
        Ok((
            Self {
                r#type: r#type.into(),
                len,
                sourceid,
                state,
                labels,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct KeyClass {
    pub r#type: DeviceClassTypeEnum,
    pub len: u16,
    pub sourceid: DeviceId,
    pub keys: alloc::vec::Vec<u32>,
}
impl VariableLengthSerialize for KeyClass {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let num_keys =
            u16::try_from(self.keys.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.r#type.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.len.serialize_fixed());
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.sourceid.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_keys).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.keys.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            self.keys,
        )?;
        let offset = list_len + 8;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for KeyClass {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let r#type = u16::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let len = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let sourceid = DeviceId::from_bytes(ptr.get(4..6).ok_or(crate::error::Error::FromBytes)?)?;
        let num_keys = u16::from_bytes(ptr.get(6..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_keys as usize;
        let keys: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(8..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 8;
        Ok((
            Self {
                r#type: r#type.into(),
                len,
                sourceid,
                keys,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ScrollClass {
    pub r#type: DeviceClassTypeEnum,
    pub len: u16,
    pub sourceid: DeviceId,
    pub number: u16,
    pub scroll_type: ScrollTypeEnum,
    pub flags: ScrollFlags,
    pub increment: Fp3232,
}
impl FixedLengthSerialize<24> for ScrollClass {
    #[inline]
    fn serialize_fixed(self) -> [u8; 24] {
        let r#type_bytes = self.r#type.serialize_fixed();
        let len_bytes = self.len.serialize_fixed();
        let sourceid_bytes = self.sourceid.serialize_fixed();
        let number_bytes = self.number.serialize_fixed();
        let scroll_type_bytes = self.scroll_type.serialize_fixed();
        let flags_bytes = self.flags.serialize_fixed();
        let increment_bytes = self.increment.serialize_fixed();
        [
            r#type_bytes[0],
            r#type_bytes[1],
            len_bytes[0],
            len_bytes[1],
            sourceid_bytes[0],
            sourceid_bytes[1],
            number_bytes[0],
            number_bytes[1],
            scroll_type_bytes[0],
            scroll_type_bytes[1],
            0,
            0,
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
            increment_bytes[0],
            increment_bytes[1],
            increment_bytes[2],
            increment_bytes[3],
            increment_bytes[4],
            increment_bytes[5],
            increment_bytes[6],
            increment_bytes[7],
        ]
    }
}
impl FixedLengthFromBytes<24> for ScrollClass {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            sourceid: DeviceId::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            number: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
            scroll_type: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            flags: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            increment: Fp3232::from_bytes(
                bytes.get(16..24).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct TouchClass {
    pub r#type: DeviceClassTypeEnum,
    pub len: u16,
    pub sourceid: DeviceId,
    pub mode: TouchModeEnum,
    pub num_touches: u8,
}
impl FixedLengthSerialize<8> for TouchClass {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let r#type_bytes = self.r#type.serialize_fixed();
        let len_bytes = self.len.serialize_fixed();
        let sourceid_bytes = self.sourceid.serialize_fixed();
        [
            r#type_bytes[0],
            r#type_bytes[1],
            len_bytes[0],
            len_bytes[1],
            sourceid_bytes[0],
            sourceid_bytes[1],
            self.mode.0 as u8,
            self.num_touches,
        ]
    }
}
impl FixedLengthFromBytes<8> for TouchClass {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            sourceid: DeviceId::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            mode: u8::from_bytes(bytes.get(6..7).ok_or(crate::error::Error::FromBytes)?)?.into(),
            num_touches: u8::from_bytes(bytes.get(7..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GestureClass {
    pub r#type: DeviceClassTypeEnum,
    pub len: u16,
    pub sourceid: DeviceId,
    pub num_touches: u8,
}
impl FixedLengthSerialize<8> for GestureClass {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let r#type_bytes = self.r#type.serialize_fixed();
        let len_bytes = self.len.serialize_fixed();
        let sourceid_bytes = self.sourceid.serialize_fixed();
        [
            r#type_bytes[0],
            r#type_bytes[1],
            len_bytes[0],
            len_bytes[1],
            sourceid_bytes[0],
            sourceid_bytes[1],
            self.num_touches,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for GestureClass {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            sourceid: DeviceId::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            num_touches: u8::from_bytes(bytes.get(6..7).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ValuatorClass {
    pub r#type: DeviceClassTypeEnum,
    pub len: u16,
    pub sourceid: DeviceId,
    pub number: u16,
    pub label: u32,
    pub min: Fp3232,
    pub max: Fp3232,
    pub value: Fp3232,
    pub resolution: u32,
    pub mode: ValuatorModeEnum,
}
impl FixedLengthSerialize<44> for ValuatorClass {
    #[inline]
    fn serialize_fixed(self) -> [u8; 44] {
        let r#type_bytes = self.r#type.serialize_fixed();
        let len_bytes = self.len.serialize_fixed();
        let sourceid_bytes = self.sourceid.serialize_fixed();
        let number_bytes = self.number.serialize_fixed();
        let label_bytes = self.label.serialize_fixed();
        let min_bytes = self.min.serialize_fixed();
        let max_bytes = self.max.serialize_fixed();
        let value_bytes = self.value.serialize_fixed();
        let resolution_bytes = self.resolution.serialize_fixed();
        [
            r#type_bytes[0],
            r#type_bytes[1],
            len_bytes[0],
            len_bytes[1],
            sourceid_bytes[0],
            sourceid_bytes[1],
            number_bytes[0],
            number_bytes[1],
            label_bytes[0],
            label_bytes[1],
            label_bytes[2],
            label_bytes[3],
            min_bytes[0],
            min_bytes[1],
            min_bytes[2],
            min_bytes[3],
            min_bytes[4],
            min_bytes[5],
            min_bytes[6],
            min_bytes[7],
            max_bytes[0],
            max_bytes[1],
            max_bytes[2],
            max_bytes[3],
            max_bytes[4],
            max_bytes[5],
            max_bytes[6],
            max_bytes[7],
            value_bytes[0],
            value_bytes[1],
            value_bytes[2],
            value_bytes[3],
            value_bytes[4],
            value_bytes[5],
            value_bytes[6],
            value_bytes[7],
            resolution_bytes[0],
            resolution_bytes[1],
            resolution_bytes[2],
            resolution_bytes[3],
            self.mode.0 as u8,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<44> for ValuatorClass {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            len: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            sourceid: DeviceId::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            number: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?,
            label: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            min: Fp3232::from_bytes(bytes.get(12..20).ok_or(crate::error::Error::FromBytes)?)?,
            max: Fp3232::from_bytes(bytes.get(20..28).ok_or(crate::error::Error::FromBytes)?)?,
            value: Fp3232::from_bytes(bytes.get(28..36).ok_or(crate::error::Error::FromBytes)?)?,
            resolution: u32::from_bytes(bytes.get(36..40).ok_or(crate::error::Error::FromBytes)?)?,
            mode: u8::from_bytes(bytes.get(40..41).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceClass {
    pub r#type: DeviceClassTypeEnum,
    pub len: u16,
    pub sourceid: DeviceId,
    pub device_class_switch_enum: DeviceClassSwitchEnum,
}
impl VariableLengthSerialize for DeviceClass {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.r#type.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.len.serialize_fixed());
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.sourceid.serialize_fixed());
        // Length
        let mut offset = 6;
        offset += self.device_class_switch_enum.serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
        )?;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for DeviceClass {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let r#type = u16::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let len = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let sourceid = DeviceId::from_bytes(ptr.get(4..6).ok_or(crate::error::Error::FromBytes)?)?;
        let mut offset = 6;
        // Length
        let (device_class_switch_enum, new_offset) = DeviceClassSwitchEnum::from_bytes(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            r#type,
        )?;
        offset += new_offset;
        Ok((
            Self {
                r#type: r#type.into(),
                len,
                sourceid,
                device_class_switch_enum,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceClassSwitchEnumKey {
    pub keys: alloc::vec::Vec<u32>,
}
impl VariableLengthSerialize for DeviceClassSwitchEnumKey {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        // Start align 4 Some(2)
        let num_keys =
            u16::try_from(self.keys.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_keys).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.keys.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(2..).ok_or(crate::error::Error::Serialize)?,
            self.keys,
        )?;
        let offset = list_len + 2;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for DeviceClassSwitchEnumKey {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Start align 4 Some(2)
        let num_keys = u16::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_keys as usize;
        let keys: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(2..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 2;
        Ok((Self { keys }, offset))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceClassSwitchEnumButton {
    pub state: alloc::vec::Vec<u32>,
    pub labels: alloc::vec::Vec<u32>,
}
impl VariableLengthSerialize for DeviceClassSwitchEnumButton {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        // Start align 4 Some(2)
        let num_buttons =
            u16::try_from(self.labels.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_buttons).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.state.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(2..).ok_or(crate::error::Error::Serialize)?,
            self.state,
        )?;
        let mut offset = list_len + 2;
        let list_len = self.labels.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            self.labels,
        )?;
        offset += list_len;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for DeviceClassSwitchEnumButton {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Start align 4 Some(2)
        let num_buttons = u16::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Div::div(
            core::ops::Add::add(num_buttons as u16, 31u16 as u16) as u16,
            32u16 as u16,
        ) as usize;
        let state: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(2..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 2;
        let length_expr = num_buttons as usize;
        let labels: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        offset += length_expr * 4;
        Ok((Self { state, labels }, offset))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceClassSwitchEnumValuator {
    pub number: u16,
    pub label: u32,
    pub min: Fp3232,
    pub max: Fp3232,
    pub value: Fp3232,
    pub resolution: u32,
    pub mode: ValuatorModeEnum,
}
impl FixedLengthSerialize<38> for DeviceClassSwitchEnumValuator {
    #[inline]
    fn serialize_fixed(self) -> [u8; 38] {
        let number_bytes = self.number.serialize_fixed();
        let label_bytes = self.label.serialize_fixed();
        let min_bytes = self.min.serialize_fixed();
        let max_bytes = self.max.serialize_fixed();
        let value_bytes = self.value.serialize_fixed();
        let resolution_bytes = self.resolution.serialize_fixed();
        [
            number_bytes[0],
            number_bytes[1],
            label_bytes[0],
            label_bytes[1],
            label_bytes[2],
            label_bytes[3],
            min_bytes[0],
            min_bytes[1],
            min_bytes[2],
            min_bytes[3],
            min_bytes[4],
            min_bytes[5],
            min_bytes[6],
            min_bytes[7],
            max_bytes[0],
            max_bytes[1],
            max_bytes[2],
            max_bytes[3],
            max_bytes[4],
            max_bytes[5],
            max_bytes[6],
            max_bytes[7],
            value_bytes[0],
            value_bytes[1],
            value_bytes[2],
            value_bytes[3],
            value_bytes[4],
            value_bytes[5],
            value_bytes[6],
            value_bytes[7],
            resolution_bytes[0],
            resolution_bytes[1],
            resolution_bytes[2],
            resolution_bytes[3],
            self.mode.0 as u8,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<38> for DeviceClassSwitchEnumValuator {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            number: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            label: u32::from_bytes(bytes.get(2..6).ok_or(crate::error::Error::FromBytes)?)?,
            min: Fp3232::from_bytes(bytes.get(6..14).ok_or(crate::error::Error::FromBytes)?)?,
            max: Fp3232::from_bytes(bytes.get(14..22).ok_or(crate::error::Error::FromBytes)?)?,
            value: Fp3232::from_bytes(bytes.get(22..30).ok_or(crate::error::Error::FromBytes)?)?,
            resolution: u32::from_bytes(bytes.get(30..34).ok_or(crate::error::Error::FromBytes)?)?,
            mode: u8::from_bytes(bytes.get(34..35).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceClassSwitchEnumScroll {
    pub number: u16,
    pub scroll_type: ScrollTypeEnum,
    pub flags: ScrollFlags,
    pub increment: Fp3232,
}
impl FixedLengthSerialize<18> for DeviceClassSwitchEnumScroll {
    #[inline]
    fn serialize_fixed(self) -> [u8; 18] {
        let number_bytes = self.number.serialize_fixed();
        let scroll_type_bytes = self.scroll_type.serialize_fixed();
        let flags_bytes = self.flags.serialize_fixed();
        let increment_bytes = self.increment.serialize_fixed();
        [
            number_bytes[0],
            number_bytes[1],
            scroll_type_bytes[0],
            scroll_type_bytes[1],
            0,
            0,
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
            increment_bytes[0],
            increment_bytes[1],
            increment_bytes[2],
            increment_bytes[3],
            increment_bytes[4],
            increment_bytes[5],
            increment_bytes[6],
            increment_bytes[7],
        ]
    }
}
impl FixedLengthFromBytes<18> for DeviceClassSwitchEnumScroll {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            number: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            scroll_type: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            flags: u32::from_bytes(bytes.get(6..10).ok_or(crate::error::Error::FromBytes)?)?.into(),
            increment: Fp3232::from_bytes(
                bytes.get(10..18).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceClassSwitchEnumTouch {
    pub mode: TouchModeEnum,
    pub num_touches: u8,
}
impl FixedLengthSerialize<2> for DeviceClassSwitchEnumTouch {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        [self.mode.0 as u8, self.num_touches]
    }
}
impl FixedLengthFromBytes<2> for DeviceClassSwitchEnumTouch {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            mode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            num_touches: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceClassSwitchEnumGesture {
    pub num_touches: u8,
}
impl FixedLengthSerialize<2> for DeviceClassSwitchEnumGesture {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        [self.num_touches, 0]
    }
}
impl FixedLengthFromBytes<2> for DeviceClassSwitchEnumGesture {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            num_touches: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceClassSwitchEnum {
    DeviceClassSwitchEnumKey(DeviceClassSwitchEnumKey),

    DeviceClassSwitchEnumButton(DeviceClassSwitchEnumButton),

    DeviceClassSwitchEnumValuator(DeviceClassSwitchEnumValuator),

    DeviceClassSwitchEnumScroll(DeviceClassSwitchEnumScroll),

    DeviceClassSwitchEnumTouch(DeviceClassSwitchEnumTouch),

    DeviceClassSwitchEnumGesture(DeviceClassSwitchEnumGesture),

    BadValue,
}
impl DeviceClassSwitchEnum {
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        match self {
            Self::DeviceClassSwitchEnumKey(case) => case.serialize_into(buf_ptr),
            Self::DeviceClassSwitchEnumButton(case) => case.serialize_into(buf_ptr),
            Self::DeviceClassSwitchEnumValuator(case) => {
                buf_ptr
                    .get_mut(..38)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(38)
            }
            Self::DeviceClassSwitchEnumScroll(case) => {
                buf_ptr
                    .get_mut(..18)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(18)
            }
            Self::DeviceClassSwitchEnumTouch(case) => {
                buf_ptr
                    .get_mut(..2)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(2)
            }
            Self::DeviceClassSwitchEnumGesture(case) => {
                buf_ptr
                    .get_mut(..2)
                    .ok_or(crate::error::Error::Serialize)?
                    .copy_from_slice(&case.serialize_fixed());
                Ok(2)
            }
            Self::BadValue => Ok(0),
        }
    }
}
impl DeviceClassSwitchEnum {
    pub fn from_bytes(bytes: &[u8], r#type: u16) -> crate::error::Result<(Self, usize)> {
        let mask = r#type;
        if mask & DeviceClassTypeEnum::KEY.0 == 0 {
            let (parsed, offset) = DeviceClassSwitchEnumKey::from_bytes(bytes)?;
            return Ok((Self::DeviceClassSwitchEnumKey(parsed), offset));
        }
        if mask & DeviceClassTypeEnum::BUTTON.0 == 0 {
            let (parsed, offset) = DeviceClassSwitchEnumButton::from_bytes(bytes)?;
            return Ok((Self::DeviceClassSwitchEnumButton(parsed), offset));
        }
        if mask & DeviceClassTypeEnum::VALUATOR.0 == 0 {
            return Ok((
                Self::DeviceClassSwitchEnumValuator(DeviceClassSwitchEnumValuator::from_bytes(
                    bytes,
                )?),
                38,
            ));
        }
        if mask & DeviceClassTypeEnum::SCROLL.0 == 0 {
            return Ok((
                Self::DeviceClassSwitchEnumScroll(DeviceClassSwitchEnumScroll::from_bytes(bytes)?),
                18,
            ));
        }
        if mask & DeviceClassTypeEnum::TOUCH.0 == 0 {
            return Ok((
                Self::DeviceClassSwitchEnumTouch(DeviceClassSwitchEnumTouch::from_bytes(bytes)?),
                2,
            ));
        }
        if mask & DeviceClassTypeEnum::GESTURE.0 == 0 {
            return Ok((
                Self::DeviceClassSwitchEnumGesture(DeviceClassSwitchEnumGesture::from_bytes(
                    bytes,
                )?),
                2,
            ));
        }
        Ok((Self::BadValue, 0))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct XIDeviceInfo {
    pub deviceid: DeviceEnum,
    pub r#type: DeviceTypeEnum,
    pub attachment: DeviceEnum,
    pub enabled: u8,
    pub name: alloc::vec::Vec<u8>,
    pub classes: alloc::vec::Vec<DeviceClass>,
}
impl VariableLengthSerialize for XIDeviceInfo {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let num_classes =
            u16::try_from(self.classes.len()).map_err(|_| crate::error::Error::Serialize)?;
        let name_len =
            u16::try_from(self.name.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Padding 1 bytes
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.deviceid.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.r#type.serialize_fixed());
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.attachment.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_classes).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(name_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(10..11)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.enabled.serialize_fixed());
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
        offset += crate::util::var_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            self.classes,
        )?;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for XIDeviceInfo {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        let deviceid = DeviceId::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let r#type = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let attachment =
            DeviceId::from_bytes(ptr.get(4..6).ok_or(crate::error::Error::FromBytes)?)?;
        let num_classes = u16::from_bytes(ptr.get(6..8).ok_or(crate::error::Error::FromBytes)?)?;
        let name_len = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let enabled = u8::from_bytes(ptr.get(10..11).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = name_len as usize;
        let name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(12..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr + 12;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let classes_length = num_classes as usize;
        let classes = crate::vec_from_bytes_var!(ptr, DeviceClass, offset, classes_length,);
        Ok((
            Self {
                deviceid: deviceid.into(),
                r#type: r#type.into(),
                attachment: attachment.into(),
                enabled,
                name,
                classes,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct XIQueryDeviceReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub infos: alloc::vec::Vec<XIDeviceInfo>,
}
impl VariableLengthFromBytes for XIQueryDeviceReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_infos = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let infos_length = num_infos as usize;
        let mut offset = 32;
        let infos = crate::vec_from_bytes_var!(ptr, XIDeviceInfo, offset, infos_length,);
        Ok((
            Self {
                response_type,
                sequence,
                length,
                infos,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct XIGetFocusReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub focus: crate::proto::xproto::Window,
}
impl FixedLengthFromBytes<32> for XIGetFocusReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            focus: crate::proto::xproto::Window::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct GrabOwnerEnum(pub u8);
impl GrabOwnerEnum {
    pub const NO_OWNER: Self = Self(0);
    pub const OWNER: Self = Self(1);
}
impl FixedLengthSerialize<1> for GrabOwnerEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for GrabOwnerEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for GrabOwnerEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for GrabOwnerEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for GrabOwnerEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct XIGrabDeviceReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: crate::proto::xproto::GrabStatusEnum,
}
impl FixedLengthFromBytes<32> for XIGrabDeviceReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            status: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct EventModeEnum(pub u8);
impl EventModeEnum {
    pub const ASYNC_DEVICE: Self = Self(0);
    pub const SYNC_DEVICE: Self = Self(1);
    pub const REPLAY_DEVICE: Self = Self(2);
    pub const ASYNC_PAIRED_DEVICE: Self = Self(3);
    pub const ASYNC_PAIR: Self = Self(4);
    pub const SYNC_PAIR: Self = Self(5);
    pub const ACCEPT_TOUCH: Self = Self(6);
    pub const REJECT_TOUCH: Self = Self(7);
}
impl FixedLengthSerialize<1> for EventModeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for EventModeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for EventModeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for EventModeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for EventModeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct GrabMode22Enum(pub u8);
impl GrabMode22Enum {
    pub const SYNC: Self = Self(0);
    pub const ASYNC: Self = Self(1);
    pub const TOUCH: Self = Self(2);
}
impl FixedLengthSerialize<1> for GrabMode22Enum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for GrabMode22Enum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for GrabMode22Enum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for GrabMode22Enum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for GrabMode22Enum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct GrabTypeEnum(pub u8);
impl GrabTypeEnum {
    pub const BUTTON: Self = Self(0);
    pub const KEYCODE: Self = Self(1);
    pub const ENTER: Self = Self(2);
    pub const FOCUS_IN: Self = Self(3);
    pub const TOUCH_BEGIN: Self = Self(4);
    pub const GESTURE_PINCH_BEGIN: Self = Self(5);
    pub const GESTURE_SWIPE_BEGIN: Self = Self(6);
}
impl FixedLengthSerialize<1> for GrabTypeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for GrabTypeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for GrabTypeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for GrabTypeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for GrabTypeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ModifierMask(pub u32);
impl ModifierMask {
    pub const ANY: Self = Self(1 << 31);
}
impl FixedLengthSerialize<4> for ModifierMask {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for ModifierMask {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for ModifierMask {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for ModifierMask {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for ModifierMask {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(ModifierMask);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GrabModifierInfo {
    pub modifiers: ModifierMask,
    pub status: crate::proto::xproto::GrabStatusEnum,
}
impl FixedLengthSerialize<8> for GrabModifierInfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let modifiers_bytes = self.modifiers.serialize_fixed();
        [
            modifiers_bytes[0],
            modifiers_bytes[1],
            modifiers_bytes[2],
            modifiers_bytes[3],
            self.status.0 as u8,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for GrabModifierInfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            modifiers: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            status: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct XIPassiveGrabDeviceReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub modifiers: alloc::vec::Vec<GrabModifierInfo>,
}
impl VariableLengthFromBytes for XIPassiveGrabDeviceReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_modifiers = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_modifiers as usize;
        let modifiers: alloc::vec::Vec<GrabModifierInfo> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            GrabModifierInfo,
            length_expr,
            8
        );
        let offset = length_expr * 8 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                modifiers,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct XIListPropertiesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub properties: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for XIListPropertiesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_properties =
            u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_properties as usize;
        let properties: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
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
                properties,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct XIChangePropertySwitchEnumAnonCase0 {
    pub data8: alloc::vec::Vec<u8>,
}
impl VariableLengthSerialize for XIChangePropertySwitchEnumAnonCase0 {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let list_len = self.data8.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(0..).ok_or(crate::error::Error::Serialize)?,
            &self.data8,
        )?;
        let mut offset = list_len;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok(offset)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct XIChangePropertySwitchEnumAnonCase1 {
    pub data16: alloc::vec::Vec<u16>,
}
impl VariableLengthSerialize for XIChangePropertySwitchEnumAnonCase1 {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let list_len = self.data16.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(0..).ok_or(crate::error::Error::Serialize)?,
            self.data16,
        )?;
        let mut offset = list_len;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok(offset)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct XIChangePropertySwitchEnumAnonCase2 {
    pub data32: alloc::vec::Vec<u32>,
}
impl VariableLengthSerialize for XIChangePropertySwitchEnumAnonCase2 {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let list_len = self.data32.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(0..).ok_or(crate::error::Error::Serialize)?,
            self.data32,
        )?;
        let offset = list_len;
        Ok(offset)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum XIChangePropertySwitchEnum {
    XIChangePropertySwitchEnumAnonCase0(XIChangePropertySwitchEnumAnonCase0),

    XIChangePropertySwitchEnumAnonCase1(XIChangePropertySwitchEnumAnonCase1),

    XIChangePropertySwitchEnumAnonCase2(XIChangePropertySwitchEnumAnonCase2),

    BadValue,
}
impl XIChangePropertySwitchEnum {
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        match self {
            Self::XIChangePropertySwitchEnumAnonCase0(case) => case.serialize_into(buf_ptr),
            Self::XIChangePropertySwitchEnumAnonCase1(case) => case.serialize_into(buf_ptr),
            Self::XIChangePropertySwitchEnumAnonCase2(case) => case.serialize_into(buf_ptr),
            Self::BadValue => Ok(0),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct XIGetPropertyReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub r#type: u32,
    pub bytes_after: u32,
    pub num_items: u32,
    pub format: PropertyFormatEnum,
    pub x_i_get_property_switch_enum: XIGetPropertySwitchEnum,
}
impl VariableLengthFromBytes for XIGetPropertyReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 11 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let r#type = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let bytes_after = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let num_items = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let format = u8::from_bytes(ptr.get(20..21).ok_or(crate::error::Error::FromBytes)?)?;
        let (x_i_get_property_switch_enum, offset) = XIGetPropertySwitchEnum::from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            format,
            num_items,
        )?;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                r#type,
                bytes_after,
                num_items,
                format: format.into(),
                x_i_get_property_switch_enum,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct XIGetPropertySwitchEnumAnonCase0 {
    pub data8: alloc::vec::Vec<u8>,
}
impl XIGetPropertySwitchEnumAnonCase0 {
    fn from_bytes(bytes: &[u8], num_items: u32) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let length_expr = num_items as usize;
        let data8: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(0..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok((Self { data8 }, offset))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct XIGetPropertySwitchEnumAnonCase1 {
    pub data16: alloc::vec::Vec<u16>,
}
impl XIGetPropertySwitchEnumAnonCase1 {
    fn from_bytes(bytes: &[u8], num_items: u32) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let length_expr = num_items as usize;
        let data16: alloc::vec::Vec<u16> = crate::vec_from_bytes_fixed!(
            ptr.get(0..).ok_or(crate::error::Error::FromBytes)?,
            u16,
            length_expr,
            2
        );
        let mut offset = length_expr * 2;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok((Self { data16 }, offset))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct XIGetPropertySwitchEnumAnonCase2 {
    pub data32: alloc::vec::Vec<u32>,
}
impl XIGetPropertySwitchEnumAnonCase2 {
    fn from_bytes(bytes: &[u8], num_items: u32) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let length_expr = num_items as usize;
        let data32: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(0..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let offset = length_expr * 4;
        Ok((Self { data32 }, offset))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum XIGetPropertySwitchEnum {
    XIGetPropertySwitchEnumAnonCase0(XIGetPropertySwitchEnumAnonCase0),

    XIGetPropertySwitchEnumAnonCase1(XIGetPropertySwitchEnumAnonCase1),

    XIGetPropertySwitchEnumAnonCase2(XIGetPropertySwitchEnumAnonCase2),

    BadValue,
}
impl XIGetPropertySwitchEnum {
    pub fn from_bytes(
        bytes: &[u8],
        format: u8,
        num_items: u32,
    ) -> crate::error::Result<(Self, usize)> {
        let mask = format;
        if mask & PropertyFormatEnum::EIGHT_BITS.0 == 0 {
            let (parsed, offset) = XIGetPropertySwitchEnumAnonCase0::from_bytes(bytes, num_items)?;
            return Ok((Self::XIGetPropertySwitchEnumAnonCase0(parsed), offset));
        }
        if mask & PropertyFormatEnum::ONE6_BITS.0 == 0 {
            let (parsed, offset) = XIGetPropertySwitchEnumAnonCase1::from_bytes(bytes, num_items)?;
            return Ok((Self::XIGetPropertySwitchEnumAnonCase1(parsed), offset));
        }
        if mask & PropertyFormatEnum::THREE2_BITS.0 == 0 {
            let (parsed, offset) = XIGetPropertySwitchEnumAnonCase2::from_bytes(bytes, num_items)?;
            return Ok((Self::XIGetPropertySwitchEnumAnonCase2(parsed), offset));
        }
        Ok((Self::BadValue, 0))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct XIGetSelectedEventsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub masks: alloc::vec::Vec<EventMask>,
}
impl VariableLengthFromBytes for XIGetSelectedEventsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 22 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_masks = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let masks_length = num_masks as usize;
        let mut offset = 32;
        let masks = crate::vec_from_bytes_var!(ptr, EventMask, offset, masks_length,);
        Ok((
            Self {
                response_type,
                sequence,
                length,
                masks,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct BarrierReleasePointerInfo {
    pub deviceid: DeviceId,
    pub barrier: crate::proto::xfixes::Barrier,
    pub eventid: u32,
}
impl FixedLengthSerialize<12> for BarrierReleasePointerInfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let deviceid_bytes = self.deviceid.serialize_fixed();
        let barrier_bytes = self.barrier.serialize_fixed();
        let eventid_bytes = self.eventid.serialize_fixed();
        [
            deviceid_bytes[0],
            deviceid_bytes[1],
            0,
            0,
            barrier_bytes[0],
            barrier_bytes[1],
            barrier_bytes[2],
            barrier_bytes[3],
            eventid_bytes[0],
            eventid_bytes[1],
            eventid_bytes[2],
            eventid_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<12> for BarrierReleasePointerInfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            deviceid: DeviceId::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            barrier: crate::proto::xfixes::Barrier::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            eventid: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const DEVICE_VALUATOR_EVENT: u8 = 0;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceValuatorEvent {
    pub opcode: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub device_state: u16,
    pub num_valuators: u8,
    pub first_valuator: u8,
    pub valuators: [i32; 6],
}
impl FixedLengthFromBytes<32> for DeviceValuatorEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            device_id: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            device_state: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?,
            num_valuators: u8::from_bytes(bytes.get(6..7).ok_or(crate::error::Error::FromBytes)?)?,
            first_valuator: u8::from_bytes(bytes.get(7..8).ok_or(crate::error::Error::FromBytes)?)?,
            // SAFETY: We know we have enough bytes, then the transmute is safe as well
            valuators: unsafe {
                let raw_bytes: [u8; 24] = bytes
                    .get(8..8 + 24)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked();
                core::mem::transmute(raw_bytes)
            },
        })
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct MoreEventsMask(pub u8);
impl MoreEventsMask {
    pub const MORE_EVENTS: Self = Self(1 << 7);
}
impl FixedLengthSerialize<1> for MoreEventsMask {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for MoreEventsMask {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for MoreEventsMask {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for MoreEventsMask {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for MoreEventsMask {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(MoreEventsMask);
pub const DEVICE_KEY_PRESS_EVENT: u8 = 1;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceKeyPressEvent {
    pub opcode: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub root: crate::proto::xproto::Window,
    pub event: crate::proto::xproto::Window,
    pub child: crate::proto::xproto::WindowEnum,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: crate::proto::xproto::KeyButMask,
    pub same_screen: u8,
    pub device_id: MoreEventsMask,
}
impl FixedLengthFromBytes<32> for DeviceKeyPressEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            detail: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            root: crate::proto::xproto::Window::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            event: crate::proto::xproto::Window::from_bytes(
                bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            child: crate::proto::xproto::Window::from_bytes(
                bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            root_x: i16::from_bytes(bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?)?,
            root_y: i16::from_bytes(bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?)?,
            event_x: i16::from_bytes(bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?)?,
            event_y: i16::from_bytes(bytes.get(26..28).ok_or(crate::error::Error::FromBytes)?)?,
            state: u16::from_bytes(bytes.get(28..30).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            same_screen: u8::from_bytes(bytes.get(30..31).ok_or(crate::error::Error::FromBytes)?)?,
            device_id: u8::from_bytes(bytes.get(31..32).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
pub const DEVICE_KEY_RELEASE_EVENT: u8 = 2;
pub type DeviceKeyReleaseEvent = DeviceKeyPressEvent;
pub const DEVICE_BUTTON_PRESS_EVENT: u8 = 3;
pub type DeviceButtonPressEvent = DeviceKeyPressEvent;
pub const DEVICE_BUTTON_RELEASE_EVENT: u8 = 4;
pub type DeviceButtonReleaseEvent = DeviceKeyPressEvent;
pub const DEVICE_MOTION_NOTIFY_EVENT: u8 = 5;
pub type DeviceMotionNotifyEvent = DeviceKeyPressEvent;
pub const DEVICE_FOCUS_IN_EVENT: u8 = 6;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceFocusInEvent {
    pub opcode: u8,
    pub detail: crate::proto::xproto::NotifyDetailEnum,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub window: crate::proto::xproto::Window,
    pub mode: crate::proto::xproto::NotifyModeEnum,
    pub device_id: u8,
}
impl FixedLengthFromBytes<32> for DeviceFocusInEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            detail: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            mode: u8::from_bytes(bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?)?.into(),
            device_id: u8::from_bytes(bytes.get(13..14).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const DEVICE_FOCUS_OUT_EVENT: u8 = 7;
pub type DeviceFocusOutEvent = DeviceFocusInEvent;
pub const PROXIMITY_IN_EVENT: u8 = 8;
pub type ProximityInEvent = DeviceKeyPressEvent;
pub const PROXIMITY_OUT_EVENT: u8 = 9;
pub type ProximityOutEvent = DeviceKeyPressEvent;
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ClassesReportedMask(pub u8);
impl ClassesReportedMask {
    pub const OUT_OF_PROXIMITY: Self = Self(1 << 7);
    pub const DEVICE_MODE_ABSOLUTE: Self = Self(1 << 6);
    pub const REPORTING_VALUATORS: Self = Self(1 << 2);
    pub const REPORTING_BUTTONS: Self = Self(1 << 1);
    pub const REPORTING_KEYS: Self = Self(1 << 0);
}
impl FixedLengthSerialize<1> for ClassesReportedMask {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ClassesReportedMask {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ClassesReportedMask {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ClassesReportedMask {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ClassesReportedMask {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(ClassesReportedMask);
pub const DEVICE_STATE_NOTIFY_EVENT: u8 = 10;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceStateNotifyEvent {
    pub opcode: u8,
    pub device_id: MoreEventsMask,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub num_keys: u8,
    pub num_buttons: u8,
    pub num_valuators: u8,
    pub classes_reported: ClassesReportedMask,
    pub buttons: [u8; 4],
    pub keys: [u8; 4],
    pub valuators: [u32; 3],
}
impl FixedLengthFromBytes<32> for DeviceStateNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            device_id: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            num_keys: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            num_buttons: u8::from_bytes(bytes.get(9..10).ok_or(crate::error::Error::FromBytes)?)?,
            num_valuators: u8::from_bytes(
                bytes.get(10..11).ok_or(crate::error::Error::FromBytes)?,
            )?,
            classes_reported: u8::from_bytes(
                bytes.get(11..12).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            // SAFETY: We know we can try into exact size slice
            buttons: unsafe {
                bytes
                    .get(12..12 + 4)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
            // SAFETY: We know we can try into exact size slice
            keys: unsafe {
                bytes
                    .get(12..12 + 4)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
            // SAFETY: We know we have enough bytes, then the transmute is safe as well
            valuators: unsafe {
                let raw_bytes: [u8; 12] = bytes
                    .get(12..12 + 12)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked();
                core::mem::transmute(raw_bytes)
            },
        })
    }
}
pub const DEVICE_MAPPING_NOTIFY_EVENT: u8 = 11;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceMappingNotifyEvent {
    pub opcode: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub request: crate::proto::xproto::MappingEnum,
    pub first_keycode: KeyCode,
    pub count: u8,
    pub time: crate::proto::xproto::Timestamp,
}
impl FixedLengthFromBytes<32> for DeviceMappingNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            device_id: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            request: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?.into(),
            first_keycode: KeyCode::from_bytes(
                bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?,
            )?,
            count: u8::from_bytes(bytes.get(6..7).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ChangeDeviceEnum(pub u8);
impl ChangeDeviceEnum {
    pub const NEW_POINTER: Self = Self(0);
    pub const NEW_KEYBOARD: Self = Self(1);
}
impl FixedLengthSerialize<1> for ChangeDeviceEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ChangeDeviceEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ChangeDeviceEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ChangeDeviceEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ChangeDeviceEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
pub const CHANGE_DEVICE_NOTIFY_EVENT: u8 = 12;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ChangeDeviceNotifyEvent {
    pub opcode: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub request: ChangeDeviceEnum,
}
impl FixedLengthFromBytes<32> for ChangeDeviceNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            device_id: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            request: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
pub const DEVICE_KEY_STATE_NOTIFY_EVENT: u8 = 13;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceKeyStateNotifyEvent {
    pub opcode: u8,
    pub device_id: MoreEventsMask,
    pub sequence: u16,
    pub keys: [u8; 28],
}
impl FixedLengthFromBytes<32> for DeviceKeyStateNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            device_id: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            // SAFETY: We know we can try into exact size slice
            keys: unsafe {
                bytes
                    .get(4..4 + 28)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}
pub const DEVICE_BUTTON_STATE_NOTIFY_EVENT: u8 = 14;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeviceButtonStateNotifyEvent {
    pub opcode: u8,
    pub device_id: MoreEventsMask,
    pub sequence: u16,
    pub buttons: [u8; 28],
}
impl FixedLengthFromBytes<32> for DeviceButtonStateNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            device_id: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            // SAFETY: We know we can try into exact size slice
            buttons: unsafe {
                bytes
                    .get(4..4 + 28)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct DeviceChangeEnum(pub u8);
impl DeviceChangeEnum {
    pub const ADDED: Self = Self(0);
    pub const REMOVED: Self = Self(1);
    pub const ENABLED: Self = Self(2);
    pub const DISABLED: Self = Self(3);
    pub const UNRECOVERABLE: Self = Self(4);
    pub const CONTROL_CHANGED: Self = Self(5);
}
impl FixedLengthSerialize<1> for DeviceChangeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for DeviceChangeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for DeviceChangeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for DeviceChangeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for DeviceChangeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
pub const DEVICE_PRESENCE_NOTIFY_EVENT: u8 = 15;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DevicePresenceNotifyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub devchange: DeviceChangeEnum,
    pub device_id: u8,
    pub control: u16,
}
impl FixedLengthFromBytes<32> for DevicePresenceNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            devchange: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            device_id: u8::from_bytes(bytes.get(9..10).ok_or(crate::error::Error::FromBytes)?)?,
            control: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const DEVICE_PROPERTY_NOTIFY_EVENT: u8 = 16;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DevicePropertyNotifyEvent {
    pub opcode: u8,
    pub state: crate::proto::xproto::PropertyEnum,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub property: u32,
    pub device_id: u8,
}
impl FixedLengthFromBytes<32> for DevicePropertyNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            state: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            property: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            device_id: u8::from_bytes(bytes.get(31..32).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ChangeReasonEnum(pub u8);
impl ChangeReasonEnum {
    pub const SLAVE_SWITCH: Self = Self(1);
    pub const DEVICE_CHANGE: Self = Self(2);
}
impl FixedLengthSerialize<1> for ChangeReasonEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ChangeReasonEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ChangeReasonEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ChangeReasonEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ChangeReasonEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
pub const DEVICE_CHANGED_EVENT: u8 = 1;
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceChangedEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub deviceid: DeviceEnum,
    pub time: crate::proto::xproto::TimeEnum,
    pub sourceid: DeviceEnum,
    pub reason: ChangeReasonEnum,
    pub classes: alloc::vec::Vec<DeviceClass>,
}
impl VariableLengthFromBytes for DeviceChangedEvent {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 11 bytes
        let opcode = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(1..3).ok_or(crate::error::Error::FromBytes)?)?;
        let deviceid = DeviceId::from_bytes(ptr.get(3..5).ok_or(crate::error::Error::FromBytes)?)?;
        let time = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(5..9).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let num_classes = u16::from_bytes(ptr.get(9..11).ok_or(crate::error::Error::FromBytes)?)?;
        let sourceid =
            DeviceId::from_bytes(ptr.get(11..13).ok_or(crate::error::Error::FromBytes)?)?;
        let reason = u8::from_bytes(ptr.get(13..14).ok_or(crate::error::Error::FromBytes)?)?;
        let classes_length = num_classes as usize;
        let mut offset = 25;
        let classes = crate::vec_from_bytes_var!(ptr, DeviceClass, offset, classes_length,);
        Ok((
            Self {
                opcode,
                sequence,
                deviceid: deviceid.into(),
                time: time.into(),
                sourceid: sourceid.into(),
                reason: reason.into(),
                classes,
            },
            offset,
        ))
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct KeyEventFlags(pub u32);
impl KeyEventFlags {
    pub const KEY_REPEAT: Self = Self(1 << 16);
}
impl FixedLengthSerialize<4> for KeyEventFlags {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for KeyEventFlags {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for KeyEventFlags {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for KeyEventFlags {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for KeyEventFlags {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(KeyEventFlags);
pub const KEY_PRESS_EVENT: u8 = 2;
#[derive(Debug, Clone, PartialEq)]
pub struct KeyPressEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub deviceid: DeviceEnum,
    pub time: crate::proto::xproto::TimeEnum,
    pub detail: u32,
    pub root: crate::proto::xproto::Window,
    pub event: crate::proto::xproto::Window,
    pub child: crate::proto::xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceEnum,
    pub flags: KeyEventFlags,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: alloc::vec::Vec<u32>,
    pub valuator_mask: alloc::vec::Vec<u32>,
    pub axisvalues: alloc::vec::Vec<Fp3232>,
}
impl VariableLengthFromBytes for KeyPressEvent {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 2 bytes
        let opcode = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(1..3).ok_or(crate::error::Error::FromBytes)?)?;
        let deviceid = DeviceId::from_bytes(ptr.get(3..5).ok_or(crate::error::Error::FromBytes)?)?;
        let time = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(5..9).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let detail = u32::from_bytes(ptr.get(9..13).ok_or(crate::error::Error::FromBytes)?)?;
        let root = crate::proto::xproto::Window::from_bytes(
            ptr.get(13..17).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let event = crate::proto::xproto::Window::from_bytes(
            ptr.get(17..21).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let child = crate::proto::xproto::Window::from_bytes(
            ptr.get(21..25).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let root_x = Fp1616::from_bytes(ptr.get(25..29).ok_or(crate::error::Error::FromBytes)?)?;
        let root_y = Fp1616::from_bytes(ptr.get(29..33).ok_or(crate::error::Error::FromBytes)?)?;
        let event_x = Fp1616::from_bytes(ptr.get(33..37).ok_or(crate::error::Error::FromBytes)?)?;
        let event_y = Fp1616::from_bytes(ptr.get(37..41).ok_or(crate::error::Error::FromBytes)?)?;
        let buttons_len = u16::from_bytes(ptr.get(41..43).ok_or(crate::error::Error::FromBytes)?)?;
        let valuators_len =
            u16::from_bytes(ptr.get(43..45).ok_or(crate::error::Error::FromBytes)?)?;
        let sourceid =
            DeviceId::from_bytes(ptr.get(45..47).ok_or(crate::error::Error::FromBytes)?)?;
        let flags = u32::from_bytes(ptr.get(49..53).ok_or(crate::error::Error::FromBytes)?)?;
        let mods =
            ModifierInfo::from_bytes(ptr.get(53..69).ok_or(crate::error::Error::FromBytes)?)?;
        let group = GroupInfo::from_bytes(ptr.get(69..73).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = buttons_len as usize;
        let button_mask: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(73..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 73;
        let length_expr = valuators_len as usize;
        let valuator_mask: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        offset += length_expr * 4;
        let length_expr = valuator_mask.iter().try_fold(0u32, |start, val| {
            start
                .checked_add(
                    u32::try_from(val.count_ones()).map_err(|_| crate::error::Error::TryFromInt)?,
                )
                .ok_or(crate::error::Error::TryFromInt)
        })? as usize;
        let axisvalues: alloc::vec::Vec<Fp3232> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Fp3232,
            length_expr,
            8
        );
        offset += length_expr * 8;
        Ok((
            Self {
                opcode,
                sequence,
                deviceid: deviceid.into(),
                time: time.into(),
                detail,
                root,
                event,
                child,
                root_x,
                root_y,
                event_x,
                event_y,
                sourceid: sourceid.into(),
                flags: flags.into(),
                mods,
                group,
                button_mask,
                valuator_mask,
                axisvalues,
            },
            offset,
        ))
    }
}
pub const KEY_RELEASE_EVENT: u8 = 3;
pub type KeyReleaseEvent = KeyPressEvent;
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct PointerEventFlags(pub u32);
impl PointerEventFlags {
    pub const POINTER_EMULATED: Self = Self(1 << 16);
}
impl FixedLengthSerialize<4> for PointerEventFlags {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for PointerEventFlags {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for PointerEventFlags {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for PointerEventFlags {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for PointerEventFlags {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(PointerEventFlags);
pub const BUTTON_PRESS_EVENT: u8 = 4;
#[derive(Debug, Clone, PartialEq)]
pub struct ButtonPressEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub deviceid: DeviceEnum,
    pub time: crate::proto::xproto::TimeEnum,
    pub detail: u32,
    pub root: crate::proto::xproto::Window,
    pub event: crate::proto::xproto::Window,
    pub child: crate::proto::xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceEnum,
    pub flags: PointerEventFlags,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: alloc::vec::Vec<u32>,
    pub valuator_mask: alloc::vec::Vec<u32>,
    pub axisvalues: alloc::vec::Vec<Fp3232>,
}
impl VariableLengthFromBytes for ButtonPressEvent {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 2 bytes
        let opcode = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(1..3).ok_or(crate::error::Error::FromBytes)?)?;
        let deviceid = DeviceId::from_bytes(ptr.get(3..5).ok_or(crate::error::Error::FromBytes)?)?;
        let time = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(5..9).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let detail = u32::from_bytes(ptr.get(9..13).ok_or(crate::error::Error::FromBytes)?)?;
        let root = crate::proto::xproto::Window::from_bytes(
            ptr.get(13..17).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let event = crate::proto::xproto::Window::from_bytes(
            ptr.get(17..21).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let child = crate::proto::xproto::Window::from_bytes(
            ptr.get(21..25).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let root_x = Fp1616::from_bytes(ptr.get(25..29).ok_or(crate::error::Error::FromBytes)?)?;
        let root_y = Fp1616::from_bytes(ptr.get(29..33).ok_or(crate::error::Error::FromBytes)?)?;
        let event_x = Fp1616::from_bytes(ptr.get(33..37).ok_or(crate::error::Error::FromBytes)?)?;
        let event_y = Fp1616::from_bytes(ptr.get(37..41).ok_or(crate::error::Error::FromBytes)?)?;
        let buttons_len = u16::from_bytes(ptr.get(41..43).ok_or(crate::error::Error::FromBytes)?)?;
        let valuators_len =
            u16::from_bytes(ptr.get(43..45).ok_or(crate::error::Error::FromBytes)?)?;
        let sourceid =
            DeviceId::from_bytes(ptr.get(45..47).ok_or(crate::error::Error::FromBytes)?)?;
        let flags = u32::from_bytes(ptr.get(49..53).ok_or(crate::error::Error::FromBytes)?)?;
        let mods =
            ModifierInfo::from_bytes(ptr.get(53..69).ok_or(crate::error::Error::FromBytes)?)?;
        let group = GroupInfo::from_bytes(ptr.get(69..73).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = buttons_len as usize;
        let button_mask: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(73..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 73;
        let length_expr = valuators_len as usize;
        let valuator_mask: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        offset += length_expr * 4;
        let length_expr = valuator_mask.iter().try_fold(0u32, |start, val| {
            start
                .checked_add(
                    u32::try_from(val.count_ones()).map_err(|_| crate::error::Error::TryFromInt)?,
                )
                .ok_or(crate::error::Error::TryFromInt)
        })? as usize;
        let axisvalues: alloc::vec::Vec<Fp3232> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Fp3232,
            length_expr,
            8
        );
        offset += length_expr * 8;
        Ok((
            Self {
                opcode,
                sequence,
                deviceid: deviceid.into(),
                time: time.into(),
                detail,
                root,
                event,
                child,
                root_x,
                root_y,
                event_x,
                event_y,
                sourceid: sourceid.into(),
                flags: flags.into(),
                mods,
                group,
                button_mask,
                valuator_mask,
                axisvalues,
            },
            offset,
        ))
    }
}
pub const BUTTON_RELEASE_EVENT: u8 = 5;
pub type ButtonReleaseEvent = ButtonPressEvent;
pub const MOTION_EVENT: u8 = 6;
pub type MotionEvent = ButtonPressEvent;
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct NotifyModeEnum(pub u8);
impl NotifyModeEnum {
    pub const NORMAL: Self = Self(0);
    pub const GRAB: Self = Self(1);
    pub const UNGRAB: Self = Self(2);
    pub const WHILE_GRABBED: Self = Self(3);
    pub const PASSIVE_GRAB: Self = Self(4);
    pub const PASSIVE_UNGRAB: Self = Self(5);
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
pub const ENTER_EVENT: u8 = 7;
#[derive(Debug, Clone, PartialEq)]
pub struct EnterEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub deviceid: DeviceEnum,
    pub time: crate::proto::xproto::TimeEnum,
    pub sourceid: DeviceEnum,
    pub mode: NotifyModeEnum,
    pub detail: NotifyDetailEnum,
    pub root: crate::proto::xproto::Window,
    pub event: crate::proto::xproto::Window,
    pub child: crate::proto::xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub same_screen: u8,
    pub focus: u8,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub buttons: alloc::vec::Vec<u32>,
}
impl VariableLengthFromBytes for EnterEvent {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let opcode = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(1..3).ok_or(crate::error::Error::FromBytes)?)?;
        let deviceid = DeviceId::from_bytes(ptr.get(3..5).ok_or(crate::error::Error::FromBytes)?)?;
        let time = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(5..9).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let sourceid = DeviceId::from_bytes(ptr.get(9..11).ok_or(crate::error::Error::FromBytes)?)?;
        let mode = u8::from_bytes(ptr.get(11..12).ok_or(crate::error::Error::FromBytes)?)?;
        let detail = u8::from_bytes(ptr.get(12..13).ok_or(crate::error::Error::FromBytes)?)?;
        let root = crate::proto::xproto::Window::from_bytes(
            ptr.get(13..17).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let event = crate::proto::xproto::Window::from_bytes(
            ptr.get(17..21).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let child = crate::proto::xproto::Window::from_bytes(
            ptr.get(21..25).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let root_x = Fp1616::from_bytes(ptr.get(25..29).ok_or(crate::error::Error::FromBytes)?)?;
        let root_y = Fp1616::from_bytes(ptr.get(29..33).ok_or(crate::error::Error::FromBytes)?)?;
        let event_x = Fp1616::from_bytes(ptr.get(33..37).ok_or(crate::error::Error::FromBytes)?)?;
        let event_y = Fp1616::from_bytes(ptr.get(37..41).ok_or(crate::error::Error::FromBytes)?)?;
        let same_screen = u8::from_bytes(ptr.get(41..42).ok_or(crate::error::Error::FromBytes)?)?;
        let focus = u8::from_bytes(ptr.get(42..43).ok_or(crate::error::Error::FromBytes)?)?;
        let buttons_len = u16::from_bytes(ptr.get(43..45).ok_or(crate::error::Error::FromBytes)?)?;
        let mods =
            ModifierInfo::from_bytes(ptr.get(45..61).ok_or(crate::error::Error::FromBytes)?)?;
        let group = GroupInfo::from_bytes(ptr.get(61..65).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = buttons_len as usize;
        let buttons: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(65..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 65;
        Ok((
            Self {
                opcode,
                sequence,
                deviceid: deviceid.into(),
                time: time.into(),
                sourceid: sourceid.into(),
                mode: mode.into(),
                detail: detail.into(),
                root,
                event,
                child,
                root_x,
                root_y,
                event_x,
                event_y,
                same_screen,
                focus,
                mods,
                group,
                buttons,
            },
            offset,
        ))
    }
}
pub const LEAVE_EVENT: u8 = 8;
pub type LeaveEvent = EnterEvent;
pub const FOCUS_IN_EVENT: u8 = 9;
pub type FocusInEvent = EnterEvent;
pub const FOCUS_OUT_EVENT: u8 = 10;
pub type FocusOutEvent = EnterEvent;
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct HierarchyMask(pub u32);
impl HierarchyMask {
    pub const MASTER_ADDED: Self = Self(1 << 0);
    pub const MASTER_REMOVED: Self = Self(1 << 1);
    pub const SLAVE_ADDED: Self = Self(1 << 2);
    pub const SLAVE_REMOVED: Self = Self(1 << 3);
    pub const SLAVE_ATTACHED: Self = Self(1 << 4);
    pub const SLAVE_DETACHED: Self = Self(1 << 5);
    pub const DEVICE_ENABLED: Self = Self(1 << 6);
    pub const DEVICE_DISABLED: Self = Self(1 << 7);
}
impl FixedLengthSerialize<4> for HierarchyMask {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for HierarchyMask {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for HierarchyMask {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for HierarchyMask {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for HierarchyMask {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(HierarchyMask);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct HierarchyInfo {
    pub deviceid: DeviceEnum,
    pub attachment: DeviceEnum,
    pub r#type: DeviceTypeEnum,
    pub enabled: u8,
    pub flags: HierarchyMask,
}
impl FixedLengthSerialize<12> for HierarchyInfo {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let deviceid_bytes = self.deviceid.serialize_fixed();
        let attachment_bytes = self.attachment.serialize_fixed();
        let flags_bytes = self.flags.serialize_fixed();
        [
            deviceid_bytes[0],
            deviceid_bytes[1],
            attachment_bytes[0],
            attachment_bytes[1],
            self.r#type.0 as u8,
            self.enabled,
            0,
            0,
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<12> for HierarchyInfo {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            deviceid: DeviceId::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            attachment: DeviceId::from_bytes(
                bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            r#type: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?.into(),
            enabled: u8::from_bytes(bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?)?,
            flags: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
pub const HIERARCHY_EVENT: u8 = 11;
#[derive(Debug, Clone, PartialEq)]
pub struct HierarchyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub deviceid: DeviceEnum,
    pub time: crate::proto::xproto::TimeEnum,
    pub flags: HierarchyMask,
    pub infos: alloc::vec::Vec<HierarchyInfo>,
}
impl VariableLengthFromBytes for HierarchyEvent {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 10 bytes
        let opcode = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(1..3).ok_or(crate::error::Error::FromBytes)?)?;
        let deviceid = DeviceId::from_bytes(ptr.get(3..5).ok_or(crate::error::Error::FromBytes)?)?;
        let time = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(5..9).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let flags = u32::from_bytes(ptr.get(9..13).ok_or(crate::error::Error::FromBytes)?)?;
        let num_infos = u16::from_bytes(ptr.get(13..15).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_infos as usize;
        let infos: alloc::vec::Vec<HierarchyInfo> = crate::vec_from_bytes_fixed!(
            ptr.get(25..).ok_or(crate::error::Error::FromBytes)?,
            HierarchyInfo,
            length_expr,
            12
        );
        let offset = length_expr * 12 + 25;
        Ok((
            Self {
                opcode,
                sequence,
                deviceid: deviceid.into(),
                time: time.into(),
                flags: flags.into(),
                infos,
            },
            offset,
        ))
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PropertyFlagEnum(pub u8);
impl PropertyFlagEnum {
    pub const DELETED: Self = Self(0);
    pub const CREATED: Self = Self(1);
    pub const MODIFIED: Self = Self(2);
}
impl FixedLengthSerialize<1> for PropertyFlagEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for PropertyFlagEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for PropertyFlagEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for PropertyFlagEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for PropertyFlagEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
pub const PROPERTY_EVENT: u8 = 12;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct PropertyEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub deviceid: DeviceEnum,
    pub time: crate::proto::xproto::TimeEnum,
    pub property: u32,
    pub what: PropertyFlagEnum,
}
impl FixedLengthFromBytes<25> for PropertyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(1..3).ok_or(crate::error::Error::FromBytes)?)?,
            deviceid: DeviceId::from_bytes(bytes.get(3..5).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(5..9).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            property: u32::from_bytes(bytes.get(9..13).ok_or(crate::error::Error::FromBytes)?)?,
            what: u8::from_bytes(bytes.get(13..14).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
pub const RAW_KEY_PRESS_EVENT: u8 = 13;
#[derive(Debug, Clone, PartialEq)]
pub struct RawKeyPressEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub deviceid: DeviceEnum,
    pub time: crate::proto::xproto::TimeEnum,
    pub detail: u32,
    pub sourceid: DeviceId,
    pub flags: KeyEventFlags,
    pub valuator_mask: alloc::vec::Vec<u32>,
    pub axisvalues: alloc::vec::Vec<Fp3232>,
    pub axisvalues_raw: alloc::vec::Vec<Fp3232>,
}
impl VariableLengthFromBytes for RawKeyPressEvent {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 4 bytes
        let opcode = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(1..3).ok_or(crate::error::Error::FromBytes)?)?;
        let deviceid = DeviceId::from_bytes(ptr.get(3..5).ok_or(crate::error::Error::FromBytes)?)?;
        let time = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(5..9).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let detail = u32::from_bytes(ptr.get(9..13).ok_or(crate::error::Error::FromBytes)?)?;
        let sourceid =
            DeviceId::from_bytes(ptr.get(13..15).ok_or(crate::error::Error::FromBytes)?)?;
        let valuators_len =
            u16::from_bytes(ptr.get(15..17).ok_or(crate::error::Error::FromBytes)?)?;
        let flags = u32::from_bytes(ptr.get(17..21).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = valuators_len as usize;
        let valuator_mask: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(25..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 25;
        let length_expr = valuator_mask.iter().try_fold(0u32, |start, val| {
            start
                .checked_add(
                    u32::try_from(val.count_ones()).map_err(|_| crate::error::Error::TryFromInt)?,
                )
                .ok_or(crate::error::Error::TryFromInt)
        })? as usize;
        let axisvalues: alloc::vec::Vec<Fp3232> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Fp3232,
            length_expr,
            8
        );
        offset += length_expr * 8;
        let length_expr = valuator_mask.iter().try_fold(0u32, |start, val| {
            start
                .checked_add(
                    u32::try_from(val.count_ones()).map_err(|_| crate::error::Error::TryFromInt)?,
                )
                .ok_or(crate::error::Error::TryFromInt)
        })? as usize;
        let axisvalues_raw: alloc::vec::Vec<Fp3232> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Fp3232,
            length_expr,
            8
        );
        offset += length_expr * 8;
        Ok((
            Self {
                opcode,
                sequence,
                deviceid: deviceid.into(),
                time: time.into(),
                detail,
                sourceid,
                flags: flags.into(),
                valuator_mask,
                axisvalues,
                axisvalues_raw,
            },
            offset,
        ))
    }
}
pub const RAW_KEY_RELEASE_EVENT: u8 = 14;
pub type RawKeyReleaseEvent = RawKeyPressEvent;
pub const RAW_BUTTON_PRESS_EVENT: u8 = 15;
#[derive(Debug, Clone, PartialEq)]
pub struct RawButtonPressEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub deviceid: DeviceEnum,
    pub time: crate::proto::xproto::TimeEnum,
    pub detail: u32,
    pub sourceid: DeviceId,
    pub flags: PointerEventFlags,
    pub valuator_mask: alloc::vec::Vec<u32>,
    pub axisvalues: alloc::vec::Vec<Fp3232>,
    pub axisvalues_raw: alloc::vec::Vec<Fp3232>,
}
impl VariableLengthFromBytes for RawButtonPressEvent {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 4 bytes
        let opcode = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(1..3).ok_or(crate::error::Error::FromBytes)?)?;
        let deviceid = DeviceId::from_bytes(ptr.get(3..5).ok_or(crate::error::Error::FromBytes)?)?;
        let time = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(5..9).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let detail = u32::from_bytes(ptr.get(9..13).ok_or(crate::error::Error::FromBytes)?)?;
        let sourceid =
            DeviceId::from_bytes(ptr.get(13..15).ok_or(crate::error::Error::FromBytes)?)?;
        let valuators_len =
            u16::from_bytes(ptr.get(15..17).ok_or(crate::error::Error::FromBytes)?)?;
        let flags = u32::from_bytes(ptr.get(17..21).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = valuators_len as usize;
        let valuator_mask: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(25..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 25;
        let length_expr = valuator_mask.iter().try_fold(0u32, |start, val| {
            start
                .checked_add(
                    u32::try_from(val.count_ones()).map_err(|_| crate::error::Error::TryFromInt)?,
                )
                .ok_or(crate::error::Error::TryFromInt)
        })? as usize;
        let axisvalues: alloc::vec::Vec<Fp3232> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Fp3232,
            length_expr,
            8
        );
        offset += length_expr * 8;
        let length_expr = valuator_mask.iter().try_fold(0u32, |start, val| {
            start
                .checked_add(
                    u32::try_from(val.count_ones()).map_err(|_| crate::error::Error::TryFromInt)?,
                )
                .ok_or(crate::error::Error::TryFromInt)
        })? as usize;
        let axisvalues_raw: alloc::vec::Vec<Fp3232> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Fp3232,
            length_expr,
            8
        );
        offset += length_expr * 8;
        Ok((
            Self {
                opcode,
                sequence,
                deviceid: deviceid.into(),
                time: time.into(),
                detail,
                sourceid,
                flags: flags.into(),
                valuator_mask,
                axisvalues,
                axisvalues_raw,
            },
            offset,
        ))
    }
}
pub const RAW_BUTTON_RELEASE_EVENT: u8 = 16;
pub type RawButtonReleaseEvent = RawButtonPressEvent;
pub const RAW_MOTION_EVENT: u8 = 17;
pub type RawMotionEvent = RawButtonPressEvent;
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct TouchEventFlags(pub u32);
impl TouchEventFlags {
    pub const TOUCH_PENDING_END: Self = Self(1 << 16);
    pub const TOUCH_EMULATING_POINTER: Self = Self(1 << 17);
}
impl FixedLengthSerialize<4> for TouchEventFlags {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for TouchEventFlags {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for TouchEventFlags {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for TouchEventFlags {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for TouchEventFlags {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(TouchEventFlags);
pub const TOUCH_BEGIN_EVENT: u8 = 18;
#[derive(Debug, Clone, PartialEq)]
pub struct TouchBeginEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub deviceid: DeviceEnum,
    pub time: crate::proto::xproto::TimeEnum,
    pub detail: u32,
    pub root: crate::proto::xproto::Window,
    pub event: crate::proto::xproto::Window,
    pub child: crate::proto::xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceEnum,
    pub flags: TouchEventFlags,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: alloc::vec::Vec<u32>,
    pub valuator_mask: alloc::vec::Vec<u32>,
    pub axisvalues: alloc::vec::Vec<Fp3232>,
}
impl VariableLengthFromBytes for TouchBeginEvent {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 2 bytes
        let opcode = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(1..3).ok_or(crate::error::Error::FromBytes)?)?;
        let deviceid = DeviceId::from_bytes(ptr.get(3..5).ok_or(crate::error::Error::FromBytes)?)?;
        let time = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(5..9).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let detail = u32::from_bytes(ptr.get(9..13).ok_or(crate::error::Error::FromBytes)?)?;
        let root = crate::proto::xproto::Window::from_bytes(
            ptr.get(13..17).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let event = crate::proto::xproto::Window::from_bytes(
            ptr.get(17..21).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let child = crate::proto::xproto::Window::from_bytes(
            ptr.get(21..25).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let root_x = Fp1616::from_bytes(ptr.get(25..29).ok_or(crate::error::Error::FromBytes)?)?;
        let root_y = Fp1616::from_bytes(ptr.get(29..33).ok_or(crate::error::Error::FromBytes)?)?;
        let event_x = Fp1616::from_bytes(ptr.get(33..37).ok_or(crate::error::Error::FromBytes)?)?;
        let event_y = Fp1616::from_bytes(ptr.get(37..41).ok_or(crate::error::Error::FromBytes)?)?;
        let buttons_len = u16::from_bytes(ptr.get(41..43).ok_or(crate::error::Error::FromBytes)?)?;
        let valuators_len =
            u16::from_bytes(ptr.get(43..45).ok_or(crate::error::Error::FromBytes)?)?;
        let sourceid =
            DeviceId::from_bytes(ptr.get(45..47).ok_or(crate::error::Error::FromBytes)?)?;
        let flags = u32::from_bytes(ptr.get(49..53).ok_or(crate::error::Error::FromBytes)?)?;
        let mods =
            ModifierInfo::from_bytes(ptr.get(53..69).ok_or(crate::error::Error::FromBytes)?)?;
        let group = GroupInfo::from_bytes(ptr.get(69..73).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = buttons_len as usize;
        let button_mask: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(73..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 73;
        let length_expr = valuators_len as usize;
        let valuator_mask: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        offset += length_expr * 4;
        let length_expr = valuator_mask.iter().try_fold(0u32, |start, val| {
            start
                .checked_add(
                    u32::try_from(val.count_ones()).map_err(|_| crate::error::Error::TryFromInt)?,
                )
                .ok_or(crate::error::Error::TryFromInt)
        })? as usize;
        let axisvalues: alloc::vec::Vec<Fp3232> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Fp3232,
            length_expr,
            8
        );
        offset += length_expr * 8;
        Ok((
            Self {
                opcode,
                sequence,
                deviceid: deviceid.into(),
                time: time.into(),
                detail,
                root,
                event,
                child,
                root_x,
                root_y,
                event_x,
                event_y,
                sourceid: sourceid.into(),
                flags: flags.into(),
                mods,
                group,
                button_mask,
                valuator_mask,
                axisvalues,
            },
            offset,
        ))
    }
}
pub const TOUCH_UPDATE_EVENT: u8 = 19;
pub type TouchUpdateEvent = TouchBeginEvent;
pub const TOUCH_END_EVENT: u8 = 20;
pub type TouchEndEvent = TouchBeginEvent;
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct TouchOwnershipFlagsEnum(pub u32);
impl TouchOwnershipFlagsEnum {
    pub const NONE: Self = Self(0);
}
impl FixedLengthSerialize<4> for TouchOwnershipFlagsEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for TouchOwnershipFlagsEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for TouchOwnershipFlagsEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for TouchOwnershipFlagsEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for TouchOwnershipFlagsEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
pub const TOUCH_OWNERSHIP_EVENT: u8 = 21;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct TouchOwnershipEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub deviceid: DeviceEnum,
    pub time: crate::proto::xproto::TimeEnum,
    pub touchid: u32,
    pub root: crate::proto::xproto::Window,
    pub event: crate::proto::xproto::Window,
    pub child: crate::proto::xproto::Window,
    pub sourceid: DeviceEnum,
    pub flags: TouchOwnershipFlagsEnum,
}
impl FixedLengthFromBytes<41> for TouchOwnershipEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(1..3).ok_or(crate::error::Error::FromBytes)?)?,
            deviceid: DeviceId::from_bytes(bytes.get(3..5).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(5..9).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            touchid: u32::from_bytes(bytes.get(9..13).ok_or(crate::error::Error::FromBytes)?)?,
            root: crate::proto::xproto::Window::from_bytes(
                bytes.get(13..17).ok_or(crate::error::Error::FromBytes)?,
            )?,
            event: crate::proto::xproto::Window::from_bytes(
                bytes.get(17..21).ok_or(crate::error::Error::FromBytes)?,
            )?,
            child: crate::proto::xproto::Window::from_bytes(
                bytes.get(21..25).ok_or(crate::error::Error::FromBytes)?,
            )?,
            sourceid: DeviceId::from_bytes(
                bytes.get(25..27).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            flags: u32::from_bytes(bytes.get(29..33).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
pub const RAW_TOUCH_BEGIN_EVENT: u8 = 22;
#[derive(Debug, Clone, PartialEq)]
pub struct RawTouchBeginEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub deviceid: DeviceEnum,
    pub time: crate::proto::xproto::TimeEnum,
    pub detail: u32,
    pub sourceid: DeviceId,
    pub flags: TouchEventFlags,
    pub valuator_mask: alloc::vec::Vec<u32>,
    pub axisvalues: alloc::vec::Vec<Fp3232>,
    pub axisvalues_raw: alloc::vec::Vec<Fp3232>,
}
impl VariableLengthFromBytes for RawTouchBeginEvent {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 4 bytes
        let opcode = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(1..3).ok_or(crate::error::Error::FromBytes)?)?;
        let deviceid = DeviceId::from_bytes(ptr.get(3..5).ok_or(crate::error::Error::FromBytes)?)?;
        let time = crate::proto::xproto::Timestamp::from_bytes(
            ptr.get(5..9).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let detail = u32::from_bytes(ptr.get(9..13).ok_or(crate::error::Error::FromBytes)?)?;
        let sourceid =
            DeviceId::from_bytes(ptr.get(13..15).ok_or(crate::error::Error::FromBytes)?)?;
        let valuators_len =
            u16::from_bytes(ptr.get(15..17).ok_or(crate::error::Error::FromBytes)?)?;
        let flags = u32::from_bytes(ptr.get(17..21).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = valuators_len as usize;
        let valuator_mask: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(25..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 25;
        let length_expr = valuator_mask.iter().try_fold(0u32, |start, val| {
            start
                .checked_add(
                    u32::try_from(val.count_ones()).map_err(|_| crate::error::Error::TryFromInt)?,
                )
                .ok_or(crate::error::Error::TryFromInt)
        })? as usize;
        let axisvalues: alloc::vec::Vec<Fp3232> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Fp3232,
            length_expr,
            8
        );
        offset += length_expr * 8;
        let length_expr = valuator_mask.iter().try_fold(0u32, |start, val| {
            start
                .checked_add(
                    u32::try_from(val.count_ones()).map_err(|_| crate::error::Error::TryFromInt)?,
                )
                .ok_or(crate::error::Error::TryFromInt)
        })? as usize;
        let axisvalues_raw: alloc::vec::Vec<Fp3232> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Fp3232,
            length_expr,
            8
        );
        offset += length_expr * 8;
        Ok((
            Self {
                opcode,
                sequence,
                deviceid: deviceid.into(),
                time: time.into(),
                detail,
                sourceid,
                flags: flags.into(),
                valuator_mask,
                axisvalues,
                axisvalues_raw,
            },
            offset,
        ))
    }
}
pub const RAW_TOUCH_UPDATE_EVENT: u8 = 23;
pub type RawTouchUpdateEvent = RawTouchBeginEvent;
pub const RAW_TOUCH_END_EVENT: u8 = 24;
pub type RawTouchEndEvent = RawTouchBeginEvent;
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct BarrierFlags(pub u32);
impl BarrierFlags {
    pub const POINTER_RELEASED: Self = Self(1 << 0);
    pub const DEVICE_IS_GRABBED: Self = Self(1 << 1);
}
impl FixedLengthSerialize<4> for BarrierFlags {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for BarrierFlags {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for BarrierFlags {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for BarrierFlags {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for BarrierFlags {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(BarrierFlags);
pub const BARRIER_HIT_EVENT: u8 = 25;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct BarrierHitEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub deviceid: DeviceEnum,
    pub time: crate::proto::xproto::TimeEnum,
    pub eventid: u32,
    pub root: crate::proto::xproto::Window,
    pub event: crate::proto::xproto::Window,
    pub barrier: crate::proto::xfixes::Barrier,
    pub dtime: u32,
    pub flags: BarrierFlags,
    pub sourceid: DeviceEnum,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub dx: Fp3232,
    pub dy: Fp3232,
}
impl FixedLengthFromBytes<61> for BarrierHitEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(1..3).ok_or(crate::error::Error::FromBytes)?)?,
            deviceid: DeviceId::from_bytes(bytes.get(3..5).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(5..9).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            eventid: u32::from_bytes(bytes.get(9..13).ok_or(crate::error::Error::FromBytes)?)?,
            root: crate::proto::xproto::Window::from_bytes(
                bytes.get(13..17).ok_or(crate::error::Error::FromBytes)?,
            )?,
            event: crate::proto::xproto::Window::from_bytes(
                bytes.get(17..21).ok_or(crate::error::Error::FromBytes)?,
            )?,
            barrier: crate::proto::xfixes::Barrier::from_bytes(
                bytes.get(21..25).ok_or(crate::error::Error::FromBytes)?,
            )?,
            dtime: u32::from_bytes(bytes.get(25..29).ok_or(crate::error::Error::FromBytes)?)?,
            flags: u32::from_bytes(bytes.get(29..33).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            sourceid: DeviceId::from_bytes(
                bytes.get(33..35).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            root_x: Fp1616::from_bytes(bytes.get(37..41).ok_or(crate::error::Error::FromBytes)?)?,
            root_y: Fp1616::from_bytes(bytes.get(41..45).ok_or(crate::error::Error::FromBytes)?)?,
            dx: Fp3232::from_bytes(bytes.get(45..53).ok_or(crate::error::Error::FromBytes)?)?,
            dy: Fp3232::from_bytes(bytes.get(53..61).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const BARRIER_LEAVE_EVENT: u8 = 26;
pub type BarrierLeaveEvent = BarrierHitEvent;
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct GesturePinchEventFlags(pub u32);
impl GesturePinchEventFlags {
    pub const GESTURE_PINCH_CANCELLED: Self = Self(1 << 0);
}
impl FixedLengthSerialize<4> for GesturePinchEventFlags {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for GesturePinchEventFlags {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for GesturePinchEventFlags {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for GesturePinchEventFlags {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for GesturePinchEventFlags {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(GesturePinchEventFlags);
pub const GESTURE_PINCH_BEGIN_EVENT: u8 = 27;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GesturePinchBeginEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub deviceid: DeviceEnum,
    pub time: crate::proto::xproto::TimeEnum,
    pub detail: u32,
    pub root: crate::proto::xproto::Window,
    pub event: crate::proto::xproto::Window,
    pub child: crate::proto::xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub delta_x: Fp1616,
    pub delta_y: Fp1616,
    pub delta_unaccel_x: Fp1616,
    pub delta_unaccel_y: Fp1616,
    pub scale: Fp1616,
    pub delta_angle: Fp1616,
    pub sourceid: DeviceEnum,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub flags: GesturePinchEventFlags,
}
impl FixedLengthFromBytes<93> for GesturePinchBeginEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(1..3).ok_or(crate::error::Error::FromBytes)?)?,
            deviceid: DeviceId::from_bytes(bytes.get(3..5).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(5..9).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            detail: u32::from_bytes(bytes.get(9..13).ok_or(crate::error::Error::FromBytes)?)?,
            root: crate::proto::xproto::Window::from_bytes(
                bytes.get(13..17).ok_or(crate::error::Error::FromBytes)?,
            )?,
            event: crate::proto::xproto::Window::from_bytes(
                bytes.get(17..21).ok_or(crate::error::Error::FromBytes)?,
            )?,
            child: crate::proto::xproto::Window::from_bytes(
                bytes.get(21..25).ok_or(crate::error::Error::FromBytes)?,
            )?,
            root_x: Fp1616::from_bytes(bytes.get(25..29).ok_or(crate::error::Error::FromBytes)?)?,
            root_y: Fp1616::from_bytes(bytes.get(29..33).ok_or(crate::error::Error::FromBytes)?)?,
            event_x: Fp1616::from_bytes(bytes.get(33..37).ok_or(crate::error::Error::FromBytes)?)?,
            event_y: Fp1616::from_bytes(bytes.get(37..41).ok_or(crate::error::Error::FromBytes)?)?,
            delta_x: Fp1616::from_bytes(bytes.get(41..45).ok_or(crate::error::Error::FromBytes)?)?,
            delta_y: Fp1616::from_bytes(bytes.get(45..49).ok_or(crate::error::Error::FromBytes)?)?,
            delta_unaccel_x: Fp1616::from_bytes(
                bytes.get(49..53).ok_or(crate::error::Error::FromBytes)?,
            )?,
            delta_unaccel_y: Fp1616::from_bytes(
                bytes.get(53..57).ok_or(crate::error::Error::FromBytes)?,
            )?,
            scale: Fp1616::from_bytes(bytes.get(57..61).ok_or(crate::error::Error::FromBytes)?)?,
            delta_angle: Fp1616::from_bytes(
                bytes.get(61..65).ok_or(crate::error::Error::FromBytes)?,
            )?,
            sourceid: DeviceId::from_bytes(
                bytes.get(65..67).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            mods: ModifierInfo::from_bytes(
                bytes.get(69..85).ok_or(crate::error::Error::FromBytes)?,
            )?,
            group: GroupInfo::from_bytes(bytes.get(85..89).ok_or(crate::error::Error::FromBytes)?)?,
            flags: u32::from_bytes(bytes.get(89..93).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
pub const GESTURE_PINCH_UPDATE_EVENT: u8 = 28;
pub type GesturePinchUpdateEvent = GesturePinchBeginEvent;
pub const GESTURE_PINCH_END_EVENT: u8 = 29;
pub type GesturePinchEndEvent = GesturePinchBeginEvent;
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct GestureSwipeEventFlags(pub u32);
impl GestureSwipeEventFlags {
    pub const GESTURE_SWIPE_CANCELLED: Self = Self(1 << 0);
}
impl FixedLengthSerialize<4> for GestureSwipeEventFlags {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for GestureSwipeEventFlags {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for GestureSwipeEventFlags {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for GestureSwipeEventFlags {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for GestureSwipeEventFlags {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(GestureSwipeEventFlags);
pub const GESTURE_SWIPE_BEGIN_EVENT: u8 = 30;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GestureSwipeBeginEvent {
    pub opcode: u8,
    pub sequence: u16,
    pub deviceid: DeviceEnum,
    pub time: crate::proto::xproto::TimeEnum,
    pub detail: u32,
    pub root: crate::proto::xproto::Window,
    pub event: crate::proto::xproto::Window,
    pub child: crate::proto::xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub delta_x: Fp1616,
    pub delta_y: Fp1616,
    pub delta_unaccel_x: Fp1616,
    pub delta_unaccel_y: Fp1616,
    pub sourceid: DeviceEnum,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub flags: GestureSwipeEventFlags,
}
impl FixedLengthFromBytes<85> for GestureSwipeBeginEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(1..3).ok_or(crate::error::Error::FromBytes)?)?,
            deviceid: DeviceId::from_bytes(bytes.get(3..5).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(5..9).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            detail: u32::from_bytes(bytes.get(9..13).ok_or(crate::error::Error::FromBytes)?)?,
            root: crate::proto::xproto::Window::from_bytes(
                bytes.get(13..17).ok_or(crate::error::Error::FromBytes)?,
            )?,
            event: crate::proto::xproto::Window::from_bytes(
                bytes.get(17..21).ok_or(crate::error::Error::FromBytes)?,
            )?,
            child: crate::proto::xproto::Window::from_bytes(
                bytes.get(21..25).ok_or(crate::error::Error::FromBytes)?,
            )?,
            root_x: Fp1616::from_bytes(bytes.get(25..29).ok_or(crate::error::Error::FromBytes)?)?,
            root_y: Fp1616::from_bytes(bytes.get(29..33).ok_or(crate::error::Error::FromBytes)?)?,
            event_x: Fp1616::from_bytes(bytes.get(33..37).ok_or(crate::error::Error::FromBytes)?)?,
            event_y: Fp1616::from_bytes(bytes.get(37..41).ok_or(crate::error::Error::FromBytes)?)?,
            delta_x: Fp1616::from_bytes(bytes.get(41..45).ok_or(crate::error::Error::FromBytes)?)?,
            delta_y: Fp1616::from_bytes(bytes.get(45..49).ok_or(crate::error::Error::FromBytes)?)?,
            delta_unaccel_x: Fp1616::from_bytes(
                bytes.get(49..53).ok_or(crate::error::Error::FromBytes)?,
            )?,
            delta_unaccel_y: Fp1616::from_bytes(
                bytes.get(53..57).ok_or(crate::error::Error::FromBytes)?,
            )?,
            sourceid: DeviceId::from_bytes(
                bytes.get(57..59).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            mods: ModifierInfo::from_bytes(
                bytes.get(61..77).ok_or(crate::error::Error::FromBytes)?,
            )?,
            group: GroupInfo::from_bytes(bytes.get(77..81).ok_or(crate::error::Error::FromBytes)?)?,
            flags: u32::from_bytes(bytes.get(81..85).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
pub const GESTURE_SWIPE_UPDATE_EVENT: u8 = 31;
pub type GestureSwipeUpdateEvent = GestureSwipeBeginEvent;
pub const GESTURE_SWIPE_END_EVENT: u8 = 32;
pub type GestureSwipeEndEvent = GestureSwipeBeginEvent;
#[derive(Debug, Copy, Clone)]
pub struct EventForSend(pub [u8; 32]);
impl FixedLengthSerialize<32> for EventForSend {
    fn serialize_fixed(self) -> [u8; 32] {
        self.0
    }
}
impl FixedLengthFromBytes<32> for EventForSend {
    fn from_bytes(buf: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(
            buf.get(0..32)
                .ok_or(crate::error::Error::FromBytes)?
                .try_into()
                .map_err(|_| crate::error::Error::FromBytes)?,
        ))
    }
}
pub const DEVICE_ERROR: u8 = 0;
pub const EVENT_ERROR: u8 = 1;
pub const MODE_ERROR: u8 = 2;
pub const DEVICE_BUSY_ERROR: u8 = 3;
pub const CLASS_ERROR: u8 = 4;
