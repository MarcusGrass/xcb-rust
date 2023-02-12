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
    client_major_version: u32,
    client_minor_version: u32,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::render::QueryVersionReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let client_major_version_bytes = client_major_version.serialize_fixed();
    let client_minor_version_bytes = client_minor_version.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                0,
                length[0],
                length[1],
                client_major_version_bytes[0],
                client_major_version_bytes[1],
                client_major_version_bytes[2],
                client_major_version_bytes[3],
                client_minor_version_bytes[0],
                client_minor_version_bytes[1],
                client_minor_version_bytes[2],
                client_minor_version_bytes[3],
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
pub fn query_pict_formats<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::render::QueryPictFormatsReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf| {
        let buf = buf.get_mut(..4).ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 1;
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
pub fn query_pict_index_values<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    format: crate::proto::render::Pictformat,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::render::QueryPictIndexValuesReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let format_bytes = format.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                2,
                length[0],
                length[1],
                format_bytes[0],
                format_bytes[1],
                format_bytes[2],
                format_bytes[3],
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
pub fn create_picture<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    pid: crate::proto::render::Picture,
    drawable: crate::proto::xproto::Drawable,
    format: crate::proto::render::Pictformat,
    create_picture_value_list: crate::proto::render::CreatePictureValueList,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 4]);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&pid.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&format.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&create_picture_value_list.switch_expr().serialize_fixed());
        let mut offset = 20
            + create_picture_value_list.serialize_into(
                buf_ptr
                    .get_mut(20..)
                    .ok_or(crate::error::Error::Serialize)?,
            )?;
        debug_assert!(offset % 4 == 0, "Bad request length offset {offset}");
        let word_len = offset / 4;
        let len = u16::try_from(word_len).map_err(|_| crate::error::Error::Serialize)?;
        let length: [u8; 2] = len.to_ne_bytes();
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&length);
        Ok::<usize, crate::error::Error>(offset)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(VoidCookie::new(seq))
}
pub fn change_picture<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    picture: crate::proto::render::Picture,
    change_picture_value_list: crate::proto::render::ChangePictureValueList,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 5]);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&picture.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&change_picture_value_list.switch_expr().serialize_fixed());
        let mut offset = 12
            + change_picture_value_list.serialize_into(
                buf_ptr
                    .get_mut(12..)
                    .ok_or(crate::error::Error::Serialize)?,
            )?;
        debug_assert!(offset % 4 == 0, "Bad request length offset {offset}");
        let word_len = offset / 4;
        let len = u16::try_from(word_len).map_err(|_| crate::error::Error::Serialize)?;
        let length: [u8; 2] = len.to_ne_bytes();
        buf_ptr
            .get_mut(2..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&length);
        Ok::<usize, crate::error::Error>(offset)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(VoidCookie::new(seq))
}
pub fn set_picture_clip_rectangles<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    picture: crate::proto::render::Picture,
    clip_x_origin: i16,
    clip_y_origin: i16,
    rectangles: &[crate::proto::xproto::Rectangle],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&picture.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&clip_x_origin.serialize_fixed());
        buf_ptr
            .get_mut(10..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&clip_y_origin.serialize_fixed());
        let list_len = rectangles.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            rectangles,
        )?;
        let mut offset = list_len + 12;
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
    Ok(VoidCookie::new(seq))
}
pub fn free_picture<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    picture: crate::proto::render::Picture,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let picture_bytes = picture.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                7,
                length[0],
                length[1],
                picture_bytes[0],
                picture_bytes[1],
                picture_bytes[2],
                picture_bytes[3],
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
pub fn composite<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    op: crate::proto::render::PictOpEnum,
    src: crate::proto::render::Picture,
    mask: crate::proto::render::PictureEnum,
    dst: crate::proto::render::Picture,
    src_x: i16,
    src_y: i16,
    mask_x: i16,
    mask_y: i16,
    dst_x: i16,
    dst_y: i16,
    width: u16,
    height: u16,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (9u16).to_ne_bytes();
    let src_bytes = src.serialize_fixed();
    let mask_bytes = (mask.0 as u32).serialize_fixed();
    let dst_bytes = dst.serialize_fixed();
    let src_x_bytes = src_x.serialize_fixed();
    let src_y_bytes = src_y.serialize_fixed();
    let mask_x_bytes = mask_x.serialize_fixed();
    let mask_y_bytes = mask_y.serialize_fixed();
    let dst_x_bytes = dst_x.serialize_fixed();
    let dst_y_bytes = dst_y.serialize_fixed();
    let width_bytes = width.serialize_fixed();
    let height_bytes = height.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..36)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                8,
                length[0],
                length[1],
                op.0 as u8,
                0,
                0,
                0,
                src_bytes[0],
                src_bytes[1],
                src_bytes[2],
                src_bytes[3],
                mask_bytes[0],
                mask_bytes[1],
                mask_bytes[2],
                mask_bytes[3],
                dst_bytes[0],
                dst_bytes[1],
                dst_bytes[2],
                dst_bytes[3],
                src_x_bytes[0],
                src_x_bytes[1],
                src_y_bytes[0],
                src_y_bytes[1],
                mask_x_bytes[0],
                mask_x_bytes[1],
                mask_y_bytes[0],
                mask_y_bytes[1],
                dst_x_bytes[0],
                dst_x_bytes[1],
                dst_y_bytes[0],
                dst_y_bytes[1],
                width_bytes[0],
                width_bytes[1],
                height_bytes[0],
                height_bytes[1],
            ]);
        Ok::<usize, crate::error::Error>(36)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(VoidCookie::new(seq))
}
pub fn trapezoids<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    op: crate::proto::render::PictOpEnum,
    src: crate::proto::render::Picture,
    dst: crate::proto::render::Picture,
    mask_format: crate::proto::render::Pictformat,
    src_x: i16,
    src_y: i16,
    traps: &[crate::proto::render::Trapezoid],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        // Pad 3 bytes
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&op.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&dst.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&mask_format.serialize_fixed());
        buf_ptr
            .get_mut(20..22)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_x.serialize_fixed());
        buf_ptr
            .get_mut(22..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_y.serialize_fixed());
        let list_len = traps.len() * 40;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            traps,
        )?;
        let mut offset = list_len + 24;
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
pub fn triangles<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    op: crate::proto::render::PictOpEnum,
    src: crate::proto::render::Picture,
    dst: crate::proto::render::Picture,
    mask_format: crate::proto::render::Pictformat,
    src_x: i16,
    src_y: i16,
    triangles: &[crate::proto::render::Triangle],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        // Pad 3 bytes
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&op.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&dst.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&mask_format.serialize_fixed());
        buf_ptr
            .get_mut(20..22)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_x.serialize_fixed());
        buf_ptr
            .get_mut(22..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_y.serialize_fixed());
        let list_len = triangles.len() * 24;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            triangles,
        )?;
        let mut offset = list_len + 24;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 11]);
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
pub fn tri_strip<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    op: crate::proto::render::PictOpEnum,
    src: crate::proto::render::Picture,
    dst: crate::proto::render::Picture,
    mask_format: crate::proto::render::Pictformat,
    src_x: i16,
    src_y: i16,
    points: &[crate::proto::render::Pointfix],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        // Pad 3 bytes
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&op.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&dst.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&mask_format.serialize_fixed());
        buf_ptr
            .get_mut(20..22)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_x.serialize_fixed());
        buf_ptr
            .get_mut(22..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_y.serialize_fixed());
        let list_len = points.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            points,
        )?;
        let mut offset = list_len + 24;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 12]);
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
pub fn tri_fan<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    op: crate::proto::render::PictOpEnum,
    src: crate::proto::render::Picture,
    dst: crate::proto::render::Picture,
    mask_format: crate::proto::render::Pictformat,
    src_x: i16,
    src_y: i16,
    points: &[crate::proto::render::Pointfix],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        // Pad 3 bytes
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&op.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&dst.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&mask_format.serialize_fixed());
        buf_ptr
            .get_mut(20..22)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_x.serialize_fixed());
        buf_ptr
            .get_mut(22..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_y.serialize_fixed());
        let list_len = points.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            points,
        )?;
        let mut offset = list_len + 24;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 13]);
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
pub fn create_glyph_set<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    gsid: crate::proto::render::Glyphset,
    format: crate::proto::render::Pictformat,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let gsid_bytes = gsid.serialize_fixed();
    let format_bytes = format.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                17,
                length[0],
                length[1],
                gsid_bytes[0],
                gsid_bytes[1],
                gsid_bytes[2],
                gsid_bytes[3],
                format_bytes[0],
                format_bytes[1],
                format_bytes[2],
                format_bytes[3],
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
pub fn reference_glyph_set<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    gsid: crate::proto::render::Glyphset,
    existing: crate::proto::render::Glyphset,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let gsid_bytes = gsid.serialize_fixed();
    let existing_bytes = existing.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                18,
                length[0],
                length[1],
                gsid_bytes[0],
                gsid_bytes[1],
                gsid_bytes[2],
                gsid_bytes[3],
                existing_bytes[0],
                existing_bytes[1],
                existing_bytes[2],
                existing_bytes[3],
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
pub fn free_glyph_set<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    glyphset: crate::proto::render::Glyphset,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let glyphset_bytes = glyphset.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                19,
                length[0],
                length[1],
                glyphset_bytes[0],
                glyphset_bytes[1],
                glyphset_bytes[2],
                glyphset_bytes[3],
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
pub fn add_glyphs<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    glyphset: crate::proto::render::Glyphset,
    glyphs_len: u32,
    glyphids: &[u32],
    glyphs: &[crate::proto::render::Glyphinfo],
    data: &[u8],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let glyphs_len = u32::try_from(glyphs_len).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&glyphset.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(glyphs_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = glyphids.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            glyphids,
        )?;
        let mut offset = list_len + 12;
        let list_len = glyphs.len() * 12;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            glyphs,
        )?;
        offset += list_len;
        let list_len = data.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            data,
        )?;
        offset += list_len;
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
pub fn free_glyphs<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    glyphset: crate::proto::render::Glyphset,
    glyphs: &[crate::proto::render::Glyph],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&glyphset.serialize_fixed());
        let list_len = glyphs.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            glyphs,
        )?;
        let mut offset = list_len + 8;
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
pub fn composite_glyphs8<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    op: crate::proto::render::PictOpEnum,
    src: crate::proto::render::Picture,
    dst: crate::proto::render::Picture,
    mask_format: crate::proto::render::Pictformat,
    glyphset: crate::proto::render::Glyphset,
    src_x: i16,
    src_y: i16,
    glyphcmds: &[u8],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        // Pad 3 bytes
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&op.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&dst.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&mask_format.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&glyphset.serialize_fixed());
        buf_ptr
            .get_mut(24..26)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_x.serialize_fixed());
        buf_ptr
            .get_mut(26..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_y.serialize_fixed());
        let list_len = glyphcmds.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(28..)
                .ok_or(crate::error::Error::Serialize)?,
            glyphcmds,
        )?;
        let mut offset = list_len + 28;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 23]);
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
pub fn composite_glyphs16<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    op: crate::proto::render::PictOpEnum,
    src: crate::proto::render::Picture,
    dst: crate::proto::render::Picture,
    mask_format: crate::proto::render::Pictformat,
    glyphset: crate::proto::render::Glyphset,
    src_x: i16,
    src_y: i16,
    glyphcmds: &[u8],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        // Pad 3 bytes
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&op.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&dst.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&mask_format.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&glyphset.serialize_fixed());
        buf_ptr
            .get_mut(24..26)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_x.serialize_fixed());
        buf_ptr
            .get_mut(26..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_y.serialize_fixed());
        let list_len = glyphcmds.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(28..)
                .ok_or(crate::error::Error::Serialize)?,
            glyphcmds,
        )?;
        let mut offset = list_len + 28;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 24]);
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
pub fn composite_glyphs32<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    op: crate::proto::render::PictOpEnum,
    src: crate::proto::render::Picture,
    dst: crate::proto::render::Picture,
    mask_format: crate::proto::render::Pictformat,
    glyphset: crate::proto::render::Glyphset,
    src_x: i16,
    src_y: i16,
    glyphcmds: &[u8],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        // Pad 3 bytes
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&op.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&dst.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&mask_format.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&glyphset.serialize_fixed());
        buf_ptr
            .get_mut(24..26)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_x.serialize_fixed());
        buf_ptr
            .get_mut(26..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_y.serialize_fixed());
        let list_len = glyphcmds.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(28..)
                .ok_or(crate::error::Error::Serialize)?,
            glyphcmds,
        )?;
        let mut offset = list_len + 28;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 25]);
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
pub fn fill_rectangles<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    op: crate::proto::render::PictOpEnum,
    dst: crate::proto::render::Picture,
    color: crate::proto::render::Color,
    rects: &[crate::proto::xproto::Rectangle],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        // Pad 3 bytes
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&op.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&dst.serialize_fixed());
        buf_ptr
            .get_mut(12..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&color.serialize_fixed());
        let list_len = rects.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(20..)
                .ok_or(crate::error::Error::Serialize)?,
            rects,
        )?;
        let mut offset = list_len + 20;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 26]);
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
pub fn create_cursor<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    cid: crate::proto::xproto::Cursor,
    source: crate::proto::render::Picture,
    x: u16,
    y: u16,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let cid_bytes = cid.serialize_fixed();
    let source_bytes = source.serialize_fixed();
    let x_bytes = x.serialize_fixed();
    let y_bytes = y.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                27,
                length[0],
                length[1],
                cid_bytes[0],
                cid_bytes[1],
                cid_bytes[2],
                cid_bytes[3],
                source_bytes[0],
                source_bytes[1],
                source_bytes[2],
                source_bytes[3],
                x_bytes[0],
                x_bytes[1],
                y_bytes[0],
                y_bytes[1],
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
pub fn set_picture_transform<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    picture: crate::proto::render::Picture,
    transform: crate::proto::render::Transform,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (11u16).to_ne_bytes();
    let picture_bytes = picture.serialize_fixed();
    let transform_bytes = transform.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..44)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                28,
                length[0],
                length[1],
                picture_bytes[0],
                picture_bytes[1],
                picture_bytes[2],
                picture_bytes[3],
                transform_bytes[0],
                transform_bytes[1],
                transform_bytes[2],
                transform_bytes[3],
                transform_bytes[4],
                transform_bytes[5],
                transform_bytes[6],
                transform_bytes[7],
                transform_bytes[8],
                transform_bytes[9],
                transform_bytes[10],
                transform_bytes[11],
                transform_bytes[12],
                transform_bytes[13],
                transform_bytes[14],
                transform_bytes[15],
                transform_bytes[16],
                transform_bytes[17],
                transform_bytes[18],
                transform_bytes[19],
                transform_bytes[20],
                transform_bytes[21],
                transform_bytes[22],
                transform_bytes[23],
                transform_bytes[24],
                transform_bytes[25],
                transform_bytes[26],
                transform_bytes[27],
                transform_bytes[28],
                transform_bytes[29],
                transform_bytes[30],
                transform_bytes[31],
                transform_bytes[32],
                transform_bytes[33],
                transform_bytes[34],
                transform_bytes[35],
            ]);
        Ok::<usize, crate::error::Error>(44)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(VoidCookie::new(seq))
}
pub fn query_filters<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    drawable: crate::proto::xproto::Drawable,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::render::QueryFiltersReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
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
pub fn set_picture_filter<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    picture: crate::proto::render::Picture,
    filter: &[u8],
    values: &[crate::proto::render::Fixed],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let filter_len = u16::try_from(filter.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&picture.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(filter_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = filter.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            filter,
        )?;
        let mut offset = list_len + 12;
        offset += (4 - (offset % 4)) % 4;
        let list_len = values.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            values,
        )?;
        offset += list_len;
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
pub fn create_anim_cursor<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    cid: crate::proto::xproto::Cursor,
    cursors: &[crate::proto::render::Animcursorelt],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&cid.serialize_fixed());
        let list_len = cursors.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            cursors,
        )?;
        let mut offset = list_len + 8;
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
pub fn add_traps<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    picture: crate::proto::render::Picture,
    x_off: i16,
    y_off: i16,
    traps: &[crate::proto::render::Trap],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&picture.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&x_off.serialize_fixed());
        buf_ptr
            .get_mut(10..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&y_off.serialize_fixed());
        let list_len = traps.len() * 24;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            traps,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 32]);
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
pub fn create_solid_fill<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    picture: crate::proto::render::Picture,
    color: crate::proto::render::Color,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let picture_bytes = picture.serialize_fixed();
    let color_bytes = color.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                33,
                length[0],
                length[1],
                picture_bytes[0],
                picture_bytes[1],
                picture_bytes[2],
                picture_bytes[3],
                color_bytes[0],
                color_bytes[1],
                color_bytes[2],
                color_bytes[3],
                color_bytes[4],
                color_bytes[5],
                color_bytes[6],
                color_bytes[7],
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
pub fn create_linear_gradient<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    picture: crate::proto::render::Picture,
    p1: crate::proto::render::Pointfix,
    p2: crate::proto::render::Pointfix,
    num_stops: u32,
    stops: &[crate::proto::render::Fixed],
    colors: &[crate::proto::render::Color],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let num_stops = u32::try_from(num_stops).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&picture.serialize_fixed());
        buf_ptr
            .get_mut(8..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&p1.serialize_fixed());
        buf_ptr
            .get_mut(16..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&p2.serialize_fixed());
        buf_ptr
            .get_mut(24..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(num_stops).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = stops.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(28..)
                .ok_or(crate::error::Error::Serialize)?,
            stops,
        )?;
        let mut offset = list_len + 28;
        let list_len = colors.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            colors,
        )?;
        offset += list_len;
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
pub fn create_radial_gradient<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    picture: crate::proto::render::Picture,
    inner: crate::proto::render::Pointfix,
    outer: crate::proto::render::Pointfix,
    inner_radius: crate::proto::render::Fixed,
    outer_radius: crate::proto::render::Fixed,
    num_stops: u32,
    stops: &[crate::proto::render::Fixed],
    colors: &[crate::proto::render::Color],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let num_stops = u32::try_from(num_stops).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&picture.serialize_fixed());
        buf_ptr
            .get_mut(8..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&inner.serialize_fixed());
        buf_ptr
            .get_mut(16..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&outer.serialize_fixed());
        buf_ptr
            .get_mut(24..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&inner_radius.serialize_fixed());
        buf_ptr
            .get_mut(28..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&outer_radius.serialize_fixed());
        buf_ptr
            .get_mut(32..36)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(num_stops).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = stops.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(36..)
                .ok_or(crate::error::Error::Serialize)?,
            stops,
        )?;
        let mut offset = list_len + 36;
        let list_len = colors.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            colors,
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
pub fn create_conical_gradient<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    picture: crate::proto::render::Picture,
    center: crate::proto::render::Pointfix,
    angle: crate::proto::render::Fixed,
    num_stops: u32,
    stops: &[crate::proto::render::Fixed],
    colors: &[crate::proto::render::Color],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::render::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::render::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let num_stops = u32::try_from(num_stops).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&picture.serialize_fixed());
        buf_ptr
            .get_mut(8..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&center.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&angle.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(num_stops).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = stops.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            stops,
        )?;
        let mut offset = list_len + 24;
        let list_len = colors.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            colors,
        )?;
        offset += list_len;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 36]);
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
