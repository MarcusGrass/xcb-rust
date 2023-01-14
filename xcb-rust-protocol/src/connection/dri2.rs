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
pub trait Dri2Connection {
    fn query_version(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u32,
        minor_version: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dri2::QueryVersionReply, 16>>;

    fn connect(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        driver_type: crate::proto::dri2::DriverTypeEnum,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::dri2::ConnectReply>>;

    fn authenticate(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        magic: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dri2::AuthenticateReply, 12>>;

    fn create_drawable(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn destroy_drawable(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_buffers(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        count: u32,
        attachments: &[u32],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::dri2::GetBuffersReply>>;

    fn copy_region(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        region: u32,
        dest: u32,
        src: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dri2::CopyRegionReply, 8>>;

    fn get_buffers_with_format(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        count: u32,
        attachments: &[crate::proto::dri2::AttachFormat],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::dri2::GetBuffersWithFormatReply>>;

    fn swap_buffers(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        target_msc_hi: u32,
        target_msc_lo: u32,
        divisor_hi: u32,
        divisor_lo: u32,
        remainder_hi: u32,
        remainder_lo: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dri2::SwapBuffersReply, 16>>;

    fn get_m_s_c(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dri2::GetMSCReply, 32>>;

    fn wait_m_s_c(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        target_msc_hi: u32,
        target_msc_lo: u32,
        divisor_hi: u32,
        divisor_lo: u32,
        remainder_hi: u32,
        remainder_lo: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dri2::WaitMSCReply, 32>>;

    fn wait_s_b_c(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        target_sbc_hi: u32,
        target_sbc_lo: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dri2::WaitSBCReply, 32>>;

    fn swap_interval(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        interval: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_param(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        param: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dri2::GetParamReply, 16>>;
}
impl<C> Dri2Connection for C
where
    C: crate::con::XcbConnection,
{
    fn query_version(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u32,
        minor_version: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dri2::QueryVersionReply, 16>> {
        let major_opcode = self
            .major_opcode(crate::proto::dri2::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dri2::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let major_version_bytes = major_version.serialize_fixed();
        let minor_version_bytes = minor_version.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                0,
                length[0],
                length[1],
                major_version_bytes[0],
                major_version_bytes[1],
                major_version_bytes[2],
                major_version_bytes[3],
                minor_version_bytes[0],
                minor_version_bytes[1],
                minor_version_bytes[2],
                minor_version_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn connect(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        driver_type: crate::proto::dri2::DriverTypeEnum,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::dri2::ConnectReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::dri2::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dri2::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let driver_type_bytes = (driver_type.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
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
                driver_type_bytes[0],
                driver_type_bytes[1],
                driver_type_bytes[2],
                driver_type_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn authenticate(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        magic: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dri2::AuthenticateReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::dri2::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dri2::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let magic_bytes = magic.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                2,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                magic_bytes[0],
                magic_bytes[1],
                magic_bytes[2],
                magic_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn create_drawable(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::dri2::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dri2::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
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

    fn destroy_drawable(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::dri2::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dri2::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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

    fn get_buffers(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        count: u32,
        attachments: &[u32],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::dri2::GetBuffersReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::dri2::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dri2::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&count.serialize_fixed());
        let list_len = attachments.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            attachments,
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

    fn copy_region(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        region: u32,
        dest: u32,
        src: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dri2::CopyRegionReply, 8>> {
        let major_opcode = self
            .major_opcode(crate::proto::dri2::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dri2::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (5u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let region_bytes = region.serialize_fixed();
        let dest_bytes = dest.serialize_fixed();
        let src_bytes = src.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                6,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                region_bytes[0],
                region_bytes[1],
                region_bytes[2],
                region_bytes[3],
                dest_bytes[0],
                dest_bytes[1],
                dest_bytes[2],
                dest_bytes[3],
                src_bytes[0],
                src_bytes[1],
                src_bytes[2],
                src_bytes[3],
            ]);
        self.advance_writer(20);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn get_buffers_with_format(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        count: u32,
        attachments: &[crate::proto::dri2::AttachFormat],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::dri2::GetBuffersWithFormatReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::dri2::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dri2::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&count.serialize_fixed());
        let list_len = attachments.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            attachments,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 7]);
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

    fn swap_buffers(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        target_msc_hi: u32,
        target_msc_lo: u32,
        divisor_hi: u32,
        divisor_lo: u32,
        remainder_hi: u32,
        remainder_lo: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dri2::SwapBuffersReply, 16>> {
        let major_opcode = self
            .major_opcode(crate::proto::dri2::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dri2::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (8u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let target_msc_hi_bytes = target_msc_hi.serialize_fixed();
        let target_msc_lo_bytes = target_msc_lo.serialize_fixed();
        let divisor_hi_bytes = divisor_hi.serialize_fixed();
        let divisor_lo_bytes = divisor_lo.serialize_fixed();
        let remainder_hi_bytes = remainder_hi.serialize_fixed();
        let remainder_lo_bytes = remainder_lo.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                8,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                target_msc_hi_bytes[0],
                target_msc_hi_bytes[1],
                target_msc_hi_bytes[2],
                target_msc_hi_bytes[3],
                target_msc_lo_bytes[0],
                target_msc_lo_bytes[1],
                target_msc_lo_bytes[2],
                target_msc_lo_bytes[3],
                divisor_hi_bytes[0],
                divisor_hi_bytes[1],
                divisor_hi_bytes[2],
                divisor_hi_bytes[3],
                divisor_lo_bytes[0],
                divisor_lo_bytes[1],
                divisor_lo_bytes[2],
                divisor_lo_bytes[3],
                remainder_hi_bytes[0],
                remainder_hi_bytes[1],
                remainder_hi_bytes[2],
                remainder_hi_bytes[3],
                remainder_lo_bytes[0],
                remainder_lo_bytes[1],
                remainder_lo_bytes[2],
                remainder_lo_bytes[3],
            ]);
        self.advance_writer(32);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn get_m_s_c(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dri2::GetMSCReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::dri2::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dri2::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                9,
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

    fn wait_m_s_c(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        target_msc_hi: u32,
        target_msc_lo: u32,
        divisor_hi: u32,
        divisor_lo: u32,
        remainder_hi: u32,
        remainder_lo: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dri2::WaitMSCReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::dri2::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dri2::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (8u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let target_msc_hi_bytes = target_msc_hi.serialize_fixed();
        let target_msc_lo_bytes = target_msc_lo.serialize_fixed();
        let divisor_hi_bytes = divisor_hi.serialize_fixed();
        let divisor_lo_bytes = divisor_lo.serialize_fixed();
        let remainder_hi_bytes = remainder_hi.serialize_fixed();
        let remainder_lo_bytes = remainder_lo.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                10,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                target_msc_hi_bytes[0],
                target_msc_hi_bytes[1],
                target_msc_hi_bytes[2],
                target_msc_hi_bytes[3],
                target_msc_lo_bytes[0],
                target_msc_lo_bytes[1],
                target_msc_lo_bytes[2],
                target_msc_lo_bytes[3],
                divisor_hi_bytes[0],
                divisor_hi_bytes[1],
                divisor_hi_bytes[2],
                divisor_hi_bytes[3],
                divisor_lo_bytes[0],
                divisor_lo_bytes[1],
                divisor_lo_bytes[2],
                divisor_lo_bytes[3],
                remainder_hi_bytes[0],
                remainder_hi_bytes[1],
                remainder_hi_bytes[2],
                remainder_hi_bytes[3],
                remainder_lo_bytes[0],
                remainder_lo_bytes[1],
                remainder_lo_bytes[2],
                remainder_lo_bytes[3],
            ]);
        self.advance_writer(32);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn wait_s_b_c(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        target_sbc_hi: u32,
        target_sbc_lo: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dri2::WaitSBCReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::dri2::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dri2::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let target_sbc_hi_bytes = target_sbc_hi.serialize_fixed();
        let target_sbc_lo_bytes = target_sbc_lo.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                11,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                target_sbc_hi_bytes[0],
                target_sbc_hi_bytes[1],
                target_sbc_hi_bytes[2],
                target_sbc_hi_bytes[3],
                target_sbc_lo_bytes[0],
                target_sbc_lo_bytes[1],
                target_sbc_lo_bytes[2],
                target_sbc_lo_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn swap_interval(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        interval: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::dri2::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dri2::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let interval_bytes = interval.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                12,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                interval_bytes[0],
                interval_bytes[1],
                interval_bytes[2],
                interval_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_param(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        param: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dri2::GetParamReply, 16>> {
        let major_opcode = self
            .major_opcode(crate::proto::dri2::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dri2::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let param_bytes = param.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                13,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                param_bytes[0],
                param_bytes[1],
                param_bytes[2],
                param_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }
}
