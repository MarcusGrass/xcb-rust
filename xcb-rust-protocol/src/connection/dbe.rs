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
    major_version: u8,
    minor_version: u8,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::dbe::QueryVersionReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dbe::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dbe::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                0,
                length[0],
                length[1],
                major_version,
                minor_version,
                0,
                0,
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
pub fn allocate_back_buffer<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    window: crate::proto::xproto::Window,
    buffer: crate::proto::dbe::BackBuffer,
    swap_action: u8,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dbe::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dbe::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let window_bytes = window.serialize_fixed();
    let buffer_bytes = buffer.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
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
                buffer_bytes[0],
                buffer_bytes[1],
                buffer_bytes[2],
                buffer_bytes[3],
                swap_action,
                0,
                0,
                0,
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(VoidCookie::new(seq))
}
pub fn deallocate_back_buffer<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    buffer: crate::proto::dbe::BackBuffer,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dbe::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dbe::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let buffer_bytes = buffer.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                2,
                length[0],
                length[1],
                buffer_bytes[0],
                buffer_bytes[1],
                buffer_bytes[2],
                buffer_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(8)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(VoidCookie::new(seq))
}
pub fn swap_buffers<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    actions: &[crate::proto::dbe::SwapInfo],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dbe::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dbe::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let n_actions = u32::try_from(actions.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(n_actions).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = actions.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            actions,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 3]);
        debug_assert!(offset % 4 == 0, "Bad request length offset {offset}");
        let word_len = offset / 4;
        if let Ok(len) = u16::try_from(word_len) {
            let length: [u8; 2] = len.to_ne_bytes();
            buf_ptr
                .get_mut(2..4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&length);
        } else {
            if word_len > xcb_state.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
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
        Ok::<usize, crate::error::Error>(offset)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(VoidCookie::new(seq))
}
pub fn begin_idiom<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dbe::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dbe::EXTENSION_NAME,
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
    Ok(VoidCookie::new(seq))
}
pub fn end_idiom<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dbe::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dbe::EXTENSION_NAME,
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
    Ok(VoidCookie::new(seq))
}
pub fn get_visual_info<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    drawables: &[crate::proto::xproto::Drawable],
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::dbe::GetVisualInfoReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dbe::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dbe::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let n_drawables =
            u32::try_from(drawables.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(n_drawables).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = drawables.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            drawables,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 6]);
        debug_assert!(offset % 4 == 0, "Bad request length offset {offset}");
        let word_len = offset / 4;
        if let Ok(len) = u16::try_from(word_len) {
            let length: [u8; 2] = len.to_ne_bytes();
            buf_ptr
                .get_mut(2..4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&length);
        } else {
            if word_len > xcb_state.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
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
        Ok::<usize, crate::error::Error>(offset)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_back_buffer_attributes<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    buffer: crate::proto::dbe::BackBuffer,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::dbe::GetBackBufferAttributesReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dbe::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dbe::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let buffer_bytes = buffer.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                7,
                length[0],
                length[1],
                buffer_bytes[0],
                buffer_bytes[1],
                buffer_bytes[2],
                buffer_bytes[3],
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
