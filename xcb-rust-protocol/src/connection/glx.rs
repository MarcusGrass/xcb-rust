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
pub fn render<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    data: &[u8],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context_tag.serialize_fixed());
        let list_len = data.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            data,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 1]);
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
pub fn render_large<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    request_num: u16,
    request_total: u16,
    data: &[u8],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let data_len = u32::try_from(data.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context_tag.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&request_num.serialize_fixed());
        buf_ptr
            .get_mut(10..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&request_total.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(data_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = data.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            data,
        )?;
        let mut offset = list_len + 16;
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
pub fn create_context<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context: crate::proto::glx::Context,
    visual: crate::proto::xproto::Visualid,
    screen: u32,
    share_list: crate::proto::glx::Context,
    is_direct: u8,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (6u16).to_ne_bytes();
    let context_bytes = context.serialize_fixed();
    let visual_bytes = visual.serialize_fixed();
    let screen_bytes = screen.serialize_fixed();
    let share_list_bytes = share_list.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
                visual_bytes[0],
                visual_bytes[1],
                visual_bytes[2],
                visual_bytes[3],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
                share_list_bytes[0],
                share_list_bytes[1],
                share_list_bytes[2],
                share_list_bytes[3],
                is_direct,
                0,
                0,
                0,
            ]);
        Ok::<usize, crate::error::Error>(24)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(VoidCookie::new(seq))
}
pub fn destroy_context<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context: crate::proto::glx::Context,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let context_bytes = context.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                4,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
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
pub fn make_current<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    drawable: crate::proto::glx::Drawable,
    context: crate::proto::glx::Context,
    old_context_tag: crate::proto::glx::ContextTag,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::glx::MakeCurrentReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let drawable_bytes = drawable.serialize_fixed();
    let context_bytes = context.serialize_fixed();
    let old_context_tag_bytes = old_context_tag.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                5,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
                old_context_tag_bytes[0],
                old_context_tag_bytes[1],
                old_context_tag_bytes[2],
                old_context_tag_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(FixedCookie::new(seq))
}
pub fn is_direct<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context: crate::proto::glx::Context,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::glx::IsDirectReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let context_bytes = context.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                6,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
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
pub fn query_version<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    major_version: u32,
    minor_version: u32,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::glx::QueryVersionReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let major_version_bytes = major_version.serialize_fixed();
    let minor_version_bytes = minor_version.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                7,
                length[0],
                length[1],
                major_version_bytes[0],
                major_version_bytes[1],
                major_version_bytes[2],
                major_version_bytes[3],
                minor_version_bytes[0],
                minor_version_bytes[1],
                minor_version_bytes[2],
                minor_version_bytes[3],
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
pub fn wait_g_l<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                8,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
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
pub fn wait_x<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                9,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
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
pub fn copy_context<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    src: crate::proto::glx::Context,
    dest: crate::proto::glx::Context,
    mask: u32,
    src_context_tag: crate::proto::glx::ContextTag,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (5u16).to_ne_bytes();
    let src_bytes = src.serialize_fixed();
    let dest_bytes = dest.serialize_fixed();
    let mask_bytes = mask.serialize_fixed();
    let src_context_tag_bytes = src_context_tag.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                10,
                length[0],
                length[1],
                src_bytes[0],
                src_bytes[1],
                src_bytes[2],
                src_bytes[3],
                dest_bytes[0],
                dest_bytes[1],
                dest_bytes[2],
                dest_bytes[3],
                mask_bytes[0],
                mask_bytes[1],
                mask_bytes[2],
                mask_bytes[3],
                src_context_tag_bytes[0],
                src_context_tag_bytes[1],
                src_context_tag_bytes[2],
                src_context_tag_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(20)
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
    context_tag: crate::proto::glx::ContextTag,
    drawable: crate::proto::glx::Drawable,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let drawable_bytes = drawable.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                11,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(12)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(VoidCookie::new(seq))
}
pub fn use_x_font<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    font: crate::proto::xproto::Font,
    first: u32,
    count: u32,
    list_base: u32,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (6u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let font_bytes = font.serialize_fixed();
    let first_bytes = first.serialize_fixed();
    let count_bytes = count.serialize_fixed();
    let list_base_bytes = list_base.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                12,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                font_bytes[0],
                font_bytes[1],
                font_bytes[2],
                font_bytes[3],
                first_bytes[0],
                first_bytes[1],
                first_bytes[2],
                first_bytes[3],
                count_bytes[0],
                count_bytes[1],
                count_bytes[2],
                count_bytes[3],
                list_base_bytes[0],
                list_base_bytes[1],
                list_base_bytes[2],
                list_base_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(24)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(VoidCookie::new(seq))
}
pub fn create_g_l_x_pixmap<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    screen: u32,
    visual: crate::proto::xproto::Visualid,
    pixmap: crate::proto::xproto::Pixmap,
    glx_pixmap: crate::proto::glx::Pixmap,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (5u16).to_ne_bytes();
    let screen_bytes = screen.serialize_fixed();
    let visual_bytes = visual.serialize_fixed();
    let pixmap_bytes = pixmap.serialize_fixed();
    let glx_pixmap_bytes = glx_pixmap.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                13,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
                visual_bytes[0],
                visual_bytes[1],
                visual_bytes[2],
                visual_bytes[3],
                pixmap_bytes[0],
                pixmap_bytes[1],
                pixmap_bytes[2],
                pixmap_bytes[3],
                glx_pixmap_bytes[0],
                glx_pixmap_bytes[1],
                glx_pixmap_bytes[2],
                glx_pixmap_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(20)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(VoidCookie::new(seq))
}
pub fn get_visual_configs<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    screen: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetVisualConfigsReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let screen_bytes = screen.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                14,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(8)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn destroy_g_l_x_pixmap<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    glx_pixmap: crate::proto::glx::Pixmap,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let glx_pixmap_bytes = glx_pixmap.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                15,
                length[0],
                length[1],
                glx_pixmap_bytes[0],
                glx_pixmap_bytes[1],
                glx_pixmap_bytes[2],
                glx_pixmap_bytes[3],
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
pub fn vendor_private<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    vendor_code: u32,
    context_tag: crate::proto::glx::ContextTag,
    data: &[u8],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vendor_code.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context_tag.serialize_fixed());
        let list_len = data.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            data,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 16]);
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
pub fn vendor_private_with_reply<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    vendor_code: u32,
    context_tag: crate::proto::glx::ContextTag,
    data: &[u8],
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::VendorPrivateWithReplyReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vendor_code.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context_tag.serialize_fixed());
        let list_len = data.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            data,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 17]);
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
pub fn query_extensions_string<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    screen: u32,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::glx::QueryExtensionsStringReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let screen_bytes = screen.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                18,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
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
pub fn query_server_string<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    screen: u32,
    name: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::QueryServerStringReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let screen_bytes = screen.serialize_fixed();
    let name_bytes = name.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                19,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
                name_bytes[0],
                name_bytes[1],
                name_bytes[2],
                name_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(12)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn client_info<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    major_version: u32,
    minor_version: u32,
    string: &[u8],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let str_len = u32::try_from(string.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&major_version.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&minor_version.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(str_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = string.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            string,
        )?;
        let mut offset = list_len + 16;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 20]);
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
pub fn get_f_b_configs<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    screen: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetFBConfigsReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let screen_bytes = screen.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                21,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(8)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn create_pixmap<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    screen: u32,
    fbconfig: crate::proto::glx::Fbconfig,
    pixmap: crate::proto::xproto::Pixmap,
    glx_pixmap: crate::proto::glx::Pixmap,
    num_attribs: u32,
    attribs: &[u32],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let num_attribs = u32::try_from(num_attribs).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&screen.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&fbconfig.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&pixmap.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&glx_pixmap.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(core::ops::Div::div(num_attribs as u32, 2u32 as u32))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = attribs.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            attribs,
        )?;
        let mut offset = list_len + 24;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 22]);
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
pub fn destroy_pixmap<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    glx_pixmap: crate::proto::glx::Pixmap,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let glx_pixmap_bytes = glx_pixmap.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                23,
                length[0],
                length[1],
                glx_pixmap_bytes[0],
                glx_pixmap_bytes[1],
                glx_pixmap_bytes[2],
                glx_pixmap_bytes[3],
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
pub fn create_new_context<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context: crate::proto::glx::Context,
    fbconfig: crate::proto::glx::Fbconfig,
    screen: u32,
    render_type: u32,
    share_list: crate::proto::glx::Context,
    is_direct: u8,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (7u16).to_ne_bytes();
    let context_bytes = context.serialize_fixed();
    let fbconfig_bytes = fbconfig.serialize_fixed();
    let screen_bytes = screen.serialize_fixed();
    let render_type_bytes = render_type.serialize_fixed();
    let share_list_bytes = share_list.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                24,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
                fbconfig_bytes[0],
                fbconfig_bytes[1],
                fbconfig_bytes[2],
                fbconfig_bytes[3],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
                render_type_bytes[0],
                render_type_bytes[1],
                render_type_bytes[2],
                render_type_bytes[3],
                share_list_bytes[0],
                share_list_bytes[1],
                share_list_bytes[2],
                share_list_bytes[3],
                is_direct,
                0,
                0,
                0,
            ]);
        Ok::<usize, crate::error::Error>(28)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(VoidCookie::new(seq))
}
pub fn query_context<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context: crate::proto::glx::Context,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::QueryContextReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let context_bytes = context.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                25,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(8)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn make_context_current<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    old_context_tag: crate::proto::glx::ContextTag,
    drawable: crate::proto::glx::Drawable,
    read_drawable: crate::proto::glx::Drawable,
    context: crate::proto::glx::Context,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::glx::MakeContextCurrentReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (5u16).to_ne_bytes();
    let old_context_tag_bytes = old_context_tag.serialize_fixed();
    let drawable_bytes = drawable.serialize_fixed();
    let read_drawable_bytes = read_drawable.serialize_fixed();
    let context_bytes = context.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                26,
                length[0],
                length[1],
                old_context_tag_bytes[0],
                old_context_tag_bytes[1],
                old_context_tag_bytes[2],
                old_context_tag_bytes[3],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                read_drawable_bytes[0],
                read_drawable_bytes[1],
                read_drawable_bytes[2],
                read_drawable_bytes[3],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(20)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(FixedCookie::new(seq))
}
pub fn create_pbuffer<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    screen: u32,
    fbconfig: crate::proto::glx::Fbconfig,
    pbuffer: crate::proto::glx::Pbuffer,
    num_attribs: u32,
    attribs: &[u32],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let num_attribs = u32::try_from(num_attribs).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&screen.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&fbconfig.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&pbuffer.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(core::ops::Div::div(num_attribs as u32, 2u32 as u32))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = attribs.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(20..)
                .ok_or(crate::error::Error::Serialize)?,
            attribs,
        )?;
        let mut offset = list_len + 20;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 27]);
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
pub fn destroy_pbuffer<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    pbuffer: crate::proto::glx::Pbuffer,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let pbuffer_bytes = pbuffer.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                28,
                length[0],
                length[1],
                pbuffer_bytes[0],
                pbuffer_bytes[1],
                pbuffer_bytes[2],
                pbuffer_bytes[3],
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
pub fn get_drawable_attributes<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    drawable: crate::proto::glx::Drawable,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetDrawableAttributesReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let drawable_bytes = drawable.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                29,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(8)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn change_drawable_attributes<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    drawable: crate::proto::glx::Drawable,
    num_attribs: u32,
    attribs: &[u32],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let num_attribs = u32::try_from(num_attribs).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(core::ops::Div::div(num_attribs as u32, 2u32 as u32))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = attribs.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            attribs,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 30]);
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
pub fn create_window<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    screen: u32,
    fbconfig: crate::proto::glx::Fbconfig,
    window: crate::proto::xproto::Window,
    glx_window: crate::proto::glx::Window,
    num_attribs: u32,
    attribs: &[u32],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let num_attribs = u32::try_from(num_attribs).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&screen.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&fbconfig.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&window.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&glx_window.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(core::ops::Div::div(num_attribs as u32, 2u32 as u32))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = attribs.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            attribs,
        )?;
        let mut offset = list_len + 24;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 31]);
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
pub fn delete_window<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    glxwindow: crate::proto::glx::Window,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let glxwindow_bytes = glxwindow.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                32,
                length[0],
                length[1],
                glxwindow_bytes[0],
                glxwindow_bytes[1],
                glxwindow_bytes[2],
                glxwindow_bytes[3],
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
pub fn set_client_info_a_r_b<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    major_version: u32,
    minor_version: u32,
    num_versions: u32,
    gl_str_len: u32,
    glx_str_len: u32,
    gl_versions: &[u32],
    gl_extension_string: &[u8],
    glx_extension_string: &[u8],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let num_versions =
            u32::try_from(num_versions).map_err(|_| crate::error::Error::Serialize)?;
        let gl_str_len = u32::try_from(gl_str_len).map_err(|_| crate::error::Error::Serialize)?;
        let glx_str_len = u32::try_from(glx_str_len).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&major_version.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&minor_version.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(core::ops::Div::div(num_versions as u32, 2u32 as u32))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(gl_str_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(glx_str_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = gl_versions.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            gl_versions,
        )?;
        let mut offset = list_len + 24;
        let list_len = gl_extension_string.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            gl_extension_string,
        )?;
        offset += list_len;
        offset += (4 - (offset % 4)) % 4;
        let list_len = glx_extension_string.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            glx_extension_string,
        )?;
        offset += list_len;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 33]);
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
pub fn create_context_attribs_a_r_b<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context: crate::proto::glx::Context,
    fbconfig: crate::proto::glx::Fbconfig,
    screen: u32,
    share_list: crate::proto::glx::Context,
    is_direct: u8,
    num_attribs: u32,
    attribs: &[u32],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        // Pad 3 bytes
        let num_attribs = u32::try_from(num_attribs).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&fbconfig.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&screen.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&share_list.serialize_fixed());
        buf_ptr
            .get_mut(20..21)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&is_direct.serialize_fixed());
        buf_ptr
            .get_mut(24..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(core::ops::Div::div(num_attribs as u32, 2u32 as u32))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = attribs.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(28..)
                .ok_or(crate::error::Error::Serialize)?,
            attribs,
        )?;
        let mut offset = list_len + 28;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 34]);
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
pub fn set_client_info2_a_r_b<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    major_version: u32,
    minor_version: u32,
    num_versions: u32,
    gl_str_len: u32,
    glx_str_len: u32,
    gl_versions: &[u32],
    gl_extension_string: &[u8],
    glx_extension_string: &[u8],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let num_versions =
            u32::try_from(num_versions).map_err(|_| crate::error::Error::Serialize)?;
        let gl_str_len = u32::try_from(gl_str_len).map_err(|_| crate::error::Error::Serialize)?;
        let glx_str_len = u32::try_from(glx_str_len).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&major_version.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&minor_version.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(core::ops::Div::div(num_versions as u32, 3u32 as u32))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(gl_str_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(glx_str_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = gl_versions.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            gl_versions,
        )?;
        let mut offset = list_len + 24;
        let list_len = gl_extension_string.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            gl_extension_string,
        )?;
        offset += list_len;
        offset += (4 - (offset % 4)) % 4;
        let list_len = glx_extension_string.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            glx_extension_string,
        )?;
        offset += list_len;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 35]);
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
pub fn new_list<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    list: u32,
    mode: u32,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let list_bytes = list.serialize_fixed();
    let mode_bytes = mode.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                101,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                list_bytes[0],
                list_bytes[1],
                list_bytes[2],
                list_bytes[3],
                mode_bytes[0],
                mode_bytes[1],
                mode_bytes[2],
                mode_bytes[3],
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
pub fn end_list<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                102,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
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
pub fn delete_lists<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    list: u32,
    range: i32,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let list_bytes = list.serialize_fixed();
    let range_bytes = range.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                103,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                list_bytes[0],
                list_bytes[1],
                list_bytes[2],
                list_bytes[3],
                range_bytes[0],
                range_bytes[1],
                range_bytes[2],
                range_bytes[3],
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
pub fn gen_lists<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    range: i32,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::glx::GenListsReply, 12>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let range_bytes = range.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                104,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                range_bytes[0],
                range_bytes[1],
                range_bytes[2],
                range_bytes[3],
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
pub fn feedback_buffer<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    size: i32,
    r#type: i32,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let size_bytes = size.serialize_fixed();
    let r#type_bytes = r#type.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                105,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                size_bytes[0],
                size_bytes[1],
                size_bytes[2],
                size_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
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
pub fn select_buffer<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    size: i32,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let size_bytes = size.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                106,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                size_bytes[0],
                size_bytes[1],
                size_bytes[2],
                size_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(12)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(VoidCookie::new(seq))
}
pub fn render_mode<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    mode: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::RenderModeReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let mode_bytes = mode.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                107,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                mode_bytes[0],
                mode_bytes[1],
                mode_bytes[2],
                mode_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(12)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn finish<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::glx::FinishReply, 8>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                108,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
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
pub fn pixel_storef<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    pname: u32,
    datum: crate::proto::glx::Float32,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    let datum_bytes = datum.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                109,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
                datum_bytes[0],
                datum_bytes[1],
                datum_bytes[2],
                datum_bytes[3],
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
pub fn pixel_storei<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    pname: u32,
    datum: i32,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    let datum_bytes = datum.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                110,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
                datum_bytes[0],
                datum_bytes[1],
                datum_bytes[2],
                datum_bytes[3],
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
pub fn read_pixels<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    r#type: u32,
    swap_bytes: u8,
    lsb_first: u8,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::ReadPixelsReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (9u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let x_bytes = x.serialize_fixed();
    let y_bytes = y.serialize_fixed();
    let width_bytes = width.serialize_fixed();
    let height_bytes = height.serialize_fixed();
    let format_bytes = format.serialize_fixed();
    let r#type_bytes = r#type.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..36)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                111,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                x_bytes[0],
                x_bytes[1],
                x_bytes[2],
                x_bytes[3],
                y_bytes[0],
                y_bytes[1],
                y_bytes[2],
                y_bytes[3],
                width_bytes[0],
                width_bytes[1],
                width_bytes[2],
                width_bytes[3],
                height_bytes[0],
                height_bytes[1],
                height_bytes[2],
                height_bytes[3],
                format_bytes[0],
                format_bytes[1],
                format_bytes[2],
                format_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                swap_bytes,
                lsb_first,
                0,
                0,
            ]);
        Ok::<usize, crate::error::Error>(36)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_booleanv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    pname: i32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetBooleanvReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                112,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(12)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_clip_plane<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    plane: i32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetClipPlaneReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let plane_bytes = plane.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                113,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                plane_bytes[0],
                plane_bytes[1],
                plane_bytes[2],
                plane_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(12)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_doublev<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetDoublevReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                114,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(12)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_error<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::glx::GetErrorReply, 12>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                115,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
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
pub fn get_floatv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetFloatvReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                116,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(12)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_integerv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetIntegervReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                117,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(12)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_lightfv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    light: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetLightfvReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let light_bytes = light.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                118,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                light_bytes[0],
                light_bytes[1],
                light_bytes[2],
                light_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_lightiv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    light: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetLightivReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let light_bytes = light.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                119,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                light_bytes[0],
                light_bytes[1],
                light_bytes[2],
                light_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_mapdv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    query: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetMapdvReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let query_bytes = query.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                120,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                query_bytes[0],
                query_bytes[1],
                query_bytes[2],
                query_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_mapfv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    query: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetMapfvReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let query_bytes = query.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                121,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                query_bytes[0],
                query_bytes[1],
                query_bytes[2],
                query_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_mapiv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    query: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetMapivReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let query_bytes = query.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                122,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                query_bytes[0],
                query_bytes[1],
                query_bytes[2],
                query_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_materialfv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    face: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetMaterialfvReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let face_bytes = face.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                123,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                face_bytes[0],
                face_bytes[1],
                face_bytes[2],
                face_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_materialiv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    face: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetMaterialivReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let face_bytes = face.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                124,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                face_bytes[0],
                face_bytes[1],
                face_bytes[2],
                face_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_pixel_mapfv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    map: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetPixelMapfvReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let map_bytes = map.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                125,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                map_bytes[0],
                map_bytes[1],
                map_bytes[2],
                map_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(12)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_pixel_mapuiv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    map: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetPixelMapuivReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let map_bytes = map.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                126,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                map_bytes[0],
                map_bytes[1],
                map_bytes[2],
                map_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(12)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_pixel_mapusv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    map: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetPixelMapusvReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let map_bytes = map.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                127,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                map_bytes[0],
                map_bytes[1],
                map_bytes[2],
                map_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(12)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_polygon_stipple<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    lsb_first: u8,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetPolygonStippleReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                128,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                lsb_first,
                0,
                0,
                0,
            ]);
        Ok::<usize, crate::error::Error>(12)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_string<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    name: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetStringReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let name_bytes = name.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                129,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                name_bytes[0],
                name_bytes[1],
                name_bytes[2],
                name_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(12)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_tex_envfv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetTexEnvfvReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                130,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_tex_enviv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetTexEnvivReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                131,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_tex_gendv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    coord: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetTexGendvReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let coord_bytes = coord.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                132,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                coord_bytes[0],
                coord_bytes[1],
                coord_bytes[2],
                coord_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_tex_genfv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    coord: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetTexGenfvReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let coord_bytes = coord.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                133,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                coord_bytes[0],
                coord_bytes[1],
                coord_bytes[2],
                coord_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_tex_geniv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    coord: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetTexGenivReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let coord_bytes = coord.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                134,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                coord_bytes[0],
                coord_bytes[1],
                coord_bytes[2],
                coord_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_tex_image<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    level: i32,
    format: u32,
    r#type: u32,
    swap_bytes: u8,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetTexImageReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (7u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let level_bytes = level.serialize_fixed();
    let format_bytes = format.serialize_fixed();
    let r#type_bytes = r#type.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                135,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                level_bytes[0],
                level_bytes[1],
                level_bytes[2],
                level_bytes[3],
                format_bytes[0],
                format_bytes[1],
                format_bytes[2],
                format_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                swap_bytes,
                0,
                0,
                0,
            ]);
        Ok::<usize, crate::error::Error>(28)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_tex_parameterfv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetTexParameterfvReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                136,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_tex_parameteriv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetTexParameterivReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                137,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_tex_level_parameterfv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    level: i32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetTexLevelParameterfvReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (5u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let level_bytes = level.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                138,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                level_bytes[0],
                level_bytes[1],
                level_bytes[2],
                level_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(20)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_tex_level_parameteriv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    level: i32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetTexLevelParameterivReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (5u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let level_bytes = level.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                139,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                level_bytes[0],
                level_bytes[1],
                level_bytes[2],
                level_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(20)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn is_enabled<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    capability: u32,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::glx::IsEnabledReply, 12>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let capability_bytes = capability.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                140,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                capability_bytes[0],
                capability_bytes[1],
                capability_bytes[2],
                capability_bytes[3],
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
pub fn is_list<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    list: u32,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::glx::IsListReply, 12>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let list_bytes = list.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                141,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                list_bytes[0],
                list_bytes[1],
                list_bytes[2],
                list_bytes[3],
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
pub fn flush<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                142,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
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
pub fn are_textures_resident<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    textures: &[u32],
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::AreTexturesResidentReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let n = u32::try_from(textures.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context_tag.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(n).map_err(|_| crate::error::Error::Serialize)?).serialize_fixed(),
            );
        let list_len = textures.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            textures,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 143]);
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
pub fn delete_textures<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    textures: &[u32],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let n = u32::try_from(textures.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context_tag.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(n).map_err(|_| crate::error::Error::Serialize)?).serialize_fixed(),
            );
        let list_len = textures.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            textures,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 144]);
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
pub fn gen_textures<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    n: i32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GenTexturesReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let n_bytes = n.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                145,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                n_bytes[0],
                n_bytes[1],
                n_bytes[2],
                n_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(12)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn is_texture<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    texture: u32,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::glx::IsTextureReply, 12>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let texture_bytes = texture.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                146,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                texture_bytes[0],
                texture_bytes[1],
                texture_bytes[2],
                texture_bytes[3],
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
pub fn get_color_table<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    format: u32,
    r#type: u32,
    swap_bytes: u8,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetColorTableReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (6u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let format_bytes = format.serialize_fixed();
    let r#type_bytes = r#type.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                147,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                format_bytes[0],
                format_bytes[1],
                format_bytes[2],
                format_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                swap_bytes,
                0,
                0,
                0,
            ]);
        Ok::<usize, crate::error::Error>(24)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_color_table_parameterfv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetColorTableParameterfvReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                148,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_color_table_parameteriv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetColorTableParameterivReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                149,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_convolution_filter<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    format: u32,
    r#type: u32,
    swap_bytes: u8,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetConvolutionFilterReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (6u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let format_bytes = format.serialize_fixed();
    let r#type_bytes = r#type.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                150,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                format_bytes[0],
                format_bytes[1],
                format_bytes[2],
                format_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                swap_bytes,
                0,
                0,
                0,
            ]);
        Ok::<usize, crate::error::Error>(24)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_convolution_parameterfv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetConvolutionParameterfvReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                151,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_convolution_parameteriv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetConvolutionParameterivReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                152,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_separable_filter<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    format: u32,
    r#type: u32,
    swap_bytes: u8,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetSeparableFilterReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (6u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let format_bytes = format.serialize_fixed();
    let r#type_bytes = r#type.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                153,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                format_bytes[0],
                format_bytes[1],
                format_bytes[2],
                format_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                swap_bytes,
                0,
                0,
                0,
            ]);
        Ok::<usize, crate::error::Error>(24)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_histogram<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    format: u32,
    r#type: u32,
    swap_bytes: u8,
    reset: u8,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetHistogramReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (6u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let format_bytes = format.serialize_fixed();
    let r#type_bytes = r#type.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                154,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                format_bytes[0],
                format_bytes[1],
                format_bytes[2],
                format_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                swap_bytes,
                reset,
                0,
                0,
            ]);
        Ok::<usize, crate::error::Error>(24)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_histogram_parameterfv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetHistogramParameterfvReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                155,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_histogram_parameteriv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetHistogramParameterivReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                156,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_minmax<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    format: u32,
    r#type: u32,
    swap_bytes: u8,
    reset: u8,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetMinmaxReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (6u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let format_bytes = format.serialize_fixed();
    let r#type_bytes = r#type.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                157,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                format_bytes[0],
                format_bytes[1],
                format_bytes[2],
                format_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                swap_bytes,
                reset,
                0,
                0,
            ]);
        Ok::<usize, crate::error::Error>(24)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_minmax_parameterfv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetMinmaxParameterfvReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                158,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_minmax_parameteriv<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetMinmaxParameterivReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                159,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_compressed_tex_image_a_r_b<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    level: i32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetCompressedTexImageARBReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let level_bytes = level.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                160,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                level_bytes[0],
                level_bytes[1],
                level_bytes[2],
                level_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn delete_queries_a_r_b<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    ids: &[u32],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let n = u32::try_from(ids.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context_tag.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(n).map_err(|_| crate::error::Error::Serialize)?).serialize_fixed(),
            );
        let list_len = ids.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            ids,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 161]);
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
pub fn gen_queries_a_r_b<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    n: i32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GenQueriesARBReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let n_bytes = n.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                162,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                n_bytes[0],
                n_bytes[1],
                n_bytes[2],
                n_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(12)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn is_query_a_r_b<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    id: u32,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::glx::IsQueryARBReply, 12>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let id_bytes = id.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                163,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                id_bytes[0],
                id_bytes[1],
                id_bytes[2],
                id_bytes[3],
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
pub fn get_queryiv_a_r_b<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetQueryivARBReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let target_bytes = target.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                164,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_query_objectiv_a_r_b<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    id: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetQueryObjectivARBReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let id_bytes = id.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                165,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                id_bytes[0],
                id_bytes[1],
                id_bytes[2],
                id_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
pub fn get_query_objectuiv_a_r_b<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    context_tag: crate::proto::glx::ContextTag,
    id: u32,
    pname: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::glx::GetQueryObjectuivARBReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::glx::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::glx::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let context_tag_bytes = context_tag.serialize_fixed();
    let id_bytes = id.serialize_fixed();
    let pname_bytes = pname.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                166,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                id_bytes[0],
                id_bytes[1],
                id_bytes[2],
                id_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(16)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(Cookie::new(seq))
}
