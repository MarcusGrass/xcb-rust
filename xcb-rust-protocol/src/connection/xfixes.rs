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
) -> crate::error::Result<FixedCookie<crate::proto::xfixes::QueryVersionReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
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
pub fn change_save_set<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    mode: crate::proto::xfixes::SaveSetModeEnum,
    target: crate::proto::xfixes::SaveSetTargetEnum,
    map: crate::proto::xfixes::SaveSetMappingEnum,
    window: crate::proto::xproto::Window,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let window_bytes = window.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                1,
                length[0],
                length[1],
                mode.0 as u8,
                target.0 as u8,
                map.0 as u8,
                0,
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
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
pub fn select_selection_input<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    window: crate::proto::xproto::Window,
    selection: u32,
    event_mask: crate::proto::xfixes::SelectionEventMask,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let window_bytes = window.serialize_fixed();
    let selection_bytes = selection.serialize_fixed();
    let event_mask_bytes = (event_mask.0 as u32).serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
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
                selection_bytes[0],
                selection_bytes[1],
                selection_bytes[2],
                selection_bytes[3],
                event_mask_bytes[0],
                event_mask_bytes[1],
                event_mask_bytes[2],
                event_mask_bytes[3],
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
pub fn select_cursor_input<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    window: crate::proto::xproto::Window,
    event_mask: crate::proto::xfixes::CursorNotifyMask,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let window_bytes = window.serialize_fixed();
    let event_mask_bytes = (event_mask.0 as u32).serialize_fixed();
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
                event_mask_bytes[0],
                event_mask_bytes[1],
                event_mask_bytes[2],
                event_mask_bytes[3],
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
pub fn get_cursor_image<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::xfixes::GetCursorImageReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
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
    Ok(Cookie::new(seq))
}
pub fn create_region<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    region: crate::proto::xfixes::Region,
    rectangles: &[crate::proto::xproto::Rectangle],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&region.serialize_fixed());
        let list_len = rectangles.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            rectangles,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 5]);
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
pub fn create_region_from_bitmap<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    region: crate::proto::xfixes::Region,
    bitmap: crate::proto::xproto::Pixmap,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let region_bytes = region.serialize_fixed();
    let bitmap_bytes = bitmap.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                6,
                length[0],
                length[1],
                region_bytes[0],
                region_bytes[1],
                region_bytes[2],
                region_bytes[3],
                bitmap_bytes[0],
                bitmap_bytes[1],
                bitmap_bytes[2],
                bitmap_bytes[3],
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
pub fn create_region_from_window<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    region: crate::proto::xfixes::Region,
    window: crate::proto::xproto::Window,
    kind: crate::proto::shape::SkEnum,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let region_bytes = region.serialize_fixed();
    let window_bytes = window.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                7,
                length[0],
                length[1],
                region_bytes[0],
                region_bytes[1],
                region_bytes[2],
                region_bytes[3],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                kind.0 as u8,
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
pub fn create_region_from_g_c<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    region: crate::proto::xfixes::Region,
    gc: crate::proto::xproto::Gcontext,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let region_bytes = region.serialize_fixed();
    let gc_bytes = gc.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                8,
                length[0],
                length[1],
                region_bytes[0],
                region_bytes[1],
                region_bytes[2],
                region_bytes[3],
                gc_bytes[0],
                gc_bytes[1],
                gc_bytes[2],
                gc_bytes[3],
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
pub fn create_region_from_picture<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    region: crate::proto::xfixes::Region,
    picture: crate::proto::render::Picture,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let region_bytes = region.serialize_fixed();
    let picture_bytes = picture.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                9,
                length[0],
                length[1],
                region_bytes[0],
                region_bytes[1],
                region_bytes[2],
                region_bytes[3],
                picture_bytes[0],
                picture_bytes[1],
                picture_bytes[2],
                picture_bytes[3],
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
pub fn destroy_region<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    region: crate::proto::xfixes::Region,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let region_bytes = region.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                10,
                length[0],
                length[1],
                region_bytes[0],
                region_bytes[1],
                region_bytes[2],
                region_bytes[3],
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
pub fn set_region<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    region: crate::proto::xfixes::Region,
    rectangles: &[crate::proto::xproto::Rectangle],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&region.serialize_fixed());
        let list_len = rectangles.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            rectangles,
        )?;
        let mut offset = list_len + 8;
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
pub fn copy_region<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    source: crate::proto::xfixes::Region,
    destination: crate::proto::xfixes::Region,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let source_bytes = source.serialize_fixed();
    let destination_bytes = destination.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                12,
                length[0],
                length[1],
                source_bytes[0],
                source_bytes[1],
                source_bytes[2],
                source_bytes[3],
                destination_bytes[0],
                destination_bytes[1],
                destination_bytes[2],
                destination_bytes[3],
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
pub fn union_region<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    source1: crate::proto::xfixes::Region,
    source2: crate::proto::xfixes::Region,
    destination: crate::proto::xfixes::Region,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let source1_bytes = source1.serialize_fixed();
    let source2_bytes = source2.serialize_fixed();
    let destination_bytes = destination.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                13,
                length[0],
                length[1],
                source1_bytes[0],
                source1_bytes[1],
                source1_bytes[2],
                source1_bytes[3],
                source2_bytes[0],
                source2_bytes[1],
                source2_bytes[2],
                source2_bytes[3],
                destination_bytes[0],
                destination_bytes[1],
                destination_bytes[2],
                destination_bytes[3],
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
pub fn intersect_region<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    source1: crate::proto::xfixes::Region,
    source2: crate::proto::xfixes::Region,
    destination: crate::proto::xfixes::Region,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let source1_bytes = source1.serialize_fixed();
    let source2_bytes = source2.serialize_fixed();
    let destination_bytes = destination.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                14,
                length[0],
                length[1],
                source1_bytes[0],
                source1_bytes[1],
                source1_bytes[2],
                source1_bytes[3],
                source2_bytes[0],
                source2_bytes[1],
                source2_bytes[2],
                source2_bytes[3],
                destination_bytes[0],
                destination_bytes[1],
                destination_bytes[2],
                destination_bytes[3],
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
pub fn subtract_region<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    source1: crate::proto::xfixes::Region,
    source2: crate::proto::xfixes::Region,
    destination: crate::proto::xfixes::Region,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let source1_bytes = source1.serialize_fixed();
    let source2_bytes = source2.serialize_fixed();
    let destination_bytes = destination.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                15,
                length[0],
                length[1],
                source1_bytes[0],
                source1_bytes[1],
                source1_bytes[2],
                source1_bytes[3],
                source2_bytes[0],
                source2_bytes[1],
                source2_bytes[2],
                source2_bytes[3],
                destination_bytes[0],
                destination_bytes[1],
                destination_bytes[2],
                destination_bytes[3],
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
pub fn invert_region<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    source: crate::proto::xfixes::Region,
    bounds: crate::proto::xproto::Rectangle,
    destination: crate::proto::xfixes::Region,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (5u16).to_ne_bytes();
    let source_bytes = source.serialize_fixed();
    let bounds_bytes = bounds.serialize_fixed();
    let destination_bytes = destination.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                16,
                length[0],
                length[1],
                source_bytes[0],
                source_bytes[1],
                source_bytes[2],
                source_bytes[3],
                bounds_bytes[0],
                bounds_bytes[1],
                bounds_bytes[2],
                bounds_bytes[3],
                bounds_bytes[4],
                bounds_bytes[5],
                bounds_bytes[6],
                bounds_bytes[7],
                destination_bytes[0],
                destination_bytes[1],
                destination_bytes[2],
                destination_bytes[3],
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
pub fn translate_region<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    region: crate::proto::xfixes::Region,
    dx: i16,
    dy: i16,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let region_bytes = region.serialize_fixed();
    let dx_bytes = dx.serialize_fixed();
    let dy_bytes = dy.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                17,
                length[0],
                length[1],
                region_bytes[0],
                region_bytes[1],
                region_bytes[2],
                region_bytes[3],
                dx_bytes[0],
                dx_bytes[1],
                dy_bytes[0],
                dy_bytes[1],
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
pub fn region_extents<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    source: crate::proto::xfixes::Region,
    destination: crate::proto::xfixes::Region,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let source_bytes = source.serialize_fixed();
    let destination_bytes = destination.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                18,
                length[0],
                length[1],
                source_bytes[0],
                source_bytes[1],
                source_bytes[2],
                source_bytes[3],
                destination_bytes[0],
                destination_bytes[1],
                destination_bytes[2],
                destination_bytes[3],
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
pub fn fetch_region<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    region: crate::proto::xfixes::Region,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::xfixes::FetchRegionReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let region_bytes = region.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                19,
                length[0],
                length[1],
                region_bytes[0],
                region_bytes[1],
                region_bytes[2],
                region_bytes[3],
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
pub fn set_g_c_clip_region<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    gc: crate::proto::xproto::Gcontext,
    region: crate::proto::xfixes::RegionEnum,
    x_origin: i16,
    y_origin: i16,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let gc_bytes = gc.serialize_fixed();
    let region_bytes = (region.0 as u32).serialize_fixed();
    let x_origin_bytes = x_origin.serialize_fixed();
    let y_origin_bytes = y_origin.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                20,
                length[0],
                length[1],
                gc_bytes[0],
                gc_bytes[1],
                gc_bytes[2],
                gc_bytes[3],
                region_bytes[0],
                region_bytes[1],
                region_bytes[2],
                region_bytes[3],
                x_origin_bytes[0],
                x_origin_bytes[1],
                y_origin_bytes[0],
                y_origin_bytes[1],
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
pub fn set_window_shape_region<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    dest: crate::proto::xproto::Window,
    dest_kind: crate::proto::shape::SkEnum,
    x_offset: i16,
    y_offset: i16,
    region: crate::proto::xfixes::RegionEnum,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (5u16).to_ne_bytes();
    let dest_bytes = dest.serialize_fixed();
    let x_offset_bytes = x_offset.serialize_fixed();
    let y_offset_bytes = y_offset.serialize_fixed();
    let region_bytes = (region.0 as u32).serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                21,
                length[0],
                length[1],
                dest_bytes[0],
                dest_bytes[1],
                dest_bytes[2],
                dest_bytes[3],
                dest_kind.0 as u8,
                0,
                0,
                0,
                x_offset_bytes[0],
                x_offset_bytes[1],
                y_offset_bytes[0],
                y_offset_bytes[1],
                region_bytes[0],
                region_bytes[1],
                region_bytes[2],
                region_bytes[3],
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
pub fn set_picture_clip_region<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    picture: crate::proto::render::Picture,
    region: crate::proto::xfixes::RegionEnum,
    x_origin: i16,
    y_origin: i16,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let picture_bytes = picture.serialize_fixed();
    let region_bytes = (region.0 as u32).serialize_fixed();
    let x_origin_bytes = x_origin.serialize_fixed();
    let y_origin_bytes = y_origin.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                22,
                length[0],
                length[1],
                picture_bytes[0],
                picture_bytes[1],
                picture_bytes[2],
                picture_bytes[3],
                region_bytes[0],
                region_bytes[1],
                region_bytes[2],
                region_bytes[3],
                x_origin_bytes[0],
                x_origin_bytes[1],
                y_origin_bytes[0],
                y_origin_bytes[1],
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
pub fn set_cursor_name<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    cursor: crate::proto::xproto::Cursor,
    name: &[u8],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let nbytes = u16::try_from(name.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&cursor.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(nbytes).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            name,
        )?;
        let mut offset = list_len + 12;
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
pub fn get_cursor_name<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    cursor: crate::proto::xproto::Cursor,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::xfixes::GetCursorNameReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let cursor_bytes = cursor.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                24,
                length[0],
                length[1],
                cursor_bytes[0],
                cursor_bytes[1],
                cursor_bytes[2],
                cursor_bytes[3],
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
pub fn get_cursor_image_and_name<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::xfixes::GetCursorImageAndNameReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf| {
        let buf = buf.get_mut(..4).ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 25;
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
pub fn change_cursor<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    source: crate::proto::xproto::Cursor,
    destination: crate::proto::xproto::Cursor,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let source_bytes = source.serialize_fixed();
    let destination_bytes = destination.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                26,
                length[0],
                length[1],
                source_bytes[0],
                source_bytes[1],
                source_bytes[2],
                source_bytes[3],
                destination_bytes[0],
                destination_bytes[1],
                destination_bytes[2],
                destination_bytes[3],
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
pub fn change_cursor_by_name<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    src: crate::proto::xproto::Cursor,
    name: &[u8],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let nbytes = u16::try_from(name.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(nbytes).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            name,
        )?;
        let mut offset = list_len + 12;
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
pub fn expand_region<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    source: crate::proto::xfixes::Region,
    destination: crate::proto::xfixes::Region,
    left: u16,
    right: u16,
    top: u16,
    bottom: u16,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (5u16).to_ne_bytes();
    let source_bytes = source.serialize_fixed();
    let destination_bytes = destination.serialize_fixed();
    let left_bytes = left.serialize_fixed();
    let right_bytes = right.serialize_fixed();
    let top_bytes = top.serialize_fixed();
    let bottom_bytes = bottom.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                28,
                length[0],
                length[1],
                source_bytes[0],
                source_bytes[1],
                source_bytes[2],
                source_bytes[3],
                destination_bytes[0],
                destination_bytes[1],
                destination_bytes[2],
                destination_bytes[3],
                left_bytes[0],
                left_bytes[1],
                right_bytes[0],
                right_bytes[1],
                top_bytes[0],
                top_bytes[1],
                bottom_bytes[0],
                bottom_bytes[1],
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
pub fn hide_cursor<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    window: crate::proto::xproto::Window,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let window_bytes = window.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                29,
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
    Ok(VoidCookie::new(seq))
}
pub fn show_cursor<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    window: crate::proto::xproto::Window,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let window_bytes = window.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                30,
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
    Ok(VoidCookie::new(seq))
}
pub fn create_pointer_barrier<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    barrier: crate::proto::xfixes::Barrier,
    window: crate::proto::xproto::Window,
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
    directions: crate::proto::xfixes::BarrierDirections,
    devices: &[u16],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        // Pad 2 bytes
        let num_devices =
            u16::try_from(devices.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&barrier.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&window.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&x1.serialize_fixed());
        buf_ptr
            .get_mut(14..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&y1.serialize_fixed());
        buf_ptr
            .get_mut(16..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&x2.serialize_fixed());
        buf_ptr
            .get_mut(18..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&y2.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&directions.serialize_fixed());
        buf_ptr
            .get_mut(26..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_devices).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = devices.len() * 2;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(28..)
                .ok_or(crate::error::Error::Serialize)?,
            devices,
        )?;
        let mut offset = list_len + 28;
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
pub fn delete_pointer_barrier<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    barrier: crate::proto::xfixes::Barrier,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let barrier_bytes = barrier.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                32,
                length[0],
                length[1],
                barrier_bytes[0],
                barrier_bytes[1],
                barrier_bytes[2],
                barrier_bytes[3],
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
pub fn set_client_disconnect_mode<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    disconnect_mode: crate::proto::xfixes::ClientDisconnectFlags,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let disconnect_mode_bytes = (disconnect_mode.0 as u32).serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                33,
                length[0],
                length[1],
                disconnect_mode_bytes[0],
                disconnect_mode_bytes[1],
                disconnect_mode_bytes[2],
                disconnect_mode_bytes[3],
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
pub fn get_client_disconnect_mode<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::xfixes::GetClientDisconnectModeReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xfixes::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xfixes::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf| {
        let buf = buf.get_mut(..4).ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 34;
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
