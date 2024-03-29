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
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::shm::QueryVersionReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::shm::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::shm::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf| {
        let buf = buf.get_mut(..4).ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 0;
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
pub fn attach<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    shmseg: crate::proto::shm::Seg,
    shmid: u32,
    read_only: u8,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::shm::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::shm::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let shmseg_bytes = shmseg.serialize_fixed();
    let shmid_bytes = shmid.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                1,
                length[0],
                length[1],
                shmseg_bytes[0],
                shmseg_bytes[1],
                shmseg_bytes[2],
                shmseg_bytes[3],
                shmid_bytes[0],
                shmid_bytes[1],
                shmid_bytes[2],
                shmid_bytes[3],
                read_only,
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
pub fn detach<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    shmseg: crate::proto::shm::Seg,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::shm::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::shm::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let shmseg_bytes = shmseg.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                2,
                length[0],
                length[1],
                shmseg_bytes[0],
                shmseg_bytes[1],
                shmseg_bytes[2],
                shmseg_bytes[3],
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
pub fn put_image<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    drawable: crate::proto::xproto::Drawable,
    gc: crate::proto::xproto::Gcontext,
    total_width: u16,
    total_height: u16,
    src_x: u16,
    src_y: u16,
    src_width: u16,
    src_height: u16,
    dst_x: i16,
    dst_y: i16,
    depth: u8,
    format: u8,
    send_event: u8,
    shmseg: crate::proto::shm::Seg,
    offset: u32,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::shm::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::shm::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (10u16).to_ne_bytes();
    let drawable_bytes = drawable.serialize_fixed();
    let gc_bytes = gc.serialize_fixed();
    let total_width_bytes = total_width.serialize_fixed();
    let total_height_bytes = total_height.serialize_fixed();
    let src_x_bytes = src_x.serialize_fixed();
    let src_y_bytes = src_y.serialize_fixed();
    let src_width_bytes = src_width.serialize_fixed();
    let src_height_bytes = src_height.serialize_fixed();
    let dst_x_bytes = dst_x.serialize_fixed();
    let dst_y_bytes = dst_y.serialize_fixed();
    let shmseg_bytes = shmseg.serialize_fixed();
    let offset_bytes = offset.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..40)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                gc_bytes[0],
                gc_bytes[1],
                gc_bytes[2],
                gc_bytes[3],
                total_width_bytes[0],
                total_width_bytes[1],
                total_height_bytes[0],
                total_height_bytes[1],
                src_x_bytes[0],
                src_x_bytes[1],
                src_y_bytes[0],
                src_y_bytes[1],
                src_width_bytes[0],
                src_width_bytes[1],
                src_height_bytes[0],
                src_height_bytes[1],
                dst_x_bytes[0],
                dst_x_bytes[1],
                dst_y_bytes[0],
                dst_y_bytes[1],
                depth,
                format,
                send_event,
                0,
                shmseg_bytes[0],
                shmseg_bytes[1],
                shmseg_bytes[2],
                shmseg_bytes[3],
                offset_bytes[0],
                offset_bytes[1],
                offset_bytes[2],
                offset_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(40)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(VoidCookie::new(seq))
}
pub fn get_image<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    drawable: crate::proto::xproto::Drawable,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    plane_mask: u32,
    format: u8,
    shmseg: crate::proto::shm::Seg,
    offset: u32,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::shm::GetImageReply, 16>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::shm::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::shm::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (8u16).to_ne_bytes();
    let drawable_bytes = drawable.serialize_fixed();
    let x_bytes = x.serialize_fixed();
    let y_bytes = y.serialize_fixed();
    let width_bytes = width.serialize_fixed();
    let height_bytes = height.serialize_fixed();
    let plane_mask_bytes = plane_mask.serialize_fixed();
    let shmseg_bytes = shmseg.serialize_fixed();
    let offset_bytes = offset.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..32)
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
                x_bytes[0],
                x_bytes[1],
                y_bytes[0],
                y_bytes[1],
                width_bytes[0],
                width_bytes[1],
                height_bytes[0],
                height_bytes[1],
                plane_mask_bytes[0],
                plane_mask_bytes[1],
                plane_mask_bytes[2],
                plane_mask_bytes[3],
                format,
                0,
                0,
                0,
                shmseg_bytes[0],
                shmseg_bytes[1],
                shmseg_bytes[2],
                shmseg_bytes[3],
                offset_bytes[0],
                offset_bytes[1],
                offset_bytes[2],
                offset_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(32)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(FixedCookie::new(seq))
}
pub fn create_pixmap<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    pid: crate::proto::xproto::Pixmap,
    drawable: crate::proto::xproto::Drawable,
    width: u16,
    height: u16,
    depth: u8,
    shmseg: crate::proto::shm::Seg,
    offset: u32,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::shm::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::shm::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (7u16).to_ne_bytes();
    let pid_bytes = pid.serialize_fixed();
    let drawable_bytes = drawable.serialize_fixed();
    let width_bytes = width.serialize_fixed();
    let height_bytes = height.serialize_fixed();
    let shmseg_bytes = shmseg.serialize_fixed();
    let offset_bytes = offset.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                5,
                length[0],
                length[1],
                pid_bytes[0],
                pid_bytes[1],
                pid_bytes[2],
                pid_bytes[3],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                width_bytes[0],
                width_bytes[1],
                height_bytes[0],
                height_bytes[1],
                depth,
                0,
                0,
                0,
                shmseg_bytes[0],
                shmseg_bytes[1],
                shmseg_bytes[2],
                shmseg_bytes[3],
                offset_bytes[0],
                offset_bytes[1],
                offset_bytes[2],
                offset_bytes[3],
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
pub fn attach_fd<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    shmseg: crate::proto::shm::Seg,
    shm_fd: (),
    read_only: u8,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::shm::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::shm::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let shmseg_bytes = shmseg.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                6,
                length[0],
                length[1],
                shmseg_bytes[0],
                shmseg_bytes[1],
                shmseg_bytes[2],
                shmseg_bytes[3],
                read_only,
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
    Ok(VoidCookie::new(seq))
}
pub fn create_segment<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    shmseg: crate::proto::shm::Seg,
    size: u32,
    read_only: u8,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::shm::CreateSegmentReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::shm::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::shm::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let shmseg_bytes = shmseg.serialize_fixed();
    let size_bytes = size.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                7,
                length[0],
                length[1],
                shmseg_bytes[0],
                shmseg_bytes[1],
                shmseg_bytes[2],
                shmseg_bytes[3],
                size_bytes[0],
                size_bytes[1],
                size_bytes[2],
                size_bytes[3],
                read_only,
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
    Ok(FixedCookie::new(seq))
}
