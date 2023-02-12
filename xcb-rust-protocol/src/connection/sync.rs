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
pub fn initialize<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    desired_major_version: u8,
    desired_minor_version: u8,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::sync::InitializeReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
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
                desired_major_version,
                desired_minor_version,
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
pub fn list_system_counters<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::sync::ListSystemCountersReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
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
pub fn create_counter<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    id: crate::proto::sync::Counter,
    initial_value: crate::proto::sync::Int64,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let id_bytes = id.serialize_fixed();
    let initial_value_bytes = initial_value.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                2,
                length[0],
                length[1],
                id_bytes[0],
                id_bytes[1],
                id_bytes[2],
                id_bytes[3],
                initial_value_bytes[0],
                initial_value_bytes[1],
                initial_value_bytes[2],
                initial_value_bytes[3],
                initial_value_bytes[4],
                initial_value_bytes[5],
                initial_value_bytes[6],
                initial_value_bytes[7],
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
pub fn destroy_counter<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    counter: crate::proto::sync::Counter,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let counter_bytes = counter.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                6,
                length[0],
                length[1],
                counter_bytes[0],
                counter_bytes[1],
                counter_bytes[2],
                counter_bytes[3],
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
pub fn query_counter<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    counter: crate::proto::sync::Counter,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::sync::QueryCounterReply, 16>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let counter_bytes = counter.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                5,
                length[0],
                length[1],
                counter_bytes[0],
                counter_bytes[1],
                counter_bytes[2],
                counter_bytes[3],
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
pub fn r#await<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    wait_list: &[crate::proto::sync::Waitcondition],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let list_len = wait_list.len() * 28;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(0..).ok_or(crate::error::Error::Serialize)?,
            wait_list,
        )?;
        let mut offset = list_len;
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
pub fn change_counter<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    counter: crate::proto::sync::Counter,
    amount: crate::proto::sync::Int64,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let counter_bytes = counter.serialize_fixed();
    let amount_bytes = amount.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                4,
                length[0],
                length[1],
                counter_bytes[0],
                counter_bytes[1],
                counter_bytes[2],
                counter_bytes[3],
                amount_bytes[0],
                amount_bytes[1],
                amount_bytes[2],
                amount_bytes[3],
                amount_bytes[4],
                amount_bytes[5],
                amount_bytes[6],
                amount_bytes[7],
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
pub fn set_counter<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    counter: crate::proto::sync::Counter,
    value: crate::proto::sync::Int64,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let counter_bytes = counter.serialize_fixed();
    let value_bytes = value.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
                length[0],
                length[1],
                counter_bytes[0],
                counter_bytes[1],
                counter_bytes[2],
                counter_bytes[3],
                value_bytes[0],
                value_bytes[1],
                value_bytes[2],
                value_bytes[3],
                value_bytes[4],
                value_bytes[5],
                value_bytes[6],
                value_bytes[7],
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
pub fn create_alarm<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    id: crate::proto::sync::Alarm,
    create_alarm_value_list: crate::proto::sync::CreateAlarmValueList,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 8]);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&id.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&create_alarm_value_list.switch_expr().serialize_fixed());
        let mut offset = 12
            + create_alarm_value_list.serialize_into(
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
pub fn change_alarm<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    id: crate::proto::sync::Alarm,
    change_alarm_value_list: crate::proto::sync::ChangeAlarmValueList,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 9]);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&id.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&change_alarm_value_list.switch_expr().serialize_fixed());
        let mut offset = 12
            + change_alarm_value_list.serialize_into(
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
pub fn destroy_alarm<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    alarm: crate::proto::sync::Alarm,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let alarm_bytes = alarm.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                11,
                length[0],
                length[1],
                alarm_bytes[0],
                alarm_bytes[1],
                alarm_bytes[2],
                alarm_bytes[3],
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
pub fn query_alarm<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    alarm: crate::proto::sync::Alarm,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::sync::QueryAlarmReply, 40>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let alarm_bytes = alarm.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                10,
                length[0],
                length[1],
                alarm_bytes[0],
                alarm_bytes[1],
                alarm_bytes[2],
                alarm_bytes[3],
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
pub fn set_priority<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    id: u32,
    priority: i32,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let id_bytes = id.serialize_fixed();
    let priority_bytes = priority.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                12,
                length[0],
                length[1],
                id_bytes[0],
                id_bytes[1],
                id_bytes[2],
                id_bytes[3],
                priority_bytes[0],
                priority_bytes[1],
                priority_bytes[2],
                priority_bytes[3],
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
pub fn get_priority<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    id: u32,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::sync::GetPriorityReply, 12>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let id_bytes = id.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                13,
                length[0],
                length[1],
                id_bytes[0],
                id_bytes[1],
                id_bytes[2],
                id_bytes[3],
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
pub fn create_fence<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    drawable: crate::proto::xproto::Drawable,
    fence: crate::proto::sync::Fence,
    initially_triggered: u8,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let drawable_bytes = drawable.serialize_fixed();
    let fence_bytes = fence.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                14,
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
pub fn trigger_fence<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    fence: crate::proto::sync::Fence,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let fence_bytes = fence.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                15,
                length[0],
                length[1],
                fence_bytes[0],
                fence_bytes[1],
                fence_bytes[2],
                fence_bytes[3],
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
pub fn reset_fence<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    fence: crate::proto::sync::Fence,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let fence_bytes = fence.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                16,
                length[0],
                length[1],
                fence_bytes[0],
                fence_bytes[1],
                fence_bytes[2],
                fence_bytes[3],
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
pub fn destroy_fence<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    fence: crate::proto::sync::Fence,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let fence_bytes = fence.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                17,
                length[0],
                length[1],
                fence_bytes[0],
                fence_bytes[1],
                fence_bytes[2],
                fence_bytes[3],
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
pub fn query_fence<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    fence: crate::proto::sync::Fence,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::sync::QueryFenceReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let fence_bytes = fence.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                18,
                length[0],
                length[1],
                fence_bytes[0],
                fence_bytes[1],
                fence_bytes[2],
                fence_bytes[3],
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
pub fn await_fence<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    fence_list: &[crate::proto::sync::Fence],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::sync::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::sync::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let list_len = fence_list.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(0..).ok_or(crate::error::Error::Serialize)?,
            fence_list,
        )?;
        let mut offset = list_len;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 19]);
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
