#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "RECORD";
pub type Context = u32;
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Range8 {
    pub first: u8,
    pub last: u8,
}
impl FixedLengthSerialize<2> for Range8 {
    #[inline]
    fn serialize_fixed(self) -> [u8; 2] {
        [self.first, self.last]
    }
}
impl FixedLengthFromBytes<2> for Range8 {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            first: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            last: u8::from_bytes(bytes.get(1..2).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Range16 {
    pub first: u16,
    pub last: u16,
}
impl FixedLengthSerialize<4> for Range16 {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        let first_bytes = self.first.serialize_fixed();
        let last_bytes = self.last.serialize_fixed();
        [first_bytes[0], first_bytes[1], last_bytes[0], last_bytes[1]]
    }
}
impl FixedLengthFromBytes<4> for Range16 {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            first: u16::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            last: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ExtRange {
    pub major: Range8,
    pub minor: Range16,
}
impl FixedLengthSerialize<6> for ExtRange {
    #[inline]
    fn serialize_fixed(self) -> [u8; 6] {
        let major_bytes = self.major.serialize_fixed();
        let minor_bytes = self.minor.serialize_fixed();
        [
            major_bytes[0],
            major_bytes[1],
            minor_bytes[0],
            minor_bytes[1],
            minor_bytes[2],
            minor_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<6> for ExtRange {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            major: Range8::from_bytes(bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?)?,
            minor: Range16::from_bytes(bytes.get(2..6).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Range {
    pub core_requests: Range8,
    pub core_replies: Range8,
    pub ext_requests: ExtRange,
    pub ext_replies: ExtRange,
    pub delivered_events: Range8,
    pub device_events: Range8,
    pub errors: Range8,
    pub client_started: u8,
    pub client_died: u8,
}
impl FixedLengthSerialize<24> for Range {
    #[inline]
    fn serialize_fixed(self) -> [u8; 24] {
        let core_requests_bytes = self.core_requests.serialize_fixed();
        let core_replies_bytes = self.core_replies.serialize_fixed();
        let ext_requests_bytes = self.ext_requests.serialize_fixed();
        let ext_replies_bytes = self.ext_replies.serialize_fixed();
        let delivered_events_bytes = self.delivered_events.serialize_fixed();
        let device_events_bytes = self.device_events.serialize_fixed();
        let errors_bytes = self.errors.serialize_fixed();
        [
            core_requests_bytes[0],
            core_requests_bytes[1],
            core_replies_bytes[0],
            core_replies_bytes[1],
            ext_requests_bytes[0],
            ext_requests_bytes[1],
            ext_requests_bytes[2],
            ext_requests_bytes[3],
            ext_requests_bytes[4],
            ext_requests_bytes[5],
            ext_replies_bytes[0],
            ext_replies_bytes[1],
            ext_replies_bytes[2],
            ext_replies_bytes[3],
            ext_replies_bytes[4],
            ext_replies_bytes[5],
            delivered_events_bytes[0],
            delivered_events_bytes[1],
            device_events_bytes[0],
            device_events_bytes[1],
            errors_bytes[0],
            errors_bytes[1],
            self.client_started,
            self.client_died,
        ]
    }
}
impl FixedLengthFromBytes<24> for Range {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            core_requests: Range8::from_bytes(
                bytes.get(0..2).ok_or(crate::error::Error::FromBytes)?,
            )?,
            core_replies: Range8::from_bytes(
                bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?,
            )?,
            ext_requests: ExtRange::from_bytes(
                bytes.get(4..10).ok_or(crate::error::Error::FromBytes)?,
            )?,
            ext_replies: ExtRange::from_bytes(
                bytes.get(10..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
            delivered_events: Range8::from_bytes(
                bytes.get(16..18).ok_or(crate::error::Error::FromBytes)?,
            )?,
            device_events: Range8::from_bytes(
                bytes.get(18..20).ok_or(crate::error::Error::FromBytes)?,
            )?,
            errors: Range8::from_bytes(bytes.get(20..22).ok_or(crate::error::Error::FromBytes)?)?,
            client_started: u8::from_bytes(
                bytes.get(22..23).ok_or(crate::error::Error::FromBytes)?,
            )?,
            client_died: u8::from_bytes(bytes.get(23..24).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
pub type ElementHeader = u8;
pub type ClientSpec = u32;
#[derive(Debug, Clone, PartialEq)]
pub struct ClientInfo {
    pub client_resource: ClientSpec,
    pub ranges: alloc::vec::Vec<Range>,
}
impl VariableLengthSerialize for ClientInfo {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let num_ranges =
            u32::try_from(self.ranges.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.client_resource.serialize_fixed());
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(num_ranges).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.ranges.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            self.ranges,
        )?;
        let offset = list_len + 8;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for ClientInfo {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let client_resource =
            ClientSpec::from_bytes(ptr.get(0..4).ok_or(crate::error::Error::FromBytes)?)?;
        let num_ranges = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_ranges as usize;
        let ranges: alloc::vec::Vec<Range> = crate::vec_from_bytes_fixed!(
            ptr.get(8..).ok_or(crate::error::Error::FromBytes)?,
            Range,
            length_expr,
            24
        );
        let offset = length_expr * 24 + 8;
        Ok((
            Self {
                client_resource,
                ranges,
            },
            offset,
        ))
    }
}
pub const BAD_CONTEXT_ERROR: u8 = 0;
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
pub struct GetContextReply {
    pub response_type: u8,
    pub enabled: u8,
    pub sequence: u16,
    pub length: u32,
    pub element_header: ElementHeader,
    pub intercepted_clients: alloc::vec::Vec<ClientInfo>,
}
impl VariableLengthFromBytes for GetContextReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 3 bytes
        // Padding 16 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let enabled = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let element_header =
            ElementHeader::from_bytes(ptr.get(8..9).ok_or(crate::error::Error::FromBytes)?)?;
        let num_intercepted_clients =
            u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let intercepted_clients_length = num_intercepted_clients as usize;
        let mut offset = 32;
        let intercepted_clients =
            crate::vec_from_bytes_var!(ptr, ClientInfo, offset, intercepted_clients_length,);
        Ok((
            Self {
                response_type,
                enabled,
                sequence,
                length,
                element_header,
                intercepted_clients,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct EnableContextReply {
    pub response_type: u8,
    pub category: u8,
    pub sequence: u16,
    pub length: u32,
    pub element_header: ElementHeader,
    pub client_swapped: u8,
    pub xid_base: u32,
    pub server_time: u32,
    pub rec_sequence_num: u32,
    pub data: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for EnableContextReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 2 bytes
        // Padding 8 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let category = u8::from_bytes(ptr.get(1..2).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let element_header =
            ElementHeader::from_bytes(ptr.get(8..9).ok_or(crate::error::Error::FromBytes)?)?;
        let client_swapped = u8::from_bytes(ptr.get(9..10).ok_or(crate::error::Error::FromBytes)?)?;
        let xid_base = u32::from_bytes(ptr.get(12..16).ok_or(crate::error::Error::FromBytes)?)?;
        let server_time = u32::from_bytes(ptr.get(16..20).ok_or(crate::error::Error::FromBytes)?)?;
        let rec_sequence_num =
            u32::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Mul::mul(length, 4) as usize;
        let data: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                category,
                sequence,
                length,
                element_header,
                client_swapped,
                xid_base,
                server_time,
                rec_sequence_num,
                data,
            },
            offset,
        ))
    }
}
