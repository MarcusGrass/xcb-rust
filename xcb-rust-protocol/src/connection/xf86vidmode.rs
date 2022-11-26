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
pub trait Xf86vidmodeConnection {
    fn query_version(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86vidmode::QueryVersionReply, 12>>;

    fn get_mode_line(
        &mut self,
        screen: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86vidmode::GetModeLineReply>>;

    fn mod_mode_line(
        &mut self,
        screen: u32,
        hdisplay: u16,
        hsyncstart: u16,
        hsyncend: u16,
        htotal: u16,
        hskew: u16,
        vdisplay: u16,
        vsyncstart: u16,
        vsyncend: u16,
        vtotal: u16,
        flags: crate::proto::xf86vidmode::ModeFlag,
        private: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn switch_mode(
        &mut self,
        screen: u16,
        zoom: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_monitor(
        &mut self,
        screen: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86vidmode::GetMonitorReply>>;

    fn lock_mode_switch(
        &mut self,
        screen: u16,
        lock: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_all_mode_lines(
        &mut self,
        screen: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86vidmode::GetAllModeLinesReply>>;

    fn add_mode_line(
        &mut self,
        screen: u32,
        dotclock: crate::proto::xf86vidmode::Dotclock,
        hdisplay: u16,
        hsyncstart: u16,
        hsyncend: u16,
        htotal: u16,
        hskew: u16,
        vdisplay: u16,
        vsyncstart: u16,
        vsyncend: u16,
        vtotal: u16,
        flags: crate::proto::xf86vidmode::ModeFlag,
        after_dotclock: crate::proto::xf86vidmode::Dotclock,
        after_hdisplay: u16,
        after_hsyncstart: u16,
        after_hsyncend: u16,
        after_htotal: u16,
        after_hskew: u16,
        after_vdisplay: u16,
        after_vsyncstart: u16,
        after_vsyncend: u16,
        after_vtotal: u16,
        after_flags: crate::proto::xf86vidmode::ModeFlag,
        private: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn delete_mode_line(
        &mut self,
        screen: u32,
        dotclock: crate::proto::xf86vidmode::Dotclock,
        hdisplay: u16,
        hsyncstart: u16,
        hsyncend: u16,
        htotal: u16,
        hskew: u16,
        vdisplay: u16,
        vsyncstart: u16,
        vsyncend: u16,
        vtotal: u16,
        flags: crate::proto::xf86vidmode::ModeFlag,
        private: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn validate_mode_line(
        &mut self,
        screen: u32,
        dotclock: crate::proto::xf86vidmode::Dotclock,
        hdisplay: u16,
        hsyncstart: u16,
        hsyncend: u16,
        htotal: u16,
        hskew: u16,
        vdisplay: u16,
        vsyncstart: u16,
        vsyncend: u16,
        vtotal: u16,
        flags: crate::proto::xf86vidmode::ModeFlag,
        private: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86vidmode::ValidateModeLineReply, 32>>;

    fn switch_to_mode(
        &mut self,
        screen: u32,
        dotclock: crate::proto::xf86vidmode::Dotclock,
        hdisplay: u16,
        hsyncstart: u16,
        hsyncend: u16,
        htotal: u16,
        hskew: u16,
        vdisplay: u16,
        vsyncstart: u16,
        vsyncend: u16,
        vtotal: u16,
        flags: crate::proto::xf86vidmode::ModeFlag,
        private: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_view_port(
        &mut self,
        screen: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86vidmode::GetViewPortReply, 32>>;

    fn set_view_port(
        &mut self,
        screen: u16,
        x: u32,
        y: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_dot_clocks(
        &mut self,
        screen: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86vidmode::GetDotClocksReply>>;

    fn set_client_version(
        &mut self,
        major: u16,
        minor: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn set_gamma(
        &mut self,
        screen: u16,
        red: u32,
        green: u32,
        blue: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_gamma(
        &mut self,
        screen: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86vidmode::GetGammaReply, 32>>;

    fn get_gamma_ramp(
        &mut self,
        screen: u16,
        size: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86vidmode::GetGammaRampReply>>;

    fn set_gamma_ramp(
        &mut self,
        screen: u16,
        size: u16,
        red: &[u16],
        green: &[u16],
        blue: &[u16],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_gamma_ramp_size(
        &mut self,
        screen: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86vidmode::GetGammaRampSizeReply, 32>>;

    fn get_permissions(
        &mut self,
        screen: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86vidmode::GetPermissionsReply, 32>>;
}
impl<C> Xf86vidmodeConnection for C
where
    C: crate::con::XcbConnection,
{
    fn query_version(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86vidmode::QueryVersionReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
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

    fn get_mode_line(
        &mut self,
        screen: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86vidmode::GetModeLineReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
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
                0,
                0,
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn mod_mode_line(
        &mut self,
        screen: u32,
        hdisplay: u16,
        hsyncstart: u16,
        hsyncend: u16,
        htotal: u16,
        hskew: u16,
        vdisplay: u16,
        vsyncstart: u16,
        vsyncend: u16,
        vtotal: u16,
        flags: crate::proto::xf86vidmode::ModeFlag,
        private: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.write_buf();
        // Pad 2 bytes
        // Pad 12 bytes
        let privsize = u32::try_from(private.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&screen.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hdisplay.serialize_fixed());
        buf_ptr
            .get_mut(10..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hsyncstart.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hsyncend.serialize_fixed());
        buf_ptr
            .get_mut(14..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&htotal.serialize_fixed());
        buf_ptr
            .get_mut(16..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hskew.serialize_fixed());
        buf_ptr
            .get_mut(18..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vdisplay.serialize_fixed());
        buf_ptr
            .get_mut(20..22)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vsyncstart.serialize_fixed());
        buf_ptr
            .get_mut(22..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vsyncend.serialize_fixed());
        buf_ptr
            .get_mut(24..26)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vtotal.serialize_fixed());
        buf_ptr
            .get_mut(28..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&flags.serialize_fixed());
        buf_ptr
            .get_mut(44..48)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(privsize).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = private.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(48..)
                .ok_or(crate::error::Error::Serialize)?,
            private,
        )?;
        let mut offset = list_len + 48;
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

    fn switch_mode(
        &mut self,
        screen: u16,
        zoom: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let zoom_bytes = zoom.serialize_fixed();
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
                zoom_bytes[0],
                zoom_bytes[1],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_monitor(
        &mut self,
        screen: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86vidmode::GetMonitorReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
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
                0,
                0,
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn lock_mode_switch(
        &mut self,
        screen: u16,
        lock: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let lock_bytes = lock.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                5,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                lock_bytes[0],
                lock_bytes[1],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_all_mode_lines(
        &mut self,
        screen: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86vidmode::GetAllModeLinesReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                6,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                0,
                0,
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn add_mode_line(
        &mut self,
        screen: u32,
        dotclock: crate::proto::xf86vidmode::Dotclock,
        hdisplay: u16,
        hsyncstart: u16,
        hsyncend: u16,
        htotal: u16,
        hskew: u16,
        vdisplay: u16,
        vsyncstart: u16,
        vsyncend: u16,
        vtotal: u16,
        flags: crate::proto::xf86vidmode::ModeFlag,
        after_dotclock: crate::proto::xf86vidmode::Dotclock,
        after_hdisplay: u16,
        after_hsyncstart: u16,
        after_hsyncend: u16,
        after_htotal: u16,
        after_hskew: u16,
        after_vdisplay: u16,
        after_vsyncstart: u16,
        after_vsyncend: u16,
        after_vtotal: u16,
        after_flags: crate::proto::xf86vidmode::ModeFlag,
        private: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.write_buf();
        // Pad 2 bytes
        // Pad 12 bytes
        let privsize = u32::try_from(private.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        // Pad 12 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&screen.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&dotclock.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hdisplay.serialize_fixed());
        buf_ptr
            .get_mut(14..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hsyncstart.serialize_fixed());
        buf_ptr
            .get_mut(16..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hsyncend.serialize_fixed());
        buf_ptr
            .get_mut(18..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&htotal.serialize_fixed());
        buf_ptr
            .get_mut(20..22)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hskew.serialize_fixed());
        buf_ptr
            .get_mut(22..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vdisplay.serialize_fixed());
        buf_ptr
            .get_mut(24..26)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vsyncstart.serialize_fixed());
        buf_ptr
            .get_mut(26..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vsyncend.serialize_fixed());
        buf_ptr
            .get_mut(28..30)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vtotal.serialize_fixed());
        buf_ptr
            .get_mut(32..36)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&flags.serialize_fixed());
        buf_ptr
            .get_mut(48..52)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(privsize).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(52..56)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&after_dotclock.serialize_fixed());
        buf_ptr
            .get_mut(56..58)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&after_hdisplay.serialize_fixed());
        buf_ptr
            .get_mut(58..60)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&after_hsyncstart.serialize_fixed());
        buf_ptr
            .get_mut(60..62)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&after_hsyncend.serialize_fixed());
        buf_ptr
            .get_mut(62..64)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&after_htotal.serialize_fixed());
        buf_ptr
            .get_mut(64..66)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&after_hskew.serialize_fixed());
        buf_ptr
            .get_mut(66..68)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&after_vdisplay.serialize_fixed());
        buf_ptr
            .get_mut(68..70)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&after_vsyncstart.serialize_fixed());
        buf_ptr
            .get_mut(70..72)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&after_vsyncend.serialize_fixed());
        buf_ptr
            .get_mut(72..74)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&after_vtotal.serialize_fixed());
        buf_ptr
            .get_mut(76..80)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&after_flags.serialize_fixed());
        let list_len = private.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(92..)
                .ok_or(crate::error::Error::Serialize)?,
            private,
        )?;
        let mut offset = list_len + 92;
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

    fn delete_mode_line(
        &mut self,
        screen: u32,
        dotclock: crate::proto::xf86vidmode::Dotclock,
        hdisplay: u16,
        hsyncstart: u16,
        hsyncend: u16,
        htotal: u16,
        hskew: u16,
        vdisplay: u16,
        vsyncstart: u16,
        vsyncend: u16,
        vtotal: u16,
        flags: crate::proto::xf86vidmode::ModeFlag,
        private: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.write_buf();
        // Pad 2 bytes
        // Pad 12 bytes
        let privsize = u32::try_from(private.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&screen.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&dotclock.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hdisplay.serialize_fixed());
        buf_ptr
            .get_mut(14..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hsyncstart.serialize_fixed());
        buf_ptr
            .get_mut(16..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hsyncend.serialize_fixed());
        buf_ptr
            .get_mut(18..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&htotal.serialize_fixed());
        buf_ptr
            .get_mut(20..22)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hskew.serialize_fixed());
        buf_ptr
            .get_mut(22..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vdisplay.serialize_fixed());
        buf_ptr
            .get_mut(24..26)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vsyncstart.serialize_fixed());
        buf_ptr
            .get_mut(26..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vsyncend.serialize_fixed());
        buf_ptr
            .get_mut(28..30)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vtotal.serialize_fixed());
        buf_ptr
            .get_mut(32..36)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&flags.serialize_fixed());
        buf_ptr
            .get_mut(48..52)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(privsize).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = private.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(52..)
                .ok_or(crate::error::Error::Serialize)?,
            private,
        )?;
        let mut offset = list_len + 52;
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

    fn validate_mode_line(
        &mut self,
        screen: u32,
        dotclock: crate::proto::xf86vidmode::Dotclock,
        hdisplay: u16,
        hsyncstart: u16,
        hsyncend: u16,
        htotal: u16,
        hskew: u16,
        vdisplay: u16,
        vsyncstart: u16,
        vsyncend: u16,
        vtotal: u16,
        flags: crate::proto::xf86vidmode::ModeFlag,
        private: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86vidmode::ValidateModeLineReply, 32>>
    {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.write_buf();
        // Pad 2 bytes
        // Pad 12 bytes
        let privsize = u32::try_from(private.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&screen.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&dotclock.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hdisplay.serialize_fixed());
        buf_ptr
            .get_mut(14..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hsyncstart.serialize_fixed());
        buf_ptr
            .get_mut(16..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hsyncend.serialize_fixed());
        buf_ptr
            .get_mut(18..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&htotal.serialize_fixed());
        buf_ptr
            .get_mut(20..22)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hskew.serialize_fixed());
        buf_ptr
            .get_mut(22..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vdisplay.serialize_fixed());
        buf_ptr
            .get_mut(24..26)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vsyncstart.serialize_fixed());
        buf_ptr
            .get_mut(26..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vsyncend.serialize_fixed());
        buf_ptr
            .get_mut(28..30)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vtotal.serialize_fixed());
        buf_ptr
            .get_mut(32..36)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&flags.serialize_fixed());
        buf_ptr
            .get_mut(48..52)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(privsize).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = private.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(52..)
                .ok_or(crate::error::Error::Serialize)?,
            private,
        )?;
        let mut offset = list_len + 52;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 9]);
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
        Ok(FixedCookie::new(seq))
    }

    fn switch_to_mode(
        &mut self,
        screen: u32,
        dotclock: crate::proto::xf86vidmode::Dotclock,
        hdisplay: u16,
        hsyncstart: u16,
        hsyncend: u16,
        htotal: u16,
        hskew: u16,
        vdisplay: u16,
        vsyncstart: u16,
        vsyncend: u16,
        vtotal: u16,
        flags: crate::proto::xf86vidmode::ModeFlag,
        private: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.write_buf();
        // Pad 2 bytes
        // Pad 12 bytes
        let privsize = u32::try_from(private.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&screen.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&dotclock.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hdisplay.serialize_fixed());
        buf_ptr
            .get_mut(14..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hsyncstart.serialize_fixed());
        buf_ptr
            .get_mut(16..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hsyncend.serialize_fixed());
        buf_ptr
            .get_mut(18..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&htotal.serialize_fixed());
        buf_ptr
            .get_mut(20..22)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&hskew.serialize_fixed());
        buf_ptr
            .get_mut(22..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vdisplay.serialize_fixed());
        buf_ptr
            .get_mut(24..26)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vsyncstart.serialize_fixed());
        buf_ptr
            .get_mut(26..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vsyncend.serialize_fixed());
        buf_ptr
            .get_mut(28..30)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vtotal.serialize_fixed());
        buf_ptr
            .get_mut(32..36)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&flags.serialize_fixed());
        buf_ptr
            .get_mut(48..52)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(privsize).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = private.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(52..)
                .ok_or(crate::error::Error::Serialize)?,
            private,
        )?;
        let mut offset = list_len + 52;
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

    fn get_view_port(
        &mut self,
        screen: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86vidmode::GetViewPortReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                11,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
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

    fn set_view_port(
        &mut self,
        screen: u16,
        x: u32,
        y: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let x_bytes = x.serialize_fixed();
        let y_bytes = y.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                12,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                0,
                0,
                x_bytes[0],
                x_bytes[1],
                x_bytes[2],
                x_bytes[3],
                y_bytes[0],
                y_bytes[1],
                y_bytes[2],
                y_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_dot_clocks(
        &mut self,
        screen: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86vidmode::GetDotClocksReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                13,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                0,
                0,
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn set_client_version(
        &mut self,
        major: u16,
        minor: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let major_bytes = major.serialize_fixed();
        let minor_bytes = minor.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                14,
                length[0],
                length[1],
                major_bytes[0],
                major_bytes[1],
                minor_bytes[0],
                minor_bytes[1],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn set_gamma(
        &mut self,
        screen: u16,
        red: u32,
        green: u32,
        blue: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (8u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let red_bytes = red.serialize_fixed();
        let green_bytes = green.serialize_fixed();
        let blue_bytes = blue.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                15,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                0,
                0,
                red_bytes[0],
                red_bytes[1],
                red_bytes[2],
                red_bytes[3],
                green_bytes[0],
                green_bytes[1],
                green_bytes[2],
                green_bytes[3],
                blue_bytes[0],
                blue_bytes[1],
                blue_bytes[2],
                blue_bytes[3],
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ]);
        self.advance_writer(32);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_gamma(
        &mut self,
        screen: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86vidmode::GetGammaReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (8u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                16,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ]);
        self.advance_writer(32);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn get_gamma_ramp(
        &mut self,
        screen: u16,
        size: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xf86vidmode::GetGammaRampReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let size_bytes = size.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                17,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                size_bytes[0],
                size_bytes[1],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn set_gamma_ramp(
        &mut self,
        screen: u16,
        size: u16,
        red: &[u16],
        green: &[u16],
        blue: &[u16],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.write_buf();
        let size = u16::try_from(size).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&screen.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(core::ops::BitAnd::bitand(
                    core::ops::Sub::sub(size as u16, 1u16 as u16) as u16,
                    core::ops::Not::not(1u16) as u16,
                ))
                .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = red.len() * 2;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            red,
        )?;
        let mut offset = list_len + 8;
        let list_len = green.len() * 2;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            green,
        )?;
        offset += list_len;
        let list_len = blue.len() * 2;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            blue,
        )?;
        offset += list_len;
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

    fn get_gamma_ramp_size(
        &mut self,
        screen: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86vidmode::GetGammaRampSizeReply, 32>>
    {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                19,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
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

    fn get_permissions(
        &mut self,
        screen: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xf86vidmode::GetPermissionsReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xf86vidmode::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xf86vidmode::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                20,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
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
}
