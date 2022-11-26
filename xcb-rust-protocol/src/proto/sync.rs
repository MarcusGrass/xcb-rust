#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "SYNC";
pub type Alarm = u32;
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct AlarmstateEnum(pub u8);
impl AlarmstateEnum {
    pub const ACTIVE: Self = Self(0);
    pub const INACTIVE: Self = Self(1);
    pub const DESTROYED: Self = Self(2);
}
impl FixedLengthSerialize<1> for AlarmstateEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 1] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<1> for AlarmstateEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u8::from_bytes(bytes)?))
    }
}
impl From<u32> for AlarmstateEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u8)
    }
}
impl From<u16> for AlarmstateEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u8)
    }
}
impl From<u8> for AlarmstateEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u8)
    }
}
pub type Counter = u32;
pub type Fence = u32;
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct TesttypeEnum(pub u32);
impl TesttypeEnum {
    pub const POSITIVE_TRANSITION: Self = Self(0);
    pub const NEGATIVE_TRANSITION: Self = Self(1);
    pub const POSITIVE_COMPARISON: Self = Self(2);
    pub const NEGATIVE_COMPARISON: Self = Self(3);
}
impl FixedLengthSerialize<4> for TesttypeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for TesttypeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for TesttypeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for TesttypeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for TesttypeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ValuetypeEnum(pub u32);
impl ValuetypeEnum {
    pub const ABSOLUTE: Self = Self(0);
    pub const RELATIVE: Self = Self(1);
}
impl FixedLengthSerialize<4> for ValuetypeEnum {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for ValuetypeEnum {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for ValuetypeEnum {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for ValuetypeEnum {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for ValuetypeEnum {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Ca(pub u32);
impl Ca {
    pub const COUNTER: Self = Self(1 << 0);
    pub const VALUE_TYPE: Self = Self(1 << 1);
    pub const VALUE: Self = Self(1 << 2);
    pub const TEST_TYPE: Self = Self(1 << 3);
    pub const DELTA: Self = Self(1 << 4);
    pub const EVENTS: Self = Self(1 << 5);
}
impl FixedLengthSerialize<4> for Ca {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for Ca {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for Ca {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for Ca {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for Ca {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(Ca);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Int64 {
    pub hi: i32,
    pub lo: u32,
}
impl FixedLengthSerialize<8> for Int64 {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let hi_bytes = self.hi.serialize_fixed();
        let lo_bytes = self.lo.serialize_fixed();
        [
            hi_bytes[0],
            hi_bytes[1],
            hi_bytes[2],
            hi_bytes[3],
            lo_bytes[0],
            lo_bytes[1],
            lo_bytes[2],
            lo_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for Int64 {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            hi: i32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            lo: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Systemcounter {
    pub counter: Counter,
    pub resolution: Int64,
    pub name: alloc::vec::Vec<u8>,
}
impl VariableLengthSerialize for Systemcounter {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let name_len =
            u16::try_from(self.name.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.counter.serialize_fixed());
        buf_ptr
            .get_mut(4..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.resolution.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(name_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(14..)
                .ok_or(crate::error::Error::Serialize)?,
            &self.name,
        )?;
        let mut offset = list_len + 14;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for Systemcounter {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let counter = Counter::from_bytes(ptr.get(0..4).ok_or(crate::error::Error::FromBytes)?)?;
        let resolution = Int64::from_bytes(ptr.get(4..12).ok_or(crate::error::Error::FromBytes)?)?;
        let name_len = u16::from_bytes(ptr.get(12..14).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = name_len as usize;
        let name: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(14..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr + 14;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok((
            Self {
                counter,
                resolution,
                name,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Trigger {
    pub counter: Counter,
    pub wait_type: ValuetypeEnum,
    pub wait_value: Int64,
    pub test_type: TesttypeEnum,
}
impl FixedLengthSerialize<20> for Trigger {
    #[inline]
    fn serialize_fixed(self) -> [u8; 20] {
        let counter_bytes = self.counter.serialize_fixed();
        let wait_type_bytes = self.wait_type.serialize_fixed();
        let wait_value_bytes = self.wait_value.serialize_fixed();
        let test_type_bytes = self.test_type.serialize_fixed();
        [
            counter_bytes[0],
            counter_bytes[1],
            counter_bytes[2],
            counter_bytes[3],
            wait_type_bytes[0],
            wait_type_bytes[1],
            wait_type_bytes[2],
            wait_type_bytes[3],
            wait_value_bytes[0],
            wait_value_bytes[1],
            wait_value_bytes[2],
            wait_value_bytes[3],
            wait_value_bytes[4],
            wait_value_bytes[5],
            wait_value_bytes[6],
            wait_value_bytes[7],
            test_type_bytes[0],
            test_type_bytes[1],
            test_type_bytes[2],
            test_type_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<20> for Trigger {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            counter: Counter::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            wait_type: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
            wait_value: Int64::from_bytes(bytes.get(8..16).ok_or(crate::error::Error::FromBytes)?)?,
            test_type: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?
                .into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Waitcondition {
    pub trigger: Trigger,
    pub event_threshold: Int64,
}
impl FixedLengthSerialize<28> for Waitcondition {
    #[inline]
    fn serialize_fixed(self) -> [u8; 28] {
        let trigger_bytes = self.trigger.serialize_fixed();
        let event_threshold_bytes = self.event_threshold.serialize_fixed();
        [
            trigger_bytes[0],
            trigger_bytes[1],
            trigger_bytes[2],
            trigger_bytes[3],
            trigger_bytes[4],
            trigger_bytes[5],
            trigger_bytes[6],
            trigger_bytes[7],
            trigger_bytes[8],
            trigger_bytes[9],
            trigger_bytes[10],
            trigger_bytes[11],
            trigger_bytes[12],
            trigger_bytes[13],
            trigger_bytes[14],
            trigger_bytes[15],
            trigger_bytes[16],
            trigger_bytes[17],
            trigger_bytes[18],
            trigger_bytes[19],
            event_threshold_bytes[0],
            event_threshold_bytes[1],
            event_threshold_bytes[2],
            event_threshold_bytes[3],
            event_threshold_bytes[4],
            event_threshold_bytes[5],
            event_threshold_bytes[6],
            event_threshold_bytes[7],
        ]
    }
}
impl FixedLengthFromBytes<28> for Waitcondition {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            trigger: Trigger::from_bytes(bytes.get(0..20).ok_or(crate::error::Error::FromBytes)?)?,
            event_threshold: Int64::from_bytes(
                bytes.get(20..28).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
pub const COUNTER_ERROR: u8 = 0;
pub const ALARM_ERROR: u8 = 1;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct InitializeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u8,
    pub minor_version: u8,
}
impl FixedLengthFromBytes<32> for InitializeReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            major_version: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
            minor_version: u8::from_bytes(bytes.get(9..10).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListSystemCountersReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub counters: alloc::vec::Vec<Systemcounter>,
}
impl VariableLengthFromBytes for ListSystemCountersReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let counters_len = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let counters_length = counters_len as usize;
        let mut offset = 32;
        let counters = crate::vec_from_bytes_var!(ptr, Systemcounter, offset, counters_length,);
        Ok((
            Self {
                response_type,
                sequence,
                length,
                counters,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryCounterReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub counter_value: Int64,
}
impl FixedLengthFromBytes<16> for QueryCounterReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            counter_value: Int64::from_bytes(
                bytes.get(8..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct CreateAlarmValueList {
    pub counter: Option<Counter>,
    pub value_type: Option<ValuetypeEnum>,
    pub value: Option<Int64>,
    pub test_type: Option<TesttypeEnum>,
    pub delta: Option<Int64>,
    pub events: Option<u32>,
}
impl CreateAlarmValueList {
    #[inline]
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let mut offset = 0;
        let buf_ptr = buf;
        if let Some(counter) = self.counter {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&counter.serialize_fixed());
            offset += 4;
        }
        if let Some(value_type) = self.value_type {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&value_type.serialize_fixed());
            offset += 4;
        }
        if let Some(value) = self.value {
            buf_ptr
                .get_mut(offset..offset + 8)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&value.serialize_fixed());
            offset += 8;
        }
        if let Some(test_type) = self.test_type {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&test_type.serialize_fixed());
            offset += 4;
        }
        if let Some(delta) = self.delta {
            buf_ptr
                .get_mut(offset..offset + 8)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&delta.serialize_fixed());
            offset += 8;
        }
        if let Some(events) = self.events {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&events.serialize_fixed());
            offset += 4;
        }
        Ok(offset)
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> Ca {
        let mut mask = Ca::default();
        if self.counter.is_some() {
            mask |= Ca::COUNTER;
        }
        if self.value_type.is_some() {
            mask |= Ca::VALUE_TYPE;
        }
        if self.value.is_some() {
            mask |= Ca::VALUE;
        }
        if self.test_type.is_some() {
            mask |= Ca::TEST_TYPE;
        }
        if self.delta.is_some() {
            mask |= Ca::DELTA;
        }
        if self.events.is_some() {
            mask |= Ca::EVENTS;
        }
        mask
    }

    #[inline]
    pub fn counter(mut self, counter: Counter) -> Self {
        self.counter = Some(counter);
        self
    }

    #[inline]
    pub fn value_type(mut self, value_type: ValuetypeEnum) -> Self {
        self.value_type = Some(value_type);
        self
    }

    #[inline]
    pub fn value(mut self, value: Int64) -> Self {
        self.value = Some(value);
        self
    }

    #[inline]
    pub fn test_type(mut self, test_type: TesttypeEnum) -> Self {
        self.test_type = Some(test_type);
        self
    }

    #[inline]
    pub fn delta(mut self, delta: Int64) -> Self {
        self.delta = Some(delta);
        self
    }

    #[inline]
    pub fn events(mut self, events: u32) -> Self {
        self.events = Some(events);
        self
    }
}
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct ChangeAlarmValueList {
    pub counter: Option<Counter>,
    pub value_type: Option<ValuetypeEnum>,
    pub value: Option<Int64>,
    pub test_type: Option<TesttypeEnum>,
    pub delta: Option<Int64>,
    pub events: Option<u32>,
}
impl ChangeAlarmValueList {
    #[inline]
    pub fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let mut offset = 0;
        let buf_ptr = buf;
        if let Some(counter) = self.counter {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&counter.serialize_fixed());
            offset += 4;
        }
        if let Some(value_type) = self.value_type {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&value_type.serialize_fixed());
            offset += 4;
        }
        if let Some(value) = self.value {
            buf_ptr
                .get_mut(offset..offset + 8)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&value.serialize_fixed());
            offset += 8;
        }
        if let Some(test_type) = self.test_type {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&test_type.serialize_fixed());
            offset += 4;
        }
        if let Some(delta) = self.delta {
            buf_ptr
                .get_mut(offset..offset + 8)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&delta.serialize_fixed());
            offset += 8;
        }
        if let Some(events) = self.events {
            buf_ptr
                .get_mut(offset..offset + 4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&events.serialize_fixed());
            offset += 4;
        }
        Ok(offset)
    }

    #[inline]
    #[must_use]
    pub fn switch_expr(&self) -> Ca {
        let mut mask = Ca::default();
        if self.counter.is_some() {
            mask |= Ca::COUNTER;
        }
        if self.value_type.is_some() {
            mask |= Ca::VALUE_TYPE;
        }
        if self.value.is_some() {
            mask |= Ca::VALUE;
        }
        if self.test_type.is_some() {
            mask |= Ca::TEST_TYPE;
        }
        if self.delta.is_some() {
            mask |= Ca::DELTA;
        }
        if self.events.is_some() {
            mask |= Ca::EVENTS;
        }
        mask
    }

    #[inline]
    pub fn counter(mut self, counter: Counter) -> Self {
        self.counter = Some(counter);
        self
    }

    #[inline]
    pub fn value_type(mut self, value_type: ValuetypeEnum) -> Self {
        self.value_type = Some(value_type);
        self
    }

    #[inline]
    pub fn value(mut self, value: Int64) -> Self {
        self.value = Some(value);
        self
    }

    #[inline]
    pub fn test_type(mut self, test_type: TesttypeEnum) -> Self {
        self.test_type = Some(test_type);
        self
    }

    #[inline]
    pub fn delta(mut self, delta: Int64) -> Self {
        self.delta = Some(delta);
        self
    }

    #[inline]
    pub fn events(mut self, events: u32) -> Self {
        self.events = Some(events);
        self
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryAlarmReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub trigger: Trigger,
    pub delta: Int64,
    pub events: u8,
    pub state: AlarmstateEnum,
}
impl FixedLengthFromBytes<40> for QueryAlarmReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            trigger: Trigger::from_bytes(bytes.get(8..28).ok_or(crate::error::Error::FromBytes)?)?,
            delta: Int64::from_bytes(bytes.get(28..36).ok_or(crate::error::Error::FromBytes)?)?,
            events: u8::from_bytes(bytes.get(36..37).ok_or(crate::error::Error::FromBytes)?)?,
            state: u8::from_bytes(bytes.get(37..38).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetPriorityReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub priority: i32,
}
impl FixedLengthFromBytes<12> for GetPriorityReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            priority: i32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryFenceReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub triggered: u8,
}
impl FixedLengthFromBytes<32> for QueryFenceReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            triggered: u8::from_bytes(bytes.get(8..9).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const COUNTER_NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CounterNotifyEvent {
    pub opcode: u8,
    pub kind: u8,
    pub sequence: u16,
    pub counter: Counter,
    pub wait_value: Int64,
    pub counter_value: Int64,
    pub timestamp: crate::proto::xproto::Timestamp,
    pub count: u16,
    pub destroyed: u8,
}
impl FixedLengthFromBytes<32> for CounterNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            kind: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            counter: Counter::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            wait_value: Int64::from_bytes(bytes.get(8..16).ok_or(crate::error::Error::FromBytes)?)?,
            counter_value: Int64::from_bytes(
                bytes.get(16..24).ok_or(crate::error::Error::FromBytes)?,
            )?,
            timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?,
            )?,
            count: u16::from_bytes(bytes.get(28..30).ok_or(crate::error::Error::FromBytes)?)?,
            destroyed: u8::from_bytes(bytes.get(30..31).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub const ALARM_NOTIFY_EVENT: u8 = 1;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct AlarmNotifyEvent {
    pub opcode: u8,
    pub kind: u8,
    pub sequence: u16,
    pub alarm: Alarm,
    pub counter_value: Int64,
    pub alarm_value: Int64,
    pub timestamp: crate::proto::xproto::Timestamp,
    pub state: AlarmstateEnum,
}
impl FixedLengthFromBytes<32> for AlarmNotifyEvent {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            opcode: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            kind: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            alarm: Alarm::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            counter_value: Int64::from_bytes(
                bytes.get(8..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            alarm_value: Int64::from_bytes(
                bytes.get(16..24).ok_or(crate::error::Error::FromBytes)?,
            )?,
            timestamp: crate::proto::xproto::Timestamp::from_bytes(
                bytes.get(24..28).ok_or(crate::error::Error::FromBytes)?,
            )?,
            state: u8::from_bytes(bytes.get(28..29).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
