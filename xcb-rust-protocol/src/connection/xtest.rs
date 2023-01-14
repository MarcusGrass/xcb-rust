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
pub trait XtestConnection {
    fn get_version(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u8,
        minor_version: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xtest::GetVersionReply, 10>>;

    fn compare_cursor(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        cursor: crate::proto::xproto::Cursor,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xtest::CompareCursorReply, 8>>;

    fn fake_input(
        &mut self,
        socket_buffer: &mut [u8],
        r#type: u8,
        detail: u8,
        time: u32,
        root: crate::proto::xproto::Window,
        root_x: i16,
        root_y: i16,
        deviceid: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn grab_control(
        &mut self,
        socket_buffer: &mut [u8],
        impervious: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;
}
impl<C> XtestConnection for C
where
    C: crate::con::XcbConnection,
{
    fn get_version(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u8,
        minor_version: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xtest::GetVersionReply, 10>> {
        let major_opcode = self
            .major_opcode(crate::proto::xtest::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xtest::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let minor_version_bytes = minor_version.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                0,
                length[0],
                length[1],
                major_version,
                0,
                minor_version_bytes[0],
                minor_version_bytes[1],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn compare_cursor(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        cursor: crate::proto::xproto::Cursor,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xtest::CompareCursorReply, 8>> {
        let major_opcode = self
            .major_opcode(crate::proto::xtest::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xtest::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let cursor_bytes = cursor.serialize_fixed();
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
                cursor_bytes[0],
                cursor_bytes[1],
                cursor_bytes[2],
                cursor_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn fake_input(
        &mut self,
        socket_buffer: &mut [u8],
        r#type: u8,
        detail: u8,
        time: u32,
        root: crate::proto::xproto::Window,
        root_x: i16,
        root_y: i16,
        deviceid: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xtest::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xtest::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (9u16).to_ne_bytes();
        let time_bytes = time.serialize_fixed();
        let root_bytes = root.serialize_fixed();
        let root_x_bytes = root_x.serialize_fixed();
        let root_y_bytes = root_y.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..36)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                2,
                length[0],
                length[1],
                r#type,
                detail,
                0,
                0,
                time_bytes[0],
                time_bytes[1],
                time_bytes[2],
                time_bytes[3],
                root_bytes[0],
                root_bytes[1],
                root_bytes[2],
                root_bytes[3],
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                root_x_bytes[0],
                root_x_bytes[1],
                root_y_bytes[0],
                root_y_bytes[1],
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                deviceid,
            ]);
        self.advance_writer(36);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn grab_control(
        &mut self,
        socket_buffer: &mut [u8],
        impervious: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xtest::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xtest::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 3, length[0], length[1], impervious, 0, 0, 0]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }
}
