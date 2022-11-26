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
pub trait DpmsConnection {
    fn get_version(
        &mut self,
        client_major_version: u16,
        client_minor_version: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dpms::GetVersionReply, 12>>;

    fn capable(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dpms::CapableReply, 32>>;

    fn get_timeouts(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dpms::GetTimeoutsReply, 32>>;

    fn set_timeouts(
        &mut self,
        standby_timeout: u16,
        suspend_timeout: u16,
        off_timeout: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn enable(&mut self, forget: bool) -> crate::error::Result<VoidCookie>;

    fn disable(&mut self, forget: bool) -> crate::error::Result<VoidCookie>;

    fn force_level(
        &mut self,
        power_level: crate::proto::dpms::DPMSModeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn info(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dpms::InfoReply, 32>>;
}
impl<C> DpmsConnection for C
where
    C: crate::con::XcbConnection,
{
    fn get_version(
        &mut self,
        client_major_version: u16,
        client_minor_version: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dpms::GetVersionReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::dpms::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dpms::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let client_major_version_bytes = client_major_version.serialize_fixed();
        let client_minor_version_bytes = client_minor_version.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                0,
                length[0],
                length[1],
                client_major_version_bytes[0],
                client_major_version_bytes[1],
                client_minor_version_bytes[0],
                client_minor_version_bytes[1],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn capable(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dpms::CapableReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::dpms::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dpms::EXTENSION_NAME,
            ))?;
        let buf = self
            .write_buf()
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 1;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn get_timeouts(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dpms::GetTimeoutsReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::dpms::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dpms::EXTENSION_NAME,
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
        Ok(FixedCookie::new(seq))
    }

    fn set_timeouts(
        &mut self,
        standby_timeout: u16,
        suspend_timeout: u16,
        off_timeout: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::dpms::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dpms::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let standby_timeout_bytes = standby_timeout.serialize_fixed();
        let suspend_timeout_bytes = suspend_timeout.serialize_fixed();
        let off_timeout_bytes = off_timeout.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
                length[0],
                length[1],
                standby_timeout_bytes[0],
                standby_timeout_bytes[1],
                suspend_timeout_bytes[0],
                suspend_timeout_bytes[1],
                off_timeout_bytes[0],
                off_timeout_bytes[1],
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

    fn enable(&mut self, forget: bool) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::dpms::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dpms::EXTENSION_NAME,
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
        Ok(VoidCookie::new(seq))
    }

    fn disable(&mut self, forget: bool) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::dpms::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dpms::EXTENSION_NAME,
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
        Ok(VoidCookie::new(seq))
    }

    fn force_level(
        &mut self,
        power_level: crate::proto::dpms::DPMSModeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::dpms::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dpms::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let power_level_bytes = (power_level.0 as u16).serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                6,
                length[0],
                length[1],
                power_level_bytes[0],
                power_level_bytes[1],
                0,
                0,
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn info(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::dpms::InfoReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::dpms::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::dpms::EXTENSION_NAME,
            ))?;
        let buf = self
            .write_buf()
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 7;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }
}
