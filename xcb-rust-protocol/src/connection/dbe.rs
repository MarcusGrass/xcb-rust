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
pub trait DbeConnection {
    fn query_version(
        &mut self,
        major_version: u8,
        minor_version: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dbe::QueryVersionReply, 32>>;

    fn allocate_back_buffer(
        &mut self,
        window: crate::proto::xproto::Window,
        buffer: crate::proto::dbe::BackBuffer,
        swap_action: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn deallocate_back_buffer(
        &mut self,
        buffer: crate::proto::dbe::BackBuffer,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn swap_buffers(
        &mut self,
        actions: &[crate::proto::dbe::SwapInfo],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn begin_idiom(&mut self, forget: bool) -> crate::error::Result<VoidCookie>;

    fn end_idiom(&mut self, forget: bool) -> crate::error::Result<VoidCookie>;

    fn get_visual_info(
        &mut self,
        drawables: &[crate::proto::xproto::Drawable],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::dbe::GetVisualInfoReply>>;

    fn get_back_buffer_attributes(
        &mut self,
        buffer: crate::proto::dbe::BackBuffer,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dbe::GetBackBufferAttributesReply, 32>>;
}
impl<C> DbeConnection for C
where
    C: crate::con::XcbConnection,
{
    fn query_version(
        &mut self,
        major_version: u8,
        minor_version: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dbe::QueryVersionReply, 32>> {
        let major_opcode = self.major_opcode(crate::proto::dbe::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::dbe::EXTENSION_NAME),
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
                major_version,
                minor_version,
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

    fn allocate_back_buffer(
        &mut self,
        window: crate::proto::xproto::Window,
        buffer: crate::proto::dbe::BackBuffer,
        swap_action: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::dbe::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::dbe::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buffer_bytes = buffer.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                1,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                buffer_bytes[0],
                buffer_bytes[1],
                buffer_bytes[2],
                buffer_bytes[3],
                swap_action,
                0,
                0,
                0,
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn deallocate_back_buffer(
        &mut self,
        buffer: crate::proto::dbe::BackBuffer,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::dbe::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::dbe::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buffer_bytes = buffer.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                2,
                length[0],
                length[1],
                buffer_bytes[0],
                buffer_bytes[1],
                buffer_bytes[2],
                buffer_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn swap_buffers(
        &mut self,
        actions: &[crate::proto::dbe::SwapInfo],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::dbe::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::dbe::EXTENSION_NAME),
        )?;
        let buf_ptr = self.write_buf();
        let n_actions = u32::try_from(actions.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(n_actions).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = actions.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            actions,
        )?;
        let mut offset = list_len + 8;
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
        Ok(VoidCookie::new(seq))
    }

    fn begin_idiom(&mut self, forget: bool) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::dbe::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::dbe::EXTENSION_NAME),
        )?;
        let buf = self
            .write_buf()
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 4;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn end_idiom(&mut self, forget: bool) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::dbe::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::dbe::EXTENSION_NAME),
        )?;
        let buf = self
            .write_buf()
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 5;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_visual_info(
        &mut self,
        drawables: &[crate::proto::xproto::Drawable],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::dbe::GetVisualInfoReply>> {
        let major_opcode = self.major_opcode(crate::proto::dbe::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::dbe::EXTENSION_NAME),
        )?;
        let buf_ptr = self.write_buf();
        let n_drawables =
            u32::try_from(drawables.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(n_drawables).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = drawables.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            drawables,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 6]);
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

    fn get_back_buffer_attributes(
        &mut self,
        buffer: crate::proto::dbe::BackBuffer,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dbe::GetBackBufferAttributesReply, 32>>
    {
        let major_opcode = self.major_opcode(crate::proto::dbe::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::dbe::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buffer_bytes = buffer.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                7,
                length[0],
                length[1],
                buffer_bytes[0],
                buffer_bytes[1],
                buffer_bytes[2],
                buffer_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }
}
