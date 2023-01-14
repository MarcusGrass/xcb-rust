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
pub trait RandrConnection {
    fn query_version(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u32,
        minor_version: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::QueryVersionReply, 32>>;

    fn set_screen_config(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        timestamp: crate::proto::xproto::Timestamp,
        config_timestamp: crate::proto::xproto::Timestamp,
        size_i_d: u16,
        rotation: crate::proto::randr::Rotation,
        rate: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::SetScreenConfigReply, 32>>;

    fn select_input(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        enable: crate::proto::randr::NotifyMask,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_screen_info(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetScreenInfoReply>>;

    fn get_screen_size_range(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::GetScreenSizeRangeReply, 32>>;

    fn set_screen_size(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        width: u16,
        height: u16,
        mm_width: u32,
        mm_height: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_screen_resources(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetScreenResourcesReply>>;

    fn get_output_info(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        config_timestamp: crate::proto::xproto::Timestamp,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetOutputInfoReply>>;

    fn list_output_properties(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::ListOutputPropertiesReply>>;

    fn query_output_property(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        property: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::QueryOutputPropertyReply>>;

    fn configure_output_property(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        property: u32,
        pending: u8,
        range: u8,
        values: &[i32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn change_output_property(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        property: u32,
        r#type: u32,
        format: u8,
        mode: crate::proto::xproto::PropModeEnum,
        num_units: u32,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn delete_output_property(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        property: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_output_property(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        property: u32,
        r#type: crate::proto::xproto::GetPropertyTypeEnum,
        long_offset: u32,
        long_length: u32,
        delete: u8,
        pending: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetOutputPropertyReply>>;

    fn create_mode(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        mode_info: crate::proto::randr::ModeInfo,
        name: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::CreateModeReply, 32>>;

    fn destroy_mode(
        &mut self,
        socket_buffer: &mut [u8],
        mode: crate::proto::randr::Mode,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn add_output_mode(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        mode: crate::proto::randr::Mode,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn delete_output_mode(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        mode: crate::proto::randr::Mode,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_crtc_info(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        config_timestamp: crate::proto::xproto::Timestamp,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetCrtcInfoReply>>;

    fn set_crtc_config(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        timestamp: crate::proto::xproto::Timestamp,
        config_timestamp: crate::proto::xproto::Timestamp,
        x: i16,
        y: i16,
        mode: crate::proto::randr::Mode,
        rotation: crate::proto::randr::Rotation,
        outputs: &[crate::proto::randr::Output],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::SetCrtcConfigReply, 32>>;

    fn get_crtc_gamma_size(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::GetCrtcGammaSizeReply, 32>>;

    fn get_crtc_gamma(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetCrtcGammaReply>>;

    fn set_crtc_gamma(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        size: u16,
        red: &[u16],
        green: &[u16],
        blue: &[u16],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_screen_resources_current(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetScreenResourcesCurrentReply>>;

    fn set_crtc_transform(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        transform: crate::proto::render::Transform,
        filter_name: &[u8],
        filter_params: &[crate::proto::render::Fixed],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_crtc_transform(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetCrtcTransformReply>>;

    fn get_panning(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::GetPanningReply, 36>>;

    fn set_panning(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        timestamp: crate::proto::xproto::Timestamp,
        left: u16,
        top: u16,
        width: u16,
        height: u16,
        track_left: u16,
        track_top: u16,
        track_width: u16,
        track_height: u16,
        border_left: i16,
        border_top: i16,
        border_right: i16,
        border_bottom: i16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::SetPanningReply, 12>>;

    fn set_output_primary(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        output: crate::proto::randr::Output,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_output_primary(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::GetOutputPrimaryReply, 12>>;

    fn get_providers(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetProvidersReply>>;

    fn get_provider_info(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        config_timestamp: crate::proto::xproto::Timestamp,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetProviderInfoReply>>;

    fn set_provider_offload_sink(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        sink_provider: crate::proto::randr::Provider,
        config_timestamp: crate::proto::xproto::Timestamp,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn set_provider_output_source(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        source_provider: crate::proto::randr::Provider,
        config_timestamp: crate::proto::xproto::Timestamp,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn list_provider_properties(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::ListProviderPropertiesReply>>;

    fn query_provider_property(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        property: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::QueryProviderPropertyReply>>;

    fn configure_provider_property(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        property: u32,
        pending: u8,
        range: u8,
        values: &[i32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn change_provider_property(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        property: u32,
        r#type: u32,
        format: u8,
        mode: u8,
        num_items: u32,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn delete_provider_property(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        property: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_provider_property(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        property: u32,
        r#type: u32,
        long_offset: u32,
        long_length: u32,
        delete: u8,
        pending: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetProviderPropertyReply>>;

    fn get_monitors(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        get_active: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetMonitorsReply>>;

    fn set_monitor(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        monitorinfo: crate::proto::randr::MonitorInfo,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn delete_monitor(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        name: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn create_lease(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        lid: crate::proto::randr::Lease,
        num_crtcs: u16,
        num_outputs: u16,
        crtcs: &[crate::proto::randr::Crtc],
        outputs: &[crate::proto::randr::Output],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::CreateLeaseReply, 32>>;

    fn free_lease(
        &mut self,
        socket_buffer: &mut [u8],
        lid: crate::proto::randr::Lease,
        terminate: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;
}
impl<C> RandrConnection for C
where
    C: crate::con::XcbConnection,
{
    fn query_version(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u32,
        minor_version: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::QueryVersionReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let major_version_bytes = major_version.serialize_fixed();
        let minor_version_bytes = minor_version.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
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
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn set_screen_config(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        timestamp: crate::proto::xproto::Timestamp,
        config_timestamp: crate::proto::xproto::Timestamp,
        size_i_d: u16,
        rotation: crate::proto::randr::Rotation,
        rate: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::SetScreenConfigReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (6u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let timestamp_bytes = timestamp.serialize_fixed();
        let config_timestamp_bytes = config_timestamp.serialize_fixed();
        let size_i_d_bytes = size_i_d.serialize_fixed();
        let rotation_bytes = (rotation.0 as u16).serialize_fixed();
        let rate_bytes = rate.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..24)
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
                timestamp_bytes[0],
                timestamp_bytes[1],
                timestamp_bytes[2],
                timestamp_bytes[3],
                config_timestamp_bytes[0],
                config_timestamp_bytes[1],
                config_timestamp_bytes[2],
                config_timestamp_bytes[3],
                size_i_d_bytes[0],
                size_i_d_bytes[1],
                rotation_bytes[0],
                rotation_bytes[1],
                rate_bytes[0],
                rate_bytes[1],
                0,
                0,
            ]);
        self.advance_writer(24);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn select_input(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        enable: crate::proto::randr::NotifyMask,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let enable_bytes = (enable.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                4,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                enable_bytes[0],
                enable_bytes[1],
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

    fn get_screen_info(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetScreenInfoReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                5,
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

    fn get_screen_size_range(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::GetScreenSizeRangeReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
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
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn set_screen_size(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        width: u16,
        height: u16,
        mm_width: u32,
        mm_height: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (5u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let width_bytes = width.serialize_fixed();
        let height_bytes = height.serialize_fixed();
        let mm_width_bytes = mm_width.serialize_fixed();
        let mm_height_bytes = mm_height.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..20)
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
                width_bytes[0],
                width_bytes[1],
                height_bytes[0],
                height_bytes[1],
                mm_width_bytes[0],
                mm_width_bytes[1],
                mm_width_bytes[2],
                mm_width_bytes[3],
                mm_height_bytes[0],
                mm_height_bytes[1],
                mm_height_bytes[2],
                mm_height_bytes[3],
            ]);
        self.advance_writer(20);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_screen_resources(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetScreenResourcesReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                8,
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

    fn get_output_info(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        config_timestamp: crate::proto::xproto::Timestamp,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetOutputInfoReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let output_bytes = output.serialize_fixed();
        let config_timestamp_bytes = config_timestamp.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                9,
                length[0],
                length[1],
                output_bytes[0],
                output_bytes[1],
                output_bytes[2],
                output_bytes[3],
                config_timestamp_bytes[0],
                config_timestamp_bytes[1],
                config_timestamp_bytes[2],
                config_timestamp_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn list_output_properties(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::ListOutputPropertiesReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let output_bytes = output.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                10,
                length[0],
                length[1],
                output_bytes[0],
                output_bytes[1],
                output_bytes[2],
                output_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn query_output_property(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        property: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::QueryOutputPropertyReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let output_bytes = output.serialize_fixed();
        let property_bytes = property.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                11,
                length[0],
                length[1],
                output_bytes[0],
                output_bytes[1],
                output_bytes[2],
                output_bytes[3],
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
        Ok(Cookie::new(seq))
    }

    fn configure_output_property(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        property: u32,
        pending: u8,
        range: u8,
        values: &[i32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&output.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&property.serialize_fixed());
        buf_ptr
            .get_mut(12..13)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&pending.serialize_fixed());
        buf_ptr
            .get_mut(13..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&range.serialize_fixed());
        let list_len = values.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            values,
        )?;
        let mut offset = list_len + 16;
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

    fn change_output_property(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        property: u32,
        r#type: u32,
        format: u8,
        mode: crate::proto::xproto::PropModeEnum,
        num_units: u32,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&output.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&property.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&r#type.serialize_fixed());
        buf_ptr
            .get_mut(16..17)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&format.serialize_fixed());
        buf_ptr
            .get_mut(17..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&mode.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&num_units.serialize_fixed());
        let list_len = data.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            data,
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

    fn delete_output_property(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        property: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let output_bytes = output.serialize_fixed();
        let property_bytes = property.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                14,
                length[0],
                length[1],
                output_bytes[0],
                output_bytes[1],
                output_bytes[2],
                output_bytes[3],
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

    fn get_output_property(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        property: u32,
        r#type: crate::proto::xproto::GetPropertyTypeEnum,
        long_offset: u32,
        long_length: u32,
        delete: u8,
        pending: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetOutputPropertyReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (7u16).to_ne_bytes();
        let output_bytes = output.serialize_fixed();
        let property_bytes = property.serialize_fixed();
        let r#type_bytes = (r#type.0 as u32).serialize_fixed();
        let long_offset_bytes = long_offset.serialize_fixed();
        let long_length_bytes = long_length.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                15,
                length[0],
                length[1],
                output_bytes[0],
                output_bytes[1],
                output_bytes[2],
                output_bytes[3],
                property_bytes[0],
                property_bytes[1],
                property_bytes[2],
                property_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                long_offset_bytes[0],
                long_offset_bytes[1],
                long_offset_bytes[2],
                long_offset_bytes[3],
                long_length_bytes[0],
                long_length_bytes[1],
                long_length_bytes[2],
                long_length_bytes[3],
                delete,
                pending,
                0,
                0,
            ]);
        self.advance_writer(28);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn create_mode(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        mode_info: crate::proto::randr::ModeInfo,
        name: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::CreateModeReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&window.serialize_fixed());
        buf_ptr
            .get_mut(8..40)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&mode_info.serialize_fixed());
        let list_len = name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(40..)
                .ok_or(crate::error::Error::Serialize)?,
            name,
        )?;
        let mut offset = list_len + 40;
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

    fn destroy_mode(
        &mut self,
        socket_buffer: &mut [u8],
        mode: crate::proto::randr::Mode,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let mode_bytes = mode.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                17,
                length[0],
                length[1],
                mode_bytes[0],
                mode_bytes[1],
                mode_bytes[2],
                mode_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn add_output_mode(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        mode: crate::proto::randr::Mode,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let output_bytes = output.serialize_fixed();
        let mode_bytes = mode.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                18,
                length[0],
                length[1],
                output_bytes[0],
                output_bytes[1],
                output_bytes[2],
                output_bytes[3],
                mode_bytes[0],
                mode_bytes[1],
                mode_bytes[2],
                mode_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn delete_output_mode(
        &mut self,
        socket_buffer: &mut [u8],
        output: crate::proto::randr::Output,
        mode: crate::proto::randr::Mode,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let output_bytes = output.serialize_fixed();
        let mode_bytes = mode.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                19,
                length[0],
                length[1],
                output_bytes[0],
                output_bytes[1],
                output_bytes[2],
                output_bytes[3],
                mode_bytes[0],
                mode_bytes[1],
                mode_bytes[2],
                mode_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_crtc_info(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        config_timestamp: crate::proto::xproto::Timestamp,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetCrtcInfoReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let crtc_bytes = crtc.serialize_fixed();
        let config_timestamp_bytes = config_timestamp.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                20,
                length[0],
                length[1],
                crtc_bytes[0],
                crtc_bytes[1],
                crtc_bytes[2],
                crtc_bytes[3],
                config_timestamp_bytes[0],
                config_timestamp_bytes[1],
                config_timestamp_bytes[2],
                config_timestamp_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn set_crtc_config(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        timestamp: crate::proto::xproto::Timestamp,
        config_timestamp: crate::proto::xproto::Timestamp,
        x: i16,
        y: i16,
        mode: crate::proto::randr::Mode,
        rotation: crate::proto::randr::Rotation,
        outputs: &[crate::proto::randr::Output],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::SetCrtcConfigReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&crtc.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&timestamp.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&config_timestamp.serialize_fixed());
        buf_ptr
            .get_mut(16..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&x.serialize_fixed());
        buf_ptr
            .get_mut(18..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&y.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&mode.serialize_fixed());
        buf_ptr
            .get_mut(24..26)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&rotation.serialize_fixed());
        let list_len = outputs.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(28..)
                .ok_or(crate::error::Error::Serialize)?,
            outputs,
        )?;
        let mut offset = list_len + 28;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 21]);
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

    fn get_crtc_gamma_size(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::GetCrtcGammaSizeReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let crtc_bytes = crtc.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                22,
                length[0],
                length[1],
                crtc_bytes[0],
                crtc_bytes[1],
                crtc_bytes[2],
                crtc_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn get_crtc_gamma(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetCrtcGammaReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let crtc_bytes = crtc.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                23,
                length[0],
                length[1],
                crtc_bytes[0],
                crtc_bytes[1],
                crtc_bytes[2],
                crtc_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn set_crtc_gamma(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        size: u16,
        red: &[u16],
        green: &[u16],
        blue: &[u16],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let size = u16::try_from(size).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&crtc.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(size).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = red.len() * 2;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            red,
        )?;
        let mut offset = list_len + 12;
        let list_len = green.len() * 2;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            green,
        )?;
        offset += list_len;
        let list_len = blue.len() * 2;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            blue,
        )?;
        offset += list_len;
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

    fn get_screen_resources_current(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetScreenResourcesCurrentReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                25,
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

    fn set_crtc_transform(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        transform: crate::proto::render::Transform,
        filter_name: &[u8],
        filter_params: &[crate::proto::render::Fixed],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let filter_len =
            u16::try_from(filter_name.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&crtc.serialize_fixed());
        buf_ptr
            .get_mut(8..44)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&transform.serialize_fixed());
        buf_ptr
            .get_mut(44..46)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(filter_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = filter_name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(48..)
                .ok_or(crate::error::Error::Serialize)?,
            filter_name,
        )?;
        let mut offset = list_len + 48;
        offset += (4 - (offset % 4)) % 4;
        let list_len = filter_params.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            filter_params,
        )?;
        offset += list_len;
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

    fn get_crtc_transform(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetCrtcTransformReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let crtc_bytes = crtc.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                27,
                length[0],
                length[1],
                crtc_bytes[0],
                crtc_bytes[1],
                crtc_bytes[2],
                crtc_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_panning(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::GetPanningReply, 36>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let crtc_bytes = crtc.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                28,
                length[0],
                length[1],
                crtc_bytes[0],
                crtc_bytes[1],
                crtc_bytes[2],
                crtc_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn set_panning(
        &mut self,
        socket_buffer: &mut [u8],
        crtc: crate::proto::randr::Crtc,
        timestamp: crate::proto::xproto::Timestamp,
        left: u16,
        top: u16,
        width: u16,
        height: u16,
        track_left: u16,
        track_top: u16,
        track_width: u16,
        track_height: u16,
        border_left: i16,
        border_top: i16,
        border_right: i16,
        border_bottom: i16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::SetPanningReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (9u16).to_ne_bytes();
        let crtc_bytes = crtc.serialize_fixed();
        let timestamp_bytes = timestamp.serialize_fixed();
        let left_bytes = left.serialize_fixed();
        let top_bytes = top.serialize_fixed();
        let width_bytes = width.serialize_fixed();
        let height_bytes = height.serialize_fixed();
        let track_left_bytes = track_left.serialize_fixed();
        let track_top_bytes = track_top.serialize_fixed();
        let track_width_bytes = track_width.serialize_fixed();
        let track_height_bytes = track_height.serialize_fixed();
        let border_left_bytes = border_left.serialize_fixed();
        let border_top_bytes = border_top.serialize_fixed();
        let border_right_bytes = border_right.serialize_fixed();
        let border_bottom_bytes = border_bottom.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..36)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                29,
                length[0],
                length[1],
                crtc_bytes[0],
                crtc_bytes[1],
                crtc_bytes[2],
                crtc_bytes[3],
                timestamp_bytes[0],
                timestamp_bytes[1],
                timestamp_bytes[2],
                timestamp_bytes[3],
                left_bytes[0],
                left_bytes[1],
                top_bytes[0],
                top_bytes[1],
                width_bytes[0],
                width_bytes[1],
                height_bytes[0],
                height_bytes[1],
                track_left_bytes[0],
                track_left_bytes[1],
                track_top_bytes[0],
                track_top_bytes[1],
                track_width_bytes[0],
                track_width_bytes[1],
                track_height_bytes[0],
                track_height_bytes[1],
                border_left_bytes[0],
                border_left_bytes[1],
                border_top_bytes[0],
                border_top_bytes[1],
                border_right_bytes[0],
                border_right_bytes[1],
                border_bottom_bytes[0],
                border_bottom_bytes[1],
            ]);
        self.advance_writer(36);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn set_output_primary(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        output: crate::proto::randr::Output,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let output_bytes = output.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
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
                output_bytes[0],
                output_bytes[1],
                output_bytes[2],
                output_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_output_primary(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::GetOutputPrimaryReply, 12>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                31,
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

    fn get_providers(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetProvidersReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                32,
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

    fn get_provider_info(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        config_timestamp: crate::proto::xproto::Timestamp,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetProviderInfoReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let provider_bytes = provider.serialize_fixed();
        let config_timestamp_bytes = config_timestamp.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                33,
                length[0],
                length[1],
                provider_bytes[0],
                provider_bytes[1],
                provider_bytes[2],
                provider_bytes[3],
                config_timestamp_bytes[0],
                config_timestamp_bytes[1],
                config_timestamp_bytes[2],
                config_timestamp_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn set_provider_offload_sink(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        sink_provider: crate::proto::randr::Provider,
        config_timestamp: crate::proto::xproto::Timestamp,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let provider_bytes = provider.serialize_fixed();
        let sink_provider_bytes = sink_provider.serialize_fixed();
        let config_timestamp_bytes = config_timestamp.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                34,
                length[0],
                length[1],
                provider_bytes[0],
                provider_bytes[1],
                provider_bytes[2],
                provider_bytes[3],
                sink_provider_bytes[0],
                sink_provider_bytes[1],
                sink_provider_bytes[2],
                sink_provider_bytes[3],
                config_timestamp_bytes[0],
                config_timestamp_bytes[1],
                config_timestamp_bytes[2],
                config_timestamp_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn set_provider_output_source(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        source_provider: crate::proto::randr::Provider,
        config_timestamp: crate::proto::xproto::Timestamp,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let provider_bytes = provider.serialize_fixed();
        let source_provider_bytes = source_provider.serialize_fixed();
        let config_timestamp_bytes = config_timestamp.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                35,
                length[0],
                length[1],
                provider_bytes[0],
                provider_bytes[1],
                provider_bytes[2],
                provider_bytes[3],
                source_provider_bytes[0],
                source_provider_bytes[1],
                source_provider_bytes[2],
                source_provider_bytes[3],
                config_timestamp_bytes[0],
                config_timestamp_bytes[1],
                config_timestamp_bytes[2],
                config_timestamp_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn list_provider_properties(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::ListProviderPropertiesReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let provider_bytes = provider.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                36,
                length[0],
                length[1],
                provider_bytes[0],
                provider_bytes[1],
                provider_bytes[2],
                provider_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn query_provider_property(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        property: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::QueryProviderPropertyReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let provider_bytes = provider.serialize_fixed();
        let property_bytes = property.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                37,
                length[0],
                length[1],
                provider_bytes[0],
                provider_bytes[1],
                provider_bytes[2],
                provider_bytes[3],
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
        Ok(Cookie::new(seq))
    }

    fn configure_provider_property(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        property: u32,
        pending: u8,
        range: u8,
        values: &[i32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&provider.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&property.serialize_fixed());
        buf_ptr
            .get_mut(12..13)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&pending.serialize_fixed());
        buf_ptr
            .get_mut(13..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&range.serialize_fixed());
        let list_len = values.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            values,
        )?;
        let mut offset = list_len + 16;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 38]);
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

    fn change_provider_property(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        property: u32,
        r#type: u32,
        format: u8,
        mode: u8,
        num_items: u32,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&provider.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&property.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&r#type.serialize_fixed());
        buf_ptr
            .get_mut(16..17)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&format.serialize_fixed());
        buf_ptr
            .get_mut(17..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&mode.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&num_items.serialize_fixed());
        let list_len = data.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            data,
        )?;
        let mut offset = list_len + 24;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 39]);
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

    fn delete_provider_property(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        property: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let provider_bytes = provider.serialize_fixed();
        let property_bytes = property.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                40,
                length[0],
                length[1],
                provider_bytes[0],
                provider_bytes[1],
                provider_bytes[2],
                provider_bytes[3],
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

    fn get_provider_property(
        &mut self,
        socket_buffer: &mut [u8],
        provider: crate::proto::randr::Provider,
        property: u32,
        r#type: u32,
        long_offset: u32,
        long_length: u32,
        delete: u8,
        pending: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetProviderPropertyReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (7u16).to_ne_bytes();
        let provider_bytes = provider.serialize_fixed();
        let property_bytes = property.serialize_fixed();
        let r#type_bytes = r#type.serialize_fixed();
        let long_offset_bytes = long_offset.serialize_fixed();
        let long_length_bytes = long_length.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                41,
                length[0],
                length[1],
                provider_bytes[0],
                provider_bytes[1],
                provider_bytes[2],
                provider_bytes[3],
                property_bytes[0],
                property_bytes[1],
                property_bytes[2],
                property_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                long_offset_bytes[0],
                long_offset_bytes[1],
                long_offset_bytes[2],
                long_offset_bytes[3],
                long_length_bytes[0],
                long_length_bytes[1],
                long_length_bytes[2],
                long_length_bytes[3],
                delete,
                pending,
                0,
                0,
            ]);
        self.advance_writer(28);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_monitors(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        get_active: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::randr::GetMonitorsReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
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
                get_active,
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
        Ok(Cookie::new(seq))
    }

    fn set_monitor(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        monitorinfo: crate::proto::randr::MonitorInfo,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&window.serialize_fixed());
        let mut offset = monitorinfo
            .serialize_into(buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?)?;
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

    fn delete_monitor(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        name: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let name_bytes = name.serialize_fixed();
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
                name_bytes[0],
                name_bytes[1],
                name_bytes[2],
                name_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn create_lease(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        lid: crate::proto::randr::Lease,
        num_crtcs: u16,
        num_outputs: u16,
        crtcs: &[crate::proto::randr::Crtc],
        outputs: &[crate::proto::randr::Output],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::randr::CreateLeaseReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_crtcs = u16::try_from(num_crtcs).map_err(|_| crate::error::Error::Serialize)?;
        let num_outputs = u16::try_from(num_outputs).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&window.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&lid.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_crtcs).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(14..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(num_outputs).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = crtcs.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            crtcs,
        )?;
        let mut offset = list_len + 16;
        let list_len = outputs.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            outputs,
        )?;
        offset += list_len;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 45]);
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

    fn free_lease(
        &mut self,
        socket_buffer: &mut [u8],
        lid: crate::proto::randr::Lease,
        terminate: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::randr::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::randr::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let lid_bytes = lid.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                46,
                length[0],
                length[1],
                lid_bytes[0],
                lid_bytes[1],
                lid_bytes[2],
                lid_bytes[3],
                terminate,
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
}
