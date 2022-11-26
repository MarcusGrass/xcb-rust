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
pub trait Xf86driConnection {
    fn query_version(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86dri::QueryVersionReply, 16>>;

    fn query_direct_rendering_capable(
        &mut self,
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86dri::QueryDirectRenderingCapableReply, 9>>;

    fn open_connection(
        &mut self,
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86dri::OpenConnectionReply>>;

    fn close_connection(&mut self, screen: u32, forget: bool) -> crate::error::Result<VoidCookie>;

    fn get_client_driver_name(
        &mut self,
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86dri::GetClientDriverNameReply>>;

    fn create_context(
        &mut self,
        screen: u32,
        visual: u32,
        context: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86dri::CreateContextReply, 12>>;

    fn destroy_context(
        &mut self,
        screen: u32,
        context: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn create_drawable(
        &mut self,
        screen: u32,
        drawable: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86dri::CreateDrawableReply, 12>>;

    fn destroy_drawable(
        &mut self,
        screen: u32,
        drawable: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_drawable_info(
        &mut self,
        screen: u32,
        drawable: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86dri::GetDrawableInfoReply>>;

    fn get_device_info(
        &mut self,
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86dri::GetDeviceInfoReply>>;

    fn auth_connection(
        &mut self,
        screen: u32,
        magic: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86dri::AuthConnectionReply, 12>>;
}
impl<C> Xf86driConnection for C
where
    C: crate::con::XcbConnection,
{
    fn query_version(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86dri::QueryVersionReply, 16>> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86dri::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86dri::EXTENSION_NAME,
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

    fn query_direct_rendering_capable(
        &mut self,
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86dri::QueryDirectRenderingCapableReply, 9>>
    {
        let major_opcode = self
            .major_opcode(crate::proto::xf86dri::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86dri::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                1,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn open_connection(
        &mut self,
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86dri::OpenConnectionReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86dri::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86dri::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                2,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn close_connection(&mut self, screen: u32, forget: bool) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86dri::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86dri::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_client_driver_name(
        &mut self,
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86dri::GetClientDriverNameReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86dri::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86dri::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                4,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn create_context(
        &mut self,
        screen: u32,
        visual: u32,
        context: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86dri::CreateContextReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86dri::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86dri::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let visual_bytes = visual.serialize_fixed();
        let context_bytes = context.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                5,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
                visual_bytes[0],
                visual_bytes[1],
                visual_bytes[2],
                visual_bytes[3],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn destroy_context(
        &mut self,
        screen: u32,
        context: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86dri::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86dri::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let context_bytes = context.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                6,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn create_drawable(
        &mut self,
        screen: u32,
        drawable: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86dri::CreateDrawableReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86dri::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86dri::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let drawable_bytes = drawable.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                7,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn destroy_drawable(
        &mut self,
        screen: u32,
        drawable: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86dri::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86dri::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let drawable_bytes = drawable.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                8,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_drawable_info(
        &mut self,
        screen: u32,
        drawable: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86dri::GetDrawableInfoReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86dri::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86dri::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let drawable_bytes = drawable.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                9,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_device_info(
        &mut self,
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86dri::GetDeviceInfoReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86dri::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86dri::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                10,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn auth_connection(
        &mut self,
        screen: u32,
        magic: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86dri::AuthConnectionReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86dri::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86dri::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let magic_bytes = magic.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                11,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
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
}
