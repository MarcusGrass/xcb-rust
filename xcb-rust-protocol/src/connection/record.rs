#[allow(unused_imports)]
use crate::cookie::Cookie;
#[allow(unused_imports)]
use crate::cookie::FixedCookie;
#[allow(unused_imports)]
use crate::cookie::VoidCookie;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub trait RecordConnection {
    fn query_version(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u16,
        minor_version: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::record::QueryVersionReply, 12>>;

    fn create_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::record::Context,
        element_header: crate::proto::record::ElementHeader,
        num_client_specs: u32,
        num_ranges: u32,
        client_specs: &[crate::proto::record::ClientSpec],
        ranges: &[crate::proto::record::Range],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn register_clients(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::record::Context,
        element_header: crate::proto::record::ElementHeader,
        num_client_specs: u32,
        num_ranges: u32,
        client_specs: &[crate::proto::record::ClientSpec],
        ranges: &[crate::proto::record::Range],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn unregister_clients(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::record::Context,
        client_specs: &[crate::proto::record::ClientSpec],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::record::Context,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::record::GetContextReply>>;

    fn enable_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::record::Context,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::record::EnableContextReply>>;

    fn disable_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::record::Context,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn free_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::record::Context,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;
}
impl<C> RecordConnection for C
where
    C: crate::con::XcbConnection,
{
    fn query_version(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u16,
        minor_version: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::record::QueryVersionReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::record::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::record::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let major_version_bytes = major_version.serialize_fixed();
        let minor_version_bytes = minor_version.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                0,
                length[0],
                length[1],
                major_version_bytes[0],
                major_version_bytes[1],
                minor_version_bytes[0],
                minor_version_bytes[1],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn create_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::record::Context,
        element_header: crate::proto::record::ElementHeader,
        num_client_specs: u32,
        num_ranges: u32,
        client_specs: &[crate::proto::record::ClientSpec],
        ranges: &[crate::proto::record::Range],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::record::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::record::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 3 bytes
        let num_client_specs =
            u32::try_from(num_client_specs).map_err(|_| crate::error::Error::Serialize)?;
        let num_ranges = u32::try_from(num_ranges).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context.serialize_fixed());
        buf_ptr
            .get_mut(8..9)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&element_header.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(num_client_specs).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(num_ranges).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = client_specs.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(20..)
                .ok_or(crate::error::Error::Serialize)?,
            client_specs,
        )?;
        let mut offset = list_len + 20;
        let list_len = ranges.len() * 24;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            ranges,
        )?;
        offset += list_len;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 1]);
        debug_assert!(offset % 4 == 0, "Bad request length offset {offset}");
        let word_len = offset / 4;
        if let Ok(len) = u16::try_from(word_len) {
            let length: [u8; 2] = len.to_ne_bytes();
            buf_ptr
                .get_mut(2..4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&length);
        } else {
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
            buf_ptr
                .get_mut(2..4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&[0, 0]);
            let length: [u8; 4] = u32::try_from(word_len)
                .map_err(|_| crate::error::Error::TooLargeRequest)?
                .checked_add(1)
                .ok_or(crate::error::Error::TooLargeRequest)?
                .to_ne_bytes();
            buf_ptr.copy_within(4..offset, 8);
            buf_ptr
                .get_mut(4..8)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&length);
            offset += 4;
        }
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn register_clients(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::record::Context,
        element_header: crate::proto::record::ElementHeader,
        num_client_specs: u32,
        num_ranges: u32,
        client_specs: &[crate::proto::record::ClientSpec],
        ranges: &[crate::proto::record::Range],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::record::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::record::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 3 bytes
        let num_client_specs =
            u32::try_from(num_client_specs).map_err(|_| crate::error::Error::Serialize)?;
        let num_ranges = u32::try_from(num_ranges).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context.serialize_fixed());
        buf_ptr
            .get_mut(8..9)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&element_header.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(num_client_specs).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(num_ranges).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = client_specs.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(20..)
                .ok_or(crate::error::Error::Serialize)?,
            client_specs,
        )?;
        let mut offset = list_len + 20;
        let list_len = ranges.len() * 24;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            ranges,
        )?;
        offset += list_len;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 2]);
        debug_assert!(offset % 4 == 0, "Bad request length offset {offset}");
        let word_len = offset / 4;
        if let Ok(len) = u16::try_from(word_len) {
            let length: [u8; 2] = len.to_ne_bytes();
            buf_ptr
                .get_mut(2..4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&length);
        } else {
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
            buf_ptr
                .get_mut(2..4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&[0, 0]);
            let length: [u8; 4] = u32::try_from(word_len)
                .map_err(|_| crate::error::Error::TooLargeRequest)?
                .checked_add(1)
                .ok_or(crate::error::Error::TooLargeRequest)?
                .to_ne_bytes();
            buf_ptr.copy_within(4..offset, 8);
            buf_ptr
                .get_mut(4..8)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&length);
            offset += 4;
        }
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn unregister_clients(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::record::Context,
        client_specs: &[crate::proto::record::ClientSpec],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::record::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::record::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_client_specs =
            u32::try_from(client_specs.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(num_client_specs).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = client_specs.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            client_specs,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 3]);
        debug_assert!(offset % 4 == 0, "Bad request length offset {offset}");
        let word_len = offset / 4;
        if let Ok(len) = u16::try_from(word_len) {
            let length: [u8; 2] = len.to_ne_bytes();
            buf_ptr
                .get_mut(2..4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&length);
        } else {
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
            buf_ptr
                .get_mut(2..4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&[0, 0]);
            let length: [u8; 4] = u32::try_from(word_len)
                .map_err(|_| crate::error::Error::TooLargeRequest)?
                .checked_add(1)
                .ok_or(crate::error::Error::TooLargeRequest)?
                .to_ne_bytes();
            buf_ptr.copy_within(4..offset, 8);
            buf_ptr
                .get_mut(4..8)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&length);
            offset += 4;
        }
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::record::Context,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::record::GetContextReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::record::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::record::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_bytes = context.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                4,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn enable_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::record::Context,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::record::EnableContextReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::record::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::record::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_bytes = context.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                5,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn disable_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::record::Context,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::record::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::record::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_bytes = context.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                6,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn free_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::record::Context,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::record::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::record::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_bytes = context.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                7,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }
}
