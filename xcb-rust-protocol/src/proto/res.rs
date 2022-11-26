#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "X-Resource";
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Client {
    pub resource_base: u32,
    pub resource_mask: u32,
}
impl FixedLengthSerialize<8> for Client {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let resource_base_bytes = self.resource_base.serialize_fixed();
        let resource_mask_bytes = self.resource_mask.serialize_fixed();
        [
            resource_base_bytes[0],
            resource_base_bytes[1],
            resource_base_bytes[2],
            resource_base_bytes[3],
            resource_mask_bytes[0],
            resource_mask_bytes[1],
            resource_mask_bytes[2],
            resource_mask_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for Client {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            resource_base: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            resource_mask: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Type {
    pub resource_type: u32,
    pub count: u32,
}
impl FixedLengthSerialize<8> for Type {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let resource_type_bytes = self.resource_type.serialize_fixed();
        let count_bytes = self.count.serialize_fixed();
        [
            resource_type_bytes[0],
            resource_type_bytes[1],
            resource_type_bytes[2],
            resource_type_bytes[3],
            count_bytes[0],
            count_bytes[1],
            count_bytes[2],
            count_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for Type {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            resource_type: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            count: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ClientIdMask(pub u32);
impl ClientIdMask {
    pub const CLIENT_X_I_D: Self = Self(1 << 0);
    pub const LOCAL_CLIENT_P_I_D: Self = Self(1 << 1);
}
impl FixedLengthSerialize<4> for ClientIdMask {
    #[inline]
    fn serialize_fixed(self) -> [u8; 4] {
        self.0.serialize_fixed()
    }
}
impl FixedLengthFromBytes<4> for ClientIdMask {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self(u32::from_bytes(bytes)?))
    }
}
impl From<u32> for ClientIdMask {
    #[inline]
    fn from(val: u32) -> Self {
        Self(val as u32)
    }
}
impl From<u16> for ClientIdMask {
    #[inline]
    fn from(val: u16) -> Self {
        Self(val as u32)
    }
}
impl From<u8> for ClientIdMask {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}
crate::implement_bit_ops!(ClientIdMask);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ClientIdSpec {
    pub client: u32,
    pub mask: ClientIdMask,
}
impl FixedLengthSerialize<8> for ClientIdSpec {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let client_bytes = self.client.serialize_fixed();
        let mask_bytes = self.mask.serialize_fixed();
        [
            client_bytes[0],
            client_bytes[1],
            client_bytes[2],
            client_bytes[3],
            mask_bytes[0],
            mask_bytes[1],
            mask_bytes[2],
            mask_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for ClientIdSpec {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            client: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            mask: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ClientIdValue {
    pub spec: ClientIdSpec,
    pub value: alloc::vec::Vec<u32>,
}
impl VariableLengthSerialize for ClientIdValue {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let length = u32::try_from(self.value.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.spec.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(core::ops::Mul::mul(length as u32, 4u32 as u32))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = self.value.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            self.value,
        )?;
        let offset = list_len + 12;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for ClientIdValue {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let spec = ClientIdSpec::from_bytes(ptr.get(0..8).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = core::ops::Div::div(length as u32, 4u32 as u32) as usize;
        let value: alloc::vec::Vec<u32> = crate::vec_from_bytes_fixed!(
            ptr.get(12..).ok_or(crate::error::Error::FromBytes)?,
            u32,
            length_expr,
            4
        );
        let offset = length_expr * 4 + 12;
        Ok((Self { spec, value }, offset))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ResourceIdSpec {
    pub resource: u32,
    pub r#type: u32,
}
impl FixedLengthSerialize<8> for ResourceIdSpec {
    #[inline]
    fn serialize_fixed(self) -> [u8; 8] {
        let resource_bytes = self.resource.serialize_fixed();
        let r#type_bytes = self.r#type.serialize_fixed();
        [
            resource_bytes[0],
            resource_bytes[1],
            resource_bytes[2],
            resource_bytes[3],
            r#type_bytes[0],
            r#type_bytes[1],
            r#type_bytes[2],
            r#type_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<8> for ResourceIdSpec {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            resource: u32::from_bytes(bytes.get(0..4).ok_or(crate::error::Error::FromBytes)?)?,
            r#type: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ResourceSizeSpec {
    pub spec: ResourceIdSpec,
    pub bytes: u32,
    pub ref_count: u32,
    pub use_count: u32,
}
impl FixedLengthSerialize<20> for ResourceSizeSpec {
    #[inline]
    fn serialize_fixed(self) -> [u8; 20] {
        let spec_bytes = self.spec.serialize_fixed();
        let bytes_bytes = self.bytes.serialize_fixed();
        let ref_count_bytes = self.ref_count.serialize_fixed();
        let use_count_bytes = self.use_count.serialize_fixed();
        [
            spec_bytes[0],
            spec_bytes[1],
            spec_bytes[2],
            spec_bytes[3],
            spec_bytes[4],
            spec_bytes[5],
            spec_bytes[6],
            spec_bytes[7],
            bytes_bytes[0],
            bytes_bytes[1],
            bytes_bytes[2],
            bytes_bytes[3],
            ref_count_bytes[0],
            ref_count_bytes[1],
            ref_count_bytes[2],
            ref_count_bytes[3],
            use_count_bytes[0],
            use_count_bytes[1],
            use_count_bytes[2],
            use_count_bytes[3],
        ]
    }
}
impl FixedLengthFromBytes<20> for ResourceSizeSpec {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            spec: ResourceIdSpec::from_bytes(
                bytes.get(0..8).ok_or(crate::error::Error::FromBytes)?,
            )?,
            bytes: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            ref_count: u32::from_bytes(bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?)?,
            use_count: u32::from_bytes(bytes.get(16..20).ok_or(crate::error::Error::FromBytes)?)?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceSizeValue {
    pub size: ResourceSizeSpec,
    pub cross_references: alloc::vec::Vec<ResourceSizeSpec>,
}
impl VariableLengthSerialize for ResourceSizeValue {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let num_cross_references = u32::try_from(self.cross_references.len())
            .map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.size.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(num_cross_references)
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = self.cross_references.len();
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            self.cross_references,
        )?;
        let offset = list_len + 24;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for ResourceSizeValue {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let size =
            ResourceSizeSpec::from_bytes(ptr.get(0..20).ok_or(crate::error::Error::FromBytes)?)?;
        let num_cross_references =
            u32::from_bytes(ptr.get(20..24).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_cross_references as usize;
        let cross_references: alloc::vec::Vec<ResourceSizeSpec> = crate::vec_from_bytes_fixed!(
            ptr.get(24..).ok_or(crate::error::Error::FromBytes)?,
            ResourceSizeSpec,
            length_expr,
            20
        );
        let offset = length_expr * 20 + 24;
        Ok((
            Self {
                size,
                cross_references,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
}
impl FixedLengthFromBytes<12> for QueryVersionReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            server_major: u16::from_bytes(bytes.get(8..10).ok_or(crate::error::Error::FromBytes)?)?,
            server_minor: u16::from_bytes(
                bytes.get(10..12).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryClientsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub clients: alloc::vec::Vec<Client>,
}
impl VariableLengthFromBytes for QueryClientsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_clients = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_clients as usize;
        let clients: alloc::vec::Vec<Client> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Client,
            length_expr,
            8
        );
        let offset = length_expr * 8 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                clients,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryClientResourcesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub types: alloc::vec::Vec<Type>,
}
impl VariableLengthFromBytes for QueryClientResourcesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_types = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = num_types as usize;
        let types: alloc::vec::Vec<Type> = crate::vec_from_bytes_fixed!(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            Type,
            length_expr,
            8
        );
        let offset = length_expr * 8 + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                types,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QueryClientPixmapBytesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub bytes: u32,
    pub bytes_overflow: u32,
}
impl FixedLengthFromBytes<16> for QueryClientPixmapBytesReply {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        Ok(Self {
            response_type: u8::from_bytes(bytes.get(0..1).ok_or(crate::error::Error::FromBytes)?)?,
            sequence: u16::from_bytes(bytes.get(2..4).ok_or(crate::error::Error::FromBytes)?)?,
            length: u32::from_bytes(bytes.get(4..8).ok_or(crate::error::Error::FromBytes)?)?,
            bytes: u32::from_bytes(bytes.get(8..12).ok_or(crate::error::Error::FromBytes)?)?,
            bytes_overflow: u32::from_bytes(
                bytes.get(12..16).ok_or(crate::error::Error::FromBytes)?,
            )?,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryClientIdsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ids: alloc::vec::Vec<ClientIdValue>,
}
impl VariableLengthFromBytes for QueryClientIdsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_ids = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let ids_length = num_ids as usize;
        let mut offset = 32;
        let ids = crate::vec_from_bytes_var!(ptr, ClientIdValue, offset, ids_length,);
        Ok((
            Self {
                response_type,
                sequence,
                length,
                ids,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryResourceBytesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub sizes: alloc::vec::Vec<ResourceSizeValue>,
}
impl VariableLengthFromBytes for QueryResourceBytesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let num_sizes = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let sizes_length = num_sizes as usize;
        let mut offset = 32;
        let sizes = crate::vec_from_bytes_var!(ptr, ResourceSizeValue, offset, sizes_length,);
        Ok((
            Self {
                response_type,
                sequence,
                length,
                sizes,
            },
            offset,
        ))
    }
}
