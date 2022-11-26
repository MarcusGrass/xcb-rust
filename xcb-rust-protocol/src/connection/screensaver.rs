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
pub trait ScreensaverConnection {
    fn query_version(
        &mut self,
        client_major_version: u8,
        client_minor_version: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::screensaver::QueryVersionReply, 32>>;

    fn query_info(
        &mut self,
        drawable: crate::proto::xproto::Drawable,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::screensaver::QueryInfoReply, 32>>;

    fn select_input(
        &mut self,
        drawable: crate::proto::xproto::Drawable,
        event_mask: crate::proto::screensaver::Event,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn set_attributes(
        &mut self,
        drawable: crate::proto::xproto::Drawable,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: crate::proto::xproto::WindowClassEnum,
        depth: u8,
        visual: crate::proto::xproto::Visualid,
        set_attributes_value_list: crate::proto::screensaver::SetAttributesValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn unset_attributes(
        &mut self,
        drawable: crate::proto::xproto::Drawable,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn suspend(&mut self, suspend: u32, forget: bool) -> crate::error::Result<VoidCookie>;
}
impl<C> ScreensaverConnection for C
where
    C: crate::con::XcbConnection,
{
    fn query_version(
        &mut self,
        client_major_version: u8,
        client_minor_version: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::screensaver::QueryVersionReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::screensaver::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::screensaver::EXTENSION_NAME,
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
                client_major_version,
                client_minor_version,
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

    fn query_info(
        &mut self,
        drawable: crate::proto::xproto::Drawable,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::screensaver::QueryInfoReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::screensaver::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::screensaver::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                1,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn select_input(
        &mut self,
        drawable: crate::proto::xproto::Drawable,
        event_mask: crate::proto::screensaver::Event,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::screensaver::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::screensaver::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let event_mask_bytes = (event_mask.0 as u32).serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                2,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
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

    fn set_attributes(
        &mut self,
        drawable: crate::proto::xproto::Drawable,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: crate::proto::xproto::WindowClassEnum,
        depth: u8,
        visual: crate::proto::xproto::Visualid,
        set_attributes_value_list: crate::proto::screensaver::SetAttributesValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::screensaver::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::screensaver::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.write_buf();
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 3]);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&x.serialize_fixed());
        buf_ptr
            .get_mut(10..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&y.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&width.serialize_fixed());
        buf_ptr
            .get_mut(14..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&height.serialize_fixed());
        buf_ptr
            .get_mut(16..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&border_width.serialize_fixed());
        buf_ptr
            .get_mut(18..19)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&class.serialize_fixed());
        buf_ptr
            .get_mut(19..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&depth.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&visual.serialize_fixed());
        buf_ptr
            .get_mut(24..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&set_attributes_value_list.switch_expr().serialize_fixed());
        let mut offset = 28
            + set_attributes_value_list.serialize_into(
                buf_ptr
                    .get_mut(28..)
                    .ok_or(crate::error::Error::Serialize)?,
            )?;
        debug_assert!(offset % 4 == 0, "Bad request length offset {offset}");
        let word_len = offset / 4;
        let len = u16::try_from(word_len).map_err(|_| crate::error::Error::Serialize)?;
        let length: [u8; 2] = len.to_ne_bytes();
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&length);
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn unset_attributes(
        &mut self,
        drawable: crate::proto::xproto::Drawable,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::screensaver::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::screensaver::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                4,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn suspend(&mut self, suspend: u32, forget: bool) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::screensaver::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::screensaver::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let suspend_bytes = suspend.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                5,
                length[0],
                length[1],
                suspend_bytes[0],
                suspend_bytes[1],
                suspend_bytes[2],
                suspend_bytes[3],
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
