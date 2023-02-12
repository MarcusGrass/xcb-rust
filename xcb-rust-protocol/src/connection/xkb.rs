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
pub fn use_extension<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    wanted_major: u16,
    wanted_minor: u16,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::xkb::UseExtensionReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let wanted_major_bytes = wanted_major.serialize_fixed();
    let wanted_minor_bytes = wanted_minor.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                0,
                length[0],
                length[1],
                wanted_major_bytes[0],
                wanted_major_bytes[1],
                wanted_minor_bytes[0],
                wanted_minor_bytes[1],
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
pub fn select_events<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    affect_which: crate::proto::xkb::EventType,
    clear: crate::proto::xkb::EventType,
    select_all: crate::proto::xkb::EventType,
    affect_map: crate::proto::xkb::MapPart,
    map: crate::proto::xkb::MapPart,
    select_events_details: crate::proto::xkb::SelectEventsDetails,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 1]);
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&device_spec.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&affect_which.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&clear.serialize_fixed());
        buf_ptr
            .get_mut(10..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&select_all.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&affect_map.serialize_fixed());
        buf_ptr
            .get_mut(14..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&map.serialize_fixed());
        let mut offset = 16
            + select_events_details.serialize_into(
                buf_ptr
                    .get_mut(16..)
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
pub fn bell<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    bell_class: crate::proto::xkb::BellClassSpec,
    bell_i_d: crate::proto::xkb::IDSpec,
    percent: i8,
    force_sound: u8,
    event_only: u8,
    pitch: i16,
    duration: i16,
    name: u32,
    window: crate::proto::xproto::Window,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (7u16).to_ne_bytes();
    let device_spec_bytes = device_spec.serialize_fixed();
    let bell_class_bytes = bell_class.serialize_fixed();
    let bell_i_d_bytes = bell_i_d.serialize_fixed();
    let percent_bytes = percent.serialize_fixed();
    let pitch_bytes = pitch.serialize_fixed();
    let duration_bytes = duration.serialize_fixed();
    let name_bytes = name.serialize_fixed();
    let window_bytes = window.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
                length[0],
                length[1],
                device_spec_bytes[0],
                device_spec_bytes[1],
                bell_class_bytes[0],
                bell_class_bytes[1],
                bell_i_d_bytes[0],
                bell_i_d_bytes[1],
                percent_bytes[0],
                force_sound,
                event_only,
                0,
                pitch_bytes[0],
                pitch_bytes[1],
                duration_bytes[0],
                duration_bytes[1],
                0,
                0,
                name_bytes[0],
                name_bytes[1],
                name_bytes[2],
                name_bytes[3],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
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
pub fn get_state<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::xkb::GetStateReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let device_spec_bytes = device_spec.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                4,
                length[0],
                length[1],
                device_spec_bytes[0],
                device_spec_bytes[1],
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
pub fn latch_lock_state<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    affect_mod_locks: crate::proto::xproto::ModMask,
    mod_locks: crate::proto::xproto::ModMask,
    lock_group: u8,
    group_lock: crate::proto::xkb::GroupEnum,
    affect_mod_latches: crate::proto::xproto::ModMask,
    latch_group: u8,
    group_latch: u16,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let device_spec_bytes = device_spec.serialize_fixed();
    let group_latch_bytes = group_latch.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                5,
                length[0],
                length[1],
                device_spec_bytes[0],
                device_spec_bytes[1],
                affect_mod_locks.0 as u8,
                mod_locks.0 as u8,
                lock_group,
                group_lock.0 as u8,
                affect_mod_latches.0 as u8,
                0,
                0,
                latch_group,
                group_latch_bytes[0],
                group_latch_bytes[1],
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
pub fn get_controls<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::xkb::GetControlsReply, 92>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let device_spec_bytes = device_spec.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                6,
                length[0],
                length[1],
                device_spec_bytes[0],
                device_spec_bytes[1],
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
pub fn set_controls<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    affect_internal_real_mods: crate::proto::xproto::ModMask,
    internal_real_mods: crate::proto::xproto::ModMask,
    affect_ignore_lock_real_mods: crate::proto::xproto::ModMask,
    ignore_lock_real_mods: crate::proto::xproto::ModMask,
    affect_internal_virtual_mods: crate::proto::xkb::VMod,
    internal_virtual_mods: crate::proto::xkb::VMod,
    affect_ignore_lock_virtual_mods: crate::proto::xkb::VMod,
    ignore_lock_virtual_mods: crate::proto::xkb::VMod,
    mouse_keys_dflt_btn: u8,
    groups_wrap: u8,
    access_x_options: crate::proto::xkb::AXOption,
    affect_enabled_controls: crate::proto::xkb::BoolCtrl,
    enabled_controls: crate::proto::xkb::BoolCtrl,
    change_controls: crate::proto::xkb::Control,
    repeat_delay: u16,
    repeat_interval: u16,
    slow_keys_delay: u16,
    debounce_delay: u16,
    mouse_keys_delay: u16,
    mouse_keys_interval: u16,
    mouse_keys_time_to_max: u16,
    mouse_keys_max_speed: u16,
    mouse_keys_curve: i16,
    access_x_timeout: u16,
    access_x_timeout_mask: crate::proto::xkb::BoolCtrl,
    access_x_timeout_values: crate::proto::xkb::BoolCtrl,
    access_x_timeout_options_mask: crate::proto::xkb::AXOption,
    access_x_timeout_options_values: crate::proto::xkb::AXOption,
    per_key_repeat: &[u8],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (25u16).to_ne_bytes();
    let device_spec_bytes = device_spec.serialize_fixed();
    let affect_internal_virtual_mods_bytes =
        (affect_internal_virtual_mods.0 as u16).serialize_fixed();
    let internal_virtual_mods_bytes = (internal_virtual_mods.0 as u16).serialize_fixed();
    let affect_ignore_lock_virtual_mods_bytes =
        (affect_ignore_lock_virtual_mods.0 as u16).serialize_fixed();
    let ignore_lock_virtual_mods_bytes = (ignore_lock_virtual_mods.0 as u16).serialize_fixed();
    let access_x_options_bytes = (access_x_options.0 as u16).serialize_fixed();
    let affect_enabled_controls_bytes = (affect_enabled_controls.0 as u32).serialize_fixed();
    let enabled_controls_bytes = (enabled_controls.0 as u32).serialize_fixed();
    let change_controls_bytes = (change_controls.0 as u32).serialize_fixed();
    let repeat_delay_bytes = repeat_delay.serialize_fixed();
    let repeat_interval_bytes = repeat_interval.serialize_fixed();
    let slow_keys_delay_bytes = slow_keys_delay.serialize_fixed();
    let debounce_delay_bytes = debounce_delay.serialize_fixed();
    let mouse_keys_delay_bytes = mouse_keys_delay.serialize_fixed();
    let mouse_keys_interval_bytes = mouse_keys_interval.serialize_fixed();
    let mouse_keys_time_to_max_bytes = mouse_keys_time_to_max.serialize_fixed();
    let mouse_keys_max_speed_bytes = mouse_keys_max_speed.serialize_fixed();
    let mouse_keys_curve_bytes = mouse_keys_curve.serialize_fixed();
    let access_x_timeout_bytes = access_x_timeout.serialize_fixed();
    let access_x_timeout_mask_bytes = (access_x_timeout_mask.0 as u32).serialize_fixed();
    let access_x_timeout_values_bytes = (access_x_timeout_values.0 as u32).serialize_fixed();
    let access_x_timeout_options_mask_bytes =
        (access_x_timeout_options_mask.0 as u16).serialize_fixed();
    let access_x_timeout_options_values_bytes =
        (access_x_timeout_options_values.0 as u16).serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..100)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                7,
                length[0],
                length[1],
                device_spec_bytes[0],
                device_spec_bytes[1],
                affect_internal_real_mods.0 as u8,
                internal_real_mods.0 as u8,
                affect_ignore_lock_real_mods.0 as u8,
                ignore_lock_real_mods.0 as u8,
                affect_internal_virtual_mods_bytes[0],
                affect_internal_virtual_mods_bytes[1],
                internal_virtual_mods_bytes[0],
                internal_virtual_mods_bytes[1],
                affect_ignore_lock_virtual_mods_bytes[0],
                affect_ignore_lock_virtual_mods_bytes[1],
                ignore_lock_virtual_mods_bytes[0],
                ignore_lock_virtual_mods_bytes[1],
                mouse_keys_dflt_btn,
                groups_wrap,
                access_x_options_bytes[0],
                access_x_options_bytes[1],
                0,
                0,
                affect_enabled_controls_bytes[0],
                affect_enabled_controls_bytes[1],
                affect_enabled_controls_bytes[2],
                affect_enabled_controls_bytes[3],
                enabled_controls_bytes[0],
                enabled_controls_bytes[1],
                enabled_controls_bytes[2],
                enabled_controls_bytes[3],
                change_controls_bytes[0],
                change_controls_bytes[1],
                change_controls_bytes[2],
                change_controls_bytes[3],
                repeat_delay_bytes[0],
                repeat_delay_bytes[1],
                repeat_interval_bytes[0],
                repeat_interval_bytes[1],
                slow_keys_delay_bytes[0],
                slow_keys_delay_bytes[1],
                debounce_delay_bytes[0],
                debounce_delay_bytes[1],
                mouse_keys_delay_bytes[0],
                mouse_keys_delay_bytes[1],
                mouse_keys_interval_bytes[0],
                mouse_keys_interval_bytes[1],
                mouse_keys_time_to_max_bytes[0],
                mouse_keys_time_to_max_bytes[1],
                mouse_keys_max_speed_bytes[0],
                mouse_keys_max_speed_bytes[1],
                mouse_keys_curve_bytes[0],
                mouse_keys_curve_bytes[1],
                access_x_timeout_bytes[0],
                access_x_timeout_bytes[1],
                access_x_timeout_mask_bytes[0],
                access_x_timeout_mask_bytes[1],
                access_x_timeout_mask_bytes[2],
                access_x_timeout_mask_bytes[3],
                access_x_timeout_values_bytes[0],
                access_x_timeout_values_bytes[1],
                access_x_timeout_values_bytes[2],
                access_x_timeout_values_bytes[3],
                access_x_timeout_options_mask_bytes[0],
                access_x_timeout_options_mask_bytes[1],
                access_x_timeout_options_values_bytes[0],
                access_x_timeout_options_values_bytes[1],
                per_key_repeat[0],
                per_key_repeat[1],
                per_key_repeat[2],
                per_key_repeat[3],
                per_key_repeat[4],
                per_key_repeat[5],
                per_key_repeat[6],
                per_key_repeat[7],
                per_key_repeat[8],
                per_key_repeat[9],
                per_key_repeat[10],
                per_key_repeat[11],
                per_key_repeat[12],
                per_key_repeat[13],
                per_key_repeat[14],
                per_key_repeat[15],
                per_key_repeat[16],
                per_key_repeat[17],
                per_key_repeat[18],
                per_key_repeat[19],
                per_key_repeat[20],
                per_key_repeat[21],
                per_key_repeat[22],
                per_key_repeat[23],
                per_key_repeat[24],
                per_key_repeat[25],
                per_key_repeat[26],
                per_key_repeat[27],
                per_key_repeat[28],
                per_key_repeat[29],
                per_key_repeat[30],
                per_key_repeat[31],
            ]);
        Ok::<usize, crate::error::Error>(100)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(VoidCookie::new(seq))
}
pub fn get_map<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    full: crate::proto::xkb::MapPart,
    partial: crate::proto::xkb::MapPart,
    first_type: u8,
    n_types: u8,
    first_key_sym: crate::proto::xproto::Keycode,
    n_key_syms: u8,
    first_key_action: crate::proto::xproto::Keycode,
    n_key_actions: u8,
    first_key_behavior: crate::proto::xproto::Keycode,
    n_key_behaviors: u8,
    virtual_mods: crate::proto::xkb::VMod,
    first_key_explicit: crate::proto::xproto::Keycode,
    n_key_explicit: u8,
    first_mod_map_key: crate::proto::xproto::Keycode,
    n_mod_map_keys: u8,
    first_v_mod_map_key: crate::proto::xproto::Keycode,
    n_v_mod_map_keys: u8,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::xkb::GetMapReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (7u16).to_ne_bytes();
    let device_spec_bytes = device_spec.serialize_fixed();
    let full_bytes = (full.0 as u16).serialize_fixed();
    let partial_bytes = (partial.0 as u16).serialize_fixed();
    let first_key_sym_bytes = first_key_sym.serialize_fixed();
    let first_key_action_bytes = first_key_action.serialize_fixed();
    let first_key_behavior_bytes = first_key_behavior.serialize_fixed();
    let virtual_mods_bytes = (virtual_mods.0 as u16).serialize_fixed();
    let first_key_explicit_bytes = first_key_explicit.serialize_fixed();
    let first_mod_map_key_bytes = first_mod_map_key.serialize_fixed();
    let first_v_mod_map_key_bytes = first_v_mod_map_key.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                8,
                length[0],
                length[1],
                device_spec_bytes[0],
                device_spec_bytes[1],
                full_bytes[0],
                full_bytes[1],
                partial_bytes[0],
                partial_bytes[1],
                first_type,
                n_types,
                first_key_sym_bytes[0],
                n_key_syms,
                first_key_action_bytes[0],
                n_key_actions,
                first_key_behavior_bytes[0],
                n_key_behaviors,
                virtual_mods_bytes[0],
                virtual_mods_bytes[1],
                first_key_explicit_bytes[0],
                n_key_explicit,
                first_mod_map_key_bytes[0],
                n_mod_map_keys,
                first_v_mod_map_key_bytes[0],
                n_v_mod_map_keys,
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
pub fn set_map<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    flags: crate::proto::xkb::SetMapFlags,
    min_key_code: crate::proto::xproto::Keycode,
    max_key_code: crate::proto::xproto::Keycode,
    first_type: u8,
    n_types: u8,
    first_key_sym: crate::proto::xproto::Keycode,
    n_key_syms: u8,
    total_syms: u16,
    first_key_action: crate::proto::xproto::Keycode,
    n_key_actions: u8,
    total_actions: u16,
    first_key_behavior: crate::proto::xproto::Keycode,
    n_key_behaviors: u8,
    total_key_behaviors: u8,
    first_key_explicit: crate::proto::xproto::Keycode,
    n_key_explicit: u8,
    total_key_explicit: u8,
    first_mod_map_key: crate::proto::xproto::Keycode,
    n_mod_map_keys: u8,
    total_mod_map_keys: u8,
    first_v_mod_map_key: crate::proto::xproto::Keycode,
    n_v_mod_map_keys: u8,
    total_v_mod_map_keys: u8,
    virtual_mods: crate::proto::xkb::VMod,
    set_map_values: crate::proto::xkb::SetMapValues,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 9]);
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&device_spec.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&set_map_values.switch_expr().serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&flags.serialize_fixed());
        buf_ptr
            .get_mut(10..11)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&min_key_code.serialize_fixed());
        buf_ptr
            .get_mut(11..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&max_key_code.serialize_fixed());
        buf_ptr
            .get_mut(12..13)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&first_type.serialize_fixed());
        buf_ptr
            .get_mut(13..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&n_types.serialize_fixed());
        buf_ptr
            .get_mut(14..15)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&first_key_sym.serialize_fixed());
        buf_ptr
            .get_mut(15..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&n_key_syms.serialize_fixed());
        buf_ptr
            .get_mut(16..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&total_syms.serialize_fixed());
        buf_ptr
            .get_mut(18..19)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&first_key_action.serialize_fixed());
        buf_ptr
            .get_mut(19..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&n_key_actions.serialize_fixed());
        buf_ptr
            .get_mut(20..22)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&total_actions.serialize_fixed());
        buf_ptr
            .get_mut(22..23)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&first_key_behavior.serialize_fixed());
        buf_ptr
            .get_mut(23..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&n_key_behaviors.serialize_fixed());
        buf_ptr
            .get_mut(24..25)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&total_key_behaviors.serialize_fixed());
        buf_ptr
            .get_mut(25..26)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&first_key_explicit.serialize_fixed());
        buf_ptr
            .get_mut(26..27)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&n_key_explicit.serialize_fixed());
        buf_ptr
            .get_mut(27..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&total_key_explicit.serialize_fixed());
        buf_ptr
            .get_mut(28..29)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&first_mod_map_key.serialize_fixed());
        buf_ptr
            .get_mut(29..30)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&n_mod_map_keys.serialize_fixed());
        buf_ptr
            .get_mut(30..31)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&total_mod_map_keys.serialize_fixed());
        buf_ptr
            .get_mut(31..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&first_v_mod_map_key.serialize_fixed());
        buf_ptr
            .get_mut(32..33)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&n_v_mod_map_keys.serialize_fixed());
        buf_ptr
            .get_mut(33..34)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&total_v_mod_map_keys.serialize_fixed());
        buf_ptr
            .get_mut(34..36)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&virtual_mods.serialize_fixed());
        let mut offset = 36
            + set_map_values.serialize_into(
                buf_ptr
                    .get_mut(36..)
                    .ok_or(crate::error::Error::Serialize)?,
            )?;
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
pub fn get_compat_map<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    groups: crate::proto::xkb::SetOfGroup,
    get_all_s_i: u8,
    first_s_i: u16,
    n_s_i: u16,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::xkb::GetCompatMapReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let device_spec_bytes = device_spec.serialize_fixed();
    let first_s_i_bytes = first_s_i.serialize_fixed();
    let n_s_i_bytes = n_s_i.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                10,
                length[0],
                length[1],
                device_spec_bytes[0],
                device_spec_bytes[1],
                groups.0 as u8,
                get_all_s_i,
                first_s_i_bytes[0],
                first_s_i_bytes[1],
                n_s_i_bytes[0],
                n_s_i_bytes[1],
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
pub fn set_compat_map<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    recompute_actions: u8,
    truncate_s_i: u8,
    groups: crate::proto::xkb::SetOfGroup,
    first_s_i: u16,
    si: &[crate::proto::xkb::SymInterpret],
    group_maps: &[crate::proto::xkb::ModDef],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        // Pad 1 bytes
        let groups = u8::try_from(groups.0).map_err(|_| crate::error::Error::Serialize)?;
        let n_s_i = u16::try_from(si.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&device_spec.serialize_fixed());
        buf_ptr
            .get_mut(7..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&recompute_actions.serialize_fixed());
        buf_ptr
            .get_mut(8..9)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&truncate_s_i.serialize_fixed());
        buf_ptr
            .get_mut(9..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(groups.count_ones()).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(10..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&first_s_i.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(n_s_i).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = si.len() * 16;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            si,
        )?;
        let mut offset = list_len + 16;
        let list_len = group_maps.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            group_maps,
        )?;
        offset += list_len;
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
pub fn get_indicator_state<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::xkb::GetIndicatorStateReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let device_spec_bytes = device_spec.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                12,
                length[0],
                length[1],
                device_spec_bytes[0],
                device_spec_bytes[1],
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
pub fn get_indicator_map<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    which: u32,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::xkb::GetIndicatorMapReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let device_spec_bytes = device_spec.serialize_fixed();
    let which_bytes = which.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                13,
                length[0],
                length[1],
                device_spec_bytes[0],
                device_spec_bytes[1],
                0,
                0,
                which_bytes[0],
                which_bytes[1],
                which_bytes[2],
                which_bytes[3],
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
pub fn set_indicator_map<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    which: u32,
    maps: &[crate::proto::xkb::IndicatorMap],
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        // Pad 2 bytes
        let which = u32::try_from(which).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&device_spec.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(which.count_ones()).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = maps.len() * 12;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            maps,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 14]);
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
pub fn get_named_indicator<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    led_class: crate::proto::xkb::LedClassEnum,
    led_i_d: crate::proto::xkb::IdEnum,
    indicator: u32,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::xkb::GetNamedIndicatorReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let device_spec_bytes = device_spec.serialize_fixed();
    let led_class_bytes = (led_class.0 as u16).serialize_fixed();
    let led_i_d_bytes = (led_i_d.0 as u16).serialize_fixed();
    let indicator_bytes = indicator.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                15,
                length[0],
                length[1],
                device_spec_bytes[0],
                device_spec_bytes[1],
                led_class_bytes[0],
                led_class_bytes[1],
                led_i_d_bytes[0],
                led_i_d_bytes[1],
                0,
                0,
                indicator_bytes[0],
                indicator_bytes[1],
                indicator_bytes[2],
                indicator_bytes[3],
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
pub fn set_named_indicator<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    led_class: crate::proto::xkb::LedClassEnum,
    led_i_d: crate::proto::xkb::IdEnum,
    indicator: u32,
    set_state: u8,
    on: u8,
    set_map: u8,
    create_map: u8,
    map_flags: crate::proto::xkb::IMFlag,
    map_whichgroups: crate::proto::xkb::IMGroupsWhich,
    map_groups: crate::proto::xkb::SetOfGroups,
    map_whichmods: crate::proto::xkb::IMModsWhich,
    map_realmods: crate::proto::xproto::ModMask,
    map_vmods: crate::proto::xkb::VMod,
    map_ctrls: crate::proto::xkb::BoolCtrl,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (8u16).to_ne_bytes();
    let device_spec_bytes = device_spec.serialize_fixed();
    let led_class_bytes = (led_class.0 as u16).serialize_fixed();
    let led_i_d_bytes = (led_i_d.0 as u16).serialize_fixed();
    let indicator_bytes = indicator.serialize_fixed();
    let map_vmods_bytes = (map_vmods.0 as u16).serialize_fixed();
    let map_ctrls_bytes = (map_ctrls.0 as u32).serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                16,
                length[0],
                length[1],
                device_spec_bytes[0],
                device_spec_bytes[1],
                led_class_bytes[0],
                led_class_bytes[1],
                led_i_d_bytes[0],
                led_i_d_bytes[1],
                0,
                0,
                indicator_bytes[0],
                indicator_bytes[1],
                indicator_bytes[2],
                indicator_bytes[3],
                set_state,
                on,
                set_map,
                create_map,
                0,
                map_flags.0 as u8,
                map_whichgroups.0 as u8,
                map_groups.0 as u8,
                map_whichmods.0 as u8,
                map_realmods.0 as u8,
                map_vmods_bytes[0],
                map_vmods_bytes[1],
                map_ctrls_bytes[0],
                map_ctrls_bytes[1],
                map_ctrls_bytes[2],
                map_ctrls_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(32)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(VoidCookie::new(seq))
}
pub fn get_names<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    which: crate::proto::xkb::NameDetail,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::xkb::GetNamesReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let device_spec_bytes = device_spec.serialize_fixed();
    let which_bytes = (which.0 as u32).serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                17,
                length[0],
                length[1],
                device_spec_bytes[0],
                device_spec_bytes[1],
                0,
                0,
                which_bytes[0],
                which_bytes[1],
                which_bytes[2],
                which_bytes[3],
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
pub fn set_names<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    virtual_mods: crate::proto::xkb::VMod,
    first_type: u8,
    n_types: u8,
    first_k_t_levelt: u8,
    n_k_t_levels: u8,
    indicators: u32,
    group_names: crate::proto::xkb::SetOfGroup,
    n_radio_groups: u8,
    first_key: crate::proto::xproto::Keycode,
    n_keys: u8,
    n_key_aliases: u8,
    total_k_t_level_names: u16,
    set_names_values: crate::proto::xkb::SetNamesValues,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        // Pad 1 bytes
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 18]);
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&device_spec.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&virtual_mods.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&set_names_values.switch_expr().serialize_fixed());
        buf_ptr
            .get_mut(12..13)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&first_type.serialize_fixed());
        buf_ptr
            .get_mut(13..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&n_types.serialize_fixed());
        buf_ptr
            .get_mut(14..15)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&first_k_t_levelt.serialize_fixed());
        buf_ptr
            .get_mut(15..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&n_k_t_levels.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&indicators.serialize_fixed());
        buf_ptr
            .get_mut(20..21)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&group_names.serialize_fixed());
        buf_ptr
            .get_mut(21..22)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&n_radio_groups.serialize_fixed());
        buf_ptr
            .get_mut(22..23)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&first_key.serialize_fixed());
        buf_ptr
            .get_mut(23..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&n_keys.serialize_fixed());
        buf_ptr
            .get_mut(24..25)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&n_key_aliases.serialize_fixed());
        buf_ptr
            .get_mut(26..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&total_k_t_level_names.serialize_fixed());
        let mut offset = 28
            + set_names_values.serialize_into(
                buf_ptr
                    .get_mut(28..)
                    .ok_or(crate::error::Error::Serialize)?,
            )?;
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
pub fn per_client_flags<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    change: crate::proto::xkb::PerClientFlag,
    value: crate::proto::xkb::PerClientFlag,
    ctrls_to_change: crate::proto::xkb::BoolCtrl,
    auto_ctrls: crate::proto::xkb::BoolCtrl,
    auto_ctrls_values: crate::proto::xkb::BoolCtrl,
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::xkb::PerClientFlagsReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (7u16).to_ne_bytes();
    let device_spec_bytes = device_spec.serialize_fixed();
    let change_bytes = (change.0 as u32).serialize_fixed();
    let value_bytes = (value.0 as u32).serialize_fixed();
    let ctrls_to_change_bytes = (ctrls_to_change.0 as u32).serialize_fixed();
    let auto_ctrls_bytes = (auto_ctrls.0 as u32).serialize_fixed();
    let auto_ctrls_values_bytes = (auto_ctrls_values.0 as u32).serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                21,
                length[0],
                length[1],
                device_spec_bytes[0],
                device_spec_bytes[1],
                0,
                0,
                change_bytes[0],
                change_bytes[1],
                change_bytes[2],
                change_bytes[3],
                value_bytes[0],
                value_bytes[1],
                value_bytes[2],
                value_bytes[3],
                ctrls_to_change_bytes[0],
                ctrls_to_change_bytes[1],
                ctrls_to_change_bytes[2],
                ctrls_to_change_bytes[3],
                auto_ctrls_bytes[0],
                auto_ctrls_bytes[1],
                auto_ctrls_bytes[2],
                auto_ctrls_bytes[3],
                auto_ctrls_values_bytes[0],
                auto_ctrls_values_bytes[1],
                auto_ctrls_values_bytes[2],
                auto_ctrls_values_bytes[3],
            ]);
        Ok::<usize, crate::error::Error>(28)
    })?;
    let seq = if forget {
        xcb_state.next_seq()
    } else {
        xcb_state.keep_and_return_next_seq()
    };
    Ok(FixedCookie::new(seq))
}
pub fn list_components<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    max_names: u16,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::xkb::ListComponentsReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (2u16).to_ne_bytes();
    let device_spec_bytes = device_spec.serialize_fixed();
    let max_names_bytes = max_names.serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                22,
                length[0],
                length[1],
                device_spec_bytes[0],
                device_spec_bytes[1],
                max_names_bytes[0],
                max_names_bytes[1],
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
pub fn get_kbd_by_name<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    need: crate::proto::xkb::GBNDetail,
    want: crate::proto::xkb::GBNDetail,
    load: u8,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::xkb::GetKbdByNameReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (3u16).to_ne_bytes();
    let device_spec_bytes = device_spec.serialize_fixed();
    let need_bytes = (need.0 as u16).serialize_fixed();
    let want_bytes = (want.0 as u16).serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                23,
                length[0],
                length[1],
                device_spec_bytes[0],
                device_spec_bytes[1],
                need_bytes[0],
                need_bytes[1],
                want_bytes[0],
                want_bytes[1],
                load,
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
pub fn get_device_info<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    wanted: crate::proto::xkb::XIFeature,
    all_buttons: u8,
    first_button: u8,
    n_buttons: u8,
    led_class: crate::proto::xkb::LedClassEnum,
    led_i_d: crate::proto::xkb::IdEnum,
    forget: bool,
) -> crate::error::Result<Cookie<crate::proto::xkb::GetDeviceInfoReply>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    let length: [u8; 2] = (4u16).to_ne_bytes();
    let device_spec_bytes = device_spec.serialize_fixed();
    let wanted_bytes = (wanted.0 as u16).serialize_fixed();
    let led_class_bytes = (led_class.0 as u16).serialize_fixed();
    let led_i_d_bytes = (led_i_d.0 as u16).serialize_fixed();
    io.use_write_buffer(|buf| {
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                24,
                length[0],
                length[1],
                device_spec_bytes[0],
                device_spec_bytes[1],
                wanted_bytes[0],
                wanted_bytes[1],
                all_buttons,
                first_button,
                n_buttons,
                0,
                led_class_bytes[0],
                led_class_bytes[1],
                led_i_d_bytes[0],
                led_i_d_bytes[1],
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
pub fn set_device_info<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    device_spec: crate::proto::xkb::DeviceSpec,
    first_btn: u8,
    n_btns: u8,
    change: crate::proto::xkb::XIFeature,
    n_device_led_f_bs: u16,
    btn_actions: &[crate::proto::xkb::Action],
    leds: alloc::vec::Vec<crate::proto::xkb::DeviceLedInfo>,
    forget: bool,
) -> crate::error::Result<VoidCookie>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let n_btns = u8::try_from(n_btns).map_err(|_| crate::error::Error::Serialize)?;
        let n_device_led_f_bs =
            u16::try_from(n_device_led_f_bs).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&device_spec.serialize_fixed());
        buf_ptr
            .get_mut(6..7)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&first_btn.serialize_fixed());
        buf_ptr
            .get_mut(7..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(n_btns).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&change.serialize_fixed());
        buf_ptr
            .get_mut(10..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(n_device_led_f_bs).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = btn_actions.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            btn_actions,
        )?;
        let mut offset = list_len + 12;
        offset += crate::util::var_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            leds,
        )?;
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
pub fn set_debugging_flags<IO, XS>(
    io: &mut IO,
    xcb_state: &mut XS,
    affect_flags: u32,
    flags: u32,
    affect_ctrls: u32,
    ctrls: u32,
    message: &[crate::proto::xkb::String8],
    forget: bool,
) -> crate::error::Result<FixedCookie<crate::proto::xkb::SetDebuggingFlagsReply, 32>>
where
    IO: crate::con::SocketIo,
    XS: crate::con::XcbState,
{
    let major_opcode = xcb_state
        .major_opcode(crate::proto::xkb::EXTENSION_NAME)
        .ok_or(crate::error::Error::MissingExtension(
            crate::proto::xkb::EXTENSION_NAME,
        ))?;
    io.use_write_buffer(|buf_ptr| {
        let msg_length =
            u16::try_from(message.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(msg_length).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&affect_flags.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&flags.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&affect_ctrls.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&ctrls.serialize_fixed());
        let list_len = message.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            message,
        )?;
        let mut offset = list_len + 24;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 101]);
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
    Ok(FixedCookie::new(seq))
}
