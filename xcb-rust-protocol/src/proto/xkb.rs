#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "XKEYBOARD";
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct EventType(pub u16);
impl EventType {
    pub const NEW_KEYBOARD_NOTIFY: Self = Self(1 << 0);
    pub const MAP_NOTIFY: Self = Self(1 << 1);
    pub const STATE_NOTIFY: Self = Self(1 << 2);
    pub const CONTROLS_NOTIFY: Self = Self(1 << 3);
    pub const INDICATOR_STATE_NOTIFY: Self = Self(1 << 4);
    pub const INDICATOR_MAP_NOTIFY: Self = Self(1 << 5);
    pub const NAMES_NOTIFY: Self = Self(1 << 6);
    pub const COMPAT_MAP_NOTIFY: Self = Self(1 << 7);
    pub const BELL_NOTIFY: Self = Self(1 << 8);
    pub const ACTION_MESSAGE: Self = Self(1 << 9);
    pub const ACCESS_X_NOTIFY: Self = Self(1 << 10);
    pub const EXTENSION_DEVICE_NOTIFY: Self = Self(1 << 11);
}
impl FixedLengthSerialize<2> for EventType {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for EventType {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for EventType {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for EventType {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for EventType {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
crate::implement_bit_ops!(EventType);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct NKNDetail(pub u16);
impl NKNDetail {
    pub const KEYCODES: Self = Self(1 << 0);
    pub const GEOMETRY: Self = Self(1 << 1);
    pub const DEVICE_I_D: Self = Self(1 << 2);
}
impl FixedLengthSerialize<2> for NKNDetail {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for NKNDetail {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for NKNDetail {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for NKNDetail {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for NKNDetail {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
crate::implement_bit_ops!(NKNDetail);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct AXNDetail(pub u16);
impl AXNDetail {
    pub const S_K_PRESS: Self = Self(1 << 0);
    pub const S_K_ACCEPT: Self = Self(1 << 1);
    pub const S_K_REJECT: Self = Self(1 << 2);
    pub const S_K_RELEASE: Self = Self(1 << 3);
    pub const B_K_ACCEPT: Self = Self(1 << 4);
    pub const B_K_REJECT: Self = Self(1 << 5);
    pub const A_X_K_WARNING: Self = Self(1 << 6);
}
impl FixedLengthSerialize<2> for AXNDetail {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for AXNDetail {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for AXNDetail {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for AXNDetail {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for AXNDetail {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
crate::implement_bit_ops!(AXNDetail);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct MapPart(pub u16);
impl MapPart {
    pub const KEY_TYPES: Self = Self(1 << 0);
    pub const KEY_SYMS: Self = Self(1 << 1);
    pub const MODIFIER_MAP: Self = Self(1 << 2);
    pub const EXPLICIT_COMPONENTS: Self = Self(1 << 3);
    pub const KEY_ACTIONS: Self = Self(1 << 4);
    pub const KEY_BEHAVIORS: Self = Self(1 << 5);
    pub const VIRTUAL_MODS: Self = Self(1 << 6);
    pub const VIRTUAL_MOD_MAP: Self = Self(1 << 7);
}
impl FixedLengthSerialize<2> for MapPart {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for MapPart {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for MapPart {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for MapPart {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for MapPart {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
crate::implement_bit_ops!(MapPart);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct SetMapFlags(pub u16);
impl SetMapFlags {
    pub const RESIZE_TYPES: Self = Self(1 << 0);
    pub const RECOMPUTE_ACTIONS: Self = Self(1 << 1);
}
impl FixedLengthSerialize<2> for SetMapFlags {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for SetMapFlags {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for SetMapFlags {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for SetMapFlags {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for SetMapFlags {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
crate::implement_bit_ops!(SetMapFlags);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct StatePart(pub u16);
impl StatePart {
    pub const MODIFIER_STATE: Self = Self(1 << 0);
    pub const MODIFIER_BASE: Self = Self(1 << 1);
    pub const MODIFIER_LATCH: Self = Self(1 << 2);
    pub const MODIFIER_LOCK: Self = Self(1 << 3);
    pub const GROUP_STATE: Self = Self(1 << 4);
    pub const GROUP_BASE: Self = Self(1 << 5);
    pub const GROUP_LATCH: Self = Self(1 << 6);
    pub const GROUP_LOCK: Self = Self(1 << 7);
    pub const COMPAT_STATE: Self = Self(1 << 8);
    pub const GRAB_MODS: Self = Self(1 << 9);
    pub const COMPAT_GRAB_MODS: Self = Self(1 << 10);
    pub const LOOKUP_MODS: Self = Self(1 << 11);
    pub const COMPAT_LOOKUP_MODS: Self = Self(1 << 12);
    pub const POINTER_BUTTONS: Self = Self(1 << 13);
}
impl FixedLengthSerialize<2> for StatePart {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for StatePart {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for StatePart {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for StatePart {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for StatePart {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
crate::implement_bit_ops!(StatePart);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct BoolCtrl(pub u32);
impl BoolCtrl {
    pub const REPEAT_KEYS: Self = Self(1 << 0);
    pub const SLOW_KEYS: Self = Self(1 << 1);
    pub const BOUNCE_KEYS: Self = Self(1 << 2);
    pub const STICKY_KEYS: Self = Self(1 << 3);
    pub const MOUSE_KEYS: Self = Self(1 << 4);
    pub const MOUSE_KEYS_ACCEL: Self = Self(1 << 5);
    pub const ACCESS_X_KEYS: Self = Self(1 << 6);
    pub const ACCESS_X_TIMEOUT_MASK: Self = Self(1 << 7);
    pub const ACCESS_X_FEEDBACK_MASK: Self = Self(1 << 8);
    pub const AUDIBLE_BELL_MASK: Self = Self(1 << 9);
    pub const OVERLAY1_MASK: Self = Self(1 << 10);
    pub const OVERLAY2_MASK: Self = Self(1 << 11);
    pub const IGNORE_GROUP_LOCK_MASK: Self = Self(1 << 12);
}
impl FixedLengthSerialize<4> for BoolCtrl {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for BoolCtrl {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for BoolCtrl {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for BoolCtrl {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for BoolCtrl {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(BoolCtrl);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Control(pub u32);
impl Control {
    pub const GROUPS_WRAP: Self = Self(1 << 27);
    pub const INTERNAL_MODS: Self = Self(1 << 28);
    pub const IGNORE_LOCK_MODS: Self = Self(1 << 29);
    pub const PER_KEY_REPEAT: Self = Self(1 << 30);
    pub const CONTROLS_ENABLED: Self = Self(1 << 31);
}
impl FixedLengthSerialize<4> for Control {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for Control {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for Control {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for Control {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for Control {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(Control);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct AXOption(pub u16);
impl AXOption {
    pub const S_K_PRESS_F_B: Self = Self(1 << 0);
    pub const S_K_ACCEPT_F_B: Self = Self(1 << 1);
    pub const FEATURE_F_B: Self = Self(1 << 2);
    pub const SLOW_WARN_F_B: Self = Self(1 << 3);
    pub const INDICATOR_F_B: Self = Self(1 << 4);
    pub const STICKY_KEYS_F_B: Self = Self(1 << 5);
    pub const TWO_KEYS: Self = Self(1 << 6);
    pub const LATCH_TO_LOCK: Self = Self(1 << 7);
    pub const S_K_RELEASE_F_B: Self = Self(1 << 8);
    pub const S_K_REJECT_F_B: Self = Self(1 << 9);
    pub const B_K_REJECT_F_B: Self = Self(1 << 10);
    pub const DUMB_BELL: Self = Self(1 << 11);
}
impl FixedLengthSerialize<2> for AXOption {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for AXOption {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for AXOption {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for AXOption {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for AXOption {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
crate::implement_bit_ops!(AXOption);
pub type DeviceSpec = u16;
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct LedClassResultEnum(pub u16);
impl LedClassResultEnum {
    pub const KBD_FEEDBACK_CLASS: Self = Self(0);
    pub const LED_FEEDBACK_CLASS: Self = Self(4);
}
impl FixedLengthSerialize<2> for LedClassResultEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for LedClassResultEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for LedClassResultEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for LedClassResultEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for LedClassResultEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct LedClassEnum(pub LedClassSpec);
impl LedClassEnum {
    pub const KBD_FEEDBACK_CLASS: Self = Self(0);
    pub const LED_FEEDBACK_CLASS: Self = Self(4);
    pub const DFLT_X_I_CLASS: Self = Self(768);
    pub const ALL_X_I_CLASSES: Self = Self(1280);
}
impl FixedLengthSerialize<2> for LedClassEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for LedClassEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(LedClassSpec::from_bytes(bytes)?))
    }
}
impl From<u32> for LedClassEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for LedClassEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for LedClassEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
pub type LedClassSpec = u16;
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct BellClassResultEnum(pub u8);
impl BellClassResultEnum {
    pub const KBD_FEEDBACK_CLASS: Self = Self(0);
    pub const BELL_FEEDBACK_CLASS: Self = Self(5);
}
impl FixedLengthSerialize<1> for BellClassResultEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for BellClassResultEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for BellClassResultEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for BellClassResultEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for BellClassResultEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
pub type BellClassSpec = u16;
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct IdEnum(pub IDSpec);
impl IdEnum {
    pub const USE_CORE_KBD: Self = Self(256);
    pub const USE_CORE_PTR: Self = Self(512);
    pub const DFLT_X_I_CLASS: Self = Self(768);
    pub const DFLT_X_I_ID: Self = Self(1024);
    pub const ALL_X_I_CLASS: Self = Self(1280);
    pub const ALL_X_I_ID: Self = Self(1536);
    pub const X_I_NONE: Self = Self(65280);
}
impl FixedLengthSerialize<2> for IdEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for IdEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(IDSpec::from_bytes(bytes)?))
    }
}
impl From<u32> for IdEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for IdEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for IdEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
pub type IDSpec = u16;
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct GroupEnum(pub u8);
impl GroupEnum {
    pub const ONE: Self = Self(0);
    pub const TWO: Self = Self(1);
    pub const THREE: Self = Self(2);
    pub const FOUR: Self = Self(3);
}
impl FixedLengthSerialize<1> for GroupEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for GroupEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for GroupEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for GroupEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for GroupEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct SetOfGroup(pub u8);
impl SetOfGroup {
    pub const GROUP1: Self = Self(1 << 0);
    pub const GROUP2: Self = Self(1 << 1);
    pub const GROUP3: Self = Self(1 << 2);
    pub const GROUP4: Self = Self(1 << 3);
}
impl FixedLengthSerialize<1> for SetOfGroup {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SetOfGroup {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for SetOfGroup {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SetOfGroup {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SetOfGroup {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(SetOfGroup);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct SetOfGroups(pub u8);
impl SetOfGroups {
    pub const ANY: Self = Self(1 << 7);
}
impl FixedLengthSerialize<1> for SetOfGroups {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SetOfGroups {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for SetOfGroups {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SetOfGroups {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SetOfGroups {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(SetOfGroups);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct VModsHigh(pub u8);
impl VModsHigh {
    pub const ONE5: Self = Self(1 << 7);
    pub const ONE4: Self = Self(1 << 6);
    pub const ONE3: Self = Self(1 << 5);
    pub const ONE2: Self = Self(1 << 4);
    pub const ONE1: Self = Self(1 << 3);
    pub const ONE0: Self = Self(1 << 2);
    pub const NINE: Self = Self(1 << 1);
    pub const EIGHT: Self = Self(1 << 0);
}
impl FixedLengthSerialize<1> for VModsHigh {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for VModsHigh {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for VModsHigh {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for VModsHigh {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for VModsHigh {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(VModsHigh);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct VModsLow(pub u8);
impl VModsLow {
    pub const SEVEN: Self = Self(1 << 7);
    pub const SIX: Self = Self(1 << 6);
    pub const FIVE: Self = Self(1 << 5);
    pub const FOUR: Self = Self(1 << 4);
    pub const THREE: Self = Self(1 << 3);
    pub const TWO: Self = Self(1 << 2);
    pub const ONE: Self = Self(1 << 1);
    pub const ZERO: Self = Self(1 << 0);
}
impl FixedLengthSerialize<1> for VModsLow {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for VModsLow {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for VModsLow {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for VModsLow {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for VModsLow {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(VModsLow);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct VMod(pub u16);
impl VMod {
    pub const ONE5: Self = Self(1 << 15);
    pub const ONE4: Self = Self(1 << 14);
    pub const ONE3: Self = Self(1 << 13);
    pub const ONE2: Self = Self(1 << 12);
    pub const ONE1: Self = Self(1 << 11);
    pub const ONE0: Self = Self(1 << 10);
    pub const NINE: Self = Self(1 << 9);
    pub const EIGHT: Self = Self(1 << 8);
    pub const SEVEN: Self = Self(1 << 7);
    pub const SIX: Self = Self(1 << 6);
    pub const FIVE: Self = Self(1 << 5);
    pub const FOUR: Self = Self(1 << 4);
    pub const THREE: Self = Self(1 << 3);
    pub const TWO: Self = Self(1 << 2);
    pub const ONE: Self = Self(1 << 1);
    pub const ZERO: Self = Self(1 << 0);
}
impl FixedLengthSerialize<2> for VMod {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for VMod {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for VMod {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for VMod {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for VMod {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
crate::implement_bit_ops!(VMod);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Explicit(pub u8);
impl Explicit {
    pub const V_MOD_MAP: Self = Self(1 << 7);
    pub const BEHAVIOR: Self = Self(1 << 6);
    pub const AUTO_REPEAT: Self = Self(1 << 5);
    pub const INTERPRET: Self = Self(1 << 4);
    pub const KEY_TYPE4: Self = Self(1 << 3);
    pub const KEY_TYPE3: Self = Self(1 << 2);
    pub const KEY_TYPE2: Self = Self(1 << 1);
    pub const KEY_TYPE1: Self = Self(1 << 0);
}
impl FixedLengthSerialize<1> for Explicit {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for Explicit {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for Explicit {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for Explicit {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for Explicit {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(Explicit);
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SymInterpretMatchEnum(pub u8);
impl SymInterpretMatchEnum {
    pub const NONE_OF: Self = Self(0);
    pub const ANY_OF_OR_NONE: Self = Self(1);
    pub const ANY_OF: Self = Self(2);
    pub const ALL_OF: Self = Self(3);
    pub const EXACTLY: Self = Self(4);
}
impl FixedLengthSerialize<1> for SymInterpretMatchEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SymInterpretMatchEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for SymInterpretMatchEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SymInterpretMatchEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SymInterpretMatchEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct IMFlag(pub u8);
impl IMFlag {
    pub const NO_EXPLICIT: Self = Self(1 << 7);
    pub const NO_AUTOMATIC: Self = Self(1 << 6);
    pub const L_E_D_DRIVES_K_B: Self = Self(1 << 5);
}
impl FixedLengthSerialize<1> for IMFlag {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for IMFlag {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for IMFlag {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for IMFlag {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for IMFlag {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(IMFlag);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct IMModsWhich(pub u8);
impl IMModsWhich {
    pub const USE_COMPAT: Self = Self(1 << 4);
    pub const USE_EFFECTIVE: Self = Self(1 << 3);
    pub const USE_LOCKED: Self = Self(1 << 2);
    pub const USE_LATCHED: Self = Self(1 << 1);
    pub const USE_BASE: Self = Self(1 << 0);
}
impl FixedLengthSerialize<1> for IMModsWhich {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for IMModsWhich {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for IMModsWhich {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for IMModsWhich {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for IMModsWhich {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(IMModsWhich);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct IMGroupsWhich(pub u8);
impl IMGroupsWhich {
    pub const USE_COMPAT: Self = Self(1 << 4);
    pub const USE_EFFECTIVE: Self = Self(1 << 3);
    pub const USE_LOCKED: Self = Self(1 << 2);
    pub const USE_LATCHED: Self = Self(1 << 1);
    pub const USE_BASE: Self = Self(1 << 0);
}
impl FixedLengthSerialize<1> for IMGroupsWhich {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for IMGroupsWhich {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for IMGroupsWhich {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for IMGroupsWhich {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for IMGroupsWhich {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(IMGroupsWhich);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct IndicatorMap {
    pub flags: IMFlag,
    pub which_groups: IMGroupsWhich,
    pub groups: SetOfGroup,
    pub which_mods: IMModsWhich,
    pub mods: crate::proto::xproto::ModMask,
    pub real_mods: crate::proto::xproto::ModMask,
    pub vmods: VMod,
    pub ctrls: BoolCtrl,
}
impl FixedLengthSerialize<12> for IndicatorMap {
    #[inline]
    fn serialize_fixed(self) -> [u8; 12] {
        let vmods_bytes = self.vmods.serialize_fixed();
        let ctrls_bytes = self.ctrls.serialize_fixed();
        [
            self.flags.0 as u8,
            self.which_groups.0 as u8,
            self.groups.0 as u8,
            self.which_mods.0 as u8,
            self.mods.0 as u8,
            self.real_mods.0 as u8,
            vmods_bytes[0],
            vmods_bytes[1],
            ctrls_bytes[0],
            ctrls_bytes[1],
            ctrls_bytes[2],
            ctrls_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<12> for IndicatorMap {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            flags: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            which_groups: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            groups: u8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?.into(),
            which_mods: u8::from_bytes(bytes.get(3..4).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            mods: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?.into(),
            real_mods: u8::from_bytes(bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            vmods: u16::from_bytes(bytes.get(6..8).ok_or(crate::error::Error::FromBytes)?)?.into(),
            ctrls: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct CMDetail(pub u8);
impl CMDetail {
    pub const SYM_INTERP: Self = Self(1 << 0);
    pub const GROUP_COMPAT: Self = Self(1 << 1);
}
impl FixedLengthSerialize<1> for CMDetail {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for CMDetail {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for CMDetail {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for CMDetail {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for CMDetail {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(CMDetail);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct NameDetail(pub u32);
impl NameDetail {
    pub const KEYCODES: Self = Self(1 << 0);
    pub const GEOMETRY: Self = Self(1 << 1);
    pub const SYMBOLS: Self = Self(1 << 2);
    pub const PHYS_SYMBOLS: Self = Self(1 << 3);
    pub const TYPES: Self = Self(1 << 4);
    pub const COMPAT: Self = Self(1 << 5);
    pub const KEY_TYPE_NAMES: Self = Self(1 << 6);
    pub const K_T_LEVEL_NAMES: Self = Self(1 << 7);
    pub const INDICATOR_NAMES: Self = Self(1 << 8);
    pub const KEY_NAMES: Self = Self(1 << 9);
    pub const KEY_ALIASES: Self = Self(1 << 10);
    pub const VIRTUAL_MOD_NAMES: Self = Self(1 << 11);
    pub const GROUP_NAMES: Self = Self(1 << 12);
    pub const R_G_NAMES: Self = Self(1 << 13);
}
impl FixedLengthSerialize<4> for NameDetail {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for NameDetail {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for NameDetail {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for NameDetail {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for NameDetail {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(NameDetail);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct GBNDetail(pub u16);
impl GBNDetail {
    pub const TYPES: Self = Self(1 << 0);
    pub const COMPAT_MAP: Self = Self(1 << 1);
    pub const CLIENT_SYMBOLS: Self = Self(1 << 2);
    pub const SERVER_SYMBOLS: Self = Self(1 << 3);
    pub const INDICATOR_MAPS: Self = Self(1 << 4);
    pub const KEY_NAMES: Self = Self(1 << 5);
    pub const GEOMETRY: Self = Self(1 << 6);
    pub const OTHER_NAMES: Self = Self(1 << 7);
}
impl FixedLengthSerialize<2> for GBNDetail {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for GBNDetail {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for GBNDetail {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for GBNDetail {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for GBNDetail {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
crate::implement_bit_ops!(GBNDetail);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct XIFeature(pub u16);
impl XIFeature {
    pub const KEYBOARDS: Self = Self(1 << 0);
    pub const BUTTON_ACTIONS: Self = Self(1 << 1);
    pub const INDICATOR_NAMES: Self = Self(1 << 2);
    pub const INDICATOR_MAPS: Self = Self(1 << 3);
    pub const INDICATOR_STATE: Self = Self(1 << 4);
}
impl FixedLengthSerialize<2> for XIFeature {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<2> for XIFeature {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u16::from_bytes(bytes)?))
    }
}
impl From<u32> for XIFeature {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u16)
    }
}
impl From<u16> for XIFeature {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u16)
    }
}
impl From<u8> for XIFeature {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u16)
    }
}
crate::implement_bit_ops!(XIFeature);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct PerClientFlag(pub u32);
impl PerClientFlag {
    pub const DETECTABLE_AUTO_REPEAT: Self = Self(1 << 0);
    pub const GRABS_USE_X_K_B_STATE: Self = Self(1 << 1);
    pub const AUTO_RESET_CONTROLS: Self = Self(1 << 2);
    pub const LOOKUP_STATE_WHEN_GRABBED: Self = Self(1 << 3);
    pub const SEND_EVENT_USES_X_K_B_STATE: Self = Self(1 << 4);
}
impl FixedLengthSerialize<4> for PerClientFlag {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for PerClientFlag {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for PerClientFlag {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for PerClientFlag {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for PerClientFlag {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(PerClientFlag);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ModDef {
    pub mask: crate::proto::xproto::ModMask,
    pub real_mods: crate::proto::xproto::ModMask,
    pub vmods: VMod,
}
impl FixedLengthSerialize<4> for ModDef {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        let vmods_bytes = self.vmods.serialize_fixed();
        [
            self.mask.0 as u8,
            self.real_mods.0 as u8,
            vmods_bytes[0],
            vmods_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<4> for ModDef {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            mask: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            real_mods: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            vmods: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct KeyName {
    pub name: [u8; 4],
}
impl FixedLengthSerialize<4> for KeyName {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        [self.name[0], self.name[1], self.name[2], self.name[3]]
    }
}
impl FixedLengthFromBytes<4> for KeyName {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            // SAFETY: We know we can try into exact size slice
            name: unsafe {
                bytes
                    .get(0..0 + 4)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct KeyAlias {
    pub real: [u8; 4],
    pub alias: [u8; 4],
}
impl FixedLengthSerialize<8> for KeyAlias {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        [
            self.real[0],
            self.real[1],
            self.real[2],
            self.real[3],
            self.alias[0],
            self.alias[1],
            self.alias[2],
            self.alias[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for KeyAlias {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            // SAFETY: We know we can try into exact size slice
            real: unsafe {
                bytes
                    .get(0..0 + 4)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
            // SAFETY: We know we can try into exact size slice
            alias: unsafe {
                bytes
                    .get(0..0 + 4)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CountedString16 {
    pub string: alloc::vec::Vec<u8>,
    pub alignment_pad: alloc::vec::Vec<u8>,
}
impl VariableLengthSerialize for CountedString16 {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let length =
            u16::try_from(self.alignment_pad.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(core::ops::Add::add(
                    core::ops::BitAnd::bitand(
                        core::ops::Sub::sub(length as u16, 5u16 as u16) as u16,
                        core::ops::Not::not(3u16) as u16,
                    ) as u16,
                    core::ops::Sub::sub(length as u16, 2u16 as u16) as u16,
                ))
                .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = self.string.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(2..).ok_or(crate::error::Error::Serialize)?,
            &self.string,
        )?;
        let mut offset = list_len + 2;
        let list_len = self.alignment_pad.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            &self.alignment_pad,
        )?;
        offset += list_len;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for CountedString16 {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let length = u16::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = length as usize;
        let string: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(2..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr + 2;
        let length_expr = core::ops::Sub::sub(
            core::ops::BitAnd::bitand(
                core::ops::Add::add(length as u16, 5u16 as u16) as u16,
                core::ops::Not::not(3u16) as u16,
            ) as u16,
            core::ops::Add::add(length as u16, 2u16 as u16) as u16,
        ) as usize;
        let alignment_pad: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        offset += length_expr;
        Ok((
            Self {
                string,
                alignment_pad,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct KTMapEntry {
    pub active: u8,
    pub mods_mask: crate::proto::xproto::ModMask,
    pub level: u8,
    pub mods_mods: crate::proto::xproto::ModMask,
    pub mods_vmods: VMod,
}
impl FixedLengthSerialize<8> for KTMapEntry {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let mods_vmods_bytes = self.mods_vmods.serialize_fixed();
        [
            self.active,
            self.mods_mask.0 as u8,
            self.level,
            self.mods_mods.0 as u8,
            mods_vmods_bytes[0],
            mods_vmods_bytes[1],
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for KTMapEntry {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            active: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            mods_mask: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            level: u8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?,
            mods_mods: u8::from_bytes(bytes.get(3..4).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            mods_vmods: u16::from_bytes(bytes.get(4..6).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct KeyType {
    pub mods_mask: crate::proto::xproto::ModMask,
    pub mods_mods: crate::proto::xproto::ModMask,
    pub mods_vmods: VMod,
    pub num_levels: u8,
    pub has_preserve: u8,
    pub map: alloc::vec::Vec<KTMapEntry>,
    pub preserve: alloc::vec::Vec<ModDef>,
}
impl VariableLengthSerialize for KeyType {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let n_map_entries =
            u8::try_from(self.preserve.len()).map_err(|_| crate::error::Error::Serialize)?;
        let has_preserve =
            u8::try_from(self.preserve.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Padding 1 bytes
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.mods_mask.serialize_fixed());
        buf_ptr
            .get_mut(1..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.mods_mods.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.mods_vmods.serialize_fixed());
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.num_levels.serialize_fixed());
        buf_ptr
            .get_mut(5..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(core::ops::Div::div(has_preserve as u8, n_map_entries as u8))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        buf_ptr
            .get_mut(6..7)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(core::ops::Div::div(has_preserve as u8, n_map_entries as u8))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = self.map.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            self.map,
        )?;
        let mut offset = list_len + 8;
        let list_len = self.preserve.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            self.preserve,
        )?;
        offset += list_len;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for KeyType {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        let mods_mask = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let mods_mods = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let mods_vmods = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let num_levels = u8::from_bytes(ptr.get(4..5).ok_or(crate::error::Error::FromBytes)?)?;
        let n_map_entries = u8::from_bytes(ptr.get(5..6).ok_or(crate::error::Error::FromBytes)?)?;
        let has_preserve = u8::from_bytes(ptr.get(6..7).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n_map_entries as usize;
        let map: alloc::vec::Vec<KTMapEntry> = crate::vec_from_bytes_fixed!(
            ptr.get(8..).ok_or(crate::error::Error::FromBytes)?,
            KTMapEntry,
            length_expr,
            8
        );
        let mut offset = length_expr * 8 + 8;
        let length_expr = core::ops::Mul::mul(has_preserve as u8, n_map_entries as u8) as usize;
        let preserve: alloc::vec::Vec<ModDef> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            ModDef,
            length_expr,
            4
        );
        offset += length_expr * 4;
        Ok((
            Self {
                mods_mask: mods_mask.into(),
                mods_mods: mods_mods.into(),
                mods_vmods: mods_vmods.into(),
                num_levels,
                has_preserve,
                map,
                preserve,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct KeySymMap {
    pub kt_index: [u8; 4],
    pub group_info: u8,
    pub width: u8,
    pub syms: alloc::vec::Vec<crate::proto::xproto::Keysym>,
}
impl VariableLengthSerialize for KeySymMap {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let list_len = self.kt_index.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(0..).ok_or(crate::error::Error::Serialize)?,
            &self.kt_index,
        )?;
        let mut offset = list_len;
        let n_syms = u16::try_from(self.syms.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(offset..offset + 1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.group_info.serialize_fixed());
        buf_ptr
            .get_mut(offset + 1..offset + 2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.width.serialize_fixed());
        buf_ptr
            .get_mut(offset + 2..offset + 4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(n_syms).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        offset += 4;
        let list_len = self.syms.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            self.syms,
        )?;
        offset += list_len;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for KeySymMap {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let kt_index = <[u8; 4]>::from_bytes(ptr.get(0..4).ok_or(crate::error::Error::FromBytes)?)?;
        let group_info = u8::from_bytes(ptr.get(4..5).ok_or(crate::error::Error::FromBytes)?)?;
        let width = u8::from_bytes(ptr.get(5..6).ok_or(crate::error::Error::FromBytes)?)?;
        let n_syms = u16::from_bytes(ptr.get(6..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n_syms as usize;
        let syms: alloc::vec::Vec<crate::proto::xproto::Keysym> = crate::vec_from_bytes_fixed!(
            ptr.get(8..).ok_or(crate::error::Error::FromBytes)?,
            crate::proto::xproto::Keysym,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 8;
        Ok((
            Self {
                kt_index,
                group_info,
                width,
                syms,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CommonBehavior {
    pub r#type: u8,
    pub data: u8,
}
impl FixedLengthSerialize<2> for CommonBehavior {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        [self.r#type, self.data]
    }
}
impl FixedLengthFromBytes<2> for CommonBehavior {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            data: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DefaultBehavior {
    pub r#type: u8,
}
impl FixedLengthSerialize<2> for DefaultBehavior {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        [self.r#type, 0]
    }
}
impl FixedLengthFromBytes<2> for DefaultBehavior {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub type LockBehavior = DefaultBehavior;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct RadioGroupBehavior {
    pub r#type: u8,
    pub group: u8,
}
impl FixedLengthSerialize<2> for RadioGroupBehavior {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        [self.r#type, self.group]
    }
}
impl FixedLengthFromBytes<2> for RadioGroupBehavior {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            group: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct OverlayBehavior {
    pub r#type: u8,
    pub key: crate::proto::xproto::Keycode,
}
impl FixedLengthSerialize<2> for OverlayBehavior {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        let key_bytes = self.key.serialize_fixed();
        [self.r#type, key_bytes[0]]
    }
}
impl FixedLengthFromBytes<2> for OverlayBehavior {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            key: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
pub type PermamentLockBehavior = LockBehavior;
pub type PermamentRadioGroupBehavior = RadioGroupBehavior;
pub type PermamentOverlayBehavior = OverlayBehavior;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Behavior(pub [u8; 2]);
impl FixedLengthFromBytes<2> for Behavior {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        // SAFETY: Checked that the bytes are available
        Ok(Self(unsafe {
            bytes
                .get(..2)
                .ok_or(crate::error::Error::FromBytes)?
                .try_into()
                .unwrap_unchecked()
        }))
    }
}
impl FixedLengthSerialize<2> for Behavior {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        self.0
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SetBehavior {
    pub keycode: crate::proto::xproto::Keycode,
    pub behavior: Behavior,
}
impl FixedLengthSerialize<4> for SetBehavior {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        let keycode_bytes = self.keycode.serialize_fixed();
        let behavior_bytes = self.behavior.serialize_fixed();
        [keycode_bytes[0], behavior_bytes[0], behavior_bytes[1], 0]
    }
}
impl FixedLengthFromBytes<4> for SetBehavior {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            keycode: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?,
            )?,
            behavior: Behavior::from_bytes(bytes.get(1..3).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SetExplicit {
    pub keycode: crate::proto::xproto::Keycode,
    pub explicit: Explicit,
}
impl FixedLengthSerialize<2> for SetExplicit {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        let keycode_bytes = self.keycode.serialize_fixed();
        [keycode_bytes[0], self.explicit.0 as u8]
    }
}
impl FixedLengthFromBytes<2> for SetExplicit {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            keycode: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?,
            )?,
            explicit: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct KeyModMap {
    pub keycode: crate::proto::xproto::Keycode,
    pub mods: crate::proto::xproto::ModMask,
}
impl FixedLengthSerialize<2> for KeyModMap {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        let keycode_bytes = self.keycode.serialize_fixed();
        [keycode_bytes[0], self.mods.0 as u8]
    }
}
impl FixedLengthFromBytes<2> for KeyModMap {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            keycode: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?,
            )?,
            mods: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct KeyVModMap {
    pub keycode: crate::proto::xproto::Keycode,
    pub vmods: VMod,
}
impl FixedLengthSerialize<4> for KeyVModMap {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        let keycode_bytes = self.keycode.serialize_fixed();
        let vmods_bytes = self.vmods.serialize_fixed();
        [keycode_bytes[0], 0, vmods_bytes[0], vmods_bytes[1]]
    }
}
impl FixedLengthFromBytes<4> for KeyVModMap {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            keycode: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?,
            )?,
            vmods: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct KTSetMapEntry {
    pub level: u8,
    pub real_mods: crate::proto::xproto::ModMask,
    pub virtual_mods: VMod,
}
impl FixedLengthSerialize<4> for KTSetMapEntry {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        let virtual_mods_bytes = self.virtual_mods.serialize_fixed();
        [
            self.level,
            self.real_mods.0 as u8,
            virtual_mods_bytes[0],
            virtual_mods_bytes[1],
        ]
    }
}
impl FixedLengthFromBytes<4> for KTSetMapEntry {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            level: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            real_mods: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            virtual_mods: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetKeyType {
    pub mask: crate::proto::xproto::ModMask,
    pub real_mods: crate::proto::xproto::ModMask,
    pub virtual_mods: VMod,
    pub num_levels: u8,
    pub preserve: u8,
    pub entries: alloc::vec::Vec<KTSetMapEntry>,
    pub preserve_entries: alloc::vec::Vec<KTSetMapEntry>,
}
impl VariableLengthSerialize for SetKeyType {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let n_map_entries = u8::try_from(self.preserve_entries.len())
            .map_err(|_| crate::error::Error::Serialize)?;
        let preserve = u8::try_from(self.preserve_entries.len())
            .map_err(|_| crate::error::Error::Serialize)?;
        // Padding 1 bytes
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.mask.serialize_fixed());
        buf_ptr
            .get_mut(1..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.real_mods.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.virtual_mods.serialize_fixed());
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.num_levels.serialize_fixed());
        buf_ptr
            .get_mut(5..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(core::ops::Div::div(preserve as u8, n_map_entries as u8))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        buf_ptr
            .get_mut(6..7)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(core::ops::Div::div(preserve as u8, n_map_entries as u8))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = self.entries.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            self.entries,
        )?;
        let mut offset = list_len + 8;
        let list_len = self.preserve_entries.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            self.preserve_entries,
        )?;
        offset += list_len;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for SetKeyType {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        let mask = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let real_mods = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let virtual_mods = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let num_levels = u8::from_bytes(ptr.get(4..5).ok_or(crate::error::Error::FromBytes)?)?;
        let n_map_entries = u8::from_bytes(ptr.get(5..6).ok_or(crate::error::Error::FromBytes)?)?;
        let preserve = u8::from_bytes(ptr.get(6..7).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n_map_entries as usize;
        let entries: alloc::vec::Vec<KTSetMapEntry> = crate::vec_from_bytes_fixed!(
            ptr.get(8..).ok_or(crate::error::Error::FromBytes)?,
            KTSetMapEntry,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 8;
        let length_expr = core::ops::Mul::mul(preserve as u8, n_map_entries as u8) as usize;
        let preserve_entries: alloc::vec::Vec<KTSetMapEntry> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            KTSetMapEntry,
            length_expr,
            4
        );
        offset += length_expr * 4;
        Ok((
            Self {
                mask: mask.into(),
                real_mods: real_mods.into(),
                virtual_mods: virtual_mods.into(),
                num_levels,
                preserve,
                entries,
                preserve_entries,
            },
            offset,
        ))
    }
}
pub type String8 = u8;
#[derive(Debug, Clone, PartialEq)]
pub struct Outline {
    pub corner_radius: u8,
    pub points: alloc::vec::Vec<crate::proto::xproto::Point>,
}
impl VariableLengthSerialize for Outline {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let n_points =
            u8::try_from(self.points.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Padding 2 bytes
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(n_points).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(1..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.corner_radius.serialize_fixed());
        let list_len = self.points.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?,
            self.points,
        )?;
        let offset = list_len + 4;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for Outline {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 2 bytes
        let n_points = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let corner_radius = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n_points as usize;
        let points: alloc::vec::Vec<crate::proto::xproto::Point> = crate::vec_from_bytes_fixed!(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            crate::proto::xproto::Point,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 4;
        Ok((
            Self {
                corner_radius,
                points,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    pub name: u32,
    pub primary_ndx: u8,
    pub approx_ndx: u8,
    pub outlines: alloc::vec::Vec<Outline>,
}
impl VariableLengthSerialize for Shape {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let n_outlines =
            u8::try_from(self.outlines.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Padding 1 bytes
        buf_ptr
            .get_mut(0..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.name.serialize_fixed());
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(n_outlines).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(5..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.primary_ndx.serialize_fixed());
        buf_ptr
            .get_mut(6..7)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.approx_ndx.serialize_fixed());
        let offset = crate::util::var_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            self.outlines,
        )?;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for Shape {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        let name = u32::from_bytes(ptr.get(0..4).ok_or(crate::error::Error::FromBytes)?)?;
        let n_outlines = u8::from_bytes(ptr.get(4..5).ok_or(crate::error::Error::FromBytes)?)?;
        let primary_ndx = u8::from_bytes(ptr.get(5..6).ok_or(crate::error::Error::FromBytes)?)?;
        let approx_ndx = u8::from_bytes(ptr.get(6..7).ok_or(crate::error::Error::FromBytes)?)?;
        let outlines_length = n_outlines as usize;
        let mut offset = 8;
        let outlines = crate::vec_from_bytes_var!(ptr, Outline, offset, outlines_length,);
        Ok((
            Self {
                name,
                primary_ndx,
                approx_ndx,
                outlines,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Key {
    pub name: [String8; 4],
    pub gap: i16,
    pub shape_ndx: u8,
    pub color_ndx: u8,
}
impl FixedLengthSerialize<8> for Key {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let gap_bytes = self.gap.serialize_fixed();
        [
            self.name[0],
            self.name[1],
            self.name[2],
            self.name[3],
            gap_bytes[0],
            gap_bytes[1],
            self.shape_ndx,
            self.color_ndx,
        ]
    }
}
impl FixedLengthFromBytes<8> for Key {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            // SAFETY: We know we can try into exact size slice
            name: unsafe {
                bytes
                    .get(0..0 + 4)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
            gap: i16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            shape_ndx: u8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?,
            color_ndx: u8::from_bytes(bytes.get(3..4).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct OverlayKey {
    pub over: [String8; 4],
    pub under: [String8; 4],
}
impl FixedLengthSerialize<8> for OverlayKey {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        [
            self.over[0],
            self.over[1],
            self.over[2],
            self.over[3],
            self.under[0],
            self.under[1],
            self.under[2],
            self.under[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for OverlayKey {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            // SAFETY: We know we can try into exact size slice
            over: unsafe {
                bytes
                    .get(0..0 + 4)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
            // SAFETY: We know we can try into exact size slice
            under: unsafe {
                bytes
                    .get(0..0 + 4)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct OverlayRow {
    pub row_under: u8,
    pub keys: alloc::vec::Vec<OverlayKey>,
}
impl VariableLengthSerialize for OverlayRow {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let n_keys = u8::try_from(self.keys.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Padding 2 bytes
        buf_ptr
            .get_mut(0..1)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.row_under.serialize_fixed());
        buf_ptr
            .get_mut(1..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(n_keys).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.keys.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?,
            self.keys,
        )?;
        let offset = list_len + 4;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for OverlayRow {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 2 bytes
        let row_under = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let n_keys = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n_keys as usize;
        let keys: alloc::vec::Vec<OverlayKey> = crate::vec_from_bytes_fixed!(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            OverlayKey,
            length_expr,
            8
        );
        let offset = length_expr * 8 + 4;
        Ok((Self { row_under, keys }, offset))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Overlay {
    pub name: u32,
    pub rows: alloc::vec::Vec<OverlayRow>,
}
impl VariableLengthSerialize for Overlay {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let n_rows = u8::try_from(self.rows.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Padding 3 bytes
        buf_ptr
            .get_mut(0..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.name.serialize_fixed());
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(n_rows).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let offset = crate::util::var_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            self.rows,
        )?;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for Overlay {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 3 bytes
        let name = u32::from_bytes(ptr.get(0..4).ok_or(crate::error::Error::FromBytes)?)?;
        let n_rows = u8::from_bytes(ptr.get(4..5).ok_or(crate::error::Error::FromBytes)?)?;
        let rows_length = n_rows as usize;
        let mut offset = 8;
        let rows = crate::vec_from_bytes_var!(ptr, OverlayRow, offset, rows_length,);
        Ok((Self { name, rows }, offset))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub top: i16,
    pub left: i16,
    pub vertical: u8,
    pub keys: alloc::vec::Vec<Key>,
}
impl VariableLengthSerialize for Row {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let n_keys = u8::try_from(self.keys.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Padding 2 bytes
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.top.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.left.serialize_fixed());
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(n_keys).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(5..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.vertical.serialize_fixed());
        let list_len = self.keys.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            self.keys,
        )?;
        let offset = list_len + 8;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for Row {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 2 bytes
        let top = i16::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let left = i16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let n_keys = u8::from_bytes(ptr.get(4..5).ok_or(crate::error::Error::FromBytes)?)?;
        let vertical = u8::from_bytes(ptr.get(5..6).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n_keys as usize;
        let keys: alloc::vec::Vec<Key> = crate::vec_from_bytes_fixed!(
            ptr.get(8..).ok_or(crate::error::Error::FromBytes)?,
            Key,
            length_expr,
            8
        );
        let offset = length_expr * 8 + 8;
        Ok((
            Self {
                top,
                left,
                vertical,
                keys,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Listing {
    pub flags: u16,
    pub string: alloc::vec::Vec<String8>,
}
impl VariableLengthSerialize for Listing {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let length =
            u16::try_from(self.string.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.flags.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(length).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.string.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(4..).ok_or(crate::error::Error::Serialize)?,
            &self.string,
        )?;
        let mut offset = list_len + 4;
        // Align 2 bytes
        offset += (2 - (offset % 2)) % 2;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for Listing {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let flags = u16::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = length as usize;
        let string: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(4..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr + 4;
        // Align 2 bytes
        offset += (2 - (offset % 2)) % 2;
        Ok((Self { flags, string }, offset))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceLedInfo {
    pub led_class: LedClassEnum,
    pub led_i_d: IdEnum,
    pub phys_indicators: u32,
    pub state: u32,
    pub names: alloc::vec::Vec<u32>,
    pub maps: alloc::vec::Vec<IndicatorMap>,
}
impl VariableLengthSerialize for DeviceLedInfo {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let names_present =
            u32::try_from(self.names.len()).map_err(|_| crate::error::Error::Serialize)?;
        let maps_present =
            u32::try_from(self.maps.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.led_class.serialize_fixed());
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.led_i_d.serialize_fixed());
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(names_present.count_ones())
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(maps_present.count_ones())
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.phys_indicators.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.state.serialize_fixed());
        let list_len = self.names.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(20..)
                .ok_or(crate::error::Error::Serialize)?,
            self.names,
        )?;
        let mut offset = list_len + 20;
        let list_len = self.maps.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            self.maps,
        )?;
        offset += list_len;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for DeviceLedInfo {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let led_class =
            LedClassSpec::from_bytes(ptr.get(0..2).ok_or(crate::error::Error::FromBytes)?)?;
        let led_i_d = IDSpec::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let names_present = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let maps_present = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let phys_indicators =
            u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let state = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = names_present.count_ones() as usize;
        let names: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(20..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let mut offset = length_expr * 4 + 20;
        let length_expr = maps_present.count_ones() as usize;
        let maps: alloc::vec::Vec<IndicatorMap> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            IndicatorMap,
            length_expr,
            12
        );
        offset += length_expr * 12;
        Ok((
            Self {
                led_class: led_class.into(),
                led_i_d: led_i_d.into(),
                phys_indicators,
                state,
                names,
                maps,
            },
            offset,
        ))
    }
}
pub const KEYBOARD_ERROR: u8 = 0;
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Sa(pub u8);
impl Sa {
    pub const CLEAR_LOCKS: Self = Self(1 << 0);
    pub const LATCH_TO_LOCK: Self = Self(1 << 1);
    pub const USE_MOD_MAP_MODS: Self = Self(1 << 2);
    pub const GROUP_ABSOLUTE: Self = Self(1 << 2);
}
impl FixedLengthSerialize<1> for Sa {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for Sa {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for Sa {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for Sa {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for Sa {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(Sa);
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SATypeEnum(pub u8);
impl SATypeEnum {
    pub const NO_ACTION: Self = Self(0);
    pub const SET_MODS: Self = Self(1);
    pub const LATCH_MODS: Self = Self(2);
    pub const LOCK_MODS: Self = Self(3);
    pub const SET_GROUP: Self = Self(4);
    pub const LATCH_GROUP: Self = Self(5);
    pub const LOCK_GROUP: Self = Self(6);
    pub const MOVE_PTR: Self = Self(7);
    pub const PTR_BTN: Self = Self(8);
    pub const LOCK_PTR_BTN: Self = Self(9);
    pub const SET_PTR_DFLT: Self = Self(10);
    pub const I_S_O_LOCK: Self = Self(11);
    pub const TERMINATE: Self = Self(12);
    pub const SWITCH_SCREEN: Self = Self(13);
    pub const SET_CONTROLS: Self = Self(14);
    pub const LOCK_CONTROLS: Self = Self(15);
    pub const ACTION_MESSAGE: Self = Self(16);
    pub const REDIRECT_KEY: Self = Self(17);
    pub const DEVICE_BTN: Self = Self(18);
    pub const LOCK_DEVICE_BTN: Self = Self(19);
    pub const DEVICE_VALUATOR: Self = Self(20);
}
impl FixedLengthSerialize<1> for SATypeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SATypeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for SATypeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SATypeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SATypeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SANoAction {
    pub r#type: SATypeEnum,
}
impl FixedLengthSerialize<8> for SANoAction {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        [self.r#type.0 as u8, 0, 0, 0, 0, 0, 0, 0]
    }
}
impl FixedLengthFromBytes<8> for SANoAction {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SASetMods {
    pub r#type: SATypeEnum,
    pub flags: Sa,
    pub mask: crate::proto::xproto::ModMask,
    pub real_mods: crate::proto::xproto::ModMask,
    pub vmods_high: VModsHigh,
    pub vmods_low: VModsLow,
}
impl FixedLengthSerialize<8> for SASetMods {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        [
            self.r#type.0 as u8,
            self.flags.0 as u8,
            self.mask.0 as u8,
            self.real_mods.0 as u8,
            self.vmods_high.0 as u8,
            self.vmods_low.0 as u8,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for SASetMods {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            flags: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            mask: u8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?.into(),
            real_mods: u8::from_bytes(bytes.get(3..4).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            vmods_high: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            vmods_low: u8::from_bytes(bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
pub type SALatchMods = SASetMods;
pub type SALockMods = SASetMods;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SASetGroup {
    pub r#type: SATypeEnum,
    pub flags: Sa,
    pub group: i8,
}
impl FixedLengthSerialize<8> for SASetGroup {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let group_bytes = self.group.serialize_fixed();
        [
            self.r#type.0 as u8,
            self.flags.0 as u8,
            group_bytes[0],
            0,
            0,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for SASetGroup {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            flags: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            group: i8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub type SALatchGroup = SASetGroup;
pub type SALockGroup = SASetGroup;
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct SAMovePtrFlag(pub u8);
impl SAMovePtrFlag {
    pub const NO_ACCELERATION: Self = Self(1 << 0);
    pub const MOVE_ABSOLUTE_X: Self = Self(1 << 1);
    pub const MOVE_ABSOLUTE_Y: Self = Self(1 << 2);
}
impl FixedLengthSerialize<1> for SAMovePtrFlag {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SAMovePtrFlag {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for SAMovePtrFlag {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SAMovePtrFlag {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SAMovePtrFlag {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(SAMovePtrFlag);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SAMovePtr {
    pub r#type: SATypeEnum,
    pub flags: SAMovePtrFlag,
    pub x_high: i8,
    pub x_low: u8,
    pub y_high: i8,
    pub y_low: u8,
}
impl FixedLengthSerialize<8> for SAMovePtr {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let x_high_bytes = self.x_high.serialize_fixed();
        let y_high_bytes = self.y_high.serialize_fixed();
        [
            self.r#type.0 as u8,
            self.flags.0 as u8,
            x_high_bytes[0],
            self.x_low,
            y_high_bytes[0],
            self.y_low,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for SAMovePtr {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            flags: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            x_high: i8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?,
            x_low: u8::from_bytes(bytes.get(3..4).ok_or(crate::error::Error::FromBytes)?)?,
            y_high: i8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?,
            y_low: u8::from_bytes(bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SAPtrBtn {
    pub r#type: SATypeEnum,
    pub flags: u8,
    pub count: u8,
    pub button: u8,
}
impl FixedLengthSerialize<8> for SAPtrBtn {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        [
            self.r#type.0 as u8,
            self.flags,
            self.count,
            self.button,
            0,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for SAPtrBtn {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            flags: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            count: u8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?,
            button: u8::from_bytes(bytes.get(3..4).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SALockPtrBtn {
    pub r#type: SATypeEnum,
    pub flags: u8,
    pub button: u8,
}
impl FixedLengthSerialize<8> for SALockPtrBtn {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        [self.r#type.0 as u8, self.flags, 0, self.button, 0, 0, 0, 0]
    }
}
impl FixedLengthFromBytes<8> for SALockPtrBtn {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            flags: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            button: u8::from_bytes(bytes.get(3..4).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct SASetPtrDfltFlag(pub u8);
impl SASetPtrDfltFlag {
    pub const DFLT_BTN_ABSOLUTE: Self = Self(1 << 2);
    pub const AFFECT_DFLT_BUTTON: Self = Self(1 << 0);
}
impl FixedLengthSerialize<1> for SASetPtrDfltFlag {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SASetPtrDfltFlag {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for SASetPtrDfltFlag {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SASetPtrDfltFlag {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SASetPtrDfltFlag {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(SASetPtrDfltFlag);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SASetPtrDflt {
    pub r#type: SATypeEnum,
    pub flags: SASetPtrDfltFlag,
    pub affect: SASetPtrDfltFlag,
    pub value: i8,
}
impl FixedLengthSerialize<8> for SASetPtrDflt {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let value_bytes = self.value.serialize_fixed();
        [
            self.r#type.0 as u8,
            self.flags.0 as u8,
            self.affect.0 as u8,
            value_bytes[0],
            0,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for SASetPtrDflt {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            flags: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            affect: u8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?.into(),
            value: i8::from_bytes(bytes.get(3..4).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct SAIsoLockFlag(pub u8);
impl SAIsoLockFlag {
    pub const NO_LOCK: Self = Self(1 << 0);
    pub const NO_UNLOCK: Self = Self(1 << 1);
    pub const USE_MOD_MAP_MODS: Self = Self(1 << 2);
    pub const GROUP_ABSOLUTE: Self = Self(1 << 2);
    pub const I_S_O_DFLT_IS_GROUP: Self = Self(1 << 3);
}
impl FixedLengthSerialize<1> for SAIsoLockFlag {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SAIsoLockFlag {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for SAIsoLockFlag {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SAIsoLockFlag {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SAIsoLockFlag {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(SAIsoLockFlag);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct SAIsoLockNoAffect(pub u8);
impl SAIsoLockNoAffect {
    pub const CTRLS: Self = Self(1 << 3);
    pub const PTR: Self = Self(1 << 4);
    pub const GROUP: Self = Self(1 << 5);
    pub const MODS: Self = Self(1 << 6);
}
impl FixedLengthSerialize<1> for SAIsoLockNoAffect {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SAIsoLockNoAffect {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for SAIsoLockNoAffect {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SAIsoLockNoAffect {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SAIsoLockNoAffect {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(SAIsoLockNoAffect);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SAIsoLock {
    pub r#type: SATypeEnum,
    pub flags: SAIsoLockFlag,
    pub mask: crate::proto::xproto::ModMask,
    pub real_mods: crate::proto::xproto::ModMask,
    pub group: i8,
    pub affect: SAIsoLockNoAffect,
    pub vmods_high: VModsHigh,
    pub vmods_low: VModsLow,
}
impl FixedLengthSerialize<8> for SAIsoLock {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let group_bytes = self.group.serialize_fixed();
        [
            self.r#type.0 as u8,
            self.flags.0 as u8,
            self.mask.0 as u8,
            self.real_mods.0 as u8,
            group_bytes[0],
            self.affect.0 as u8,
            self.vmods_high.0 as u8,
            self.vmods_low.0 as u8,
        ]
    }
}
impl FixedLengthFromBytes<8> for SAIsoLock {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            flags: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            mask: u8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?.into(),
            real_mods: u8::from_bytes(bytes.get(3..4).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            group: i8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?,
            affect: u8::from_bytes(bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?)?.into(),
            vmods_high: u8::from_bytes(bytes.get(6..7).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            vmods_low: u8::from_bytes(bytes.get(7..8).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SATerminate {
    pub r#type: SATypeEnum,
}
impl FixedLengthSerialize<8> for SATerminate {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        [self.r#type.0 as u8, 0, 0, 0, 0, 0, 0, 0]
    }
}
impl FixedLengthFromBytes<8> for SATerminate {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SASwitchScreen {
    pub r#type: SATypeEnum,
    pub flags: u8,
    pub new_screen: i8,
}
impl FixedLengthSerialize<8> for SASwitchScreen {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let new_screen_bytes = self.new_screen.serialize_fixed();
        [
            self.r#type.0 as u8,
            self.flags,
            new_screen_bytes[0],
            0,
            0,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for SASwitchScreen {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            flags: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            new_screen: i8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct BoolCtrlsHigh(pub u8);
impl BoolCtrlsHigh {
    pub const ACCESS_X_FEEDBACK: Self = Self(1 << 0);
    pub const AUDIBLE_BELL: Self = Self(1 << 1);
    pub const OVERLAY1: Self = Self(1 << 2);
    pub const OVERLAY2: Self = Self(1 << 3);
    pub const IGNORE_GROUP_LOCK: Self = Self(1 << 4);
}
impl FixedLengthSerialize<1> for BoolCtrlsHigh {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for BoolCtrlsHigh {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for BoolCtrlsHigh {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for BoolCtrlsHigh {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for BoolCtrlsHigh {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(BoolCtrlsHigh);
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct BoolCtrlsLow(pub u8);
impl BoolCtrlsLow {
    pub const REPEAT_KEYS: Self = Self(1 << 0);
    pub const SLOW_KEYS: Self = Self(1 << 1);
    pub const BOUNCE_KEYS: Self = Self(1 << 2);
    pub const STICKY_KEYS: Self = Self(1 << 3);
    pub const MOUSE_KEYS: Self = Self(1 << 4);
    pub const MOUSE_KEYS_ACCEL: Self = Self(1 << 5);
    pub const ACCESS_X_KEYS: Self = Self(1 << 6);
    pub const ACCESS_X_TIMEOUT: Self = Self(1 << 7);
}
impl FixedLengthSerialize<1> for BoolCtrlsLow {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for BoolCtrlsLow {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for BoolCtrlsLow {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for BoolCtrlsLow {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for BoolCtrlsLow {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(BoolCtrlsLow);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SASetControls {
    pub r#type: SATypeEnum,
    pub bool_ctrls_high: BoolCtrlsHigh,
    pub bool_ctrls_low: BoolCtrlsLow,
}
impl FixedLengthSerialize<8> for SASetControls {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        [
            self.r#type.0 as u8,
            0,
            0,
            0,
            self.bool_ctrls_high.0 as u8,
            self.bool_ctrls_low.0 as u8,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for SASetControls {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            bool_ctrls_high: u8::from_bytes(
                bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            bool_ctrls_low: u8::from_bytes(bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
pub type SALockControls = SASetControls;
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ActionMessageFlag(pub u8);
impl ActionMessageFlag {
    pub const ON_PRESS: Self = Self(1 << 0);
    pub const ON_RELEASE: Self = Self(1 << 1);
    pub const GEN_KEY_EVENT: Self = Self(1 << 2);
}
impl FixedLengthSerialize<1> for ActionMessageFlag {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for ActionMessageFlag {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for ActionMessageFlag {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for ActionMessageFlag {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for ActionMessageFlag {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(ActionMessageFlag);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SAActionMessage {
    pub r#type: SATypeEnum,
    pub flags: ActionMessageFlag,
    pub message: [u8; 6],
}
impl FixedLengthSerialize<8> for SAActionMessage {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        [
            self.r#type.0 as u8,
            self.flags.0 as u8,
            self.message[0],
            self.message[1],
            self.message[2],
            self.message[3],
            self.message[4],
            self.message[5],
        ]
    }
}
impl FixedLengthFromBytes<8> for SAActionMessage {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            flags: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            // SAFETY: We know we can try into exact size slice
            message: unsafe {
                bytes
                    .get(2..2 + 6)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SARedirectKey {
    pub r#type: SATypeEnum,
    pub newkey: crate::proto::xproto::Keycode,
    pub mask: crate::proto::xproto::ModMask,
    pub real_modifiers: crate::proto::xproto::ModMask,
    pub vmods_mask_high: VModsHigh,
    pub vmods_mask_low: VModsLow,
    pub vmods_high: VModsHigh,
    pub vmods_low: VModsLow,
}
impl FixedLengthSerialize<8> for SARedirectKey {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let newkey_bytes = self.newkey.serialize_fixed();
        [
            self.r#type.0 as u8,
            newkey_bytes[0],
            self.mask.0 as u8,
            self.real_modifiers.0 as u8,
            self.vmods_mask_high.0 as u8,
            self.vmods_mask_low.0 as u8,
            self.vmods_high.0 as u8,
            self.vmods_low.0 as u8,
        ]
    }
}
impl FixedLengthFromBytes<8> for SARedirectKey {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            newkey: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?,
            )?,
            mask: u8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?.into(),
            real_modifiers: u8::from_bytes(bytes.get(3..4).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            vmods_mask_high: u8::from_bytes(
                bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            vmods_mask_low: u8::from_bytes(bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            vmods_high: u8::from_bytes(bytes.get(6..7).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            vmods_low: u8::from_bytes(bytes.get(7..8).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SADeviceBtn {
    pub r#type: SATypeEnum,
    pub flags: u8,
    pub count: u8,
    pub button: u8,
    pub device: u8,
}
impl FixedLengthSerialize<8> for SADeviceBtn {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        [
            self.r#type.0 as u8,
            self.flags,
            self.count,
            self.button,
            self.device,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for SADeviceBtn {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            flags: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            count: u8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?,
            button: u8::from_bytes(bytes.get(3..4).ok_or(crate::error::Error::FromBytes)?)?,
            device: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct LockDeviceFlags(pub u8);
impl LockDeviceFlags {
    pub const NO_LOCK: Self = Self(1 << 0);
    pub const NO_UNLOCK: Self = Self(1 << 1);
}
impl FixedLengthSerialize<1> for LockDeviceFlags {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for LockDeviceFlags {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for LockDeviceFlags {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for LockDeviceFlags {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for LockDeviceFlags {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
crate::implement_bit_ops!(LockDeviceFlags);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SALockDeviceBtn {
    pub r#type: SATypeEnum,
    pub flags: LockDeviceFlags,
    pub button: u8,
    pub device: u8,
}
impl FixedLengthSerialize<8> for SALockDeviceBtn {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        [
            self.r#type.0 as u8,
            self.flags.0 as u8,
            0,
            self.button,
            self.device,
            0,
            0,
            0,
        ]
    }
}
impl FixedLengthFromBytes<8> for SALockDeviceBtn {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            flags: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?.into(),
            button: u8::from_bytes(bytes.get(3..4).ok_or(crate::error::Error::FromBytes)?)?,
            device: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SAValWhatEnum(pub u8);
impl SAValWhatEnum {
    pub const IGNORE_VAL: Self = Self(0);
    pub const SET_VAL_MIN: Self = Self(1);
    pub const SET_VAL_CENTER: Self = Self(2);
    pub const SET_VAL_MAX: Self = Self(3);
    pub const SET_VAL_RELATIVE: Self = Self(4);
    pub const SET_VAL_ABSOLUTE: Self = Self(5);
}
impl FixedLengthSerialize<1> for SAValWhatEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for SAValWhatEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for SAValWhatEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for SAValWhatEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for SAValWhatEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SADeviceValuator {
    pub r#type: SATypeEnum,
    pub device: u8,
    pub val1what: SAValWhatEnum,
    pub val1index: u8,
    pub val1value: u8,
    pub val2what: SAValWhatEnum,
    pub val2index: u8,
    pub val2value: u8,
}
impl FixedLengthSerialize<8> for SADeviceValuator {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        [
            self.r#type.0 as u8,
            self.device,
            self.val1what.0 as u8,
            self.val1index,
            self.val1value,
            self.val2what.0 as u8,
            self.val2index,
            self.val2value,
        ]
    }
}
impl FixedLengthFromBytes<8> for SADeviceValuator {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            device: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            val1what: u8::from_bytes(bytes.get(2..3).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            val1index: u8::from_bytes(bytes.get(3..4).ok_or(crate::error::Error::FromBytes)?)?,
            val1value: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?,
            val2what: u8::from_bytes(bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            val2index: u8::from_bytes(bytes.get(6..7).ok_or(crate::error::Error::FromBytes)?)?,
            val2value: u8::from_bytes(bytes.get(7..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SIAction {
    pub r#type: SATypeEnum,
    pub data: [u8; 7],
}
impl FixedLengthSerialize<8> for SIAction {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        [
            self.r#type.0 as u8,
            self.data[0],
            self.data[1],
            self.data[2],
            self.data[3],
            self.data[4],
            self.data[5],
            self.data[6],
        ]
    }
}
impl FixedLengthFromBytes<8> for SIAction {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            r#type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?.into(),
            // SAFETY: We know we can try into exact size slice
            data: unsafe {
                bytes
                    .get(1..1 + 7)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SymInterpret {
    pub sym: crate::proto::xproto::Keysym,
    pub mods: crate::proto::xproto::ModMask,
    pub r#match: SymInterpretMatchEnum,
    pub virtual_mod: VModsLow,
    pub flags: u8,
    pub action: SIAction,
}
impl FixedLengthSerialize<16> for SymInterpret {
    #[inline]
    fn serialize_fixed(self) -> [u8; 16] {
        let sym_bytes = self.sym.serialize_fixed();
        let action_bytes = self.action.serialize_fixed();
        [
            sym_bytes[0],
            sym_bytes[1],
            sym_bytes[2],
            sym_bytes[3],
            self.mods.0 as u8,
            self.r#match.0 as u8,
            self.virtual_mod.0 as u8,
            self.flags,
            action_bytes[0],
            action_bytes[1],
            action_bytes[2],
            action_bytes[3],
            action_bytes[4],
            action_bytes[5],
            action_bytes[6],
            action_bytes[7],
        ]
    }
}
impl FixedLengthFromBytes<16> for SymInterpret {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            sym: crate::proto::xproto::Keysym::from_bytes(
                bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            mods: u8::from_bytes(bytes.get(4..5).ok_or(crate::error::Error::FromBytes)?)?.into(),
            r#match: u8::from_bytes(bytes.get(5..6).ok_or(crate::error::Error::FromBytes)?)?.into(),
            virtual_mod: u8::from_bytes(bytes.get(6..7).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            flags: u8::from_bytes(bytes.get(7..8).ok_or(crate::error::Error::FromBytes)?)?,
            action: SIAction::from_bytes(bytes.get(8..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Action(pub [u8; 8]);
impl FixedLengthFromBytes<8> for Action {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        // SAFETY: Checked that the bytes are available
        Ok(Self(unsafe {
            bytes
                .get(..8)
                .ok_or(crate::error::Error::FromBytes)?
                .try_into()
                .unwrap_unchecked()
        }))
    }
}
impl FixedLengthSerialize<8> for Action {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        self.0
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct UseExtensionReply {
    pub response_type: u8,
    pub supported: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
}
impl FixedLengthFromBytes<32> for UseExtensionReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            supported: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            server_major: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            server_minor: u16::from_bytes(
                bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SelectEventsDetailsBitCase0 {
    pub affect_new_keyboard: NKNDetail,
    pub new_keyboard_details: NKNDetail,
}
impl FixedLengthSerialize<4> for SelectEventsDetailsBitCase0 {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        let affect_new_keyboard_bytes = self.affect_new_keyboard.serialize_fixed();
        let new_keyboard_details_bytes = self.new_keyboard_details.serialize_fixed();
        [
            affect_new_keyboard_bytes[0],
            affect_new_keyboard_bytes[1],
            new_keyboard_details_bytes[0],
            new_keyboard_details_bytes[1],
        ]
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SelectEventsDetailsBitCase1 {
    pub affect_state: StatePart,
    pub state_details: StatePart,
}
impl FixedLengthSerialize<4> for SelectEventsDetailsBitCase1 {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        let affect_state_bytes = self.affect_state.serialize_fixed();
        let state_details_bytes = self.state_details.serialize_fixed();
        [
            affect_state_bytes[0],
            affect_state_bytes[1],
            state_details_bytes[0],
            state_details_bytes[1],
        ]
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SelectEventsDetailsBitCase2 {
    pub affect_ctrls: Control,
    pub ctrl_details: Control,
}
impl FixedLengthSerialize<8> for SelectEventsDetailsBitCase2 {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let affect_ctrls_bytes = self.affect_ctrls.serialize_fixed();
        let ctrl_details_bytes = self.ctrl_details.serialize_fixed();
        [
            affect_ctrls_bytes[0],
            affect_ctrls_bytes[1],
            affect_ctrls_bytes[2],
            affect_ctrls_bytes[3],
            ctrl_details_bytes[0],
            ctrl_details_bytes[1],
            ctrl_details_bytes[2],
            ctrl_details_bytes[3],
        ]
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SelectEventsDetailsBitCase3 {
    pub affect_indicator_state: u32,
    pub indicator_state_details: u32,
}
impl FixedLengthSerialize<8> for SelectEventsDetailsBitCase3 {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let affect_indicator_state_bytes = self.affect_indicator_state.serialize_fixed();
        let indicator_state_details_bytes = self.indicator_state_details.serialize_fixed();
        [
            affect_indicator_state_bytes[0],
            affect_indicator_state_bytes[1],
            affect_indicator_state_bytes[2],
            affect_indicator_state_bytes[3],
            indicator_state_details_bytes[0],
            indicator_state_details_bytes[1],
            indicator_state_details_bytes[2],
            indicator_state_details_bytes[3],
        ]
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SelectEventsDetailsBitCase4 {
    pub affect_indicator_map: u32,
    pub indicator_map_details: u32,
}
impl FixedLengthSerialize<8> for SelectEventsDetailsBitCase4 {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let affect_indicator_map_bytes = self.affect_indicator_map.serialize_fixed();
        let indicator_map_details_bytes = self.indicator_map_details.serialize_fixed();
        [
            affect_indicator_map_bytes[0],
            affect_indicator_map_bytes[1],
            affect_indicator_map_bytes[2],
            affect_indicator_map_bytes[3],
            indicator_map_details_bytes[0],
            indicator_map_details_bytes[1],
            indicator_map_details_bytes[2],
            indicator_map_details_bytes[3],
        ]
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SelectEventsDetailsBitCase5 {
    pub affect_names: NameDetail,
    pub names_details: NameDetail,
}
impl FixedLengthSerialize<4> for SelectEventsDetailsBitCase5 {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        let affect_names_bytes = self.affect_names.serialize_fixed();
        let names_details_bytes = self.names_details.serialize_fixed();
        [
            affect_names_bytes[0],
            affect_names_bytes[1],
            names_details_bytes[0],
            names_details_bytes[1],
        ]
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SelectEventsDetailsBitCase6 {
    pub affect_compat: CMDetail,
    pub compat_details: CMDetail,
}
impl FixedLengthSerialize<2> for SelectEventsDetailsBitCase6 {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        [self.affect_compat.0 as u8, self.compat_details.0 as u8]
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SelectEventsDetailsBitCase7 {
    pub affect_bell: u8,
    pub bell_details: u8,
}
impl FixedLengthSerialize<2> for SelectEventsDetailsBitCase7 {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        [self.affect_bell, self.bell_details]
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SelectEventsDetailsBitCase8 {
    pub affect_msg_details: u8,
    pub msg_details: u8,
}
impl FixedLengthSerialize<2> for SelectEventsDetailsBitCase8 {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        [self.affect_msg_details, self.msg_details]
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SelectEventsDetailsBitCase9 {
    pub affect_access_x: AXNDetail,
    pub access_x_details: AXNDetail,
}
impl FixedLengthSerialize<4> for SelectEventsDetailsBitCase9 {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        let affect_access_x_bytes = self.affect_access_x.serialize_fixed();
        let access_x_details_bytes = self.access_x_details.serialize_fixed();
        [
            affect_access_x_bytes[0],
            affect_access_x_bytes[1],
            access_x_details_bytes[0],
            access_x_details_bytes[1],
        ]
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SelectEventsDetailsBitCase10 {
    pub affect_ext_dev: XIFeature,
    pub extdev_details: XIFeature,
}
impl FixedLengthSerialize<4> for SelectEventsDetailsBitCase10 {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        let affect_ext_dev_bytes = self.affect_ext_dev.serialize_fixed();
        let extdev_details_bytes = self.extdev_details.serialize_fixed();
        [
            affect_ext_dev_bytes[0],
            affect_ext_dev_bytes[1],
            extdev_details_bytes[0],
            extdev_details_bytes[1],
        ]
    }
}
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct SelectEventsDetails {
    pub select_events_details_bit_case0: Option<SelectEventsDetailsBitCase0>,
    pub select_events_details_bit_case1: Option<SelectEventsDetailsBitCase1>,
    pub select_events_details_bit_case2: Option<SelectEventsDetailsBitCase2>,
    pub select_events_details_bit_case3: Option<SelectEventsDetailsBitCase3>,
    pub select_events_details_bit_case4: Option<SelectEventsDetailsBitCase4>,
    pub select_events_details_bit_case5: Option<SelectEventsDetailsBitCase5>,
    pub select_events_details_bit_case6: Option<SelectEventsDetailsBitCase6>,
    pub select_events_details_bit_case7: Option<SelectEventsDetailsBitCase7>,
    pub select_events_details_bit_case8: Option<SelectEventsDetailsBitCase8>,
    pub select_events_details_bit_case9: Option<SelectEventsDetailsBitCase9>,
    pub select_events_details_bit_case10: Option<SelectEventsDetailsBitCase10>,
}
impl SelectEventsDetails {
    #[inline]
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let mut offset = 0;
        let buf_ptr = buf;
        if let Some(select_events_details_bit_case0) = self.select_events_details_bit_case0 {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&select_events_details_bit_case0.serialize_fixed());
            offset += 4;
        }
        if let Some(select_events_details_bit_case1) = self.select_events_details_bit_case1 {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&select_events_details_bit_case1.serialize_fixed());
            offset += 4;
        }
        if let Some(select_events_details_bit_case2) = self.select_events_details_bit_case2 {
            buf_ptr
                .get_mut(offset..offset + 8)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&select_events_details_bit_case2.serialize_fixed());
            offset += 8;
        }
        if let Some(select_events_details_bit_case3) = self.select_events_details_bit_case3 {
            buf_ptr
                .get_mut(offset..offset + 8)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&select_events_details_bit_case3.serialize_fixed());
            offset += 8;
        }
        if let Some(select_events_details_bit_case4) = self.select_events_details_bit_case4 {
            buf_ptr
                .get_mut(offset..offset + 8)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&select_events_details_bit_case4.serialize_fixed());
            offset += 8;
        }
        if let Some(select_events_details_bit_case5) = self.select_events_details_bit_case5 {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&select_events_details_bit_case5.serialize_fixed());
            offset += 4;
        }
        if let Some(select_events_details_bit_case6) = self.select_events_details_bit_case6 {
            buf_ptr
                .get_mut(offset..offset + 2)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&select_events_details_bit_case6.serialize_fixed());
            offset += 2;
        }
        if let Some(select_events_details_bit_case7) = self.select_events_details_bit_case7 {
            buf_ptr
                .get_mut(offset..offset + 2)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&select_events_details_bit_case7.serialize_fixed());
            offset += 2;
        }
        if let Some(select_events_details_bit_case8) = self.select_events_details_bit_case8 {
            buf_ptr
                .get_mut(offset..offset + 2)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&select_events_details_bit_case8.serialize_fixed());
            offset += 2;
        }
        if let Some(select_events_details_bit_case9) = self.select_events_details_bit_case9 {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&select_events_details_bit_case9.serialize_fixed());
            offset += 4;
        }
        if let Some(select_events_details_bit_case10) = self.select_events_details_bit_case10 {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&select_events_details_bit_case10.serialize_fixed());
            offset += 4;
        }
        Ok(offset)
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> EventType {
        let mut mask = EventType::default();
        if self.select_events_details_bit_case0.is_some() {
            mask |= EventType::NEW_KEYBOARD_NOTIFY;
        }
        if self.select_events_details_bit_case1.is_some() {
            mask |= EventType::STATE_NOTIFY;
        }
        if self.select_events_details_bit_case2.is_some() {
            mask |= EventType::CONTROLS_NOTIFY;
        }
        if self.select_events_details_bit_case3.is_some() {
            mask |= EventType::INDICATOR_STATE_NOTIFY;
        }
        if self.select_events_details_bit_case4.is_some() {
            mask |= EventType::INDICATOR_MAP_NOTIFY;
        }
        if self.select_events_details_bit_case5.is_some() {
            mask |= EventType::NAMES_NOTIFY;
        }
        if self.select_events_details_bit_case6.is_some() {
            mask |= EventType::COMPAT_MAP_NOTIFY;
        }
        if self.select_events_details_bit_case7.is_some() {
            mask |= EventType::BELL_NOTIFY;
        }
        if self.select_events_details_bit_case8.is_some() {
            mask |= EventType::ACTION_MESSAGE;
        }
        if self.select_events_details_bit_case9.is_some() {
            mask |= EventType::ACCESS_X_NOTIFY;
        }
        if self.select_events_details_bit_case10.is_some() {
            mask |= EventType::EXTENSION_DEVICE_NOTIFY;
        }
        mask
    }

    #[inline]
    pub fn select_events_details_bit_case0(
        mut self,
        select_events_details_bit_case0: SelectEventsDetailsBitCase0,
    ) -> Self {
        self.select_events_details_bit_case0 = Some(select_events_details_bit_case0);
        self
    }

    #[inline]
    pub fn select_events_details_bit_case1(
        mut self,
        select_events_details_bit_case1: SelectEventsDetailsBitCase1,
    ) -> Self {
        self.select_events_details_bit_case1 = Some(select_events_details_bit_case1);
        self
    }

    #[inline]
    pub fn select_events_details_bit_case2(
        mut self,
        select_events_details_bit_case2: SelectEventsDetailsBitCase2,
    ) -> Self {
        self.select_events_details_bit_case2 = Some(select_events_details_bit_case2);
        self
    }

    #[inline]
    pub fn select_events_details_bit_case3(
        mut self,
        select_events_details_bit_case3: SelectEventsDetailsBitCase3,
    ) -> Self {
        self.select_events_details_bit_case3 = Some(select_events_details_bit_case3);
        self
    }

    #[inline]
    pub fn select_events_details_bit_case4(
        mut self,
        select_events_details_bit_case4: SelectEventsDetailsBitCase4,
    ) -> Self {
        self.select_events_details_bit_case4 = Some(select_events_details_bit_case4);
        self
    }

    #[inline]
    pub fn select_events_details_bit_case5(
        mut self,
        select_events_details_bit_case5: SelectEventsDetailsBitCase5,
    ) -> Self {
        self.select_events_details_bit_case5 = Some(select_events_details_bit_case5);
        self
    }

    #[inline]
    pub fn select_events_details_bit_case6(
        mut self,
        select_events_details_bit_case6: SelectEventsDetailsBitCase6,
    ) -> Self {
        self.select_events_details_bit_case6 = Some(select_events_details_bit_case6);
        self
    }

    #[inline]
    pub fn select_events_details_bit_case7(
        mut self,
        select_events_details_bit_case7: SelectEventsDetailsBitCase7,
    ) -> Self {
        self.select_events_details_bit_case7 = Some(select_events_details_bit_case7);
        self
    }

    #[inline]
    pub fn select_events_details_bit_case8(
        mut self,
        select_events_details_bit_case8: SelectEventsDetailsBitCase8,
    ) -> Self {
        self.select_events_details_bit_case8 = Some(select_events_details_bit_case8);
        self
    }

    #[inline]
    pub fn select_events_details_bit_case9(
        mut self,
        select_events_details_bit_case9: SelectEventsDetailsBitCase9,
    ) -> Self {
        self.select_events_details_bit_case9 = Some(select_events_details_bit_case9);
        self
    }

    #[inline]
    pub fn select_events_details_bit_case10(
        mut self,
        select_events_details_bit_case10: SelectEventsDetailsBitCase10,
    ) -> Self {
        self.select_events_details_bit_case10 = Some(select_events_details_bit_case10);
        self
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetStateReply {
    pub response_type: u8,
    pub device_i_d: u8,
    pub sequence: u16,
    pub length: u32,
    pub mods: crate::proto::xproto::ModMask,
    pub base_mods: crate::proto::xproto::ModMask,
    pub latched_mods: crate::proto::xproto::ModMask,
    pub locked_mods: crate::proto::xproto::ModMask,
    pub group: GroupEnum,
    pub locked_group: GroupEnum,
    pub base_group: i16,
    pub latched_group: i16,
    pub compat_state: crate::proto::xproto::ModMask,
    pub grab_mods: crate::proto::xproto::ModMask,
    pub compat_grab_mods: crate::proto::xproto::ModMask,
    pub lookup_mods: crate::proto::xproto::ModMask,
    pub compat_lookup_mods: crate::proto::xproto::ModMask,
    pub ptr_btn_state: crate::proto::xproto::KeyButMask,
}
impl FixedLengthFromBytes<32> for GetStateReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            device_i_d: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            mods: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?.into(),
            base_mods: u8::from_bytes(bytes.get(9..10).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            latched_mods: u8::from_bytes(bytes.get(10..11).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            locked_mods: u8::from_bytes(bytes.get(11..12).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            group: u8::from_bytes(bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?)?.into(),
            locked_group: u8::from_bytes(bytes.get(13..14).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            base_group: i16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
            latched_group: i16::from_bytes(
                bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?,
            )?,
            compat_state: u8::from_bytes(bytes.get(18..19).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            grab_mods: u8::from_bytes(bytes.get(19..20).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            compat_grab_mods: u8::from_bytes(
                bytes.get(20..21).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            lookup_mods: u8::from_bytes(bytes.get(21..22).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            compat_lookup_mods: u8::from_bytes(
                bytes.get(22..23).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            ptr_btn_state: u16::from_bytes(
                bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetControlsReply {
    pub response_type: u8,
    pub device_i_d: u8,
    pub sequence: u16,
    pub length: u32,
    pub mouse_keys_dflt_btn: u8,
    pub num_groups: u8,
    pub groups_wrap: u8,
    pub internal_mods_mask: crate::proto::xproto::ModMask,
    pub ignore_lock_mods_mask: crate::proto::xproto::ModMask,
    pub internal_mods_real_mods: crate::proto::xproto::ModMask,
    pub ignore_lock_mods_real_mods: crate::proto::xproto::ModMask,
    pub internal_mods_vmods: VMod,
    pub ignore_lock_mods_vmods: VMod,
    pub repeat_delay: u16,
    pub repeat_interval: u16,
    pub slow_keys_delay: u16,
    pub debounce_delay: u16,
    pub mouse_keys_delay: u16,
    pub mouse_keys_interval: u16,
    pub mouse_keys_time_to_max: u16,
    pub mouse_keys_max_speed: u16,
    pub mouse_keys_curve: i16,
    pub access_x_option: AXOption,
    pub access_x_timeout: u16,
    pub access_x_timeout_options_mask: AXOption,
    pub access_x_timeout_options_values: AXOption,
    pub access_x_timeout_mask: BoolCtrl,
    pub access_x_timeout_values: BoolCtrl,
    pub enabled_controls: BoolCtrl,
    pub per_key_repeat: [u8; 32],
}
impl FixedLengthFromBytes<92> for GetControlsReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            device_i_d: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            mouse_keys_dflt_btn: u8::from_bytes(
                bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?,
            )?,
            num_groups: u8::from_bytes(bytes.get(9..10).ok_or(crate::error::Error::FromBytes)?)?,
            groups_wrap: u8::from_bytes(bytes.get(10..11).ok_or(crate::error::Error::FromBytes)?)?,
            internal_mods_mask: u8::from_bytes(
                bytes.get(11..12).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            ignore_lock_mods_mask: u8::from_bytes(
                bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            internal_mods_real_mods: u8::from_bytes(
                bytes.get(13..14).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            ignore_lock_mods_real_mods: u8::from_bytes(
                bytes.get(14..15).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            internal_mods_vmods: u16::from_bytes(
                bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            ignore_lock_mods_vmods: u16::from_bytes(
                bytes.get(18..20).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            repeat_delay: u16::from_bytes(
                bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?,
            )?,
            repeat_interval: u16::from_bytes(
                bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?,
            )?,
            slow_keys_delay: u16::from_bytes(
                bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?,
            )?,
            debounce_delay: u16::from_bytes(
                bytes.get(26..28).ok_or(crate::error::Error::FromBytes)?,
            )?,
            mouse_keys_delay: u16::from_bytes(
                bytes.get(28..30).ok_or(crate::error::Error::FromBytes)?,
            )?,
            mouse_keys_interval: u16::from_bytes(
                bytes.get(30..32).ok_or(crate::error::Error::FromBytes)?,
            )?,
            mouse_keys_time_to_max: u16::from_bytes(
                bytes.get(32..34).ok_or(crate::error::Error::FromBytes)?,
            )?,
            mouse_keys_max_speed: u16::from_bytes(
                bytes.get(34..36).ok_or(crate::error::Error::FromBytes)?,
            )?,
            mouse_keys_curve: i16::from_bytes(
                bytes.get(36..38).ok_or(crate::error::Error::FromBytes)?,
            )?,
            access_x_option: u16::from_bytes(
                bytes.get(38..40).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            access_x_timeout: u16::from_bytes(
                bytes.get(40..42).ok_or(crate::error::Error::FromBytes)?,
            )?,
            access_x_timeout_options_mask: u16::from_bytes(
                bytes.get(42..44).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            access_x_timeout_options_values: u16::from_bytes(
                bytes.get(44..46).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            access_x_timeout_mask: u32::from_bytes(
                bytes.get(48..52).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            access_x_timeout_values: u32::from_bytes(
                bytes.get(52..56).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            enabled_controls: u32::from_bytes(
                bytes.get(56..60).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            // SAFETY: We know we can try into exact size slice
            per_key_repeat: unsafe {
                bytes
                    .get(60..60 + 32)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetMapReply {
    pub response_type: u8,
    pub device_i_d: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_key_code: crate::proto::xproto::Keycode,
    pub max_key_code: crate::proto::xproto::Keycode,
    pub present: MapPart,
    pub first_type: u8,
    pub n_types: u8,
    pub total_types: u8,
    pub first_key_sym: crate::proto::xproto::Keycode,
    pub total_syms: u16,
    pub n_key_syms: u8,
    pub first_key_action: crate::proto::xproto::Keycode,
    pub total_actions: u16,
    pub n_key_actions: u8,
    pub first_key_behavior: crate::proto::xproto::Keycode,
    pub n_key_behaviors: u8,
    pub total_key_behaviors: u8,
    pub first_key_explicit: crate::proto::xproto::Keycode,
    pub n_key_explicit: u8,
    pub total_key_explicit: u8,
    pub first_mod_map_key: crate::proto::xproto::Keycode,
    pub n_mod_map_keys: u8,
    pub total_mod_map_keys: u8,
    pub first_v_mod_map_key: crate::proto::xproto::Keycode,
    pub n_v_mod_map_keys: u8,
    pub total_v_mod_map_keys: u8,
    pub virtual_mods: VMod,
    pub get_map_map: GetMapMap,
}
impl VariableLengthFromBytes for GetMapReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 2 bytes
        // Padding 1 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let device_i_d = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let min_key_code = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(10..11).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let max_key_code = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(11..12).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let present = u16::from_bytes(ptr.get(12..14).ok_or(crate::error::Error::FromBytes)?)?;
        let first_type = u8::from_bytes(ptr.get(14..15).ok_or(crate::error::Error::FromBytes)?)?;
        let n_types = u8::from_bytes(ptr.get(15..16).ok_or(crate::error::Error::FromBytes)?)?;
        let total_types = u8::from_bytes(ptr.get(16..17).ok_or(crate::error::Error::FromBytes)?)?;
        let first_key_sym = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(17..18).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let total_syms = u16::from_bytes(ptr.get(18..20).ok_or(crate::error::Error::FromBytes)?)?;
        let n_key_syms = u8::from_bytes(ptr.get(20..21).ok_or(crate::error::Error::FromBytes)?)?;
        let first_key_action = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(21..22).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let total_actions =
            u16::from_bytes(ptr.get(22..24).ok_or(crate::error::Error::FromBytes)?)?;
        let n_key_actions = u8::from_bytes(ptr.get(24..25).ok_or(crate::error::Error::FromBytes)?)?;
        let first_key_behavior = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(25..26).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let n_key_behaviors =
            u8::from_bytes(ptr.get(26..27).ok_or(crate::error::Error::FromBytes)?)?;
        let total_key_behaviors =
            u8::from_bytes(ptr.get(27..28).ok_or(crate::error::Error::FromBytes)?)?;
        let first_key_explicit = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(28..29).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let n_key_explicit =
            u8::from_bytes(ptr.get(29..30).ok_or(crate::error::Error::FromBytes)?)?;
        let total_key_explicit =
            u8::from_bytes(ptr.get(30..31).ok_or(crate::error::Error::FromBytes)?)?;
        let first_mod_map_key = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(31..32).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let n_mod_map_keys =
            u8::from_bytes(ptr.get(32..33).ok_or(crate::error::Error::FromBytes)?)?;
        let total_mod_map_keys =
            u8::from_bytes(ptr.get(33..34).ok_or(crate::error::Error::FromBytes)?)?;
        let first_v_mod_map_key = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(34..35).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let n_v_mod_map_keys =
            u8::from_bytes(ptr.get(35..36).ok_or(crate::error::Error::FromBytes)?)?;
        let total_v_mod_map_keys =
            u8::from_bytes(ptr.get(36..37).ok_or(crate::error::Error::FromBytes)?)?;
        let virtual_mods = u16::from_bytes(ptr.get(38..40).ok_or(crate::error::Error::FromBytes)?)?;
        let (get_map_map, offset) = GetMapMap::from_bytes(
            ptr.get(40..).ok_or(crate::error::Error::FromBytes)?,
            n_key_actions,
            n_key_syms,
            n_types,
            present,
            total_actions,
            total_key_behaviors,
            total_key_explicit,
            total_mod_map_keys,
            total_v_mod_map_keys,
            virtual_mods,
        )?;
        Ok((
            Self {
                response_type,
                device_i_d,
                sequence,
                length,
                min_key_code,
                max_key_code,
                present: present.into(),
                first_type,
                n_types,
                total_types,
                first_key_sym,
                total_syms,
                n_key_syms,
                first_key_action,
                total_actions,
                n_key_actions,
                first_key_behavior,
                n_key_behaviors,
                total_key_behaviors,
                first_key_explicit,
                n_key_explicit,
                total_key_explicit,
                first_mod_map_key,
                n_mod_map_keys,
                total_mod_map_keys,
                first_v_mod_map_key,
                n_v_mod_map_keys,
                total_v_mod_map_keys,
                virtual_mods: virtual_mods.into(),
                get_map_map,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetMapMapBitCase0 {
    pub acts_rtrn_count: alloc::vec::Vec<u8>,
    pub acts_rtrn_acts: alloc::vec::Vec<Action>,
}
impl GetMapMapBitCase0 {
    fn from_bytes(
        bytes: &[u8],
        n_key_actions: u8,
        total_actions: u16,
    ) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let length_expr = n_key_actions as usize;
        let acts_rtrn_count: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(0..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let length_expr = total_actions as usize;
        let acts_rtrn_acts: alloc::vec::Vec<Action> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Action,
            length_expr,
            8
        );
        offset += length_expr * 8;
        Ok((
            Self {
                acts_rtrn_count,
                acts_rtrn_acts,
            },
            offset,
        ))
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetMapMap {
    pub types_rtrn: Option<alloc::vec::Vec<KeyType>>,
    pub syms_rtrn: Option<alloc::vec::Vec<KeySymMap>>,
    pub get_map_map_bit_case0: Option<GetMapMapBitCase0>,
    pub behaviors_rtrn: Option<alloc::vec::Vec<SetBehavior>>,
    pub vmods_rtrn: Option<alloc::vec::Vec<crate::proto::xproto::ModMask>>,
    pub explicit_rtrn: Option<alloc::vec::Vec<SetExplicit>>,
    pub modmap_rtrn: Option<alloc::vec::Vec<KeyModMap>>,
    pub vmodmap_rtrn: Option<alloc::vec::Vec<KeyVModMap>>,
}
impl GetMapMap {
    #[inline]
    pub fn from_bytes(
        buf: &[u8],
        n_key_actions: u8,
        n_key_syms: u8,
        n_types: u8,
        present: u16,
        total_actions: u16,
        total_key_behaviors: u8,
        total_key_explicit: u8,
        total_mod_map_keys: u8,
        total_v_mod_map_keys: u8,
        virtual_mods: u16,
    ) -> crate::error::Result<(Self, usize)> {
        let mask = present;
        let mut offset = 0;
        let mut slf = Self::default();
        let buf_ptr = buf;
        if mask & MapPart::KEY_TYPES.0 != 0 {
            let length = n_types as usize;
            let types_rtrn = crate::vec_from_bytes_var!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                KeyType,
                offset,
                length
            );
            slf.types_rtrn = Some(types_rtrn);
        }
        if mask & MapPart::KEY_SYMS.0 != 0 {
            let length = n_key_syms as usize;
            let syms_rtrn = crate::vec_from_bytes_var!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                KeySymMap,
                offset,
                length
            );
            slf.syms_rtrn = Some(syms_rtrn);
        }
        if mask & MapPart::KEY_ACTIONS.0 != 0 {
            let (get_map_map_bit_case0, new_offset) = GetMapMapBitCase0::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                n_key_actions,
                total_actions,
            )?;
            offset += new_offset;
            slf.get_map_map_bit_case0 = Some(get_map_map_bit_case0);
        }
        if mask & MapPart::KEY_BEHAVIORS.0 != 0 {
            let length = total_key_behaviors as usize;
            slf.behaviors_rtrn = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                SetBehavior,
                length,
                4
            ));
            offset += length;
        }
        if mask & MapPart::VIRTUAL_MODS.0 != 0 {
            let length = virtual_mods.count_ones() as usize;
            slf.vmods_rtrn = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                crate::proto::xproto::ModMask,
                length,
                1
            ));
            offset += length;
        }
        if mask & MapPart::EXPLICIT_COMPONENTS.0 != 0 {
            let length = total_key_explicit as usize;
            slf.explicit_rtrn = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                SetExplicit,
                length,
                2
            ));
            offset += length;
        }
        if mask & MapPart::MODIFIER_MAP.0 != 0 {
            let length = total_mod_map_keys as usize;
            slf.modmap_rtrn = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                KeyModMap,
                length,
                2
            ));
            offset += length;
        }
        if mask & MapPart::VIRTUAL_MOD_MAP.0 != 0 {
            let length = total_v_mod_map_keys as usize;
            slf.vmodmap_rtrn = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                KeyVModMap,
                length,
                4
            ));
            offset += length;
        }
        Ok((slf, offset))
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> MapPart {
        let mut mask = MapPart::default();
        if self.get_map_map_bit_case0.is_some() {
            mask |= MapPart::KEY_ACTIONS;
        }
        mask
    }

    #[inline]
    pub fn types_rtrn(mut self, types_rtrn: alloc::vec::Vec<KeyType>) -> Self {
        self.types_rtrn = Some(types_rtrn);
        self
    }

    #[inline]
    pub fn syms_rtrn(mut self, syms_rtrn: alloc::vec::Vec<KeySymMap>) -> Self {
        self.syms_rtrn = Some(syms_rtrn);
        self
    }

    #[inline]
    pub fn get_map_map_bit_case0(mut self, get_map_map_bit_case0: GetMapMapBitCase0) -> Self {
        self.get_map_map_bit_case0 = Some(get_map_map_bit_case0);
        self
    }

    #[inline]
    pub fn behaviors_rtrn(mut self, behaviors_rtrn: alloc::vec::Vec<SetBehavior>) -> Self {
        self.behaviors_rtrn = Some(behaviors_rtrn);
        self
    }

    #[inline]
    pub fn vmods_rtrn(
        mut self,
        vmods_rtrn: alloc::vec::Vec<crate::proto::xproto::ModMask>,
    ) -> Self {
        self.vmods_rtrn = Some(vmods_rtrn);
        self
    }

    #[inline]
    pub fn explicit_rtrn(mut self, explicit_rtrn: alloc::vec::Vec<SetExplicit>) -> Self {
        self.explicit_rtrn = Some(explicit_rtrn);
        self
    }

    #[inline]
    pub fn modmap_rtrn(mut self, modmap_rtrn: alloc::vec::Vec<KeyModMap>) -> Self {
        self.modmap_rtrn = Some(modmap_rtrn);
        self
    }

    #[inline]
    pub fn vmodmap_rtrn(mut self, vmodmap_rtrn: alloc::vec::Vec<KeyVModMap>) -> Self {
        self.vmodmap_rtrn = Some(vmodmap_rtrn);
        self
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetMapValuesBitCase0 {
    pub actions_count: alloc::vec::Vec<u8>,
    pub actions: alloc::vec::Vec<Action>,
}
impl VariableLengthSerialize for SetMapValuesBitCase0 {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let list_len = self.actions_count.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(0..).ok_or(crate::error::Error::Serialize)?,
            &self.actions_count,
        )?;
        let mut offset = list_len;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let list_len = self.actions.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            self.actions,
        )?;
        offset += list_len;
        Ok(offset)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetMapValues {
    pub types: Option<alloc::vec::Vec<SetKeyType>>,
    pub syms: Option<alloc::vec::Vec<KeySymMap>>,
    pub set_map_values_bit_case0: Option<SetMapValuesBitCase0>,
    pub behaviors: Option<alloc::vec::Vec<SetBehavior>>,
    pub vmods: Option<alloc::vec::Vec<u8>>,
    pub explicit: Option<alloc::vec::Vec<SetExplicit>>,
    pub modmap: Option<alloc::vec::Vec<KeyModMap>>,
    pub vmodmap: Option<alloc::vec::Vec<KeyVModMap>>,
}
impl SetMapValues {
    #[inline]
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let mut offset = 0;
        let buf_ptr = buf;
        if let Some(types) = self.types {
            offset += crate::util::var_vec_serialize_into(
                buf_ptr
                    .get_mut(offset..)
                    .ok_or(crate::error::Error::Serialize)?,
                types,
            )?;
        }
        if let Some(syms) = self.syms {
            offset += crate::util::var_vec_serialize_into(
                buf_ptr
                    .get_mut(offset..)
                    .ok_or(crate::error::Error::Serialize)?,
                syms,
            )?;
        }
        if let Some(set_map_values_bit_case0) = self.set_map_values_bit_case0 {
            offset += set_map_values_bit_case0.serialize_into(
                buf_ptr
                    .get_mut(offset..)
                    .ok_or(crate::error::Error::Serialize)?,
            )?;
        }
        if let Some(behaviors) = self.behaviors {
            let out_len = behaviors.len() * 4;
            crate::util::fixed_vec_serialize_into(
                buf_ptr
                    .get_mut(offset..)
                    .ok_or(crate::error::Error::Serialize)?,
                behaviors,
            )?;
            offset += out_len;
        }
        if let Some(vmods) = self.vmods {
            let out_len = vmods.len() * 1;
            crate::util::u8_vec_serialize_into(
                buf_ptr
                    .get_mut(offset..)
                    .ok_or(crate::error::Error::Serialize)?,
                &vmods,
            )?;
            offset += out_len;
        }
        if let Some(explicit) = self.explicit {
            let out_len = explicit.len() * 2;
            crate::util::fixed_vec_serialize_into(
                buf_ptr
                    .get_mut(offset..)
                    .ok_or(crate::error::Error::Serialize)?,
                explicit,
            )?;
            offset += out_len;
        }
        if let Some(modmap) = self.modmap {
            let out_len = modmap.len() * 2;
            crate::util::fixed_vec_serialize_into(
                buf_ptr
                    .get_mut(offset..)
                    .ok_or(crate::error::Error::Serialize)?,
                modmap,
            )?;
            offset += out_len;
        }
        if let Some(vmodmap) = self.vmodmap {
            let out_len = vmodmap.len() * 4;
            crate::util::fixed_vec_serialize_into(
                buf_ptr
                    .get_mut(offset..)
                    .ok_or(crate::error::Error::Serialize)?,
                vmodmap,
            )?;
            offset += out_len;
        }
        Ok(offset)
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> MapPart {
        let mut mask = MapPart::default();
        if self.set_map_values_bit_case0.is_some() {
            mask |= MapPart::KEY_ACTIONS;
        }
        mask
    }

    #[inline]
    pub fn types(mut self, types: alloc::vec::Vec<SetKeyType>) -> Self {
        self.types = Some(types);
        self
    }

    #[inline]
    pub fn syms(mut self, syms: alloc::vec::Vec<KeySymMap>) -> Self {
        self.syms = Some(syms);
        self
    }

    #[inline]
    pub fn set_map_values_bit_case0(
        mut self,
        set_map_values_bit_case0: SetMapValuesBitCase0,
    ) -> Self {
        self.set_map_values_bit_case0 = Some(set_map_values_bit_case0);
        self
    }

    #[inline]
    pub fn behaviors(mut self, behaviors: alloc::vec::Vec<SetBehavior>) -> Self {
        self.behaviors = Some(behaviors);
        self
    }

    #[inline]
    pub fn vmods(mut self, vmods: alloc::vec::Vec<u8>) -> Self {
        self.vmods = Some(vmods);
        self
    }

    #[inline]
    pub fn explicit(mut self, explicit: alloc::vec::Vec<SetExplicit>) -> Self {
        self.explicit = Some(explicit);
        self
    }

    #[inline]
    pub fn modmap(mut self, modmap: alloc::vec::Vec<KeyModMap>) -> Self {
        self.modmap = Some(modmap);
        self
    }

    #[inline]
    pub fn vmodmap(mut self, vmodmap: alloc::vec::Vec<KeyVModMap>) -> Self {
        self.vmodmap = Some(vmodmap);
        self
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetCompatMapReply {
    pub response_type: u8,
    pub device_i_d: u8,
    pub sequence: u16,
    pub length: u32,
    pub first_s_i_rtrn: u16,
    pub n_total_s_i: u16,
    pub si_rtrn: alloc::vec::Vec<SymInterpret>,
    pub group_rtrn: alloc::vec::Vec<ModDef>,
}
impl VariableLengthFromBytes for GetCompatMapReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 16 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let device_i_d = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let groups_rtrn = u8::from_bytes(ptr.get(8..9).ok_or(crate::error::Error::FromBytes)?)?;
        let first_s_i_rtrn =
            u16::from_bytes(ptr.get(10..12).ok_or(crate::error::Error::FromBytes)?)?;
        let n_s_i_rtrn = u16::from_bytes(ptr.get(12..14).ok_or(crate::error::Error::FromBytes)?)?;
        let n_total_s_i = u16::from_bytes(ptr.get(14..16).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n_s_i_rtrn as usize;
        let si_rtrn: alloc::vec::Vec<SymInterpret> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            SymInterpret,
            length_expr,
            16
        );
        let mut offset = length_expr * 16 + 32;
        let length_expr = groups_rtrn.count_ones() as usize;
        let group_rtrn: alloc::vec::Vec<ModDef> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            ModDef,
            length_expr,
            4
        );
        offset += length_expr * 4;
        Ok((
            Self {
                response_type,
                device_i_d,
                sequence,
                length,
                first_s_i_rtrn,
                n_total_s_i,
                si_rtrn,
                group_rtrn,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetIndicatorStateReply {
    pub response_type: u8,
    pub device_i_d: u8,
    pub sequence: u16,
    pub length: u32,
    pub state: u32,
}
impl FixedLengthFromBytes<32> for GetIndicatorStateReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            device_i_d: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            state: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetIndicatorMapReply {
    pub response_type: u8,
    pub device_i_d: u8,
    pub sequence: u16,
    pub length: u32,
    pub real_indicators: u32,
    pub n_indicators: u8,
    pub maps: alloc::vec::Vec<IndicatorMap>,
}
impl VariableLengthFromBytes for GetIndicatorMapReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 15 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let device_i_d = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let which = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let real_indicators =
            u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let n_indicators = u8::from_bytes(ptr.get(16..17).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = which.count_ones() as usize;
        let maps: alloc::vec::Vec<IndicatorMap> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            IndicatorMap,
            length_expr,
            12
        );
        let offset = length_expr * 12 + 32;
        Ok((
            Self {
                response_type,
                device_i_d,
                sequence,
                length,
                real_indicators,
                n_indicators,
                maps,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetNamedIndicatorReply {
    pub response_type: u8,
    pub device_i_d: u8,
    pub sequence: u16,
    pub length: u32,
    pub indicator: u32,
    pub found: u8,
    pub on: u8,
    pub real_indicator: u8,
    pub ndx: u8,
    pub map_flags: IMFlag,
    pub map_whichgroups: IMGroupsWhich,
    pub map_groups: SetOfGroups,
    pub map_whichmods: IMModsWhich,
    pub map_mods: crate::proto::xproto::ModMask,
    pub map_realmods: crate::proto::xproto::ModMask,
    pub map_vmod: VMod,
    pub map_ctrls: BoolCtrl,
    pub supported: u8,
}
impl FixedLengthFromBytes<32> for GetNamedIndicatorReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            device_i_d: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            indicator: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            found: u8::from_bytes(bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?)?,
            on: u8::from_bytes(bytes.get(13..14).ok_or(crate::error::Error::FromBytes)?)?,
            real_indicator: u8::from_bytes(
                bytes.get(14..15).ok_or(crate::error::Error::FromBytes)?,
            )?,
            ndx: u8::from_bytes(bytes.get(15..16).ok_or(crate::error::Error::FromBytes)?)?,
            map_flags: u8::from_bytes(bytes.get(16..17).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            map_whichgroups: u8::from_bytes(
                bytes.get(17..18).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            map_groups: u8::from_bytes(bytes.get(18..19).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            map_whichmods: u8::from_bytes(
                bytes.get(19..20).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            map_mods: u8::from_bytes(bytes.get(20..21).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            map_realmods: u8::from_bytes(bytes.get(21..22).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            map_vmod: u16::from_bytes(bytes.get(22..24).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            map_ctrls: u32::from_bytes(bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            supported: u8::from_bytes(bytes.get(28..29).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetNamesReply {
    pub response_type: u8,
    pub device_i_d: u8,
    pub sequence: u16,
    pub length: u32,
    pub which: NameDetail,
    pub min_key_code: crate::proto::xproto::Keycode,
    pub max_key_code: crate::proto::xproto::Keycode,
    pub n_types: u8,
    pub group_names: SetOfGroup,
    pub virtual_mods: VMod,
    pub first_key: crate::proto::xproto::Keycode,
    pub n_keys: u8,
    pub indicators: u32,
    pub n_radio_groups: u8,
    pub n_key_aliases: u8,
    pub n_k_t_levels: u16,
    pub get_names_value_list: GetNamesValueList,
}
impl VariableLengthFromBytes for GetNamesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 4 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let device_i_d = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let which = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let min_key_code = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(12..13).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let max_key_code = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(13..14).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let n_types = u8::from_bytes(ptr.get(14..15).ok_or(crate::error::Error::FromBytes)?)?;
        let group_names = u8::from_bytes(ptr.get(15..16).ok_or(crate::error::Error::FromBytes)?)?;
        let virtual_mods = u16::from_bytes(ptr.get(16..18).ok_or(crate::error::Error::FromBytes)?)?;
        let first_key = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(18..19).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let n_keys = u8::from_bytes(ptr.get(19..20).ok_or(crate::error::Error::FromBytes)?)?;
        let indicators = u32::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let n_radio_groups =
            u8::from_bytes(ptr.get(24..25).ok_or(crate::error::Error::FromBytes)?)?;
        let n_key_aliases = u8::from_bytes(ptr.get(25..26).ok_or(crate::error::Error::FromBytes)?)?;
        let n_k_t_levels = u16::from_bytes(ptr.get(26..28).ok_or(crate::error::Error::FromBytes)?)?;
        let (get_names_value_list, offset) = GetNamesValueList::from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            group_names,
            indicators,
            n_key_aliases,
            n_keys,
            n_radio_groups,
            n_types,
            virtual_mods,
            which,
        )?;
        Ok((
            Self {
                response_type,
                device_i_d,
                sequence,
                length,
                which: which.into(),
                min_key_code,
                max_key_code,
                n_types,
                group_names: group_names.into(),
                virtual_mods: virtual_mods.into(),
                first_key,
                n_keys,
                indicators,
                n_radio_groups,
                n_key_aliases,
                n_k_t_levels,
                get_names_value_list,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetNamesValueListBitCase0 {
    pub n_levels_per_type: alloc::vec::Vec<u8>,
    pub kt_level_names: alloc::vec::Vec<u32>,
}
impl GetNamesValueListBitCase0 {
    fn from_bytes(bytes: &[u8], n_types: u8) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let length_expr = n_types as usize;
        let n_levels_per_type: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(0..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let length_expr = n_levels_per_type.iter().try_fold(0usize, |start, val| {
            start
                .checked_add(usize::try_from(*val).map_err(|_| crate::error::Error::TryFromInt)?)
                .ok_or(crate::error::Error::TryFromInt)
        })? as usize;
        let kt_level_names: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        offset += length_expr * 4;
        Ok((
            Self {
                n_levels_per_type,
                kt_level_names,
            },
            offset,
        ))
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetNamesValueList {
    pub keycodes_name: Option<u32>,
    pub geometry_name: Option<u32>,
    pub symbols_name: Option<u32>,
    pub phys_symbols_name: Option<u32>,
    pub types_name: Option<u32>,
    pub compat_name: Option<u32>,
    pub type_names: Option<alloc::vec::Vec<u32>>,
    pub get_names_value_list_bit_case0: Option<GetNamesValueListBitCase0>,
    pub indicator_names: Option<alloc::vec::Vec<u32>>,
    pub virtual_mod_names: Option<alloc::vec::Vec<u32>>,
    pub groups: Option<alloc::vec::Vec<u32>>,
    pub key_names: Option<alloc::vec::Vec<KeyName>>,
    pub key_aliases: Option<alloc::vec::Vec<KeyAlias>>,
    pub radio_group_names: Option<alloc::vec::Vec<u32>>,
}
impl GetNamesValueList {
    #[inline]
    pub fn from_bytes(
        buf: &[u8],
        group_names: u8,
        indicators: u32,
        n_key_aliases: u8,
        n_keys: u8,
        n_radio_groups: u8,
        n_types: u8,
        virtual_mods: u16,
        which: u32,
    ) -> crate::error::Result<(Self, usize)> {
        let mask = which;
        let mut offset = 0;
        let mut slf = Self::default();
        let buf_ptr = buf;
        if mask & NameDetail::KEYCODES.0 != 0 {
            slf.keycodes_name = Some(u32::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
            )?);
            offset += 4;
        }
        if mask & NameDetail::GEOMETRY.0 != 0 {
            slf.geometry_name = Some(u32::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
            )?);
            offset += 4;
        }
        if mask & NameDetail::SYMBOLS.0 != 0 {
            slf.symbols_name = Some(u32::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
            )?);
            offset += 4;
        }
        if mask & NameDetail::PHYS_SYMBOLS.0 != 0 {
            slf.phys_symbols_name = Some(u32::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
            )?);
            offset += 4;
        }
        if mask & NameDetail::TYPES.0 != 0 {
            slf.types_name = Some(u32::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
            )?);
            offset += 4;
        }
        if mask & NameDetail::COMPAT.0 != 0 {
            slf.compat_name = Some(u32::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
            )?);
            offset += 4;
        }
        if mask & NameDetail::KEY_TYPE_NAMES.0 != 0 {
            let length = n_types as usize;
            slf.type_names = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                u32,
                length,
                4
            ));
            offset += length;
        }
        if mask & NameDetail::K_T_LEVEL_NAMES.0 != 0 {
            let (get_names_value_list_bit_case0, new_offset) =
                GetNamesValueListBitCase0::from_bytes(
                    buf_ptr
                        .get(offset..)
                        .ok_or(crate::error::Error::FromBytes)?,
                    n_types,
                )?;
            offset += new_offset;
            slf.get_names_value_list_bit_case0 = Some(get_names_value_list_bit_case0);
        }
        if mask & NameDetail::INDICATOR_NAMES.0 != 0 {
            let length = indicators.count_ones() as usize;
            slf.indicator_names = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                u32,
                length,
                4
            ));
            offset += length;
        }
        if mask & NameDetail::VIRTUAL_MOD_NAMES.0 != 0 {
            let length = virtual_mods.count_ones() as usize;
            slf.virtual_mod_names = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                u32,
                length,
                4
            ));
            offset += length;
        }
        if mask & NameDetail::GROUP_NAMES.0 != 0 {
            let length = group_names.count_ones() as usize;
            slf.groups = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                u32,
                length,
                4
            ));
            offset += length;
        }
        if mask & NameDetail::KEY_NAMES.0 != 0 {
            let length = n_keys as usize;
            slf.key_names = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                KeyName,
                length,
                4
            ));
            offset += length;
        }
        if mask & NameDetail::KEY_ALIASES.0 != 0 {
            let length = n_key_aliases as usize;
            slf.key_aliases = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                KeyAlias,
                length,
                8
            ));
            offset += length;
        }
        if mask & NameDetail::R_G_NAMES.0 != 0 {
            let length = n_radio_groups as usize;
            slf.radio_group_names = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                u32,
                length,
                4
            ));
            offset += length;
        }
        Ok((slf, offset))
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> NameDetail {
        let mut mask = NameDetail::default();
        if self.keycodes_name.is_some() {
            mask |= NameDetail::KEYCODES;
        }
        if self.geometry_name.is_some() {
            mask |= NameDetail::GEOMETRY;
        }
        if self.symbols_name.is_some() {
            mask |= NameDetail::SYMBOLS;
        }
        if self.phys_symbols_name.is_some() {
            mask |= NameDetail::PHYS_SYMBOLS;
        }
        if self.types_name.is_some() {
            mask |= NameDetail::TYPES;
        }
        if self.compat_name.is_some() {
            mask |= NameDetail::COMPAT;
        }
        if self.get_names_value_list_bit_case0.is_some() {
            mask |= NameDetail::K_T_LEVEL_NAMES;
        }
        mask
    }

    #[inline]
    pub fn keycodes_name(mut self, keycodes_name: u32) -> Self {
        self.keycodes_name = Some(keycodes_name);
        self
    }

    #[inline]
    pub fn geometry_name(mut self, geometry_name: u32) -> Self {
        self.geometry_name = Some(geometry_name);
        self
    }

    #[inline]
    pub fn symbols_name(mut self, symbols_name: u32) -> Self {
        self.symbols_name = Some(symbols_name);
        self
    }

    #[inline]
    pub fn phys_symbols_name(mut self, phys_symbols_name: u32) -> Self {
        self.phys_symbols_name = Some(phys_symbols_name);
        self
    }

    #[inline]
    pub fn types_name(mut self, types_name: u32) -> Self {
        self.types_name = Some(types_name);
        self
    }

    #[inline]
    pub fn compat_name(mut self, compat_name: u32) -> Self {
        self.compat_name = Some(compat_name);
        self
    }

    #[inline]
    pub fn type_names(mut self, type_names: alloc::vec::Vec<u32>) -> Self {
        self.type_names = Some(type_names);
        self
    }

    #[inline]
    pub fn get_names_value_list_bit_case0(
        mut self,
        get_names_value_list_bit_case0: GetNamesValueListBitCase0,
    ) -> Self {
        self.get_names_value_list_bit_case0 = Some(get_names_value_list_bit_case0);
        self
    }

    #[inline]
    pub fn indicator_names(mut self, indicator_names: alloc::vec::Vec<u32>) -> Self {
        self.indicator_names = Some(indicator_names);
        self
    }

    #[inline]
    pub fn virtual_mod_names(mut self, virtual_mod_names: alloc::vec::Vec<u32>) -> Self {
        self.virtual_mod_names = Some(virtual_mod_names);
        self
    }

    #[inline]
    pub fn groups(mut self, groups: alloc::vec::Vec<u32>) -> Self {
        self.groups = Some(groups);
        self
    }

    #[inline]
    pub fn key_names(mut self, key_names: alloc::vec::Vec<KeyName>) -> Self {
        self.key_names = Some(key_names);
        self
    }

    #[inline]
    pub fn key_aliases(mut self, key_aliases: alloc::vec::Vec<KeyAlias>) -> Self {
        self.key_aliases = Some(key_aliases);
        self
    }

    #[inline]
    pub fn radio_group_names(mut self, radio_group_names: alloc::vec::Vec<u32>) -> Self {
        self.radio_group_names = Some(radio_group_names);
        self
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetNamesValuesBitCase0 {
    pub n_levels_per_type: alloc::vec::Vec<u8>,
    pub kt_level_names: alloc::vec::Vec<u32>,
}
impl VariableLengthSerialize for SetNamesValuesBitCase0 {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let list_len = self.n_levels_per_type.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(0..).ok_or(crate::error::Error::Serialize)?,
            &self.n_levels_per_type,
        )?;
        let mut offset = list_len;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let list_len = self.kt_level_names.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            self.kt_level_names,
        )?;
        offset += list_len;
        Ok(offset)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetNamesValues {
    pub keycodes_name: Option<u32>,
    pub geometry_name: Option<u32>,
    pub symbols_name: Option<u32>,
    pub phys_symbols_name: Option<u32>,
    pub types_name: Option<u32>,
    pub compat_name: Option<u32>,
    pub type_names: Option<alloc::vec::Vec<u32>>,
    pub set_names_values_bit_case0: Option<SetNamesValuesBitCase0>,
    pub indicator_names: Option<alloc::vec::Vec<u32>>,
    pub virtual_mod_names: Option<alloc::vec::Vec<u32>>,
    pub groups: Option<alloc::vec::Vec<u32>>,
    pub key_names: Option<alloc::vec::Vec<KeyName>>,
    pub key_aliases: Option<alloc::vec::Vec<KeyAlias>>,
    pub radio_group_names: Option<alloc::vec::Vec<u32>>,
}
impl SetNamesValues {
    #[inline]
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let mut offset = 0;
        let buf_ptr = buf;
        if let Some(keycodes_name) = self.keycodes_name {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&keycodes_name.serialize_fixed());
            offset += 4;
        }
        if let Some(geometry_name) = self.geometry_name {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&geometry_name.serialize_fixed());
            offset += 4;
        }
        if let Some(symbols_name) = self.symbols_name {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&symbols_name.serialize_fixed());
            offset += 4;
        }
        if let Some(phys_symbols_name) = self.phys_symbols_name {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&phys_symbols_name.serialize_fixed());
            offset += 4;
        }
        if let Some(types_name) = self.types_name {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&types_name.serialize_fixed());
            offset += 4;
        }
        if let Some(compat_name) = self.compat_name {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&compat_name.serialize_fixed());
            offset += 4;
        }
        if let Some(type_names) = self.type_names {
            let out_len = type_names.len() * 4;
            crate::util::fixed_vec_serialize_into(
                buf_ptr
                    .get_mut(offset..)
                    .ok_or(crate::error::Error::Serialize)?,
                type_names,
            )?;
            offset += out_len;
        }
        if let Some(set_names_values_bit_case0) = self.set_names_values_bit_case0 {
            offset += set_names_values_bit_case0.serialize_into(
                buf_ptr
                    .get_mut(offset..)
                    .ok_or(crate::error::Error::Serialize)?,
            )?;
        }
        if let Some(indicator_names) = self.indicator_names {
            let out_len = indicator_names.len() * 4;
            crate::util::fixed_vec_serialize_into(
                buf_ptr
                    .get_mut(offset..)
                    .ok_or(crate::error::Error::Serialize)?,
                indicator_names,
            )?;
            offset += out_len;
        }
        if let Some(virtual_mod_names) = self.virtual_mod_names {
            let out_len = virtual_mod_names.len() * 4;
            crate::util::fixed_vec_serialize_into(
                buf_ptr
                    .get_mut(offset..)
                    .ok_or(crate::error::Error::Serialize)?,
                virtual_mod_names,
            )?;
            offset += out_len;
        }
        if let Some(groups) = self.groups {
            let out_len = groups.len() * 4;
            crate::util::fixed_vec_serialize_into(
                buf_ptr
                    .get_mut(offset..)
                    .ok_or(crate::error::Error::Serialize)?,
                groups,
            )?;
            offset += out_len;
        }
        if let Some(key_names) = self.key_names {
            let out_len = key_names.len() * 4;
            crate::util::fixed_vec_serialize_into(
                buf_ptr
                    .get_mut(offset..)
                    .ok_or(crate::error::Error::Serialize)?,
                key_names,
            )?;
            offset += out_len;
        }
        if let Some(key_aliases) = self.key_aliases {
            let out_len = key_aliases.len() * 8;
            crate::util::fixed_vec_serialize_into(
                buf_ptr
                    .get_mut(offset..)
                    .ok_or(crate::error::Error::Serialize)?,
                key_aliases,
            )?;
            offset += out_len;
        }
        if let Some(radio_group_names) = self.radio_group_names {
            let out_len = radio_group_names.len() * 4;
            crate::util::fixed_vec_serialize_into(
                buf_ptr
                    .get_mut(offset..)
                    .ok_or(crate::error::Error::Serialize)?,
                radio_group_names,
            )?;
            offset += out_len;
        }
        Ok(offset)
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> NameDetail {
        let mut mask = NameDetail::default();
        if self.keycodes_name.is_some() {
            mask |= NameDetail::KEYCODES;
        }
        if self.geometry_name.is_some() {
            mask |= NameDetail::GEOMETRY;
        }
        if self.symbols_name.is_some() {
            mask |= NameDetail::SYMBOLS;
        }
        if self.phys_symbols_name.is_some() {
            mask |= NameDetail::PHYS_SYMBOLS;
        }
        if self.types_name.is_some() {
            mask |= NameDetail::TYPES;
        }
        if self.compat_name.is_some() {
            mask |= NameDetail::COMPAT;
        }
        if self.set_names_values_bit_case0.is_some() {
            mask |= NameDetail::K_T_LEVEL_NAMES;
        }
        mask
    }

    #[inline]
    pub fn keycodes_name(mut self, keycodes_name: u32) -> Self {
        self.keycodes_name = Some(keycodes_name);
        self
    }

    #[inline]
    pub fn geometry_name(mut self, geometry_name: u32) -> Self {
        self.geometry_name = Some(geometry_name);
        self
    }

    #[inline]
    pub fn symbols_name(mut self, symbols_name: u32) -> Self {
        self.symbols_name = Some(symbols_name);
        self
    }

    #[inline]
    pub fn phys_symbols_name(mut self, phys_symbols_name: u32) -> Self {
        self.phys_symbols_name = Some(phys_symbols_name);
        self
    }

    #[inline]
    pub fn types_name(mut self, types_name: u32) -> Self {
        self.types_name = Some(types_name);
        self
    }

    #[inline]
    pub fn compat_name(mut self, compat_name: u32) -> Self {
        self.compat_name = Some(compat_name);
        self
    }

    #[inline]
    pub fn type_names(mut self, type_names: alloc::vec::Vec<u32>) -> Self {
        self.type_names = Some(type_names);
        self
    }

    #[inline]
    pub fn set_names_values_bit_case0(
        mut self,
        set_names_values_bit_case0: SetNamesValuesBitCase0,
    ) -> Self {
        self.set_names_values_bit_case0 = Some(set_names_values_bit_case0);
        self
    }

    #[inline]
    pub fn indicator_names(mut self, indicator_names: alloc::vec::Vec<u32>) -> Self {
        self.indicator_names = Some(indicator_names);
        self
    }

    #[inline]
    pub fn virtual_mod_names(mut self, virtual_mod_names: alloc::vec::Vec<u32>) -> Self {
        self.virtual_mod_names = Some(virtual_mod_names);
        self
    }

    #[inline]
    pub fn groups(mut self, groups: alloc::vec::Vec<u32>) -> Self {
        self.groups = Some(groups);
        self
    }

    #[inline]
    pub fn key_names(mut self, key_names: alloc::vec::Vec<KeyName>) -> Self {
        self.key_names = Some(key_names);
        self
    }

    #[inline]
    pub fn key_aliases(mut self, key_aliases: alloc::vec::Vec<KeyAlias>) -> Self {
        self.key_aliases = Some(key_aliases);
        self
    }

    #[inline]
    pub fn radio_group_names(mut self, radio_group_names: alloc::vec::Vec<u32>) -> Self {
        self.radio_group_names = Some(radio_group_names);
        self
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct PerClientFlagsReply {
    pub response_type: u8,
    pub device_i_d: u8,
    pub sequence: u16,
    pub length: u32,
    pub supported: PerClientFlag,
    pub value: PerClientFlag,
    pub auto_ctrls: BoolCtrl,
    pub auto_ctrls_values: BoolCtrl,
}
impl FixedLengthFromBytes<32> for PerClientFlagsReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            device_i_d: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            supported: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            value: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            auto_ctrls: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            auto_ctrls_values: u32::from_bytes(
                bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListComponentsReply {
    pub response_type: u8,
    pub device_i_d: u8,
    pub sequence: u16,
    pub length: u32,
    pub extra: u16,
    pub keymaps: alloc::vec::Vec<Listing>,
    pub keycodes: alloc::vec::Vec<Listing>,
    pub types: alloc::vec::Vec<Listing>,
    pub compat_maps: alloc::vec::Vec<Listing>,
    pub symbols: alloc::vec::Vec<Listing>,
    pub geometries: alloc::vec::Vec<Listing>,
}
impl VariableLengthFromBytes for ListComponentsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 10 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let device_i_d = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let n_keymaps = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let n_keycodes = u16::from_bytes(ptr.get(10..12).ok_or(crate::error::Error::FromBytes)?)?;
        let n_types = u16::from_bytes(ptr.get(12..14).ok_or(crate::error::Error::FromBytes)?)?;
        let n_compat_maps =
            u16::from_bytes(ptr.get(14..16).ok_or(crate::error::Error::FromBytes)?)?;
        let n_symbols = u16::from_bytes(ptr.get(16..18).ok_or(crate::error::Error::FromBytes)?)?;
        let n_geometries = u16::from_bytes(ptr.get(18..20).ok_or(crate::error::Error::FromBytes)?)?;
        let extra = u16::from_bytes(ptr.get(20..22).ok_or(crate::error::Error::FromBytes)?)?;
        let keymaps_length = n_keymaps as usize;
        let mut offset = 32;
        let keymaps = crate::vec_from_bytes_var!(ptr, Listing, offset, keymaps_length,);
        let keycodes_length = n_keycodes as usize;
        let keycodes = crate::vec_from_bytes_var!(ptr, Listing, offset, keycodes_length,);
        let types_length = n_types as usize;
        let types = crate::vec_from_bytes_var!(ptr, Listing, offset, types_length,);
        let compat_maps_length = n_compat_maps as usize;
        let compat_maps = crate::vec_from_bytes_var!(ptr, Listing, offset, compat_maps_length,);
        let symbols_length = n_symbols as usize;
        let symbols = crate::vec_from_bytes_var!(ptr, Listing, offset, symbols_length,);
        let geometries_length = n_geometries as usize;
        let geometries = crate::vec_from_bytes_var!(ptr, Listing, offset, geometries_length,);
        Ok((
            Self {
                response_type,
                device_i_d,
                sequence,
                length,
                extra,
                keymaps,
                keycodes,
                types,
                compat_maps,
                symbols,
                geometries,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetKbdByNameReply {
    pub response_type: u8,
    pub device_i_d: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_key_code: crate::proto::xproto::Keycode,
    pub max_key_code: crate::proto::xproto::Keycode,
    pub loaded: u8,
    pub new_keyboard: u8,
    pub found: GBNDetail,
    pub reported: GBNDetail,
    pub get_kbd_by_name_replies: GetKbdByNameReplies,
}
impl VariableLengthFromBytes for GetKbdByNameReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 16 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let device_i_d = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let min_key_code = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(8..9).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let max_key_code = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(9..10).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let loaded = u8::from_bytes(ptr.get(10..11).ok_or(crate::error::Error::FromBytes)?)?;
        let new_keyboard = u8::from_bytes(ptr.get(11..12).ok_or(crate::error::Error::FromBytes)?)?;
        let found = u16::from_bytes(ptr.get(12..14).ok_or(crate::error::Error::FromBytes)?)?;
        let reported = u16::from_bytes(ptr.get(14..16).ok_or(crate::error::Error::FromBytes)?)?;
        let (get_kbd_by_name_replies, offset) = GetKbdByNameReplies::from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            reported,
        )?;
        Ok((
            Self {
                response_type,
                device_i_d,
                sequence,
                length,
                min_key_code,
                max_key_code,
                loaded,
                new_keyboard,
                found: found.into(),
                reported: reported.into(),
                get_kbd_by_name_replies,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Types {
    pub getmap_type: u8,
    pub type_device_i_d: u8,
    pub getmap_sequence: u16,
    pub getmap_length: u32,
    pub type_min_key_code: crate::proto::xproto::Keycode,
    pub type_max_key_code: crate::proto::xproto::Keycode,
    pub present: MapPart,
    pub first_type: u8,
    pub n_types: u8,
    pub total_types: u8,
    pub first_key_sym: crate::proto::xproto::Keycode,
    pub total_syms: u16,
    pub n_key_syms: u8,
    pub first_key_action: crate::proto::xproto::Keycode,
    pub total_actions: u16,
    pub n_key_actions: u8,
    pub first_key_behavior: crate::proto::xproto::Keycode,
    pub n_key_behaviors: u8,
    pub total_key_behaviors: u8,
    pub first_key_explicit: crate::proto::xproto::Keycode,
    pub n_key_explicit: u8,
    pub total_key_explicit: u8,
    pub first_mod_map_key: crate::proto::xproto::Keycode,
    pub n_mod_map_keys: u8,
    pub total_mod_map_keys: u8,
    pub first_v_mod_map_key: crate::proto::xproto::Keycode,
    pub n_v_mod_map_keys: u8,
    pub total_v_mod_map_keys: u8,
    pub virtual_mods: VMod,
    pub get_kbd_by_name_map: GetKbdByNameMap,
}
impl VariableLengthFromBytes for Types {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 2 bytes
        // Padding 1 bytes
        let getmap_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let type_device_i_d = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let getmap_sequence =
            u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let getmap_length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let type_min_key_code = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(10..11).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let type_max_key_code = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(11..12).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let present = u16::from_bytes(ptr.get(12..14).ok_or(crate::error::Error::FromBytes)?)?;
        let first_type = u8::from_bytes(ptr.get(14..15).ok_or(crate::error::Error::FromBytes)?)?;
        let n_types = u8::from_bytes(ptr.get(15..16).ok_or(crate::error::Error::FromBytes)?)?;
        let total_types = u8::from_bytes(ptr.get(16..17).ok_or(crate::error::Error::FromBytes)?)?;
        let first_key_sym = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(17..18).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let total_syms = u16::from_bytes(ptr.get(18..20).ok_or(crate::error::Error::FromBytes)?)?;
        let n_key_syms = u8::from_bytes(ptr.get(20..21).ok_or(crate::error::Error::FromBytes)?)?;
        let first_key_action = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(21..22).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let total_actions =
            u16::from_bytes(ptr.get(22..24).ok_or(crate::error::Error::FromBytes)?)?;
        let n_key_actions = u8::from_bytes(ptr.get(24..25).ok_or(crate::error::Error::FromBytes)?)?;
        let first_key_behavior = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(25..26).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let n_key_behaviors =
            u8::from_bytes(ptr.get(26..27).ok_or(crate::error::Error::FromBytes)?)?;
        let total_key_behaviors =
            u8::from_bytes(ptr.get(27..28).ok_or(crate::error::Error::FromBytes)?)?;
        let first_key_explicit = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(28..29).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let n_key_explicit =
            u8::from_bytes(ptr.get(29..30).ok_or(crate::error::Error::FromBytes)?)?;
        let total_key_explicit =
            u8::from_bytes(ptr.get(30..31).ok_or(crate::error::Error::FromBytes)?)?;
        let first_mod_map_key = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(31..32).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let n_mod_map_keys =
            u8::from_bytes(ptr.get(32..33).ok_or(crate::error::Error::FromBytes)?)?;
        let total_mod_map_keys =
            u8::from_bytes(ptr.get(33..34).ok_or(crate::error::Error::FromBytes)?)?;
        let first_v_mod_map_key = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(34..35).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let n_v_mod_map_keys =
            u8::from_bytes(ptr.get(35..36).ok_or(crate::error::Error::FromBytes)?)?;
        let total_v_mod_map_keys =
            u8::from_bytes(ptr.get(36..37).ok_or(crate::error::Error::FromBytes)?)?;
        let virtual_mods = u16::from_bytes(ptr.get(38..40).ok_or(crate::error::Error::FromBytes)?)?;
        let (get_kbd_by_name_map, offset) = GetKbdByNameMap::from_bytes(
            ptr.get(40..).ok_or(crate::error::Error::FromBytes)?,
            n_key_actions,
            n_key_syms,
            n_types,
            present,
            total_actions,
            total_key_behaviors,
            total_key_explicit,
            total_mod_map_keys,
            total_v_mod_map_keys,
            virtual_mods,
        )?;
        Ok((
            Self {
                getmap_type,
                type_device_i_d,
                getmap_sequence,
                getmap_length,
                type_min_key_code,
                type_max_key_code,
                present: present.into(),
                first_type,
                n_types,
                total_types,
                first_key_sym,
                total_syms,
                n_key_syms,
                first_key_action,
                total_actions,
                n_key_actions,
                first_key_behavior,
                n_key_behaviors,
                total_key_behaviors,
                first_key_explicit,
                n_key_explicit,
                total_key_explicit,
                first_mod_map_key,
                n_mod_map_keys,
                total_mod_map_keys,
                first_v_mod_map_key,
                n_v_mod_map_keys,
                total_v_mod_map_keys,
                virtual_mods: virtual_mods.into(),
                get_kbd_by_name_map,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CompatMap {
    pub compatmap_type: u8,
    pub compat_device_i_d: u8,
    pub compatmap_sequence: u16,
    pub compatmap_length: u32,
    pub first_s_i_rtrn: u16,
    pub n_total_s_i: u16,
    pub si_rtrn: alloc::vec::Vec<SymInterpret>,
    pub group_rtrn: alloc::vec::Vec<ModDef>,
}
impl VariableLengthFromBytes for CompatMap {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 16 bytes
        let compatmap_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let compat_device_i_d =
            u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let compatmap_sequence =
            u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let compatmap_length =
            u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let groups_rtrn = u8::from_bytes(ptr.get(8..9).ok_or(crate::error::Error::FromBytes)?)?;
        let first_s_i_rtrn =
            u16::from_bytes(ptr.get(10..12).ok_or(crate::error::Error::FromBytes)?)?;
        let n_s_i_rtrn = u16::from_bytes(ptr.get(12..14).ok_or(crate::error::Error::FromBytes)?)?;
        let n_total_s_i = u16::from_bytes(ptr.get(14..16).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n_s_i_rtrn as usize;
        let si_rtrn: alloc::vec::Vec<SymInterpret> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            SymInterpret,
            length_expr,
            16
        );
        let mut offset = length_expr * 16 + 32;
        let length_expr = groups_rtrn.count_ones() as usize;
        let group_rtrn: alloc::vec::Vec<ModDef> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            ModDef,
            length_expr,
            4
        );
        offset += length_expr * 4;
        Ok((
            Self {
                compatmap_type,
                compat_device_i_d,
                compatmap_sequence,
                compatmap_length,
                first_s_i_rtrn,
                n_total_s_i,
                si_rtrn,
                group_rtrn,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct IndicatorMaps {
    pub indicatormap_type: u8,
    pub indicator_device_i_d: u8,
    pub indicatormap_sequence: u16,
    pub indicatormap_length: u32,
    pub which: u32,
    pub real_indicators: u32,
    pub maps: alloc::vec::Vec<IndicatorMap>,
}
impl VariableLengthFromBytes for IndicatorMaps {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 15 bytes
        let indicatormap_type =
            u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let indicator_device_i_d =
            u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let indicatormap_sequence =
            u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let indicatormap_length =
            u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let which = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let real_indicators =
            u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let n_indicators = u8::from_bytes(ptr.get(16..17).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = n_indicators as usize;
        let maps: alloc::vec::Vec<IndicatorMap> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            IndicatorMap,
            length_expr,
            12
        );
        let offset = length_expr * 12 + 32;
        Ok((
            Self {
                indicatormap_type,
                indicator_device_i_d,
                indicatormap_sequence,
                indicatormap_length,
                which,
                real_indicators,
                maps,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct KeyNames {
    pub keyname_type: u8,
    pub key_device_i_d: u8,
    pub keyname_sequence: u16,
    pub keyname_length: u32,
    pub which: NameDetail,
    pub key_min_key_code: crate::proto::xproto::Keycode,
    pub key_max_key_code: crate::proto::xproto::Keycode,
    pub n_types: u8,
    pub group_names: SetOfGroup,
    pub virtual_mods: VMod,
    pub first_key: crate::proto::xproto::Keycode,
    pub n_keys: u8,
    pub indicators: u32,
    pub n_radio_groups: u8,
    pub n_key_aliases: u8,
    pub n_k_t_levels: u16,
    pub get_kbd_by_name_value_list: GetKbdByNameValueList,
}
impl VariableLengthFromBytes for KeyNames {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 4 bytes
        let keyname_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let key_device_i_d = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let keyname_sequence =
            u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let keyname_length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let which = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let key_min_key_code = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(12..13).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let key_max_key_code = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(13..14).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let n_types = u8::from_bytes(ptr.get(14..15).ok_or(crate::error::Error::FromBytes)?)?;
        let group_names = u8::from_bytes(ptr.get(15..16).ok_or(crate::error::Error::FromBytes)?)?;
        let virtual_mods = u16::from_bytes(ptr.get(16..18).ok_or(crate::error::Error::FromBytes)?)?;
        let first_key = crate::proto::xproto::Keycode::from_bytes(
            ptr.get(18..19).ok_or(crate::error::Error::FromBytes)?,
        )?;
        let n_keys = u8::from_bytes(ptr.get(19..20).ok_or(crate::error::Error::FromBytes)?)?;
        let indicators = u32::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let n_radio_groups =
            u8::from_bytes(ptr.get(24..25).ok_or(crate::error::Error::FromBytes)?)?;
        let n_key_aliases = u8::from_bytes(ptr.get(25..26).ok_or(crate::error::Error::FromBytes)?)?;
        let n_k_t_levels = u16::from_bytes(ptr.get(26..28).ok_or(crate::error::Error::FromBytes)?)?;
        let (get_kbd_by_name_value_list, offset) = GetKbdByNameValueList::from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            group_names,
            indicators,
            n_key_aliases,
            n_keys,
            n_radio_groups,
            n_types,
            virtual_mods,
            which,
        )?;
        Ok((
            Self {
                keyname_type,
                key_device_i_d,
                keyname_sequence,
                keyname_length,
                which: which.into(),
                key_min_key_code,
                key_max_key_code,
                n_types,
                group_names: group_names.into(),
                virtual_mods: virtual_mods.into(),
                first_key,
                n_keys,
                indicators,
                n_radio_groups,
                n_key_aliases,
                n_k_t_levels,
                get_kbd_by_name_value_list,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Geometry {
    pub geometry_type: u8,
    pub geometry_device_i_d: u8,
    pub geometry_sequence: u16,
    pub geometry_length: u32,
    pub name: u32,
    pub geometry_found: u8,
    pub width_m_m: u16,
    pub height_m_m: u16,
    pub n_properties: u16,
    pub n_colors: u16,
    pub n_shapes: u16,
    pub n_sections: u16,
    pub n_doodads: u16,
    pub n_key_aliases: u16,
    pub base_color_ndx: u8,
    pub label_color_ndx: u8,
    pub label_font: CountedString16,
}
impl VariableLengthFromBytes for Geometry {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        let geometry_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let geometry_device_i_d =
            u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let geometry_sequence =
            u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let geometry_length =
            u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let name = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let geometry_found =
            u8::from_bytes(ptr.get(12..13).ok_or(crate::error::Error::FromBytes)?)?;
        let width_m_m = u16::from_bytes(ptr.get(14..16).ok_or(crate::error::Error::FromBytes)?)?;
        let height_m_m = u16::from_bytes(ptr.get(16..18).ok_or(crate::error::Error::FromBytes)?)?;
        let n_properties = u16::from_bytes(ptr.get(18..20).ok_or(crate::error::Error::FromBytes)?)?;
        let n_colors = u16::from_bytes(ptr.get(20..22).ok_or(crate::error::Error::FromBytes)?)?;
        let n_shapes = u16::from_bytes(ptr.get(22..24).ok_or(crate::error::Error::FromBytes)?)?;
        let n_sections = u16::from_bytes(ptr.get(24..26).ok_or(crate::error::Error::FromBytes)?)?;
        let n_doodads = u16::from_bytes(ptr.get(26..28).ok_or(crate::error::Error::FromBytes)?)?;
        let n_key_aliases =
            u16::from_bytes(ptr.get(28..30).ok_or(crate::error::Error::FromBytes)?)?;
        let base_color_ndx =
            u8::from_bytes(ptr.get(30..31).ok_or(crate::error::Error::FromBytes)?)?;
        let label_color_ndx =
            u8::from_bytes(ptr.get(31..32).ok_or(crate::error::Error::FromBytes)?)?;
        let (label_font, offset) =
            CountedString16::from_bytes(ptr.get(32..).ok_or(crate::error::Error::FromBytes)?)?;
        Ok((
            Self {
                geometry_type,
                geometry_device_i_d,
                geometry_sequence,
                geometry_length,
                name,
                geometry_found,
                width_m_m,
                height_m_m,
                n_properties,
                n_colors,
                n_shapes,
                n_sections,
                n_doodads,
                n_key_aliases,
                base_color_ndx,
                label_color_ndx,
                label_font,
            },
            offset,
        ))
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetKbdByNameReplies {
    pub types: Option<Types>,
    pub compat_map: Option<CompatMap>,
    pub indicator_maps: Option<IndicatorMaps>,
    pub key_names: Option<KeyNames>,
    pub geometry: Option<Geometry>,
}
impl GetKbdByNameReplies {
    #[inline]
    pub fn from_bytes(buf: &[u8], reported: u16) -> crate::error::Result<(Self, usize)> {
        let mask = reported;
        let mut offset = 0;
        let mut slf = Self::default();
        let buf_ptr = buf;
        if mask & GBNDetail::TYPES.0 != 0 {
            let (types, new_offset) = Types::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
            )?;
            offset += new_offset;
            slf.types = Some(types);
        }
        if mask & GBNDetail::COMPAT_MAP.0 != 0 {
            let (compat_map, new_offset) = CompatMap::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
            )?;
            offset += new_offset;
            slf.compat_map = Some(compat_map);
        }
        if mask & GBNDetail::INDICATOR_MAPS.0 != 0 {
            let (indicator_maps, new_offset) = IndicatorMaps::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
            )?;
            offset += new_offset;
            slf.indicator_maps = Some(indicator_maps);
        }
        if mask & GBNDetail::KEY_NAMES.0 != 0 {
            let (key_names, new_offset) = KeyNames::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
            )?;
            offset += new_offset;
            slf.key_names = Some(key_names);
        }
        if mask & GBNDetail::GEOMETRY.0 != 0 {
            let (geometry, new_offset) = Geometry::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
            )?;
            offset += new_offset;
            slf.geometry = Some(geometry);
        }
        Ok((slf, offset))
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> GBNDetail {
        let mut mask = GBNDetail::default();
        if self.types.is_some() {
            mask |= GBNDetail::TYPES;
        }
        if self.compat_map.is_some() {
            mask |= GBNDetail::COMPAT_MAP;
        }
        if self.indicator_maps.is_some() {
            mask |= GBNDetail::INDICATOR_MAPS;
        }
        if self.key_names.is_some() {
            mask |= GBNDetail::KEY_NAMES;
        }
        if self.geometry.is_some() {
            mask |= GBNDetail::GEOMETRY;
        }
        mask
    }

    #[inline]
    pub fn types(mut self, types: Types) -> Self {
        self.types = Some(types);
        self
    }

    #[inline]
    pub fn compat_map(mut self, compat_map: CompatMap) -> Self {
        self.compat_map = Some(compat_map);
        self
    }

    #[inline]
    pub fn indicator_maps(mut self, indicator_maps: IndicatorMaps) -> Self {
        self.indicator_maps = Some(indicator_maps);
        self
    }

    #[inline]
    pub fn key_names(mut self, key_names: KeyNames) -> Self {
        self.key_names = Some(key_names);
        self
    }

    #[inline]
    pub fn geometry(mut self, geometry: Geometry) -> Self {
        self.geometry = Some(geometry);
        self
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetKbdByNameMapBitCase0 {
    pub acts_rtrn_count: alloc::vec::Vec<u8>,
    pub acts_rtrn_acts: alloc::vec::Vec<Action>,
}
impl GetKbdByNameMapBitCase0 {
    fn from_bytes(
        bytes: &[u8],
        n_key_actions: u8,
        total_actions: u16,
    ) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let length_expr = n_key_actions as usize;
        let acts_rtrn_count: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(0..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let length_expr = total_actions as usize;
        let acts_rtrn_acts: alloc::vec::Vec<Action> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Action,
            length_expr,
            8
        );
        offset += length_expr * 8;
        Ok((
            Self {
                acts_rtrn_count,
                acts_rtrn_acts,
            },
            offset,
        ))
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetKbdByNameMap {
    pub types_rtrn: Option<alloc::vec::Vec<KeyType>>,
    pub syms_rtrn: Option<alloc::vec::Vec<KeySymMap>>,
    pub get_kbd_by_name_map_bit_case0: Option<GetKbdByNameMapBitCase0>,
    pub behaviors_rtrn: Option<alloc::vec::Vec<SetBehavior>>,
    pub vmods_rtrn: Option<alloc::vec::Vec<crate::proto::xproto::ModMask>>,
    pub explicit_rtrn: Option<alloc::vec::Vec<SetExplicit>>,
    pub modmap_rtrn: Option<alloc::vec::Vec<KeyModMap>>,
    pub vmodmap_rtrn: Option<alloc::vec::Vec<KeyVModMap>>,
}
impl GetKbdByNameMap {
    #[inline]
    pub fn from_bytes(
        buf: &[u8],
        n_key_actions: u8,
        n_key_syms: u8,
        n_types: u8,
        present: u16,
        total_actions: u16,
        total_key_behaviors: u8,
        total_key_explicit: u8,
        total_mod_map_keys: u8,
        total_v_mod_map_keys: u8,
        virtual_mods: u16,
    ) -> crate::error::Result<(Self, usize)> {
        let mask = present;
        let mut offset = 0;
        let mut slf = Self::default();
        let buf_ptr = buf;
        if mask & MapPart::KEY_TYPES.0 != 0 {
            let length = n_types as usize;
            let types_rtrn = crate::vec_from_bytes_var!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                KeyType,
                offset,
                length
            );
            slf.types_rtrn = Some(types_rtrn);
        }
        if mask & MapPart::KEY_SYMS.0 != 0 {
            let length = n_key_syms as usize;
            let syms_rtrn = crate::vec_from_bytes_var!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                KeySymMap,
                offset,
                length
            );
            slf.syms_rtrn = Some(syms_rtrn);
        }
        if mask & MapPart::KEY_ACTIONS.0 != 0 {
            let (get_kbd_by_name_map_bit_case0, new_offset) = GetKbdByNameMapBitCase0::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                n_key_actions,
                total_actions,
            )?;
            offset += new_offset;
            slf.get_kbd_by_name_map_bit_case0 = Some(get_kbd_by_name_map_bit_case0);
        }
        if mask & MapPart::KEY_BEHAVIORS.0 != 0 {
            let length = total_key_behaviors as usize;
            slf.behaviors_rtrn = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                SetBehavior,
                length,
                4
            ));
            offset += length;
        }
        if mask & MapPart::VIRTUAL_MODS.0 != 0 {
            let length = virtual_mods.count_ones() as usize;
            slf.vmods_rtrn = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                crate::proto::xproto::ModMask,
                length,
                1
            ));
            offset += length;
        }
        if mask & MapPart::EXPLICIT_COMPONENTS.0 != 0 {
            let length = total_key_explicit as usize;
            slf.explicit_rtrn = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                SetExplicit,
                length,
                2
            ));
            offset += length;
        }
        if mask & MapPart::MODIFIER_MAP.0 != 0 {
            let length = total_mod_map_keys as usize;
            slf.modmap_rtrn = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                KeyModMap,
                length,
                2
            ));
            offset += length;
        }
        if mask & MapPart::VIRTUAL_MOD_MAP.0 != 0 {
            let length = total_v_mod_map_keys as usize;
            slf.vmodmap_rtrn = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                KeyVModMap,
                length,
                4
            ));
            offset += length;
        }
        Ok((slf, offset))
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> MapPart {
        let mut mask = MapPart::default();
        if self.get_kbd_by_name_map_bit_case0.is_some() {
            mask |= MapPart::KEY_ACTIONS;
        }
        mask
    }

    #[inline]
    pub fn types_rtrn(mut self, types_rtrn: alloc::vec::Vec<KeyType>) -> Self {
        self.types_rtrn = Some(types_rtrn);
        self
    }

    #[inline]
    pub fn syms_rtrn(mut self, syms_rtrn: alloc::vec::Vec<KeySymMap>) -> Self {
        self.syms_rtrn = Some(syms_rtrn);
        self
    }

    #[inline]
    pub fn get_kbd_by_name_map_bit_case0(
        mut self,
        get_kbd_by_name_map_bit_case0: GetKbdByNameMapBitCase0,
    ) -> Self {
        self.get_kbd_by_name_map_bit_case0 = Some(get_kbd_by_name_map_bit_case0);
        self
    }

    #[inline]
    pub fn behaviors_rtrn(mut self, behaviors_rtrn: alloc::vec::Vec<SetBehavior>) -> Self {
        self.behaviors_rtrn = Some(behaviors_rtrn);
        self
    }

    #[inline]
    pub fn vmods_rtrn(
        mut self,
        vmods_rtrn: alloc::vec::Vec<crate::proto::xproto::ModMask>,
    ) -> Self {
        self.vmods_rtrn = Some(vmods_rtrn);
        self
    }

    #[inline]
    pub fn explicit_rtrn(mut self, explicit_rtrn: alloc::vec::Vec<SetExplicit>) -> Self {
        self.explicit_rtrn = Some(explicit_rtrn);
        self
    }

    #[inline]
    pub fn modmap_rtrn(mut self, modmap_rtrn: alloc::vec::Vec<KeyModMap>) -> Self {
        self.modmap_rtrn = Some(modmap_rtrn);
        self
    }

    #[inline]
    pub fn vmodmap_rtrn(mut self, vmodmap_rtrn: alloc::vec::Vec<KeyVModMap>) -> Self {
        self.vmodmap_rtrn = Some(vmodmap_rtrn);
        self
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetKbdByNameValueListBitCase0 {
    pub n_levels_per_type: alloc::vec::Vec<u8>,
    pub kt_level_names: alloc::vec::Vec<u32>,
}
impl GetKbdByNameValueListBitCase0 {
    fn from_bytes(bytes: &[u8], n_types: u8) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let length_expr = n_types as usize;
        let n_levels_per_type: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(0..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let length_expr = n_levels_per_type.iter().try_fold(0usize, |start, val| {
            start
                .checked_add(usize::try_from(*val).map_err(|_| crate::error::Error::TryFromInt)?)
                .ok_or(crate::error::Error::TryFromInt)
        })? as usize;
        let kt_level_names: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        offset += length_expr * 4;
        Ok((
            Self {
                n_levels_per_type,
                kt_level_names,
            },
            offset,
        ))
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetKbdByNameValueList {
    pub keycodes_name: Option<u32>,
    pub geometry_name: Option<u32>,
    pub symbols_name: Option<u32>,
    pub phys_symbols_name: Option<u32>,
    pub types_name: Option<u32>,
    pub compat_name: Option<u32>,
    pub type_names: Option<alloc::vec::Vec<u32>>,
    pub get_kbd_by_name_value_list_bit_case0: Option<GetKbdByNameValueListBitCase0>,
    pub indicator_names: Option<alloc::vec::Vec<u32>>,
    pub virtual_mod_names: Option<alloc::vec::Vec<u32>>,
    pub groups: Option<alloc::vec::Vec<u32>>,
    pub key_names: Option<alloc::vec::Vec<KeyName>>,
    pub key_aliases: Option<alloc::vec::Vec<KeyAlias>>,
    pub radio_group_names: Option<alloc::vec::Vec<u32>>,
}
impl GetKbdByNameValueList {
    #[inline]
    pub fn from_bytes(
        buf: &[u8],
        group_names: u8,
        indicators: u32,
        n_key_aliases: u8,
        n_keys: u8,
        n_radio_groups: u8,
        n_types: u8,
        virtual_mods: u16,
        which: u32,
    ) -> crate::error::Result<(Self, usize)> {
        let mask = which;
        let mut offset = 0;
        let mut slf = Self::default();
        let buf_ptr = buf;
        if mask & NameDetail::KEYCODES.0 != 0 {
            slf.keycodes_name = Some(u32::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
            )?);
            offset += 4;
        }
        if mask & NameDetail::GEOMETRY.0 != 0 {
            slf.geometry_name = Some(u32::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
            )?);
            offset += 4;
        }
        if mask & NameDetail::SYMBOLS.0 != 0 {
            slf.symbols_name = Some(u32::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
            )?);
            offset += 4;
        }
        if mask & NameDetail::PHYS_SYMBOLS.0 != 0 {
            slf.phys_symbols_name = Some(u32::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
            )?);
            offset += 4;
        }
        if mask & NameDetail::TYPES.0 != 0 {
            slf.types_name = Some(u32::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
            )?);
            offset += 4;
        }
        if mask & NameDetail::COMPAT.0 != 0 {
            slf.compat_name = Some(u32::from_bytes(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
            )?);
            offset += 4;
        }
        if mask & NameDetail::KEY_TYPE_NAMES.0 != 0 {
            let length = n_types as usize;
            slf.type_names = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                u32,
                length,
                4
            ));
            offset += length;
        }
        if mask & NameDetail::K_T_LEVEL_NAMES.0 != 0 {
            let (get_kbd_by_name_value_list_bit_case0, new_offset) =
                GetKbdByNameValueListBitCase0::from_bytes(
                    buf_ptr
                        .get(offset..)
                        .ok_or(crate::error::Error::FromBytes)?,
                    n_types,
                )?;
            offset += new_offset;
            slf.get_kbd_by_name_value_list_bit_case0 = Some(get_kbd_by_name_value_list_bit_case0);
        }
        if mask & NameDetail::INDICATOR_NAMES.0 != 0 {
            let length = indicators.count_ones() as usize;
            slf.indicator_names = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                u32,
                length,
                4
            ));
            offset += length;
        }
        if mask & NameDetail::VIRTUAL_MOD_NAMES.0 != 0 {
            let length = virtual_mods.count_ones() as usize;
            slf.virtual_mod_names = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                u32,
                length,
                4
            ));
            offset += length;
        }
        if mask & NameDetail::GROUP_NAMES.0 != 0 {
            let length = group_names.count_ones() as usize;
            slf.groups = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                u32,
                length,
                4
            ));
            offset += length;
        }
        if mask & NameDetail::KEY_NAMES.0 != 0 {
            let length = n_keys as usize;
            slf.key_names = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                KeyName,
                length,
                4
            ));
            offset += length;
        }
        if mask & NameDetail::KEY_ALIASES.0 != 0 {
            let length = n_key_aliases as usize;
            slf.key_aliases = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                KeyAlias,
                length,
                8
            ));
            offset += length;
        }
        if mask & NameDetail::R_G_NAMES.0 != 0 {
            let length = n_radio_groups as usize;
            slf.radio_group_names = Some(crate::vec_from_bytes_fixed!(
                buf_ptr
                    .get(offset..)
                    .ok_or(crate::error::Error::FromBytes)?,
                u32,
                length,
                4
            ));
            offset += length;
        }
        Ok((slf, offset))
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> NameDetail {
        let mut mask = NameDetail::default();
        if self.keycodes_name.is_some() {
            mask |= NameDetail::KEYCODES;
        }
        if self.geometry_name.is_some() {
            mask |= NameDetail::GEOMETRY;
        }
        if self.symbols_name.is_some() {
            mask |= NameDetail::SYMBOLS;
        }
        if self.phys_symbols_name.is_some() {
            mask |= NameDetail::PHYS_SYMBOLS;
        }
        if self.types_name.is_some() {
            mask |= NameDetail::TYPES;
        }
        if self.compat_name.is_some() {
            mask |= NameDetail::COMPAT;
        }
        if self.get_kbd_by_name_value_list_bit_case0.is_some() {
            mask |= NameDetail::K_T_LEVEL_NAMES;
        }
        mask
    }

    #[inline]
    pub fn keycodes_name(mut self, keycodes_name: u32) -> Self {
        self.keycodes_name = Some(keycodes_name);
        self
    }

    #[inline]
    pub fn geometry_name(mut self, geometry_name: u32) -> Self {
        self.geometry_name = Some(geometry_name);
        self
    }

    #[inline]
    pub fn symbols_name(mut self, symbols_name: u32) -> Self {
        self.symbols_name = Some(symbols_name);
        self
    }

    #[inline]
    pub fn phys_symbols_name(mut self, phys_symbols_name: u32) -> Self {
        self.phys_symbols_name = Some(phys_symbols_name);
        self
    }

    #[inline]
    pub fn types_name(mut self, types_name: u32) -> Self {
        self.types_name = Some(types_name);
        self
    }

    #[inline]
    pub fn compat_name(mut self, compat_name: u32) -> Self {
        self.compat_name = Some(compat_name);
        self
    }

    #[inline]
    pub fn type_names(mut self, type_names: alloc::vec::Vec<u32>) -> Self {
        self.type_names = Some(type_names);
        self
    }

    #[inline]
    pub fn get_kbd_by_name_value_list_bit_case0(
        mut self,
        get_kbd_by_name_value_list_bit_case0: GetKbdByNameValueListBitCase0,
    ) -> Self {
        self.get_kbd_by_name_value_list_bit_case0 = Some(get_kbd_by_name_value_list_bit_case0);
        self
    }

    #[inline]
    pub fn indicator_names(mut self, indicator_names: alloc::vec::Vec<u32>) -> Self {
        self.indicator_names = Some(indicator_names);
        self
    }

    #[inline]
    pub fn virtual_mod_names(mut self, virtual_mod_names: alloc::vec::Vec<u32>) -> Self {
        self.virtual_mod_names = Some(virtual_mod_names);
        self
    }

    #[inline]
    pub fn groups(mut self, groups: alloc::vec::Vec<u32>) -> Self {
        self.groups = Some(groups);
        self
    }

    #[inline]
    pub fn key_names(mut self, key_names: alloc::vec::Vec<KeyName>) -> Self {
        self.key_names = Some(key_names);
        self
    }

    #[inline]
    pub fn key_aliases(mut self, key_aliases: alloc::vec::Vec<KeyAlias>) -> Self {
        self.key_aliases = Some(key_aliases);
        self
    }

    #[inline]
    pub fn radio_group_names(mut self, radio_group_names: alloc::vec::Vec<u32>) -> Self {
        self.radio_group_names = Some(radio_group_names);
        self
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetDeviceInfoReply {
    pub response_type: u8,
    pub device_i_d: u8,
    pub sequence: u16,
    pub length: u32,
    pub present: XIFeature,
    pub supported: XIFeature,
    pub unsupported: XIFeature,
    pub first_btn_wanted: u8,
    pub n_btns_wanted: u8,
    pub first_btn_rtrn: u8,
    pub total_btns: u8,
    pub has_own_state: u8,
    pub dflt_kbd_f_b: IdEnum,
    pub dflt_led_f_b: IdEnum,
    pub dev_type: u32,
    pub name: alloc::vec::Vec<String8>,
    pub btn_actions: alloc::vec::Vec<Action>,
    pub leds: alloc::vec::Vec<DeviceLedInfo>,
}
impl VariableLengthFromBytes for GetDeviceInfoReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 2 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let device_i_d = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let present = u16::from_bytes(ptr.get(8..10).ok_or(crate::error::Error::FromBytes)?)?;
        let supported = u16::from_bytes(ptr.get(10..12).ok_or(crate::error::Error::FromBytes)?)?;
        let unsupported = u16::from_bytes(ptr.get(12..14).ok_or(crate::error::Error::FromBytes)?)?;
        let n_device_led_f_bs =
            u16::from_bytes(ptr.get(14..16).ok_or(crate::error::Error::FromBytes)?)?;
        let first_btn_wanted =
            u8::from_bytes(ptr.get(16..17).ok_or(crate::error::Error::FromBytes)?)?;
        let n_btns_wanted = u8::from_bytes(ptr.get(17..18).ok_or(crate::error::Error::FromBytes)?)?;
        let first_btn_rtrn =
            u8::from_bytes(ptr.get(18..19).ok_or(crate::error::Error::FromBytes)?)?;
        let n_btns_rtrn = u8::from_bytes(ptr.get(19..20).ok_or(crate::error::Error::FromBytes)?)?;
        let total_btns = u8::from_bytes(ptr.get(20..21).ok_or(crate::error::Error::FromBytes)?)?;
        let has_own_state = u8::from_bytes(ptr.get(21..22).ok_or(crate::error::Error::FromBytes)?)?;
        let dflt_kbd_f_b = u16::from_bytes(ptr.get(22..24).ok_or(crate::error::Error::FromBytes)?)?;
        let dflt_led_f_b = u16::from_bytes(ptr.get(24..26).ok_or(crate::error::Error::FromBytes)?)?;
        let dev_type = u32::from_bytes(ptr.get(28..32).ok_or(crate::error::Error::FromBytes)?)?;
        let name_len = u16::from_bytes(ptr.get(32..34).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = name_len as usize;
        let name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(34..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr + 34;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let length_expr = n_btns_rtrn as usize;
        let btn_actions: alloc::vec::Vec<Action> = crate::vec_from_bytes_fixed!(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            Action,
            length_expr,
            8
        );
        offset += length_expr * 8;
        let leds_length = n_device_led_f_bs as usize;
        let leds = crate::vec_from_bytes_var!(ptr, DeviceLedInfo, offset, leds_length,);
        Ok((
            Self {
                response_type,
                device_i_d,
                sequence,
                length,
                present: present.into(),
                supported: supported.into(),
                unsupported: unsupported.into(),
                first_btn_wanted,
                n_btns_wanted,
                first_btn_rtrn,
                total_btns,
                has_own_state,
                dflt_kbd_f_b: dflt_kbd_f_b.into(),
                dflt_led_f_b: dflt_led_f_b.into(),
                dev_type,
                name,
                btn_actions,
                leds,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SetDebuggingFlagsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub current_flags: u32,
    pub current_ctrls: u32,
    pub supported_flags: u32,
    pub supported_ctrls: u32,
}
impl FixedLengthFromBytes<32> for SetDebuggingFlagsReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            current_flags: u32::from_bytes(
                bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            current_ctrls: u32::from_bytes(
                bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            supported_flags: u32::from_bytes(
                bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?,
            )?,
            supported_ctrls: u32::from_bytes(
                bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
pub const NEW_KEYBOARD_NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct NewKeyboardNotifyEvent {
    pub opcode: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub device_i_d: u8,
    pub old_device_i_d: u8,
    pub min_key_code: crate::proto::xproto::Keycode,
    pub max_key_code: crate::proto::xproto::Keycode,
    pub old_min_key_code: crate::proto::xproto::Keycode,
    pub old_max_key_code: crate::proto::xproto::Keycode,
    pub request_major: u8,
    pub request_minor: u8,
    pub changed: NKNDetail,
}
impl FixedLengthFromBytes<32> for NewKeyboardNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xkb_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            device_i_d: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            old_device_i_d: u8::from_bytes(
                bytes.get(9..10).ok_or(crate::error::Error::FromBytes)?,
            )?,
            min_key_code: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(10..11).ok_or(crate::error::Error::FromBytes)?,
            )?,
            max_key_code: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(11..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            old_min_key_code: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?,
            )?,
            old_max_key_code: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(13..14).ok_or(crate::error::Error::FromBytes)?,
            )?,
            request_major: u8::from_bytes(
                bytes.get(14..15).ok_or(crate::error::Error::FromBytes)?,
            )?,
            request_minor: u8::from_bytes(
                bytes.get(15..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            changed: u16::from_bytes(bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
pub const MAP_NOTIFY_EVENT: u8 = 1;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct MapNotifyEvent {
    pub opcode: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub device_i_d: u8,
    pub ptr_btn_actions: u8,
    pub changed: MapPart,
    pub min_key_code: crate::proto::xproto::Keycode,
    pub max_key_code: crate::proto::xproto::Keycode,
    pub first_type: u8,
    pub n_types: u8,
    pub first_key_sym: crate::proto::xproto::Keycode,
    pub n_key_syms: u8,
    pub first_key_act: crate::proto::xproto::Keycode,
    pub n_key_acts: u8,
    pub first_key_behavior: crate::proto::xproto::Keycode,
    pub n_key_behavior: u8,
    pub first_key_explicit: crate::proto::xproto::Keycode,
    pub n_key_explicit: u8,
    pub first_mod_map_key: crate::proto::xproto::Keycode,
    pub n_mod_map_keys: u8,
    pub first_v_mod_map_key: crate::proto::xproto::Keycode,
    pub n_v_mod_map_keys: u8,
    pub virtual_mods: VMod,
}
impl FixedLengthFromBytes<32> for MapNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xkb_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            device_i_d: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            ptr_btn_actions: u8::from_bytes(
                bytes.get(9..10).ok_or(crate::error::Error::FromBytes)?,
            )?,
            changed: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            min_key_code: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?,
            )?,
            max_key_code: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(13..14).ok_or(crate::error::Error::FromBytes)?,
            )?,
            first_type: u8::from_bytes(bytes.get(14..15).ok_or(crate::error::Error::FromBytes)?)?,
            n_types: u8::from_bytes(bytes.get(15..16).ok_or(crate::error::Error::FromBytes)?)?,
            first_key_sym: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(16..17).ok_or(crate::error::Error::FromBytes)?,
            )?,
            n_key_syms: u8::from_bytes(bytes.get(17..18).ok_or(crate::error::Error::FromBytes)?)?,
            first_key_act: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(18..19).ok_or(crate::error::Error::FromBytes)?,
            )?,
            n_key_acts: u8::from_bytes(bytes.get(19..20).ok_or(crate::error::Error::FromBytes)?)?,
            first_key_behavior: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(20..21).ok_or(crate::error::Error::FromBytes)?,
            )?,
            n_key_behavior: u8::from_bytes(
                bytes.get(21..22).ok_or(crate::error::Error::FromBytes)?,
            )?,
            first_key_explicit: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(22..23).ok_or(crate::error::Error::FromBytes)?,
            )?,
            n_key_explicit: u8::from_bytes(
                bytes.get(23..24).ok_or(crate::error::Error::FromBytes)?,
            )?,
            first_mod_map_key: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(24..25).ok_or(crate::error::Error::FromBytes)?,
            )?,
            n_mod_map_keys: u8::from_bytes(
                bytes.get(25..26).ok_or(crate::error::Error::FromBytes)?,
            )?,
            first_v_mod_map_key: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(26..27).ok_or(crate::error::Error::FromBytes)?,
            )?,
            n_v_mod_map_keys: u8::from_bytes(
                bytes.get(27..28).ok_or(crate::error::Error::FromBytes)?,
            )?,
            virtual_mods: u16::from_bytes(
                bytes.get(28..30).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
        })
    }
}
pub const STATE_NOTIFY_EVENT: u8 = 2;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct StateNotifyEvent {
    pub opcode: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub device_i_d: u8,
    pub mods: crate::proto::xproto::ModMask,
    pub base_mods: crate::proto::xproto::ModMask,
    pub latched_mods: crate::proto::xproto::ModMask,
    pub locked_mods: crate::proto::xproto::ModMask,
    pub group: GroupEnum,
    pub base_group: i16,
    pub latched_group: i16,
    pub locked_group: GroupEnum,
    pub compat_state: crate::proto::xproto::ModMask,
    pub grab_mods: crate::proto::xproto::ModMask,
    pub compat_grab_mods: crate::proto::xproto::ModMask,
    pub lookup_mods: crate::proto::xproto::ModMask,
    pub compat_loockup_mods: crate::proto::xproto::ModMask,
    pub ptr_btn_state: crate::proto::xproto::KeyButMask,
    pub changed: StatePart,
    pub keycode: crate::proto::xproto::Keycode,
    pub event_type: u8,
    pub request_major: u8,
    pub request_minor: u8,
}
impl FixedLengthFromBytes<32> for StateNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xkb_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            device_i_d: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            mods: u8::from_bytes(bytes.get(9..10).ok_or(crate::error::Error::FromBytes)?)?.into(),
            base_mods: u8::from_bytes(bytes.get(10..11).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            latched_mods: u8::from_bytes(bytes.get(11..12).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            locked_mods: u8::from_bytes(bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            group: u8::from_bytes(bytes.get(13..14).ok_or(crate::error::Error::FromBytes)?)?.into(),
            base_group: i16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
            latched_group: i16::from_bytes(
                bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?,
            )?,
            locked_group: u8::from_bytes(bytes.get(18..19).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            compat_state: u8::from_bytes(bytes.get(19..20).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            grab_mods: u8::from_bytes(bytes.get(20..21).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            compat_grab_mods: u8::from_bytes(
                bytes.get(21..22).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            lookup_mods: u8::from_bytes(bytes.get(22..23).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            compat_loockup_mods: u8::from_bytes(
                bytes.get(23..24).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            ptr_btn_state: u16::from_bytes(
                bytes.get(24..26).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            changed: u16::from_bytes(bytes.get(26..28).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            keycode: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(28..29).ok_or(crate::error::Error::FromBytes)?,
            )?,
            event_type: u8::from_bytes(bytes.get(29..30).ok_or(crate::error::Error::FromBytes)?)?,
            request_major: u8::from_bytes(
                bytes.get(30..31).ok_or(crate::error::Error::FromBytes)?,
            )?,
            request_minor: u8::from_bytes(
                bytes.get(31..32).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
pub const CONTROLS_NOTIFY_EVENT: u8 = 3;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ControlsNotifyEvent {
    pub opcode: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub device_i_d: u8,
    pub num_groups: u8,
    pub changed_controls: Control,
    pub enabled_controls: BoolCtrl,
    pub enabled_control_changes: BoolCtrl,
    pub keycode: crate::proto::xproto::Keycode,
    pub event_type: u8,
    pub request_major: u8,
    pub request_minor: u8,
}
impl FixedLengthFromBytes<32> for ControlsNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xkb_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            device_i_d: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            num_groups: u8::from_bytes(bytes.get(9..10).ok_or(crate::error::Error::FromBytes)?)?,
            changed_controls: u32::from_bytes(
                bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            enabled_controls: u32::from_bytes(
                bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            enabled_control_changes: u32::from_bytes(
                bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            keycode: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(24..25).ok_or(crate::error::Error::FromBytes)?,
            )?,
            event_type: u8::from_bytes(bytes.get(25..26).ok_or(crate::error::Error::FromBytes)?)?,
            request_major: u8::from_bytes(
                bytes.get(26..27).ok_or(crate::error::Error::FromBytes)?,
            )?,
            request_minor: u8::from_bytes(
                bytes.get(27..28).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
pub const INDICATOR_STATE_NOTIFY_EVENT: u8 = 4;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct IndicatorStateNotifyEvent {
    pub opcode: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub device_i_d: u8,
    pub state: u32,
    pub state_changed: u32,
}
impl FixedLengthFromBytes<32> for IndicatorStateNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xkb_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            device_i_d: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            state: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            state_changed: u32::from_bytes(
                bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
pub const INDICATOR_MAP_NOTIFY_EVENT: u8 = 5;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct IndicatorMapNotifyEvent {
    pub opcode: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub device_i_d: u8,
    pub state: u32,
    pub map_changed: u32,
}
impl FixedLengthFromBytes<32> for IndicatorMapNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xkb_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            device_i_d: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            state: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            map_changed: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const NAMES_NOTIFY_EVENT: u8 = 6;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct NamesNotifyEvent {
    pub opcode: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub device_i_d: u8,
    pub changed: NameDetail,
    pub first_type: u8,
    pub n_types: u8,
    pub first_level_name: u8,
    pub n_level_names: u8,
    pub n_radio_groups: u8,
    pub n_key_aliases: u8,
    pub changed_group_names: SetOfGroup,
    pub changed_virtual_mods: VMod,
    pub first_key: crate::proto::xproto::Keycode,
    pub n_keys: u8,
    pub changed_indicators: u32,
}
impl FixedLengthFromBytes<32> for NamesNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xkb_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            device_i_d: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            changed: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            first_type: u8::from_bytes(bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?)?,
            n_types: u8::from_bytes(bytes.get(13..14).ok_or(crate::error::Error::FromBytes)?)?,
            first_level_name: u8::from_bytes(
                bytes.get(14..15).ok_or(crate::error::Error::FromBytes)?,
            )?,
            n_level_names: u8::from_bytes(
                bytes.get(15..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            n_radio_groups: u8::from_bytes(
                bytes.get(17..18).ok_or(crate::error::Error::FromBytes)?,
            )?,
            n_key_aliases: u8::from_bytes(
                bytes.get(18..19).ok_or(crate::error::Error::FromBytes)?,
            )?,
            changed_group_names: u8::from_bytes(
                bytes.get(19..20).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            changed_virtual_mods: u16::from_bytes(
                bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            first_key: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(22..23).ok_or(crate::error::Error::FromBytes)?,
            )?,
            n_keys: u8::from_bytes(bytes.get(23..24).ok_or(crate::error::Error::FromBytes)?)?,
            changed_indicators: u32::from_bytes(
                bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
pub const COMPAT_MAP_NOTIFY_EVENT: u8 = 7;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CompatMapNotifyEvent {
    pub opcode: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub device_i_d: u8,
    pub changed_groups: SetOfGroup,
    pub first_s_i: u16,
    pub n_s_i: u16,
    pub n_total_s_i: u16,
}
impl FixedLengthFromBytes<32> for CompatMapNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xkb_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            device_i_d: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            changed_groups: u8::from_bytes(
                bytes.get(9..10).ok_or(crate::error::Error::FromBytes)?,
            )?
            .into(),
            first_s_i: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?,
            n_s_i: u16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
            n_total_s_i: u16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const BELL_NOTIFY_EVENT: u8 = 8;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct BellNotifyEvent {
    pub opcode: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub device_i_d: u8,
    pub bell_class: BellClassResultEnum,
    pub bell_i_d: u8,
    pub percent: u8,
    pub pitch: u16,
    pub duration: u16,
    pub name: u32,
    pub window: crate::proto::xproto::Window,
    pub event_only: u8,
}
impl FixedLengthFromBytes<32> for BellNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xkb_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            device_i_d: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            bell_class: u8::from_bytes(bytes.get(9..10).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            bell_i_d: u8::from_bytes(bytes.get(10..11).ok_or(crate::error::Error::FromBytes)?)?,
            percent: u8::from_bytes(bytes.get(11..12).ok_or(crate::error::Error::FromBytes)?)?,
            pitch: u16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?,
            duration: u16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
            name: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
            window: crate::proto::xproto::Window::from_bytes(
                bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?,
            )?,
            event_only: u8::from_bytes(bytes.get(24..25).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const ACTION_MESSAGE_EVENT: u8 = 9;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ActionMessageEvent {
    pub opcode: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub device_i_d: u8,
    pub keycode: crate::proto::xproto::Keycode,
    pub press: u8,
    pub key_event_follows: u8,
    pub mods: crate::proto::xproto::ModMask,
    pub group: GroupEnum,
    pub message: [String8; 8],
}
impl FixedLengthFromBytes<32> for ActionMessageEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xkb_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            device_i_d: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            keycode: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(9..10).ok_or(crate::error::Error::FromBytes)?,
            )?,
            press: u8::from_bytes(bytes.get(10..11).ok_or(crate::error::Error::FromBytes)?)?,
            key_event_follows: u8::from_bytes(
                bytes.get(11..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
            mods: u8::from_bytes(bytes.get(12..13).ok_or(crate::error::Error::FromBytes)?)?.into(),
            group: u8::from_bytes(bytes.get(13..14).ok_or(crate::error::Error::FromBytes)?)?.into(),
            // SAFETY: We know we can try into exact size slice
            message: unsafe {
                bytes
                    .get(14..14 + 8)
                    .ok_or(crate::error::Error::FromBytes)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}
pub const ACCESS_X_NOTIFY_EVENT: u8 = 10;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct AccessXNotifyEvent {
    pub opcode: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub device_i_d: u8,
    pub keycode: crate::proto::xproto::Keycode,
    pub detailt: AXNDetail,
    pub slow_keys_delay: u16,
    pub debounce_delay: u16,
}
impl FixedLengthFromBytes<32> for AccessXNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xkb_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            device_i_d: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            keycode: crate::proto::xproto::Keycode::from_bytes(
                bytes.get(9..10).ok_or(crate::error::Error::FromBytes)?,
            )?,
            detailt: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            slow_keys_delay: u16::from_bytes(
                bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?,
            )?,
            debounce_delay: u16::from_bytes(
                bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
pub const EXTENSION_DEVICE_NOTIFY_EVENT: u8 = 11;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ExtensionDeviceNotifyEvent {
    pub opcode: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: crate::proto::xproto::Timestamp,
    pub device_i_d: u8,
    pub reason: XIFeature,
    pub led_class: LedClassResultEnum,
    pub led_i_d: u16,
    pub leds_defined: u32,
    pub led_state: u32,
    pub first_button: u8,
    pub n_buttons: u8,
    pub supported: XIFeature,
    pub unsupported: XIFeature,
}
impl FixedLengthFromBytes<32> for ExtensionDeviceNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            xkb_type: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            time: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            device_i_d: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            reason: u16::from_bytes(bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            led_class: u16::from_bytes(bytes.get(12..14).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            led_i_d: u16::from_bytes(bytes.get(14..16).ok_or(crate::error::Error::FromBytes)?)?,
            leds_defined: u32::from_bytes(
                bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?,
            )?,
            led_state: u32::from_bytes(bytes.get(20..24).ok_or(crate::error::Error::FromBytes)?)?,
            first_button: u8::from_bytes(bytes.get(24..25).ok_or(crate::error::Error::FromBytes)?)?,
            n_buttons: u8::from_bytes(bytes.get(25..26).ok_or(crate::error::Error::FromBytes)?)?,
            supported: u16::from_bytes(bytes.get(26..28).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            unsupported: u16::from_bytes(bytes.get(28..30).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
