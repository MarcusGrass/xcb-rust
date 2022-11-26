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
pub trait CompositeConnection {
    fn query_version(
        &mut self,
        client_major_version: u32,
        client_minor_version: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::composite::QueryVersionReply, 32>>;

    fn redirect_window(
        &mut self,
        window: crate::proto::xproto::Window,
        update: crate::proto::composite::RedirectEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn redirect_subwindows(
        &mut self,
        window: crate::proto::xproto::Window,
        update: crate::proto::composite::RedirectEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn unredirect_window(
        &mut self,
        window: crate::proto::xproto::Window,
        update: crate::proto::composite::RedirectEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn unredirect_subwindows(
        &mut self,
        window: crate::proto::xproto::Window,
        update: crate::proto::composite::RedirectEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn create_region_from_border_clip(
        &mut self,
        region: crate::proto::xfixes::Region,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn name_window_pixmap(
        &mut self,
        window: crate::proto::xproto::Window,
        pixmap: crate::proto::xproto::Pixmap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_overlay_window(
        &mut self,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::composite::GetOverlayWindowReply, 32>>;

    fn release_overlay_window(
        &mut self,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;
}
impl<C> CompositeConnection for C
where
    C: crate::con::XcbConnection,
{
    fn query_version(
        &mut self,
        client_major_version: u32,
        client_minor_version: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::composite::QueryVersionReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::composite::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::composite::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let client_major_version_bytes = client_major_version.serialize_fixed();
        let client_minor_version_bytes = client_minor_version.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                0,
                length[0],
                length[1],
                client_major_version_bytes[0],
                client_major_version_bytes[1],
                client_major_version_bytes[2],
                client_major_version_bytes[3],
                client_minor_version_bytes[0],
                client_minor_version_bytes[1],
                client_minor_version_bytes[2],
                client_minor_version_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn redirect_window(
        &mut self,
        window: crate::proto::xproto::Window,
        update: crate::proto::composite::RedirectEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::composite::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::composite::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.write_buf();
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
                update.0 as u8,
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

    fn redirect_subwindows(
        &mut self,
        window: crate::proto::xproto::Window,
        update: crate::proto::composite::RedirectEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::composite::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::composite::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.write_buf();
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
                update.0 as u8,
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

    fn unredirect_window(
        &mut self,
        window: crate::proto::xproto::Window,
        update: crate::proto::composite::RedirectEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::composite::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::composite::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
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
                update.0 as u8,
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

    fn unredirect_subwindows(
        &mut self,
        window: crate::proto::xproto::Window,
        update: crate::proto::composite::RedirectEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::composite::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::composite::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                4,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                update.0 as u8,
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

    fn create_region_from_border_clip(
        &mut self,
        region: crate::proto::xfixes::Region,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::composite::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::composite::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let region_bytes = region.serialize_fixed();
        let window_bytes = window.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                5,
                length[0],
                length[1],
                region_bytes[0],
                region_bytes[1],
                region_bytes[2],
                region_bytes[3],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn name_window_pixmap(
        &mut self,
        window: crate::proto::xproto::Window,
        pixmap: crate::proto::xproto::Pixmap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::composite::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::composite::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let pixmap_bytes = pixmap.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                6,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                pixmap_bytes[0],
                pixmap_bytes[1],
                pixmap_bytes[2],
                pixmap_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_overlay_window(
        &mut self,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::composite::GetOverlayWindowReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::composite::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::composite::EXTENSION_NAME,
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
        Ok(FixedCookie::new(seq))
    }

    fn release_overlay_window(
        &mut self,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::composite::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::composite::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
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
