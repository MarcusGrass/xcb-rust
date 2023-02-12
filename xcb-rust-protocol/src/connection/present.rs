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
) -> crate::error::Result<FixedCookie<crate::proto::present::QueryVersionReply, 16>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::present::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::present::EXTENSION_NAME,
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
pub fn pixmap<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    window: crate::proto::xproto::Window,
    pixmap: crate::proto::xproto::Pixmap,
    serial: u32,
    valid: crate::proto::xfixes::Region,
    update: crate::proto::xfixes::Region,
    x_off: i16,
    y_off: i16,
    target_crtc: crate::proto::randr::Crtc,
    wait_fence: crate::proto::sync::Fence,
    idle_fence: crate::proto::sync::Fence,
    options: u32,
    target_msc: u64,
    divisor: u64,
    remainder: u64,
    notifies: &[crate::proto::present::Notify],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::present::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::present::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        // Start align 8, offset None
        // Pad 4 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&window.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&pixmap.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&serial.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&valid.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&update.serialize_fixed());
        buf_ptr
            .get_mut(24..26)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&x_off.serialize_fixed());
        buf_ptr
            .get_mut(26..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&y_off.serialize_fixed());
        buf_ptr
            .get_mut(28..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&target_crtc.serialize_fixed());
        buf_ptr
            .get_mut(32..36)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&wait_fence.serialize_fixed());
        buf_ptr
            .get_mut(36..40)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&idle_fence.serialize_fixed());
        buf_ptr
            .get_mut(40..44)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&options.serialize_fixed());
        buf_ptr
            .get_mut(48..56)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&target_msc.serialize_fixed());
        buf_ptr
            .get_mut(56..64)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&divisor.serialize_fixed());
        buf_ptr
            .get_mut(64..72)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&remainder.serialize_fixed());
        let list_len = notifies.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(72..)
                .ok_or(crate::error::Error::Serialize)?,
            notifies,
        )?;
        let mut offset = list_len + 72;
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
pub fn notify_m_s_c<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    window: crate::proto::xproto::Window,
    serial: u32,
    target_msc: u64,
    divisor: u64,
    remainder: u64,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::present::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::present::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (10u16).to_ne_bytes();
    let window_bytes = window.serialize_fixed();
    let serial_bytes = serial.serialize_fixed();
    let target_msc_bytes = target_msc.serialize_fixed();
    let divisor_bytes = divisor.serialize_fixed();
    let remainder_bytes = remainder.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..40)
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
                serial_bytes[0],
                serial_bytes[1],
                serial_bytes[2],
                serial_bytes[3],
                0,
                0,
                0,
                0,
                target_msc_bytes[0],
                target_msc_bytes[1],
                target_msc_bytes[2],
                target_msc_bytes[3],
                target_msc_bytes[4],
                target_msc_bytes[5],
                target_msc_bytes[6],
                target_msc_bytes[7],
                divisor_bytes[0],
                divisor_bytes[1],
                divisor_bytes[2],
                divisor_bytes[3],
                divisor_bytes[4],
                divisor_bytes[5],
                divisor_bytes[6],
                divisor_bytes[7],
                remainder_bytes[0],
                remainder_bytes[1],
                remainder_bytes[2],
                remainder_bytes[3],
                remainder_bytes[4],
                remainder_bytes[5],
                remainder_bytes[6],
                remainder_bytes[7],
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
pub fn select_input<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    eid: crate::proto::present::Event,
    window: crate::proto::xproto::Window,
    event_mask: crate::proto::present::EventMask,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::present::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::present::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let eid_bytes = eid.serialize_fixed();
    let window_bytes = window.serialize_fixed();
    let event_mask_bytes = (event_mask.0 as u32).serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
                length[0],
                length[1],
                eid_bytes[0],
                eid_bytes[1],
                eid_bytes[2],
                eid_bytes[3],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
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
pub fn query_capabilities<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    target: u32,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::present::QueryCapabilitiesReply, 12>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::present::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::present::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let target_bytes = target.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                4,
                length[0],
                length[1],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
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
