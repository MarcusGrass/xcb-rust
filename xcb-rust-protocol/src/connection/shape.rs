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
pub trait ShapeConnection {
    fn query_version(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::shape::QueryVersionReply, 12>>;

    fn rectangles(
        &mut self,
        operation: crate::proto::shape::SoEnum,
        destination_kind: crate::proto::shape::SkEnum,
        ordering: crate::proto::xproto::ClipOrderingEnum,
        destination_window: crate::proto::xproto::Window,
        x_offset: i16,
        y_offset: i16,
        rectangles: &[crate::proto::xproto::Rectangle],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn mask(
        &mut self,
        operation: crate::proto::shape::SoEnum,
        destination_kind: crate::proto::shape::SkEnum,
        destination_window: crate::proto::xproto::Window,
        x_offset: i16,
        y_offset: i16,
        source_bitmap: crate::proto::xproto::PixmapEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn combine(
        &mut self,
        operation: crate::proto::shape::SoEnum,
        destination_kind: crate::proto::shape::SkEnum,
        source_kind: crate::proto::shape::SkEnum,
        destination_window: crate::proto::xproto::Window,
        x_offset: i16,
        y_offset: i16,
        source_window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn offset(
        &mut self,
        destination_kind: crate::proto::shape::SkEnum,
        destination_window: crate::proto::xproto::Window,
        x_offset: i16,
        y_offset: i16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn query_extents(
        &mut self,
        destination_window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::shape::QueryExtentsReply, 28>>;

    fn select_input(
        &mut self,
        destination_window: crate::proto::xproto::Window,
        enable: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn input_selected(
        &mut self,
        destination_window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::shape::InputSelectedReply, 8>>;

    fn get_rectangles(
        &mut self,
        window: crate::proto::xproto::Window,
        source_kind: crate::proto::shape::SkEnum,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::shape::GetRectanglesReply>>;
}
impl<C> ShapeConnection for C
where
    C: crate::con::XcbConnection,
{
    fn query_version(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::shape::QueryVersionReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::shape::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::shape::EXTENSION_NAME,
            ))?;
        let buf = self
            .write_buf()
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

    fn rectangles(
        &mut self,
        operation: crate::proto::shape::SoEnum,
        destination_kind: crate::proto::shape::SkEnum,
        ordering: crate::proto::xproto::ClipOrderingEnum,
        destination_window: crate::proto::xproto::Window,
        x_offset: i16,
        y_offset: i16,
        rectangles: &[crate::proto::xproto::Rectangle],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::shape::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::shape::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.write_buf();
        // Pad 1 bytes
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&operation.serialize_fixed());
        buf_ptr
            .get_mut(5..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&destination_kind.serialize_fixed());
        buf_ptr
            .get_mut(6..7)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&ordering.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&destination_window.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&x_offset.serialize_fixed());
        buf_ptr
            .get_mut(14..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&y_offset.serialize_fixed());
        let list_len = rectangles.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            rectangles,
        )?;
        let mut offset = list_len + 16;
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

    fn mask(
        &mut self,
        operation: crate::proto::shape::SoEnum,
        destination_kind: crate::proto::shape::SkEnum,
        destination_window: crate::proto::xproto::Window,
        x_offset: i16,
        y_offset: i16,
        source_bitmap: crate::proto::xproto::PixmapEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::shape::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::shape::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (5u16).to_ne_bytes();
        let destination_window_bytes = destination_window.serialize_fixed();
        let x_offset_bytes = x_offset.serialize_fixed();
        let y_offset_bytes = y_offset.serialize_fixed();
        let source_bitmap_bytes = (source_bitmap.0 as u32).serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                2,
                length[0],
                length[1],
                operation.0 as u8,
                destination_kind.0 as u8,
                0,
                0,
                destination_window_bytes[0],
                destination_window_bytes[1],
                destination_window_bytes[2],
                destination_window_bytes[3],
                x_offset_bytes[0],
                x_offset_bytes[1],
                y_offset_bytes[0],
                y_offset_bytes[1],
                source_bitmap_bytes[0],
                source_bitmap_bytes[1],
                source_bitmap_bytes[2],
                source_bitmap_bytes[3],
            ]);
        self.advance_writer(20);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn combine(
        &mut self,
        operation: crate::proto::shape::SoEnum,
        destination_kind: crate::proto::shape::SkEnum,
        source_kind: crate::proto::shape::SkEnum,
        destination_window: crate::proto::xproto::Window,
        x_offset: i16,
        y_offset: i16,
        source_window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::shape::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::shape::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (5u16).to_ne_bytes();
        let destination_window_bytes = destination_window.serialize_fixed();
        let x_offset_bytes = x_offset.serialize_fixed();
        let y_offset_bytes = y_offset.serialize_fixed();
        let source_window_bytes = source_window.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
                length[0],
                length[1],
                operation.0 as u8,
                destination_kind.0 as u8,
                source_kind.0 as u8,
                0,
                destination_window_bytes[0],
                destination_window_bytes[1],
                destination_window_bytes[2],
                destination_window_bytes[3],
                x_offset_bytes[0],
                x_offset_bytes[1],
                y_offset_bytes[0],
                y_offset_bytes[1],
                source_window_bytes[0],
                source_window_bytes[1],
                source_window_bytes[2],
                source_window_bytes[3],
            ]);
        self.advance_writer(20);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn offset(
        &mut self,
        destination_kind: crate::proto::shape::SkEnum,
        destination_window: crate::proto::xproto::Window,
        x_offset: i16,
        y_offset: i16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::shape::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::shape::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let destination_window_bytes = destination_window.serialize_fixed();
        let x_offset_bytes = x_offset.serialize_fixed();
        let y_offset_bytes = y_offset.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                4,
                length[0],
                length[1],
                destination_kind.0 as u8,
                0,
                0,
                0,
                destination_window_bytes[0],
                destination_window_bytes[1],
                destination_window_bytes[2],
                destination_window_bytes[3],
                x_offset_bytes[0],
                x_offset_bytes[1],
                y_offset_bytes[0],
                y_offset_bytes[1],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn query_extents(
        &mut self,
        destination_window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::shape::QueryExtentsReply, 28>> {
        let major_opcode = self
            .major_opcode(crate::proto::shape::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::shape::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let destination_window_bytes = destination_window.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                5,
                length[0],
                length[1],
                destination_window_bytes[0],
                destination_window_bytes[1],
                destination_window_bytes[2],
                destination_window_bytes[3],
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
        destination_window: crate::proto::xproto::Window,
        enable: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::shape::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::shape::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let destination_window_bytes = destination_window.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                6,
                length[0],
                length[1],
                destination_window_bytes[0],
                destination_window_bytes[1],
                destination_window_bytes[2],
                destination_window_bytes[3],
                enable,
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
        Ok(VoidCookie::new(seq))
    }

    fn input_selected(
        &mut self,
        destination_window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::shape::InputSelectedReply, 8>> {
        let major_opcode = self
            .major_opcode(crate::proto::shape::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::shape::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let destination_window_bytes = destination_window.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                7,
                length[0],
                length[1],
                destination_window_bytes[0],
                destination_window_bytes[1],
                destination_window_bytes[2],
                destination_window_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn get_rectangles(
        &mut self,
        window: crate::proto::xproto::Window,
        source_kind: crate::proto::shape::SkEnum,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::shape::GetRectanglesReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::shape::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::shape::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                8,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                source_kind.0 as u8,
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
}
