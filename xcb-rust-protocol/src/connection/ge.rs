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
pub trait GeConnection {
    fn query_version(
        &mut self,
        socket_buffer: &mut [u8],
        client_major_version: u16,
        client_minor_version: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::ge::QueryVersionReply, 32>>;
}
impl<C> GeConnection for C
where
    C: crate::con::XcbConnection,
{
    fn query_version(
        &mut self,
        socket_buffer: &mut [u8],
        client_major_version: u16,
        client_minor_version: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::ge::QueryVersionReply, 32>> {
        let major_opcode = self.major_opcode(crate::proto::ge::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::ge::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let client_major_version_bytes = client_major_version.serialize_fixed();
        let client_minor_version_bytes = client_minor_version.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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
}
