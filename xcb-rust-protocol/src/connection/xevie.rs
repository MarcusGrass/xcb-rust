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
pub trait XevieConnection {
    fn query_version(
        &mut self,
        socket_buffer: &mut [u8],
        client_major_version: u16,
        client_minor_version: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xevie::QueryVersionReply, 32>>;

    fn start(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xevie::StartReply, 32>>;

    fn end(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xevie::EndReply, 32>>;

    fn send(
        &mut self,
        socket_buffer: &mut [u8],
        event: crate::proto::xevie::Event,
        data_type: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xevie::SendReply, 32>>;

    fn select_input(
        &mut self,
        socket_buffer: &mut [u8],
        event_mask: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xevie::SelectInputReply, 32>>;
}
impl<C> XevieConnection for C
where
    C: crate::con::XcbConnection,
{
    fn query_version(
        &mut self,
        socket_buffer: &mut [u8],
        client_major_version: u16,
        client_minor_version: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xevie::QueryVersionReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xevie::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xevie::EXTENSION_NAME,
            ))?;
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

    fn start(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xevie::StartReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xevie::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xevie::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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

    fn end(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xevie::EndReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xevie::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xevie::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let cmap_bytes = cmap.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                2,
                length[0],
                length[1],
                cmap_bytes[0],
                cmap_bytes[1],
                cmap_bytes[2],
                cmap_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn send(
        &mut self,
        socket_buffer: &mut [u8],
        event: crate::proto::xevie::Event,
        data_type: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xevie::SendReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xevie::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xevie::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (26u16).to_ne_bytes();
        let event_bytes = event.serialize_fixed();
        let data_type_bytes = data_type.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..104)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
                length[0],
                length[1],
                event_bytes[0],
                event_bytes[1],
                event_bytes[2],
                event_bytes[3],
                event_bytes[4],
                event_bytes[5],
                event_bytes[6],
                event_bytes[7],
                event_bytes[8],
                event_bytes[9],
                event_bytes[10],
                event_bytes[11],
                event_bytes[12],
                event_bytes[13],
                event_bytes[14],
                event_bytes[15],
                event_bytes[16],
                event_bytes[17],
                event_bytes[18],
                event_bytes[19],
                event_bytes[20],
                event_bytes[21],
                event_bytes[22],
                event_bytes[23],
                event_bytes[24],
                event_bytes[25],
                event_bytes[26],
                event_bytes[27],
                event_bytes[28],
                event_bytes[29],
                event_bytes[30],
                event_bytes[31],
                data_type_bytes[0],
                data_type_bytes[1],
                data_type_bytes[2],
                data_type_bytes[3],
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
        self.advance_writer(104);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn select_input(
        &mut self,
        socket_buffer: &mut [u8],
        event_mask: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xevie::SelectInputReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xevie::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xevie::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let event_mask_bytes = event_mask.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                4,
                length[0],
                length[1],
                event_mask_bytes[0],
                event_mask_bytes[1],
                event_mask_bytes[2],
                event_mask_bytes[3],
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
