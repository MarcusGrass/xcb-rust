#[allow(unused_imports)]
use crate::util::FixedLengthFromBytes;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthFromBytes;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub const EXTENSION_NAME: &str = "SELinux";
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
pub struct GetDeviceCreateContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetDeviceCreateContextReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let context_len = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = context_len as usize;
        let context: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                context,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetDeviceContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetDeviceContextReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let context_len = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = context_len as usize;
        let context: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                context,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetWindowCreateContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetWindowCreateContextReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let context_len = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = context_len as usize;
        let context: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                context,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetWindowContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetWindowContextReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let context_len = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = context_len as usize;
        let context: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                context,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    pub name: u32,
    pub object_context: alloc::vec::Vec<u8>,
    pub data_context: alloc::vec::Vec<u8>,
}
impl VariableLengthSerialize for ListItem {
    fn serialize_into(self, buf: &mut [u8]) -> crate::error::Result<usize> {
        let buf_ptr = buf;
        let object_context_len =
            u32::try_from(self.object_context.len()).map_err(|_| crate::error::Error::Serialize)?;
        let data_context_len =
            u32::try_from(self.data_context.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(0..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&self.name.serialize_fixed());
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(object_context_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(data_context_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = self.object_context.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            &self.object_context,
        )?;
        let mut offset = list_len + 12;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let list_len = self.data_context.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            &self.data_context,
        )?;
        offset += list_len;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok(offset)
    }
}
impl VariableLengthFromBytes for ListItem {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        let name = u32::from_bytes(ptr.get(0..4).ok_or(crate::error::Error::FromBytes)?)?;
        let object_context_len =
            u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let data_context_len =
            u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = object_context_len as usize;
        let object_context: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(12..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let mut offset = length_expr + 12;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        let length_expr = data_context_len as usize;
        let data_context: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(offset..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        offset += length_expr;
        // Align 4 bytes
        offset += (4 - (offset % 4)) % 4;
        Ok((
            Self {
                name,
                object_context,
                data_context,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetPropertyCreateContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetPropertyCreateContextReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let context_len = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = context_len as usize;
        let context: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                context,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetPropertyUseContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetPropertyUseContextReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let context_len = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = context_len as usize;
        let context: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                context,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetPropertyContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetPropertyContextReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let context_len = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = context_len as usize;
        let context: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                context,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetPropertyDataContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetPropertyDataContextReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let context_len = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = context_len as usize;
        let context: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                context,
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
    pub properties: alloc::vec::Vec<ListItem>,
}
impl VariableLengthFromBytes for ListPropertiesReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let properties_len =
            u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let properties_length = properties_len as usize;
        let mut offset = 32;
        let properties = crate::vec_from_bytes_var!(ptr, ListItem, offset, properties_length,);
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
pub struct GetSelectionCreateContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetSelectionCreateContextReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let context_len = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = context_len as usize;
        let context: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                context,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetSelectionUseContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetSelectionUseContextReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let context_len = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = context_len as usize;
        let context: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                context,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetSelectionContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetSelectionContextReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let context_len = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = context_len as usize;
        let context: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                context,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetSelectionDataContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetSelectionDataContextReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let context_len = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = context_len as usize;
        let context: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                context,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListSelectionsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub selections: alloc::vec::Vec<ListItem>,
}
impl VariableLengthFromBytes for ListSelectionsReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let selections_len =
            u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let selections_length = selections_len as usize;
        let mut offset = 32;
        let selections = crate::vec_from_bytes_var!(ptr, ListItem, offset, selections_length,);
        Ok((
            Self {
                response_type,
                sequence,
                length,
                selections,
            },
            offset,
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetClientContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: alloc::vec::Vec<u8>,
}
impl VariableLengthFromBytes for GetClientContextReply {
    fn from_bytes(bytes: &[u8]) -> crate::error::Result<(Self, usize)> {
        let ptr = bytes;
        // Padding 1 bytes
        // Padding 20 bytes
        let response_type = u8::from_bytes(ptr.get(0..1).ok_or(crate::error::Error::FromBytes)?)?;
        let sequence = u16::from_bytes(ptr.get(2..4).ok_or(crate::error::Error::FromBytes)?)?;
        let length = u32::from_bytes(ptr.get(4..8).ok_or(crate::error::Error::FromBytes)?)?;
        let context_len = u32::from_bytes(ptr.get(8..12).ok_or(crate::error::Error::FromBytes)?)?;
        let length_expr = context_len as usize;
        let context: alloc::vec::Vec<u8> = crate::util::u8_vec_from_bytes(
            ptr.get(32..).ok_or(crate::error::Error::FromBytes)?,
            length_expr,
        )?;
        let offset = length_expr + 32;
        Ok((
            Self {
                response_type,
                sequence,
                length,
                context,
            },
            offset,
        ))
    }
}
