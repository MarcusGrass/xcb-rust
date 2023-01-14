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
pub trait XinputConnection {
    fn get_extension_version(
        &mut self,
        socket_buffer: &mut [u8],
        name: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::GetExtensionVersionReply, 32>>;

    fn list_input_devices(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::ListInputDevicesReply>>;

    fn open_device(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::OpenDeviceReply>>;

    fn close_device(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn set_device_mode(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        mode: crate::proto::xinput::ValuatorModeEnum,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::SetDeviceModeReply, 32>>;

    fn select_extension_event(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        classes: &[crate::proto::xinput::EventClass],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_selected_extension_events(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetSelectedExtensionEventsReply>>;

    fn change_device_dont_propagate_list(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        mode: crate::proto::xinput::PropagateModeEnum,
        classes: &[crate::proto::xinput::EventClass],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_device_dont_propagate_list(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetDeviceDontPropagateListReply>>;

    fn get_device_motion_events(
        &mut self,
        socket_buffer: &mut [u8],
        start: crate::proto::xproto::Timestamp,
        stop: crate::proto::xproto::TimeEnum,
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetDeviceMotionEventsReply>>;

    fn change_keyboard_device(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::ChangeKeyboardDeviceReply, 32>>;

    fn change_pointer_device(
        &mut self,
        socket_buffer: &mut [u8],
        x_axis: u8,
        y_axis: u8,
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::ChangePointerDeviceReply, 32>>;

    fn grab_device(
        &mut self,
        socket_buffer: &mut [u8],
        grab_window: crate::proto::xproto::Window,
        time: crate::proto::xproto::TimeEnum,
        this_device_mode: crate::proto::xproto::GrabModeEnum,
        other_device_mode: crate::proto::xproto::GrabModeEnum,
        owner_events: u8,
        device_id: u8,
        classes: &[crate::proto::xinput::EventClass],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::GrabDeviceReply, 32>>;

    fn ungrab_device(
        &mut self,
        socket_buffer: &mut [u8],
        time: crate::proto::xproto::TimeEnum,
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn grab_device_key(
        &mut self,
        socket_buffer: &mut [u8],
        grab_window: crate::proto::xproto::Window,
        modifiers: crate::proto::xproto::ModMask,
        modifier_device: crate::proto::xinput::ModifierDeviceEnum,
        grabbed_device: u8,
        key: crate::proto::xproto::GrabEnum,
        this_device_mode: crate::proto::xproto::GrabModeEnum,
        other_device_mode: crate::proto::xproto::GrabModeEnum,
        owner_events: u8,
        classes: &[crate::proto::xinput::EventClass],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn ungrab_device_key(
        &mut self,
        socket_buffer: &mut [u8],
        grab_window: crate::proto::xproto::Window,
        modifiers: crate::proto::xproto::ModMask,
        modifier_device: crate::proto::xinput::ModifierDeviceEnum,
        key: crate::proto::xproto::GrabEnum,
        grabbed_device: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn grab_device_button(
        &mut self,
        socket_buffer: &mut [u8],
        grab_window: crate::proto::xproto::Window,
        grabbed_device: u8,
        modifier_device: crate::proto::xinput::ModifierDeviceEnum,
        modifiers: crate::proto::xproto::ModMask,
        this_device_mode: crate::proto::xproto::GrabModeEnum,
        other_device_mode: crate::proto::xproto::GrabModeEnum,
        button: crate::proto::xproto::GrabEnum,
        owner_events: u8,
        classes: &[crate::proto::xinput::EventClass],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn ungrab_device_button(
        &mut self,
        socket_buffer: &mut [u8],
        grab_window: crate::proto::xproto::Window,
        modifiers: crate::proto::xproto::ModMask,
        modifier_device: crate::proto::xinput::ModifierDeviceEnum,
        button: crate::proto::xproto::GrabEnum,
        grabbed_device: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn allow_device_events(
        &mut self,
        socket_buffer: &mut [u8],
        time: crate::proto::xproto::TimeEnum,
        mode: crate::proto::xinput::DeviceInputModeEnum,
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_device_focus(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::GetDeviceFocusReply, 32>>;

    fn set_device_focus(
        &mut self,
        socket_buffer: &mut [u8],
        focus: crate::proto::xproto::InputFocusEnum,
        time: crate::proto::xproto::TimeEnum,
        revert_to: crate::proto::xproto::InputFocusEnum,
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_feedback_control(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetFeedbackControlReply>>;

    fn change_feedback_control(
        &mut self,
        socket_buffer: &mut [u8],
        mask: crate::proto::xinput::ChangeFeedbackControlMask,
        device_id: u8,
        feedback_id: u8,
        feedback: crate::proto::xinput::FeedbackCtl,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_device_key_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        first_keycode: crate::proto::xinput::KeyCode,
        count: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetDeviceKeyMappingReply>>;

    fn change_device_key_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        first_keycode: crate::proto::xinput::KeyCode,
        keysyms_per_keycode: u8,
        keycode_count: u8,
        keysyms: &[crate::proto::xproto::Keysym],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_device_modifier_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetDeviceModifierMappingReply>>;

    fn set_device_modifier_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        keycodes_per_modifier: u8,
        keymaps: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::SetDeviceModifierMappingReply, 32>>;

    fn get_device_button_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetDeviceButtonMappingReply>>;

    fn set_device_button_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        map: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::SetDeviceButtonMappingReply, 32>>;

    fn query_device_state(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::QueryDeviceStateReply>>;

    fn device_bell(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        feedback_id: u8,
        feedback_class: u8,
        percent: i8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn set_device_valuators(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        first_valuator: u8,
        valuators: &[i32],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::SetDeviceValuatorsReply, 32>>;

    fn get_device_control(
        &mut self,
        socket_buffer: &mut [u8],
        control_id: crate::proto::xinput::DeviceControlEnum,
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetDeviceControlReply>>;

    fn change_device_control(
        &mut self,
        socket_buffer: &mut [u8],
        control_id: crate::proto::xinput::DeviceControlEnum,
        device_id: u8,
        control: crate::proto::xinput::DeviceCtl,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::ChangeDeviceControlReply, 32>>;

    fn list_device_properties(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::ListDevicePropertiesReply>>;

    fn change_device_property(
        &mut self,
        socket_buffer: &mut [u8],
        property: u32,
        r#type: u32,
        device_id: u8,
        format: crate::proto::xinput::PropertyFormatEnum,
        mode: crate::proto::xproto::PropModeEnum,
        num_items: u32,
        change_device_property_switch_enum: crate::proto::xinput::ChangeDevicePropertySwitchEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn delete_device_property(
        &mut self,
        socket_buffer: &mut [u8],
        property: u32,
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_device_property(
        &mut self,
        socket_buffer: &mut [u8],
        property: u32,
        r#type: u32,
        offset: u32,
        len: u32,
        device_id: u8,
        delete: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetDevicePropertyReply>>;

    fn x_i_query_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::XIQueryPointerReply>>;

    fn x_i_warp_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        src_win: crate::proto::xproto::Window,
        dst_win: crate::proto::xproto::Window,
        src_x: crate::proto::xinput::Fp1616,
        src_y: crate::proto::xinput::Fp1616,
        src_width: u16,
        src_height: u16,
        dst_x: crate::proto::xinput::Fp1616,
        dst_y: crate::proto::xinput::Fp1616,
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn x_i_change_cursor(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        cursor: crate::proto::xproto::Cursor,
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn x_i_change_hierarchy(
        &mut self,
        socket_buffer: &mut [u8],
        changes: alloc::vec::Vec<crate::proto::xinput::HierarchyChange>,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn x_i_set_client_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn x_i_get_client_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::XIGetClientPointerReply, 32>>;

    fn x_i_select_events(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        masks: alloc::vec::Vec<crate::proto::xinput::EventMask>,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn x_i_query_version(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u16,
        minor_version: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::XIQueryVersionReply, 32>>;

    fn x_i_query_device(
        &mut self,
        socket_buffer: &mut [u8],
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::XIQueryDeviceReply>>;

    fn x_i_set_focus(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        time: crate::proto::xproto::TimeEnum,
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn x_i_get_focus(
        &mut self,
        socket_buffer: &mut [u8],
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::XIGetFocusReply, 32>>;

    fn x_i_grab_device(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        time: crate::proto::xproto::TimeEnum,
        cursor: crate::proto::xproto::Cursor,
        deviceid: crate::proto::xinput::DeviceEnum,
        mode: crate::proto::xproto::GrabModeEnum,
        paired_device_mode: crate::proto::xproto::GrabModeEnum,
        owner_events: crate::proto::xinput::GrabOwnerEnum,
        mask: &[u32],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::XIGrabDeviceReply, 32>>;

    fn x_i_ungrab_device(
        &mut self,
        socket_buffer: &mut [u8],
        time: crate::proto::xproto::TimeEnum,
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn x_i_allow_events(
        &mut self,
        socket_buffer: &mut [u8],
        time: crate::proto::xproto::TimeEnum,
        deviceid: crate::proto::xinput::DeviceEnum,
        event_mode: crate::proto::xinput::EventModeEnum,
        touchid: u32,
        grab_window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn x_i_passive_grab_device(
        &mut self,
        socket_buffer: &mut [u8],
        time: crate::proto::xproto::TimeEnum,
        grab_window: crate::proto::xproto::Window,
        cursor: crate::proto::xproto::Cursor,
        detail: u32,
        deviceid: crate::proto::xinput::DeviceEnum,
        num_modifiers: u16,
        mask_len: u16,
        grab_type: crate::proto::xinput::GrabTypeEnum,
        grab_mode: crate::proto::xinput::GrabMode22Enum,
        paired_device_mode: crate::proto::xproto::GrabModeEnum,
        owner_events: crate::proto::xinput::GrabOwnerEnum,
        mask: &[u32],
        modifiers: &[u32],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::XIPassiveGrabDeviceReply>>;

    fn x_i_passive_ungrab_device(
        &mut self,
        socket_buffer: &mut [u8],
        grab_window: crate::proto::xproto::Window,
        detail: u32,
        deviceid: crate::proto::xinput::DeviceEnum,
        grab_type: crate::proto::xinput::GrabTypeEnum,
        modifiers: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn x_i_list_properties(
        &mut self,
        socket_buffer: &mut [u8],
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::XIListPropertiesReply>>;

    fn x_i_change_property(
        &mut self,
        socket_buffer: &mut [u8],
        deviceid: crate::proto::xinput::DeviceEnum,
        mode: crate::proto::xproto::PropModeEnum,
        format: crate::proto::xinput::PropertyFormatEnum,
        property: u32,
        r#type: u32,
        num_items: u32,
        x_i_change_property_switch_enum: crate::proto::xinput::XIChangePropertySwitchEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn x_i_delete_property(
        &mut self,
        socket_buffer: &mut [u8],
        deviceid: crate::proto::xinput::DeviceEnum,
        property: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn x_i_get_property(
        &mut self,
        socket_buffer: &mut [u8],
        deviceid: crate::proto::xinput::DeviceEnum,
        delete: u8,
        property: u32,
        r#type: u32,
        offset: u32,
        len: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::XIGetPropertyReply>>;

    fn x_i_get_selected_events(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::XIGetSelectedEventsReply>>;

    fn x_i_barrier_release_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        barriers: &[crate::proto::xinput::BarrierReleasePointerInfo],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn send_extension_event(
        &mut self,
        socket_buffer: &mut [u8],
        destination: crate::proto::xproto::Window,
        device_id: u8,
        propagate: u8,
        num_classes: u16,
        num_events: u8,
        events: &[crate::proto::xinput::EventForSend],
        classes: &[crate::proto::xinput::EventClass],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;
}
impl<C> XinputConnection for C
where
    C: crate::con::XcbConnection,
{
    fn get_extension_version(
        &mut self,
        socket_buffer: &mut [u8],
        name: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::GetExtensionVersionReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let name_len = u16::try_from(name.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(name_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            name,
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
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn list_input_devices(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::ListInputDevicesReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 2;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn open_device(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::OpenDeviceReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 3, length[0], length[1], device_id, 0, 0, 0]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn close_device(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 4, length[0], length[1], device_id, 0, 0, 0]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn set_device_mode(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        mode: crate::proto::xinput::ValuatorModeEnum,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::SetDeviceModeReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                5,
                length[0],
                length[1],
                device_id,
                mode.0 as u8,
                0,
                0,
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn select_extension_event(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        classes: &[crate::proto::xinput::EventClass],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_classes =
            u16::try_from(classes.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&window.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_classes).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = classes.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            classes,
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
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_selected_extension_events(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetSelectedExtensionEventsReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                7,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn change_device_dont_propagate_list(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        mode: crate::proto::xinput::PropagateModeEnum,
        classes: &[crate::proto::xinput::EventClass],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_classes =
            u16::try_from(classes.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 1 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&window.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_classes).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(10..11)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&mode.serialize_fixed());
        let list_len = classes.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            classes,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 8]);
        debug_assert!(offset % 4 == 0, "Bad request length offset {offset}");
        let word_len = offset / 4;
        if let Ok(len) = u16::try_from(word_len) {
            let length: [u8; 2] = len.to_ne_bytes();
            buf_ptr
                .get_mut(2..4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&length);
        } else {
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_device_dont_propagate_list(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetDeviceDontPropagateListReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
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
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_device_motion_events(
        &mut self,
        socket_buffer: &mut [u8],
        start: crate::proto::xproto::Timestamp,
        stop: crate::proto::xproto::TimeEnum,
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetDeviceMotionEventsReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let start_bytes = start.serialize_fixed();
        let stop_bytes = (stop.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                10,
                length[0],
                length[1],
                start_bytes[0],
                start_bytes[1],
                start_bytes[2],
                start_bytes[3],
                stop_bytes[0],
                stop_bytes[1],
                stop_bytes[2],
                stop_bytes[3],
                device_id,
                0,
                0,
                0,
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn change_keyboard_device(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::ChangeKeyboardDeviceReply, 32>>
    {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 11, length[0], length[1], device_id, 0, 0, 0]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn change_pointer_device(
        &mut self,
        socket_buffer: &mut [u8],
        x_axis: u8,
        y_axis: u8,
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::ChangePointerDeviceReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                12,
                length[0],
                length[1],
                x_axis,
                y_axis,
                device_id,
                0,
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn grab_device(
        &mut self,
        socket_buffer: &mut [u8],
        grab_window: crate::proto::xproto::Window,
        time: crate::proto::xproto::TimeEnum,
        this_device_mode: crate::proto::xproto::GrabModeEnum,
        other_device_mode: crate::proto::xproto::GrabModeEnum,
        owner_events: u8,
        device_id: u8,
        classes: &[crate::proto::xinput::EventClass],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::GrabDeviceReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_classes =
            u16::try_from(classes.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&grab_window.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&time.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_classes).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(14..15)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&this_device_mode.serialize_fixed());
        buf_ptr
            .get_mut(15..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&other_device_mode.serialize_fixed());
        buf_ptr
            .get_mut(16..17)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&owner_events.serialize_fixed());
        buf_ptr
            .get_mut(17..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&device_id.serialize_fixed());
        let list_len = classes.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(20..)
                .ok_or(crate::error::Error::Serialize)?,
            classes,
        )?;
        let mut offset = list_len + 20;
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
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn ungrab_device(
        &mut self,
        socket_buffer: &mut [u8],
        time: crate::proto::xproto::TimeEnum,
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let time_bytes = (time.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                14,
                length[0],
                length[1],
                time_bytes[0],
                time_bytes[1],
                time_bytes[2],
                time_bytes[3],
                device_id,
                0,
                0,
                0,
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn grab_device_key(
        &mut self,
        socket_buffer: &mut [u8],
        grab_window: crate::proto::xproto::Window,
        modifiers: crate::proto::xproto::ModMask,
        modifier_device: crate::proto::xinput::ModifierDeviceEnum,
        grabbed_device: u8,
        key: crate::proto::xproto::GrabEnum,
        this_device_mode: crate::proto::xproto::GrabModeEnum,
        other_device_mode: crate::proto::xproto::GrabModeEnum,
        owner_events: u8,
        classes: &[crate::proto::xinput::EventClass],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_classes =
            u16::try_from(classes.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&grab_window.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_classes).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(10..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&modifiers.serialize_fixed());
        buf_ptr
            .get_mut(12..13)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&modifier_device.serialize_fixed());
        buf_ptr
            .get_mut(13..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&grabbed_device.serialize_fixed());
        buf_ptr
            .get_mut(14..15)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&key.serialize_fixed());
        buf_ptr
            .get_mut(15..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&this_device_mode.serialize_fixed());
        buf_ptr
            .get_mut(16..17)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&other_device_mode.serialize_fixed());
        buf_ptr
            .get_mut(17..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&owner_events.serialize_fixed());
        let list_len = classes.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(20..)
                .ok_or(crate::error::Error::Serialize)?,
            classes,
        )?;
        let mut offset = list_len + 20;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 15]);
        debug_assert!(offset % 4 == 0, "Bad request length offset {offset}");
        let word_len = offset / 4;
        if let Ok(len) = u16::try_from(word_len) {
            let length: [u8; 2] = len.to_ne_bytes();
            buf_ptr
                .get_mut(2..4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&length);
        } else {
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn ungrab_device_key(
        &mut self,
        socket_buffer: &mut [u8],
        grab_window: crate::proto::xproto::Window,
        modifiers: crate::proto::xproto::ModMask,
        modifier_device: crate::proto::xinput::ModifierDeviceEnum,
        key: crate::proto::xproto::GrabEnum,
        grabbed_device: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let grab_window_bytes = grab_window.serialize_fixed();
        let modifiers_bytes = (modifiers.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                16,
                length[0],
                length[1],
                grab_window_bytes[0],
                grab_window_bytes[1],
                grab_window_bytes[2],
                grab_window_bytes[3],
                modifiers_bytes[0],
                modifiers_bytes[1],
                modifier_device.0 as u8,
                key.0 as u8,
                grabbed_device,
                0,
                0,
                0,
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn grab_device_button(
        &mut self,
        socket_buffer: &mut [u8],
        grab_window: crate::proto::xproto::Window,
        grabbed_device: u8,
        modifier_device: crate::proto::xinput::ModifierDeviceEnum,
        modifiers: crate::proto::xproto::ModMask,
        this_device_mode: crate::proto::xproto::GrabModeEnum,
        other_device_mode: crate::proto::xproto::GrabModeEnum,
        button: crate::proto::xproto::GrabEnum,
        owner_events: u8,
        classes: &[crate::proto::xinput::EventClass],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_classes =
            u16::try_from(classes.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&grab_window.serialize_fixed());
        buf_ptr
            .get_mut(8..9)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&grabbed_device.serialize_fixed());
        buf_ptr
            .get_mut(9..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&modifier_device.serialize_fixed());
        buf_ptr
            .get_mut(10..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_classes).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&modifiers.serialize_fixed());
        buf_ptr
            .get_mut(14..15)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&this_device_mode.serialize_fixed());
        buf_ptr
            .get_mut(15..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&other_device_mode.serialize_fixed());
        buf_ptr
            .get_mut(16..17)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&button.serialize_fixed());
        buf_ptr
            .get_mut(17..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&owner_events.serialize_fixed());
        let list_len = classes.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(20..)
                .ok_or(crate::error::Error::Serialize)?,
            classes,
        )?;
        let mut offset = list_len + 20;
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
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn ungrab_device_button(
        &mut self,
        socket_buffer: &mut [u8],
        grab_window: crate::proto::xproto::Window,
        modifiers: crate::proto::xproto::ModMask,
        modifier_device: crate::proto::xinput::ModifierDeviceEnum,
        button: crate::proto::xproto::GrabEnum,
        grabbed_device: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let grab_window_bytes = grab_window.serialize_fixed();
        let modifiers_bytes = (modifiers.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                18,
                length[0],
                length[1],
                grab_window_bytes[0],
                grab_window_bytes[1],
                grab_window_bytes[2],
                grab_window_bytes[3],
                modifiers_bytes[0],
                modifiers_bytes[1],
                modifier_device.0 as u8,
                button.0 as u8,
                grabbed_device,
                0,
                0,
                0,
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn allow_device_events(
        &mut self,
        socket_buffer: &mut [u8],
        time: crate::proto::xproto::TimeEnum,
        mode: crate::proto::xinput::DeviceInputModeEnum,
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let time_bytes = (time.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                19,
                length[0],
                length[1],
                time_bytes[0],
                time_bytes[1],
                time_bytes[2],
                time_bytes[3],
                mode.0 as u8,
                device_id,
                0,
                0,
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_device_focus(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::GetDeviceFocusReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 20, length[0], length[1], device_id, 0, 0, 0]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn set_device_focus(
        &mut self,
        socket_buffer: &mut [u8],
        focus: crate::proto::xproto::InputFocusEnum,
        time: crate::proto::xproto::TimeEnum,
        revert_to: crate::proto::xproto::InputFocusEnum,
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let focus_bytes = (focus.0 as u32).serialize_fixed();
        let time_bytes = (time.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                21,
                length[0],
                length[1],
                focus_bytes[0],
                focus_bytes[1],
                focus_bytes[2],
                focus_bytes[3],
                time_bytes[0],
                time_bytes[1],
                time_bytes[2],
                time_bytes[3],
                revert_to.0 as u8,
                device_id,
                0,
                0,
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_feedback_control(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetFeedbackControlReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 22, length[0], length[1], device_id, 0, 0, 0]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn change_feedback_control(
        &mut self,
        socket_buffer: &mut [u8],
        mask: crate::proto::xinput::ChangeFeedbackControlMask,
        device_id: u8,
        feedback_id: u8,
        feedback: crate::proto::xinput::FeedbackCtl,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&mask.serialize_fixed());
        buf_ptr
            .get_mut(8..9)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&device_id.serialize_fixed());
        buf_ptr
            .get_mut(9..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&feedback_id.serialize_fixed());
        let mut offset = feedback.serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
        )?;
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
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_device_key_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        first_keycode: crate::proto::xinput::KeyCode,
        count: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetDeviceKeyMappingReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let first_keycode_bytes = first_keycode.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                24,
                length[0],
                length[1],
                device_id,
                first_keycode_bytes[0],
                count,
                0,
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn change_device_key_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        first_keycode: crate::proto::xinput::KeyCode,
        keysyms_per_keycode: u8,
        keycode_count: u8,
        keysyms: &[crate::proto::xproto::Keysym],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&device_id.serialize_fixed());
        buf_ptr
            .get_mut(5..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&first_keycode.serialize_fixed());
        buf_ptr
            .get_mut(6..7)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&keysyms_per_keycode.serialize_fixed());
        buf_ptr
            .get_mut(7..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&keycode_count.serialize_fixed());
        let list_len = keysyms.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            keysyms,
        )?;
        let mut offset = list_len + 8;
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
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_device_modifier_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetDeviceModifierMappingReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 26, length[0], length[1], device_id, 0, 0, 0]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn set_device_modifier_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        keycodes_per_modifier: u8,
        keymaps: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::SetDeviceModifierMappingReply, 32>>
    {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let keycodes_per_modifier =
            u8::try_from(keycodes_per_modifier).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&device_id.serialize_fixed());
        buf_ptr
            .get_mut(5..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(core::ops::Div::div(keycodes_per_modifier as u8, 8u8 as u8))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = keymaps.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            keymaps,
        )?;
        let mut offset = list_len + 8;
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
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn get_device_button_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetDeviceButtonMappingReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 28, length[0], length[1], device_id, 0, 0, 0]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn set_device_button_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        map: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::SetDeviceButtonMappingReply, 32>>
    {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let map_size = u8::try_from(map.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&device_id.serialize_fixed());
        buf_ptr
            .get_mut(5..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(map_size).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = map.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            map,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 29]);
        debug_assert!(offset % 4 == 0, "Bad request length offset {offset}");
        let word_len = offset / 4;
        if let Ok(len) = u16::try_from(word_len) {
            let length: [u8; 2] = len.to_ne_bytes();
            buf_ptr
                .get_mut(2..4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&length);
        } else {
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn query_device_state(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::QueryDeviceStateReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 30, length[0], length[1], device_id, 0, 0, 0]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn device_bell(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        feedback_id: u8,
        feedback_class: u8,
        percent: i8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let percent_bytes = percent.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                32,
                length[0],
                length[1],
                device_id,
                feedback_id,
                feedback_class,
                percent_bytes[0],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn set_device_valuators(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        first_valuator: u8,
        valuators: &[i32],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::SetDeviceValuatorsReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_valuators =
            u8::try_from(valuators.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 1 bytes
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&device_id.serialize_fixed());
        buf_ptr
            .get_mut(5..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&first_valuator.serialize_fixed());
        buf_ptr
            .get_mut(6..7)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(num_valuators).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = valuators.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            valuators,
        )?;
        let mut offset = list_len + 8;
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
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn get_device_control(
        &mut self,
        socket_buffer: &mut [u8],
        control_id: crate::proto::xinput::DeviceControlEnum,
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetDeviceControlReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let control_id_bytes = (control_id.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                34,
                length[0],
                length[1],
                control_id_bytes[0],
                control_id_bytes[1],
                device_id,
                0,
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn change_device_control(
        &mut self,
        socket_buffer: &mut [u8],
        control_id: crate::proto::xinput::DeviceControlEnum,
        device_id: u8,
        control: crate::proto::xinput::DeviceCtl,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::ChangeDeviceControlReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&control_id.serialize_fixed());
        buf_ptr
            .get_mut(6..7)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&device_id.serialize_fixed());
        let mut offset =
            control.serialize_into(buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?)?;
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
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn list_device_properties(
        &mut self,
        socket_buffer: &mut [u8],
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::ListDevicePropertiesReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 36, length[0], length[1], device_id, 0, 0, 0]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn change_device_property(
        &mut self,
        socket_buffer: &mut [u8],
        property: u32,
        r#type: u32,
        device_id: u8,
        format: crate::proto::xinput::PropertyFormatEnum,
        mode: crate::proto::xproto::PropModeEnum,
        num_items: u32,
        change_device_property_switch_enum: crate::proto::xinput::ChangeDevicePropertySwitchEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 37]);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&property.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&r#type.serialize_fixed());
        buf_ptr
            .get_mut(12..13)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&device_id.serialize_fixed());
        buf_ptr
            .get_mut(13..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&format.serialize_fixed());
        buf_ptr
            .get_mut(14..15)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&mode.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&num_items.serialize_fixed());
        let mut offset = 20
            + change_device_property_switch_enum.serialize_into(
                buf_ptr
                    .get_mut(20..)
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
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn delete_device_property(
        &mut self,
        socket_buffer: &mut [u8],
        property: u32,
        device_id: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let property_bytes = property.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                38,
                length[0],
                length[1],
                property_bytes[0],
                property_bytes[1],
                property_bytes[2],
                property_bytes[3],
                device_id,
                0,
                0,
                0,
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_device_property(
        &mut self,
        socket_buffer: &mut [u8],
        property: u32,
        r#type: u32,
        offset: u32,
        len: u32,
        device_id: u8,
        delete: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::GetDevicePropertyReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (6u16).to_ne_bytes();
        let property_bytes = property.serialize_fixed();
        let r#type_bytes = r#type.serialize_fixed();
        let offset_bytes = offset.serialize_fixed();
        let len_bytes = len.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                39,
                length[0],
                length[1],
                property_bytes[0],
                property_bytes[1],
                property_bytes[2],
                property_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                offset_bytes[0],
                offset_bytes[1],
                offset_bytes[2],
                offset_bytes[3],
                len_bytes[0],
                len_bytes[1],
                len_bytes[2],
                len_bytes[3],
                device_id,
                delete,
                0,
                0,
            ]);
        self.advance_writer(24);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn x_i_query_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::XIQueryPointerReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let deviceid_bytes = (deviceid.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                40,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                deviceid_bytes[0],
                deviceid_bytes[1],
                0,
                0,
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn x_i_warp_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        src_win: crate::proto::xproto::Window,
        dst_win: crate::proto::xproto::Window,
        src_x: crate::proto::xinput::Fp1616,
        src_y: crate::proto::xinput::Fp1616,
        src_width: u16,
        src_height: u16,
        dst_x: crate::proto::xinput::Fp1616,
        dst_y: crate::proto::xinput::Fp1616,
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (9u16).to_ne_bytes();
        let src_win_bytes = src_win.serialize_fixed();
        let dst_win_bytes = dst_win.serialize_fixed();
        let src_x_bytes = src_x.serialize_fixed();
        let src_y_bytes = src_y.serialize_fixed();
        let src_width_bytes = src_width.serialize_fixed();
        let src_height_bytes = src_height.serialize_fixed();
        let dst_x_bytes = dst_x.serialize_fixed();
        let dst_y_bytes = dst_y.serialize_fixed();
        let deviceid_bytes = (deviceid.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..36)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                41,
                length[0],
                length[1],
                src_win_bytes[0],
                src_win_bytes[1],
                src_win_bytes[2],
                src_win_bytes[3],
                dst_win_bytes[0],
                dst_win_bytes[1],
                dst_win_bytes[2],
                dst_win_bytes[3],
                src_x_bytes[0],
                src_x_bytes[1],
                src_x_bytes[2],
                src_x_bytes[3],
                src_y_bytes[0],
                src_y_bytes[1],
                src_y_bytes[2],
                src_y_bytes[3],
                src_width_bytes[0],
                src_width_bytes[1],
                src_height_bytes[0],
                src_height_bytes[1],
                dst_x_bytes[0],
                dst_x_bytes[1],
                dst_x_bytes[2],
                dst_x_bytes[3],
                dst_y_bytes[0],
                dst_y_bytes[1],
                dst_y_bytes[2],
                dst_y_bytes[3],
                deviceid_bytes[0],
                deviceid_bytes[1],
                0,
                0,
            ]);
        self.advance_writer(36);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn x_i_change_cursor(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        cursor: crate::proto::xproto::Cursor,
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let cursor_bytes = cursor.serialize_fixed();
        let deviceid_bytes = (deviceid.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                42,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                cursor_bytes[0],
                cursor_bytes[1],
                cursor_bytes[2],
                cursor_bytes[3],
                deviceid_bytes[0],
                deviceid_bytes[1],
                0,
                0,
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn x_i_change_hierarchy(
        &mut self,
        socket_buffer: &mut [u8],
        changes: alloc::vec::Vec<crate::proto::xinput::HierarchyChange>,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_changes =
            u8::try_from(changes.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 3 bytes
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(num_changes).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let mut offset = crate::util::var_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            changes,
        )?;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 43]);
        debug_assert!(offset % 4 == 0, "Bad request length offset {offset}");
        let word_len = offset / 4;
        if let Ok(len) = u16::try_from(word_len) {
            let length: [u8; 2] = len.to_ne_bytes();
            buf_ptr
                .get_mut(2..4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&length);
        } else {
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn x_i_set_client_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let deviceid_bytes = (deviceid.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                44,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                deviceid_bytes[0],
                deviceid_bytes[1],
                0,
                0,
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn x_i_get_client_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::XIGetClientPointerReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                45,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn x_i_select_events(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        masks: alloc::vec::Vec<crate::proto::xinput::EventMask>,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_mask = u16::try_from(masks.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&window.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_mask).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let mut offset = crate::util::var_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            masks,
        )?;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 46]);
        debug_assert!(offset % 4 == 0, "Bad request length offset {offset}");
        let word_len = offset / 4;
        if let Ok(len) = u16::try_from(word_len) {
            let length: [u8; 2] = len.to_ne_bytes();
            buf_ptr
                .get_mut(2..4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&length);
        } else {
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn x_i_query_version(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u16,
        minor_version: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::XIQueryVersionReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let major_version_bytes = major_version.serialize_fixed();
        let minor_version_bytes = minor_version.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                47,
                length[0],
                length[1],
                major_version_bytes[0],
                major_version_bytes[1],
                minor_version_bytes[0],
                minor_version_bytes[1],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn x_i_query_device(
        &mut self,
        socket_buffer: &mut [u8],
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::XIQueryDeviceReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let deviceid_bytes = (deviceid.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                48,
                length[0],
                length[1],
                deviceid_bytes[0],
                deviceid_bytes[1],
                0,
                0,
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn x_i_set_focus(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        time: crate::proto::xproto::TimeEnum,
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let time_bytes = (time.0 as u32).serialize_fixed();
        let deviceid_bytes = (deviceid.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                49,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                time_bytes[0],
                time_bytes[1],
                time_bytes[2],
                time_bytes[3],
                deviceid_bytes[0],
                deviceid_bytes[1],
                0,
                0,
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn x_i_get_focus(
        &mut self,
        socket_buffer: &mut [u8],
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::XIGetFocusReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let deviceid_bytes = (deviceid.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                50,
                length[0],
                length[1],
                deviceid_bytes[0],
                deviceid_bytes[1],
                0,
                0,
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn x_i_grab_device(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        time: crate::proto::xproto::TimeEnum,
        cursor: crate::proto::xproto::Cursor,
        deviceid: crate::proto::xinput::DeviceEnum,
        mode: crate::proto::xproto::GrabModeEnum,
        paired_device_mode: crate::proto::xproto::GrabModeEnum,
        owner_events: crate::proto::xinput::GrabOwnerEnum,
        mask: &[u32],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xinput::XIGrabDeviceReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        let mask_len = u16::try_from(mask.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&window.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&time.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&cursor.serialize_fixed());
        buf_ptr
            .get_mut(16..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&deviceid.serialize_fixed());
        buf_ptr
            .get_mut(18..19)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&mode.serialize_fixed());
        buf_ptr
            .get_mut(19..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&paired_device_mode.serialize_fixed());
        buf_ptr
            .get_mut(20..21)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&owner_events.serialize_fixed());
        buf_ptr
            .get_mut(22..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(mask_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = mask.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            mask,
        )?;
        let mut offset = list_len + 24;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 51]);
        debug_assert!(offset % 4 == 0, "Bad request length offset {offset}");
        let word_len = offset / 4;
        if let Ok(len) = u16::try_from(word_len) {
            let length: [u8; 2] = len.to_ne_bytes();
            buf_ptr
                .get_mut(2..4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&length);
        } else {
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn x_i_ungrab_device(
        &mut self,
        socket_buffer: &mut [u8],
        time: crate::proto::xproto::TimeEnum,
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let time_bytes = (time.0 as u32).serialize_fixed();
        let deviceid_bytes = (deviceid.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                52,
                length[0],
                length[1],
                time_bytes[0],
                time_bytes[1],
                time_bytes[2],
                time_bytes[3],
                deviceid_bytes[0],
                deviceid_bytes[1],
                0,
                0,
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn x_i_allow_events(
        &mut self,
        socket_buffer: &mut [u8],
        time: crate::proto::xproto::TimeEnum,
        deviceid: crate::proto::xinput::DeviceEnum,
        event_mode: crate::proto::xinput::EventModeEnum,
        touchid: u32,
        grab_window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (5u16).to_ne_bytes();
        let time_bytes = (time.0 as u32).serialize_fixed();
        let deviceid_bytes = (deviceid.0 as u16).serialize_fixed();
        let touchid_bytes = touchid.serialize_fixed();
        let grab_window_bytes = grab_window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                53,
                length[0],
                length[1],
                time_bytes[0],
                time_bytes[1],
                time_bytes[2],
                time_bytes[3],
                deviceid_bytes[0],
                deviceid_bytes[1],
                event_mode.0 as u8,
                0,
                touchid_bytes[0],
                touchid_bytes[1],
                touchid_bytes[2],
                touchid_bytes[3],
                grab_window_bytes[0],
                grab_window_bytes[1],
                grab_window_bytes[2],
                grab_window_bytes[3],
            ]);
        self.advance_writer(20);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn x_i_passive_grab_device(
        &mut self,
        socket_buffer: &mut [u8],
        time: crate::proto::xproto::TimeEnum,
        grab_window: crate::proto::xproto::Window,
        cursor: crate::proto::xproto::Cursor,
        detail: u32,
        deviceid: crate::proto::xinput::DeviceEnum,
        num_modifiers: u16,
        mask_len: u16,
        grab_type: crate::proto::xinput::GrabTypeEnum,
        grab_mode: crate::proto::xinput::GrabMode22Enum,
        paired_device_mode: crate::proto::xproto::GrabModeEnum,
        owner_events: crate::proto::xinput::GrabOwnerEnum,
        mask: &[u32],
        modifiers: &[u32],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::XIPassiveGrabDeviceReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_modifiers =
            u16::try_from(num_modifiers).map_err(|_| crate::error::Error::Serialize)?;
        let mask_len = u16::try_from(mask_len).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&time.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&grab_window.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&cursor.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&detail.serialize_fixed());
        buf_ptr
            .get_mut(20..22)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&deviceid.serialize_fixed());
        buf_ptr
            .get_mut(22..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_modifiers).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(24..26)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(mask_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(26..27)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&grab_type.serialize_fixed());
        buf_ptr
            .get_mut(27..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&grab_mode.serialize_fixed());
        buf_ptr
            .get_mut(28..29)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&paired_device_mode.serialize_fixed());
        buf_ptr
            .get_mut(29..30)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&owner_events.serialize_fixed());
        let list_len = mask.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(32..)
                .ok_or(crate::error::Error::Serialize)?,
            mask,
        )?;
        let mut offset = list_len + 32;
        let list_len = modifiers.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            modifiers,
        )?;
        offset += list_len;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 54]);
        debug_assert!(offset % 4 == 0, "Bad request length offset {offset}");
        let word_len = offset / 4;
        if let Ok(len) = u16::try_from(word_len) {
            let length: [u8; 2] = len.to_ne_bytes();
            buf_ptr
                .get_mut(2..4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&length);
        } else {
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn x_i_passive_ungrab_device(
        &mut self,
        socket_buffer: &mut [u8],
        grab_window: crate::proto::xproto::Window,
        detail: u32,
        deviceid: crate::proto::xinput::DeviceEnum,
        grab_type: crate::proto::xinput::GrabTypeEnum,
        modifiers: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_modifiers =
            u16::try_from(modifiers.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 3 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&grab_window.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&detail.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&deviceid.serialize_fixed());
        buf_ptr
            .get_mut(14..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_modifiers).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(16..17)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&grab_type.serialize_fixed());
        let list_len = modifiers.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(20..)
                .ok_or(crate::error::Error::Serialize)?,
            modifiers,
        )?;
        let mut offset = list_len + 20;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 55]);
        debug_assert!(offset % 4 == 0, "Bad request length offset {offset}");
        let word_len = offset / 4;
        if let Ok(len) = u16::try_from(word_len) {
            let length: [u8; 2] = len.to_ne_bytes();
            buf_ptr
                .get_mut(2..4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&length);
        } else {
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn x_i_list_properties(
        &mut self,
        socket_buffer: &mut [u8],
        deviceid: crate::proto::xinput::DeviceEnum,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::XIListPropertiesReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let deviceid_bytes = (deviceid.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                56,
                length[0],
                length[1],
                deviceid_bytes[0],
                deviceid_bytes[1],
                0,
                0,
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn x_i_change_property(
        &mut self,
        socket_buffer: &mut [u8],
        deviceid: crate::proto::xinput::DeviceEnum,
        mode: crate::proto::xproto::PropModeEnum,
        format: crate::proto::xinput::PropertyFormatEnum,
        property: u32,
        r#type: u32,
        num_items: u32,
        x_i_change_property_switch_enum: crate::proto::xinput::XIChangePropertySwitchEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 57]);
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&deviceid.serialize_fixed());
        buf_ptr
            .get_mut(6..7)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&mode.serialize_fixed());
        buf_ptr
            .get_mut(7..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&format.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&property.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&r#type.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&num_items.serialize_fixed());
        let mut offset = 20
            + x_i_change_property_switch_enum.serialize_into(
                buf_ptr
                    .get_mut(20..)
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
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn x_i_delete_property(
        &mut self,
        socket_buffer: &mut [u8],
        deviceid: crate::proto::xinput::DeviceEnum,
        property: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let deviceid_bytes = (deviceid.0 as u16).serialize_fixed();
        let property_bytes = property.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                58,
                length[0],
                length[1],
                deviceid_bytes[0],
                deviceid_bytes[1],
                0,
                0,
                property_bytes[0],
                property_bytes[1],
                property_bytes[2],
                property_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn x_i_get_property(
        &mut self,
        socket_buffer: &mut [u8],
        deviceid: crate::proto::xinput::DeviceEnum,
        delete: u8,
        property: u32,
        r#type: u32,
        offset: u32,
        len: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::XIGetPropertyReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (6u16).to_ne_bytes();
        let deviceid_bytes = (deviceid.0 as u16).serialize_fixed();
        let property_bytes = property.serialize_fixed();
        let r#type_bytes = r#type.serialize_fixed();
        let offset_bytes = offset.serialize_fixed();
        let len_bytes = len.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                59,
                length[0],
                length[1],
                deviceid_bytes[0],
                deviceid_bytes[1],
                delete,
                0,
                property_bytes[0],
                property_bytes[1],
                property_bytes[2],
                property_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                offset_bytes[0],
                offset_bytes[1],
                offset_bytes[2],
                offset_bytes[3],
                len_bytes[0],
                len_bytes[1],
                len_bytes[2],
                len_bytes[3],
            ]);
        self.advance_writer(24);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn x_i_get_selected_events(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xinput::XIGetSelectedEventsReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                60,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn x_i_barrier_release_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        barriers: &[crate::proto::xinput::BarrierReleasePointerInfo],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_barriers =
            u32::try_from(barriers.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(num_barriers).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = barriers.len() * 12;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            barriers,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 61]);
        debug_assert!(offset % 4 == 0, "Bad request length offset {offset}");
        let word_len = offset / 4;
        if let Ok(len) = u16::try_from(word_len) {
            let length: [u8; 2] = len.to_ne_bytes();
            buf_ptr
                .get_mut(2..4)
                .ok_or(crate::error::Error::Serialize)?
                .copy_from_slice(&length);
        } else {
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn send_extension_event(
        &mut self,
        socket_buffer: &mut [u8],
        destination: crate::proto::xproto::Window,
        device_id: u8,
        propagate: u8,
        num_classes: u16,
        num_events: u8,
        events: &[crate::proto::xinput::EventForSend],
        classes: &[crate::proto::xinput::EventClass],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xinput::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xinput::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_classes = u16::try_from(num_classes).map_err(|_| crate::error::Error::Serialize)?;
        let num_events = u8::try_from(num_events).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 3 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&destination.serialize_fixed());
        buf_ptr
            .get_mut(8..9)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&device_id.serialize_fixed());
        buf_ptr
            .get_mut(9..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&propagate.serialize_fixed());
        buf_ptr
            .get_mut(10..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_classes).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(12..13)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u8::try_from(num_events).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = events.len() * 32;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            events,
        )?;
        let mut offset = list_len + 16;
        let list_len = classes.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            classes,
        )?;
        offset += list_len;
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
            if word_len > self.max_request_size() {
                return Err(crate::error::Error::TooLargeRequest);
            }
            let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }
}
