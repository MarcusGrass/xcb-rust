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
pub trait XcMiscConnection {
    fn get_version(
        &mut self,
        client_major_version: u16,
        client_minor_version: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xc_misc::GetVersionReply, 12>>;

    fn get_x_i_d_range(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xc_misc::GetXIDRangeReply, 16>>;

    fn get_x_i_d_list(
        &mut self,
        count: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xc_misc::GetXIDListReply>>;
}
impl<C> XcMiscConnection for C
where
    C: crate::con::XcbConnection,
{
    fn get_version(
        &mut self,
        client_major_version: u16,
        client_minor_version: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xc_misc::GetVersionReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::xc_misc::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xc_misc::EXTENSION_NAME,
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

    fn get_x_i_d_range(
        &mut self,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xc_misc::GetXIDRangeReply, 16>> {
        let major_opcode = self
            .major_opcode(crate::proto::xc_misc::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xc_misc::EXTENSION_NAME,
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

    fn get_x_i_d_list(
        &mut self,
        count: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xc_misc::GetXIDListReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xc_misc::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xc_misc::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let count_bytes = count.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                2,
                length[0],
                length[1],
                count_bytes[0],
                count_bytes[1],
                count_bytes[2],
                count_bytes[3],
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
