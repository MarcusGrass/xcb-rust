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
pub trait XineramaConnection {
    fn query_version(
        &mut self,
        major: u8,
        minor: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinerama::QueryVersionReply, 12>>;

    fn get_state(
        &mut self,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinerama::GetStateReply, 12>>;

    fn get_screen_count(
        &mut self,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinerama::GetScreenCountReply, 12>>;

    fn get_screen_size(
        &mut self,
        window: crate::proto::xproto::Window,
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinerama::GetScreenSizeReply, 24>>;

    fn is_active(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinerama::IsActiveReply, 12>>;

    fn query_screens(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinerama::QueryScreensReply>>;
}
impl<C> XineramaConnection for C
where
    C: crate::con::XcbConnection,
{
    fn query_version(
        &mut self,
        major: u8,
        minor: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinerama::QueryVersionReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinerama::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinerama::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 0, length[0], length[1], major, minor, 0, 0]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn get_state(
        &mut self,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinerama::GetStateReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinerama::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinerama::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
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
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn get_screen_count(
        &mut self,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinerama::GetScreenCountReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinerama::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinerama::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
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
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn get_screen_size(
        &mut self,
        window: crate::proto::xproto::Window,
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinerama::GetScreenSizeReply, 24>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinerama::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinerama::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let screen_bytes = screen.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn is_active(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinerama::IsActiveReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinerama::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinerama::EXTENSION_NAME,
            ))?;
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
        Ok(FixedCookie::new(seq))
    }

    fn query_screens(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinerama::QueryScreensReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinerama::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinerama::EXTENSION_NAME,
            ))?;
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
        Ok(Cookie::new(seq))
    }
}
