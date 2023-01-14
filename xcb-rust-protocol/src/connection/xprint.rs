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
pub trait XprintConnection {
    fn print_query_version(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xprint::PrintQueryVersionReply, 12>>;

    fn print_get_printer_list(
        &mut self,
        socket_buffer: &mut [u8],
        printer_name_len: u32,
        locale_len: u32,
        printer_name: &[crate::proto::xprint::String8],
        locale: &[crate::proto::xprint::String8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xprint::PrintGetPrinterListReply>>;

    fn print_rehash_printer_list(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn create_context(
        &mut self,
        socket_buffer: &mut [u8],
        context_id: u32,
        printer_name_len: u32,
        locale_len: u32,
        printer_name: &[crate::proto::xprint::String8],
        locale: &[crate::proto::xprint::String8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn print_set_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn print_get_context(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xprint::PrintGetContextReply, 12>>;

    fn print_destroy_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn print_get_screen_of_context(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xprint::PrintGetScreenOfContextReply, 12>>;

    fn print_start_job(
        &mut self,
        socket_buffer: &mut [u8],
        output_mode: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn print_end_job(
        &mut self,
        socket_buffer: &mut [u8],
        cancel: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn print_start_doc(
        &mut self,
        socket_buffer: &mut [u8],
        driver_mode: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn print_end_doc(
        &mut self,
        socket_buffer: &mut [u8],
        cancel: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn print_put_document_data(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        len_data: u32,
        len_fmt: u16,
        len_options: u16,
        data: &[u8],
        doc_format: &[crate::proto::xprint::String8],
        options: &[crate::proto::xprint::String8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn print_get_document_data(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        max_bytes: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xprint::PrintGetDocumentDataReply>>;

    fn print_start_page(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn print_end_page(
        &mut self,
        socket_buffer: &mut [u8],
        cancel: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn print_select_input(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        event_mask: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn print_input_selected(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xprint::PrintInputSelectedReply, 16>>;

    fn print_get_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        pool: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xprint::PrintGetAttributesReply>>;

    fn print_get_one_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        pool: u8,
        name: &[crate::proto::xprint::String8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xprint::PrintGetOneAttributesReply>>;

    fn print_set_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        string_len: u32,
        pool: u8,
        rule: u8,
        attributes: &[crate::proto::xprint::String8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn print_get_page_dimensions(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xprint::PrintGetPageDimensionsReply, 20>>;

    fn print_query_screens(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xprint::PrintQueryScreensReply>>;

    fn print_set_image_resolution(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        image_resolution: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xprint::PrintSetImageResolutionReply, 10>>;

    fn print_get_image_resolution(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xprint::PrintGetImageResolutionReply, 10>>;
}
impl<C> XprintConnection for C
where
    C: crate::con::XcbConnection,
{
    fn print_query_version(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xprint::PrintQueryVersionReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 0;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn print_get_printer_list(
        &mut self,
        socket_buffer: &mut [u8],
        printer_name_len: u32,
        locale_len: u32,
        printer_name: &[crate::proto::xprint::String8],
        locale: &[crate::proto::xprint::String8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xprint::PrintGetPrinterListReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let printer_name_len =
            u32::try_from(printer_name_len).map_err(|_| crate::error::Error::Serialize)?;
        let locale_len = u32::try_from(locale_len).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(printer_name_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(locale_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = printer_name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            printer_name,
        )?;
        let mut offset = list_len + 12;
        offset += (4 - (offset % 4)) % 4;
        let list_len = locale.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            locale,
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
        Ok(Cookie::new(seq))
    }

    fn print_rehash_printer_list(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 20;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn create_context(
        &mut self,
        socket_buffer: &mut [u8],
        context_id: u32,
        printer_name_len: u32,
        locale_len: u32,
        printer_name: &[crate::proto::xprint::String8],
        locale: &[crate::proto::xprint::String8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let printer_name_len =
            u32::try_from(printer_name_len).map_err(|_| crate::error::Error::Serialize)?;
        let locale_len = u32::try_from(locale_len).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context_id.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(printer_name_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(locale_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = printer_name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            printer_name,
        )?;
        let mut offset = list_len + 16;
        offset += (4 - (offset % 4)) % 4;
        let list_len = locale.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            locale,
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

    fn print_set_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_bytes = context.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
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

    fn print_get_context(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xprint::PrintGetContextReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let buf = self
            .apply_offset(socket_buffer)
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
        Ok(FixedCookie::new(seq))
    }

    fn print_destroy_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
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
        Ok(VoidCookie::new(seq))
    }

    fn print_get_screen_of_context(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xprint::PrintGetScreenOfContextReply, 12>>
    {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 6;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn print_start_job(
        &mut self,
        socket_buffer: &mut [u8],
        output_mode: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 7, length[0], length[1], output_mode, 0, 0, 0]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn print_end_job(
        &mut self,
        socket_buffer: &mut [u8],
        cancel: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 8, length[0], length[1], cancel, 0, 0, 0]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn print_start_doc(
        &mut self,
        socket_buffer: &mut [u8],
        driver_mode: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 9, length[0], length[1], driver_mode, 0, 0, 0]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn print_end_doc(
        &mut self,
        socket_buffer: &mut [u8],
        cancel: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 10, length[0], length[1], cancel, 0, 0, 0]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn print_put_document_data(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        len_data: u32,
        len_fmt: u16,
        len_options: u16,
        data: &[u8],
        doc_format: &[crate::proto::xprint::String8],
        options: &[crate::proto::xprint::String8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let len_data = u32::try_from(len_data).map_err(|_| crate::error::Error::Serialize)?;
        let len_fmt = u16::try_from(len_fmt).map_err(|_| crate::error::Error::Serialize)?;
        let len_options = u16::try_from(len_options).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(len_data).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(len_fmt).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(14..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(len_options).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = data.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            data,
        )?;
        let mut offset = list_len + 16;
        offset += (4 - (offset % 4)) % 4;
        let list_len = doc_format.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            doc_format,
        )?;
        offset += list_len;
        offset += (4 - (offset % 4)) % 4;
        let list_len = options.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            options,
        )?;
        offset += list_len;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 11]);
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

    fn print_get_document_data(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        max_bytes: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xprint::PrintGetDocumentDataReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_bytes = context.serialize_fixed();
        let max_bytes_bytes = max_bytes.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                12,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
                max_bytes_bytes[0],
                max_bytes_bytes[1],
                max_bytes_bytes[2],
                max_bytes_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn print_start_page(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                13,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn print_end_page(
        &mut self,
        socket_buffer: &mut [u8],
        cancel: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 14, length[0], length[1], cancel, 0, 0, 0]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn print_select_input(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        event_mask: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_bytes = context.serialize_fixed();
        let event_mask_bytes = event_mask.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                15,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
                event_mask_bytes[0],
                event_mask_bytes[1],
                event_mask_bytes[2],
                event_mask_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn print_input_selected(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xprint::PrintInputSelectedReply, 16>> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_bytes = context.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                16,
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
        Ok(FixedCookie::new(seq))
    }

    fn print_get_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        pool: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xprint::PrintGetAttributesReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_bytes = context.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                17,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
                pool,
                0,
                0,
                0,
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn print_get_one_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        pool: u8,
        name: &[crate::proto::xprint::String8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xprint::PrintGetOneAttributesReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let name_len = u32::try_from(name.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 3 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(name_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(12..13)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&pool.serialize_fixed());
        let list_len = name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            name,
        )?;
        let mut offset = list_len + 16;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 19]);
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
        Ok(Cookie::new(seq))
    }

    fn print_set_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        string_len: u32,
        pool: u8,
        rule: u8,
        attributes: &[crate::proto::xprint::String8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&string_len.serialize_fixed());
        buf_ptr
            .get_mut(12..13)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&pool.serialize_fixed());
        buf_ptr
            .get_mut(13..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&rule.serialize_fixed());
        let list_len = attributes.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            attributes,
        )?;
        let mut offset = list_len + 16;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 18]);
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

    fn print_get_page_dimensions(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xprint::PrintGetPageDimensionsReply, 20>>
    {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_bytes = context.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                21,
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
        Ok(FixedCookie::new(seq))
    }

    fn print_query_screens(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xprint::PrintQueryScreensReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 22;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn print_set_image_resolution(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        image_resolution: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xprint::PrintSetImageResolutionReply, 10>>
    {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_bytes = context.serialize_fixed();
        let image_resolution_bytes = image_resolution.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                23,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
                image_resolution_bytes[0],
                image_resolution_bytes[1],
                0,
                0,
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn print_get_image_resolution(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::xprint::Pcontext,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xprint::PrintGetImageResolutionReply, 10>>
    {
        let major_opcode = self
            .major_opcode(crate::proto::xprint::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xprint::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_bytes = context.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                24,
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
        Ok(FixedCookie::new(seq))
    }
}
