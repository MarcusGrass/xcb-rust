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
pub trait SyncConnection {
    fn initialize(
        &mut self,
        socket_buffer: &mut [u8],
        desired_major_version: u8,
        desired_minor_version: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::sync::InitializeReply, 32>>;

    fn list_system_counters(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::sync::ListSystemCountersReply>>;

    fn create_counter(
        &mut self,
        socket_buffer: &mut [u8],
        id: crate::proto::sync::Counter,
        initial_value: crate::proto::sync::Int64,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn destroy_counter(
        &mut self,
        socket_buffer: &mut [u8],
        counter: crate::proto::sync::Counter,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn query_counter(
        &mut self,
        socket_buffer: &mut [u8],
        counter: crate::proto::sync::Counter,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::sync::QueryCounterReply, 16>>;

    fn r#await(
        &mut self,
        socket_buffer: &mut [u8],
        wait_list: &[crate::proto::sync::Waitcondition],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn change_counter(
        &mut self,
        socket_buffer: &mut [u8],
        counter: crate::proto::sync::Counter,
        amount: crate::proto::sync::Int64,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn set_counter(
        &mut self,
        socket_buffer: &mut [u8],
        counter: crate::proto::sync::Counter,
        value: crate::proto::sync::Int64,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn create_alarm(
        &mut self,
        socket_buffer: &mut [u8],
        id: crate::proto::sync::Alarm,
        create_alarm_value_list: crate::proto::sync::CreateAlarmValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn change_alarm(
        &mut self,
        socket_buffer: &mut [u8],
        id: crate::proto::sync::Alarm,
        change_alarm_value_list: crate::proto::sync::ChangeAlarmValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn destroy_alarm(
        &mut self,
        socket_buffer: &mut [u8],
        alarm: crate::proto::sync::Alarm,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn query_alarm(
        &mut self,
        socket_buffer: &mut [u8],
        alarm: crate::proto::sync::Alarm,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::sync::QueryAlarmReply, 40>>;

    fn set_priority(
        &mut self,
        socket_buffer: &mut [u8],
        id: u32,
        priority: i32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_priority(
        &mut self,
        socket_buffer: &mut [u8],
        id: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::sync::GetPriorityReply, 12>>;

    fn create_fence(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        fence: crate::proto::sync::Fence,
        initially_triggered: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn trigger_fence(
        &mut self,
        socket_buffer: &mut [u8],
        fence: crate::proto::sync::Fence,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn reset_fence(
        &mut self,
        socket_buffer: &mut [u8],
        fence: crate::proto::sync::Fence,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn destroy_fence(
        &mut self,
        socket_buffer: &mut [u8],
        fence: crate::proto::sync::Fence,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn query_fence(
        &mut self,
        socket_buffer: &mut [u8],
        fence: crate::proto::sync::Fence,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::sync::QueryFenceReply, 32>>;

    fn await_fence(
        &mut self,
        socket_buffer: &mut [u8],
        fence_list: &[crate::proto::sync::Fence],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;
}
impl<C> SyncConnection for C
where
    C: crate::con::XcbConnection,
{
    fn initialize(
        &mut self,
        socket_buffer: &mut [u8],
        desired_major_version: u8,
        desired_minor_version: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::sync::InitializeReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
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
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn list_system_counters(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::sync::ListSystemCountersReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 1;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn create_counter(
        &mut self,
        socket_buffer: &mut [u8],
        id: crate::proto::sync::Counter,
        initial_value: crate::proto::sync::Int64,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let id_bytes = id.serialize_fixed();
        let initial_value_bytes = initial_value.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn destroy_counter(
        &mut self,
        socket_buffer: &mut [u8],
        counter: crate::proto::sync::Counter,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let counter_bytes = counter.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn query_counter(
        &mut self,
        socket_buffer: &mut [u8],
        counter: crate::proto::sync::Counter,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::sync::QueryCounterReply, 16>> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let counter_bytes = counter.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn r#await(
        &mut self,
        socket_buffer: &mut [u8],
        wait_list: &[crate::proto::sync::Waitcondition],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
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

    fn change_counter(
        &mut self,
        socket_buffer: &mut [u8],
        counter: crate::proto::sync::Counter,
        amount: crate::proto::sync::Int64,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let counter_bytes = counter.serialize_fixed();
        let amount_bytes = amount.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn set_counter(
        &mut self,
        socket_buffer: &mut [u8],
        counter: crate::proto::sync::Counter,
        value: crate::proto::sync::Int64,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let counter_bytes = counter.serialize_fixed();
        let value_bytes = value.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn create_alarm(
        &mut self,
        socket_buffer: &mut [u8],
        id: crate::proto::sync::Alarm,
        create_alarm_value_list: crate::proto::sync::CreateAlarmValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn change_alarm(
        &mut self,
        socket_buffer: &mut [u8],
        id: crate::proto::sync::Alarm,
        change_alarm_value_list: crate::proto::sync::ChangeAlarmValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn destroy_alarm(
        &mut self,
        socket_buffer: &mut [u8],
        alarm: crate::proto::sync::Alarm,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let alarm_bytes = alarm.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn query_alarm(
        &mut self,
        socket_buffer: &mut [u8],
        alarm: crate::proto::sync::Alarm,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::sync::QueryAlarmReply, 40>> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let alarm_bytes = alarm.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn set_priority(
        &mut self,
        socket_buffer: &mut [u8],
        id: u32,
        priority: i32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let id_bytes = id.serialize_fixed();
        let priority_bytes = priority.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_priority(
        &mut self,
        socket_buffer: &mut [u8],
        id: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::sync::GetPriorityReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let id_bytes = id.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn create_fence(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        fence: crate::proto::sync::Fence,
        initially_triggered: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let fence_bytes = fence.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn trigger_fence(
        &mut self,
        socket_buffer: &mut [u8],
        fence: crate::proto::sync::Fence,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let fence_bytes = fence.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn reset_fence(
        &mut self,
        socket_buffer: &mut [u8],
        fence: crate::proto::sync::Fence,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let fence_bytes = fence.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn destroy_fence(
        &mut self,
        socket_buffer: &mut [u8],
        fence: crate::proto::sync::Fence,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let fence_bytes = fence.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn query_fence(
        &mut self,
        socket_buffer: &mut [u8],
        fence: crate::proto::sync::Fence,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::sync::QueryFenceReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let fence_bytes = fence.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn await_fence(
        &mut self,
        socket_buffer: &mut [u8],
        fence_list: &[crate::proto::sync::Fence],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::sync::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::sync::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
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
