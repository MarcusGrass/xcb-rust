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
pub trait XselinuxConnection {
    fn query_version(
        &mut self,
        client_major: u8,
        client_minor: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xselinux::QueryVersionReply, 12>>;

    fn set_device_create_context(
        &mut self,
        context: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_device_create_context(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetDeviceCreateContextReply>>;

    fn set_device_context(
        &mut self,
        device: u32,
        context: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_device_context(
        &mut self,
        device: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetDeviceContextReply>>;

    fn set_window_create_context(
        &mut self,
        context: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_window_create_context(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetWindowCreateContextReply>>;

    fn get_window_context(
        &mut self,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetWindowContextReply>>;

    fn set_property_create_context(
        &mut self,
        context: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_property_create_context(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetPropertyCreateContextReply>>;

    fn set_property_use_context(
        &mut self,
        context: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_property_use_context(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetPropertyUseContextReply>>;

    fn get_property_context(
        &mut self,
        window: crate::proto::xproto::Window,
        property: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetPropertyContextReply>>;

    fn get_property_data_context(
        &mut self,
        window: crate::proto::xproto::Window,
        property: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetPropertyDataContextReply>>;

    fn list_properties(
        &mut self,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::ListPropertiesReply>>;

    fn set_selection_create_context(
        &mut self,
        context: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_selection_create_context(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetSelectionCreateContextReply>>;

    fn set_selection_use_context(
        &mut self,
        context: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_selection_use_context(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetSelectionUseContextReply>>;

    fn get_selection_context(
        &mut self,
        selection: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetSelectionContextReply>>;

    fn get_selection_data_context(
        &mut self,
        selection: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetSelectionDataContextReply>>;

    fn list_selections(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::ListSelectionsReply>>;

    fn get_client_context(
        &mut self,
        resource: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetClientContextReply>>;
}
impl<C> XselinuxConnection for C
where
    C: crate::con::XcbConnection,
{
    fn query_version(
        &mut self,
        client_major: u8,
        client_minor: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xselinux::QueryVersionReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
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

    fn set_device_create_context(
        &mut self,
        context: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.write_buf();
        let context_len =
            u32::try_from(context.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(context_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = context.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            context,
        )?;
        let mut offset = list_len + 8;
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

    fn get_device_create_context(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetDeviceCreateContextReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let buf = self
            .write_buf()
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 2;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn set_device_context(
        &mut self,
        device: u32,
        context: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.write_buf();
        let context_len =
            u32::try_from(context.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&device.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(context_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = context.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            context,
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

    fn get_device_context(
        &mut self,
        device: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetDeviceContextReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let device_bytes = device.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                4,
                length[0],
                length[1],
                device_bytes[0],
                device_bytes[1],
                device_bytes[2],
                device_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn set_window_create_context(
        &mut self,
        context: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.write_buf();
        let context_len =
            u32::try_from(context.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(context_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = context.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            context,
        )?;
        let mut offset = list_len + 8;
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
        Ok(VoidCookie::new(seq))
    }

    fn get_window_create_context(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetWindowCreateContextReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let buf = self
            .write_buf()
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
        Ok(Cookie::new(seq))
    }

    fn get_window_context(
        &mut self,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetWindowContextReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                7,
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
        Ok(Cookie::new(seq))
    }

    fn set_property_create_context(
        &mut self,
        context: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.write_buf();
        let context_len =
            u32::try_from(context.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(context_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = context.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            context,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 8]);
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

    fn get_property_create_context(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetPropertyCreateContextReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let buf = self
            .write_buf()
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 9;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn set_property_use_context(
        &mut self,
        context: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.write_buf();
        let context_len =
            u32::try_from(context.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(context_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = context.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            context,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 10]);
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

    fn get_property_use_context(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetPropertyUseContextReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let buf = self
            .write_buf()
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 11;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_property_context(
        &mut self,
        window: crate::proto::xproto::Window,
        property: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetPropertyContextReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let property_bytes = property.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                12,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                property_bytes[0],
                property_bytes[1],
                property_bytes[2],
                property_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_property_data_context(
        &mut self,
        window: crate::proto::xproto::Window,
        property: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetPropertyDataContextReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let property_bytes = property.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
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
                property_bytes[0],
                property_bytes[1],
                property_bytes[2],
                property_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn list_properties(
        &mut self,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::ListPropertiesReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                14,
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
        Ok(Cookie::new(seq))
    }

    fn set_selection_create_context(
        &mut self,
        context: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.write_buf();
        let context_len =
            u32::try_from(context.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(context_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = context.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            context,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 15]);
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

    fn get_selection_create_context(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetSelectionCreateContextReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let buf = self
            .write_buf()
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 16;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn set_selection_use_context(
        &mut self,
        context: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.write_buf();
        let context_len =
            u32::try_from(context.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(context_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = context.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            context,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 17]);
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

    fn get_selection_use_context(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetSelectionUseContextReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let buf = self
            .write_buf()
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 18;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_selection_context(
        &mut self,
        selection: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetSelectionContextReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let selection_bytes = selection.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                19,
                length[0],
                length[1],
                selection_bytes[0],
                selection_bytes[1],
                selection_bytes[2],
                selection_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_selection_data_context(
        &mut self,
        selection: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetSelectionDataContextReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let selection_bytes = selection.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                20,
                length[0],
                length[1],
                selection_bytes[0],
                selection_bytes[1],
                selection_bytes[2],
                selection_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn list_selections(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::ListSelectionsReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let buf = self
            .write_buf()
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 21;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_client_context(
        &mut self,
        resource: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xselinux::GetClientContextReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xselinux::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xselinux::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let resource_bytes = resource.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                22,
                length[0],
                length[1],
                resource_bytes[0],
                resource_bytes[1],
                resource_bytes[2],
                resource_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }
}
