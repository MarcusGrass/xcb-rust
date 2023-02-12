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
    major_version: u32,
    minor_version: u32,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::dri3::QueryVersionReply, 16>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dri3::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dri3::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let major_version_bytes = major_version.serialize_fixed();
    let minor_version_bytes = minor_version.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                0,
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
pub fn open<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    drawable: crate::proto::xproto::Drawable,
    provider: u32,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::dri3::OpenReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dri3::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dri3::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let drawable_bytes = drawable.serialize_fixed();
    let provider_bytes = provider.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                1,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                provider_bytes[0],
                provider_bytes[1],
                provider_bytes[2],
                provider_bytes[3],
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
pub fn pixmap_from_buffer<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    pixmap: crate::proto::xproto::Pixmap,
    drawable: crate::proto::xproto::Drawable,
    size: u32,
    width: u16,
    height: u16,
    stride: u16,
    depth: u8,
    bpp: u8,
    pixmap_fd: (),
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dri3::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dri3::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (6u16).to_ne_bytes();
    let pixmap_bytes = pixmap.serialize_fixed();
    let drawable_bytes = drawable.serialize_fixed();
    let size_bytes = size.serialize_fixed();
    let width_bytes = width.serialize_fixed();
    let height_bytes = height.serialize_fixed();
    let stride_bytes = stride.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                2,
                length[0],
                length[1],
                pixmap_bytes[0],
                pixmap_bytes[1],
                pixmap_bytes[2],
                pixmap_bytes[3],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                size_bytes[0],
                size_bytes[1],
                size_bytes[2],
                size_bytes[3],
                width_bytes[0],
                width_bytes[1],
                height_bytes[0],
                height_bytes[1],
                stride_bytes[0],
                stride_bytes[1],
                depth,
                bpp,
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
pub fn buffer_from_pixmap<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    pixmap: crate::proto::xproto::Pixmap,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::dri3::BufferFromPixmapReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dri3::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dri3::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let pixmap_bytes = pixmap.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
                length[0],
                length[1],
                pixmap_bytes[0],
                pixmap_bytes[1],
                pixmap_bytes[2],
                pixmap_bytes[3],
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
pub fn fence_from_f_d<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    drawable: crate::proto::xproto::Drawable,
    fence: u32,
    initially_triggered: u8,
    fence_fd: (),
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dri3::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dri3::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let drawable_bytes = drawable.serialize_fixed();
    let fence_bytes = fence.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                4,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                fence_bytes[0],
                fence_bytes[1],
                fence_bytes[2],
                fence_bytes[3],
                initially_triggered,
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
pub fn f_d_from_fence<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    drawable: crate::proto::xproto::Drawable,
    fence: u32,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::dri3::FDFromFenceReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dri3::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dri3::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let drawable_bytes = drawable.serialize_fixed();
    let fence_bytes = fence.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
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
                fence_bytes[0],
                fence_bytes[1],
                fence_bytes[2],
                fence_bytes[3],
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
pub fn get_supported_modifiers<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    window: u32,
    depth: u8,
    bpp: u8,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::dri3::GetSupportedModifiersReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dri3::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dri3::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let window_bytes = window.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                6,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                depth,
                bpp,
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
pub fn pixmap_from_buffers<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    pixmap: crate::proto::xproto::Pixmap,
    window: crate::proto::xproto::Window,
    width: u16,
    height: u16,
    stride0: u32,
    offset0: u32,
    stride1: u32,
    offset1: u32,
    stride2: u32,
    offset2: u32,
    stride3: u32,
    offset3: u32,
    depth: u8,
    bpp: u8,
    modifier: u64,
    buffers: &[()],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dri3::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dri3::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        // Start align 8, offset None
        let num_buffers =
            u8::try_from(buffers.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 3 bytes
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&pixmap.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&window.serialize_fixed());
        buf_ptr
            .get_mut(12..13)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(num_buffers).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(16..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&width.serialize_fixed());
        buf_ptr
            .get_mut(18..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&height.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&stride0.serialize_fixed());
        buf_ptr
            .get_mut(24..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&offset0.serialize_fixed());
        buf_ptr
            .get_mut(28..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&stride1.serialize_fixed());
        buf_ptr
            .get_mut(32..36)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&offset1.serialize_fixed());
        buf_ptr
            .get_mut(36..40)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&stride2.serialize_fixed());
        buf_ptr
            .get_mut(40..44)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&offset2.serialize_fixed());
        buf_ptr
            .get_mut(44..48)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&stride3.serialize_fixed());
        buf_ptr
            .get_mut(48..52)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&offset3.serialize_fixed());
        buf_ptr
            .get_mut(52..53)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&depth.serialize_fixed());
        buf_ptr
            .get_mut(53..54)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&bpp.serialize_fixed());
        buf_ptr
            .get_mut(56..64)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&modifier.serialize_fixed());
        let offset = 64;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 7]);
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
pub fn buffers_from_pixmap<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    pixmap: crate::proto::xproto::Pixmap,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::dri3::BuffersFromPixmapReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dri3::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dri3::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let pixmap_bytes = pixmap.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                8,
                length[0],
                length[1],
                pixmap_bytes[0],
                pixmap_bytes[1],
                pixmap_bytes[2],
                pixmap_bytes[3],
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
pub fn set_d_r_m_device_in_use<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    window: crate::proto::xproto::Window,
    drm_major: u32,
    drm_minor: u32,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::dri3::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::dri3::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let window_bytes = window.serialize_fixed();
    let drm_major_bytes = drm_major.serialize_fixed();
    let drm_minor_bytes = drm_minor.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                9,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                drm_major_bytes[0],
                drm_major_bytes[1],
                drm_major_bytes[2],
                drm_major_bytes[3],
                drm_minor_bytes[0],
                drm_minor_bytes[1],
                drm_minor_bytes[2],
                drm_minor_bytes[3],
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
