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
pub trait BigreqConnection {
    fn enable(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::bigreq::EnableReply, 12>>;
}
impl<C> BigreqConnection for C
where
    C: crate::con::XcbConnection,
{
    fn enable(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::bigreq::EnableReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::bigreq::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::bigreq::EXTENSION_NAME,
            ))?;
        let buf = self
            .apply_offset(socket_buffer)
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
}
