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
pub fn query_version<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    major: u8,
    minor: u8,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::xinerama::QueryVersionReply, 12>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xinerama::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xinerama::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 0, length[0], length[1], major, minor, 0, 0]);
        Ok::<usize, crate::error::Error>(8)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(FixedCookie::new(seq))
}
pub fn get_state<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    window: crate::proto::xproto::Window,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::xinerama::GetStateReply, 12>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xinerama::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xinerama::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let window_bytes = window.serialize_fixed();
    io.use_write_buffer(|buf| {
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
        Ok::<usize, crate::error::Error>(8)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(FixedCookie::new(seq))
}
pub fn get_screen_count<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    window: crate::proto::xproto::Window,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::xinerama::GetScreenCountReply, 12>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xinerama::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xinerama::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let window_bytes = window.serialize_fixed();
    io.use_write_buffer(|buf| {
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
        Ok::<usize, crate::error::Error>(8)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(FixedCookie::new(seq))
}
pub fn get_screen_size<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    window: crate::proto::xproto::Window,
    screen: u32,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::xinerama::GetScreenSizeReply, 24>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xinerama::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xinerama::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let window_bytes = window.serialize_fixed();
    let screen_bytes = screen.serialize_fixed();
    io.use_write_buffer(|buf| {
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
        Ok::<usize, crate::error::Error>(12)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(FixedCookie::new(seq))
}
pub fn is_active<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::xinerama::IsActiveReply, 12>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xinerama::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xinerama::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf| {
        let buf = buf.get_mut(..4).ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 4;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        Ok::<usize, crate::error::Error>(4)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(FixedCookie::new(seq))
}
pub fn query_screens<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::xinerama::QueryScreensReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xinerama::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xinerama::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf| {
        let buf = buf.get_mut(..4).ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 5;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        Ok::<usize, crate::error::Error>(4)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
