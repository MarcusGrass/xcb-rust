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
pub trait ResConnection {
    fn query_version(
        &mut self,
        client_major: u8,
        client_minor: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::res::QueryVersionReply, 12>>;

    fn query_clients(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::res::QueryClientsReply>>;

    fn query_client_resources(
        &mut self,
        xid: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::res::QueryClientResourcesReply>>;

    fn query_client_pixmap_bytes(
        &mut self,
        xid: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::res::QueryClientPixmapBytesReply, 16>>;

    fn query_client_ids(
        &mut self,
        specs: &[crate::proto::res::ClientIdSpec],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::res::QueryClientIdsReply>>;

    fn query_resource_bytes(
        &mut self,
        client: u32,
        specs: &[crate::proto::res::ResourceIdSpec],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::res::QueryResourceBytesReply>>;
}
impl<C> ResConnection for C
where
    C: crate::con::XcbConnection,
{
    fn query_version(
        &mut self,
        client_major: u8,
        client_minor: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::res::QueryVersionReply, 12>> {
        let major_opcode = self.major_opcode(crate::proto::res::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::res::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                0,
                length[0],
                length[1],
                client_major,
                client_minor,
                0,
                0,
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn query_clients(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::res::QueryClientsReply>> {
        let major_opcode = self.major_opcode(crate::proto::res::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::res::EXTENSION_NAME),
        )?;
        let buf = self
            .write_buf()
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 1;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn query_client_resources(
        &mut self,
        xid: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::res::QueryClientResourcesReply>> {
        let major_opcode = self.major_opcode(crate::proto::res::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::res::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let xid_bytes = xid.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                2,
                length[0],
                length[1],
                xid_bytes[0],
                xid_bytes[1],
                xid_bytes[2],
                xid_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn query_client_pixmap_bytes(
        &mut self,
        xid: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::res::QueryClientPixmapBytesReply, 16>> {
        let major_opcode = self.major_opcode(crate::proto::res::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::res::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let xid_bytes = xid.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
                length[0],
                length[1],
                xid_bytes[0],
                xid_bytes[1],
                xid_bytes[2],
                xid_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn query_client_ids(
        &mut self,
        specs: &[crate::proto::res::ClientIdSpec],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::res::QueryClientIdsReply>> {
        let major_opcode = self.major_opcode(crate::proto::res::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::res::EXTENSION_NAME),
        )?;
        let buf_ptr = self.write_buf();
        let num_specs = u32::try_from(specs.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(num_specs).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = specs.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            specs,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 4]);
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
            let buf_ptr = self.write_buf();
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
        Ok(Cookie::new(seq))
    }

    fn query_resource_bytes(
        &mut self,
        client: u32,
        specs: &[crate::proto::res::ResourceIdSpec],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::res::QueryResourceBytesReply>> {
        let major_opcode = self.major_opcode(crate::proto::res::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::res::EXTENSION_NAME),
        )?;
        let buf_ptr = self.write_buf();
        let num_specs = u32::try_from(specs.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&client.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(num_specs).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = specs.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            specs,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 5]);
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
            let buf_ptr = self.write_buf();
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
        Ok(Cookie::new(seq))
    }
}
