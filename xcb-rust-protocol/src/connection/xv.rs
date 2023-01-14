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
pub trait XvConnection {
    fn query_extension(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xv::QueryExtensionReply, 12>>;

    fn query_adaptors(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xv::QueryAdaptorsReply>>;

    fn query_encodings(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xv::QueryEncodingsReply>>;

    fn grab_port(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xv::GrabPortReply, 8>>;

    fn ungrab_port(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn put_video(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn put_still(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_video(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_still(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn stop_video(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        drawable: crate::proto::xproto::Drawable,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn select_video_notify(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        onoff: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn select_port_notify(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        onoff: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn query_best_size(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        vid_w: u16,
        vid_h: u16,
        drw_w: u16,
        drw_h: u16,
        motion: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xv::QueryBestSizeReply, 12>>;

    fn set_port_attribute(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        attribute: u32,
        value: i32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_port_attribute(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        attribute: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xv::GetPortAttributeReply, 12>>;

    fn query_port_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xv::QueryPortAttributesReply>>;

    fn list_image_formats(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xv::ListImageFormatsReply>>;

    fn query_image_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        id: u32,
        width: u16,
        height: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xv::QueryImageAttributesReply>>;

    fn put_image(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        id: u32,
        src_x: i16,
        src_y: i16,
        src_w: u16,
        src_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        width: u16,
        height: u16,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn shm_put_image(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        shmseg: crate::proto::shm::Seg,
        id: u32,
        offset: u32,
        src_x: i16,
        src_y: i16,
        src_w: u16,
        src_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        width: u16,
        height: u16,
        send_event: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;
}
impl<C> XvConnection for C
where
    C: crate::con::XcbConnection,
{
    fn query_extension(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xv::QueryExtensionReply, 12>> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = major_opcode;
        buf[1] = 0;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn query_adaptors(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xv::QueryAdaptorsReply>> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                1,
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

    fn query_encodings(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xv::QueryEncodingsReply>> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let port_bytes = port.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                2,
                length[0],
                length[1],
                port_bytes[0],
                port_bytes[1],
                port_bytes[2],
                port_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn grab_port(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xv::GrabPortReply, 8>> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let port_bytes = port.serialize_fixed();
        let time_bytes = (time.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
                length[0],
                length[1],
                port_bytes[0],
                port_bytes[1],
                port_bytes[2],
                port_bytes[3],
                time_bytes[0],
                time_bytes[1],
                time_bytes[2],
                time_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn ungrab_port(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let port_bytes = port.serialize_fixed();
        let time_bytes = (time.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                4,
                length[0],
                length[1],
                port_bytes[0],
                port_bytes[1],
                port_bytes[2],
                port_bytes[3],
                time_bytes[0],
                time_bytes[1],
                time_bytes[2],
                time_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn put_video(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (8u16).to_ne_bytes();
        let port_bytes = port.serialize_fixed();
        let drawable_bytes = drawable.serialize_fixed();
        let gc_bytes = gc.serialize_fixed();
        let vid_x_bytes = vid_x.serialize_fixed();
        let vid_y_bytes = vid_y.serialize_fixed();
        let vid_w_bytes = vid_w.serialize_fixed();
        let vid_h_bytes = vid_h.serialize_fixed();
        let drw_x_bytes = drw_x.serialize_fixed();
        let drw_y_bytes = drw_y.serialize_fixed();
        let drw_w_bytes = drw_w.serialize_fixed();
        let drw_h_bytes = drw_h.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                5,
                length[0],
                length[1],
                port_bytes[0],
                port_bytes[1],
                port_bytes[2],
                port_bytes[3],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                gc_bytes[0],
                gc_bytes[1],
                gc_bytes[2],
                gc_bytes[3],
                vid_x_bytes[0],
                vid_x_bytes[1],
                vid_y_bytes[0],
                vid_y_bytes[1],
                vid_w_bytes[0],
                vid_w_bytes[1],
                vid_h_bytes[0],
                vid_h_bytes[1],
                drw_x_bytes[0],
                drw_x_bytes[1],
                drw_y_bytes[0],
                drw_y_bytes[1],
                drw_w_bytes[0],
                drw_w_bytes[1],
                drw_h_bytes[0],
                drw_h_bytes[1],
            ]);
        self.advance_writer(32);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn put_still(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (8u16).to_ne_bytes();
        let port_bytes = port.serialize_fixed();
        let drawable_bytes = drawable.serialize_fixed();
        let gc_bytes = gc.serialize_fixed();
        let vid_x_bytes = vid_x.serialize_fixed();
        let vid_y_bytes = vid_y.serialize_fixed();
        let vid_w_bytes = vid_w.serialize_fixed();
        let vid_h_bytes = vid_h.serialize_fixed();
        let drw_x_bytes = drw_x.serialize_fixed();
        let drw_y_bytes = drw_y.serialize_fixed();
        let drw_w_bytes = drw_w.serialize_fixed();
        let drw_h_bytes = drw_h.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                6,
                length[0],
                length[1],
                port_bytes[0],
                port_bytes[1],
                port_bytes[2],
                port_bytes[3],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                gc_bytes[0],
                gc_bytes[1],
                gc_bytes[2],
                gc_bytes[3],
                vid_x_bytes[0],
                vid_x_bytes[1],
                vid_y_bytes[0],
                vid_y_bytes[1],
                vid_w_bytes[0],
                vid_w_bytes[1],
                vid_h_bytes[0],
                vid_h_bytes[1],
                drw_x_bytes[0],
                drw_x_bytes[1],
                drw_y_bytes[0],
                drw_y_bytes[1],
                drw_w_bytes[0],
                drw_w_bytes[1],
                drw_h_bytes[0],
                drw_h_bytes[1],
            ]);
        self.advance_writer(32);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_video(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (8u16).to_ne_bytes();
        let port_bytes = port.serialize_fixed();
        let drawable_bytes = drawable.serialize_fixed();
        let gc_bytes = gc.serialize_fixed();
        let vid_x_bytes = vid_x.serialize_fixed();
        let vid_y_bytes = vid_y.serialize_fixed();
        let vid_w_bytes = vid_w.serialize_fixed();
        let vid_h_bytes = vid_h.serialize_fixed();
        let drw_x_bytes = drw_x.serialize_fixed();
        let drw_y_bytes = drw_y.serialize_fixed();
        let drw_w_bytes = drw_w.serialize_fixed();
        let drw_h_bytes = drw_h.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                7,
                length[0],
                length[1],
                port_bytes[0],
                port_bytes[1],
                port_bytes[2],
                port_bytes[3],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                gc_bytes[0],
                gc_bytes[1],
                gc_bytes[2],
                gc_bytes[3],
                vid_x_bytes[0],
                vid_x_bytes[1],
                vid_y_bytes[0],
                vid_y_bytes[1],
                vid_w_bytes[0],
                vid_w_bytes[1],
                vid_h_bytes[0],
                vid_h_bytes[1],
                drw_x_bytes[0],
                drw_x_bytes[1],
                drw_y_bytes[0],
                drw_y_bytes[1],
                drw_w_bytes[0],
                drw_w_bytes[1],
                drw_h_bytes[0],
                drw_h_bytes[1],
            ]);
        self.advance_writer(32);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_still(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (8u16).to_ne_bytes();
        let port_bytes = port.serialize_fixed();
        let drawable_bytes = drawable.serialize_fixed();
        let gc_bytes = gc.serialize_fixed();
        let vid_x_bytes = vid_x.serialize_fixed();
        let vid_y_bytes = vid_y.serialize_fixed();
        let vid_w_bytes = vid_w.serialize_fixed();
        let vid_h_bytes = vid_h.serialize_fixed();
        let drw_x_bytes = drw_x.serialize_fixed();
        let drw_y_bytes = drw_y.serialize_fixed();
        let drw_w_bytes = drw_w.serialize_fixed();
        let drw_h_bytes = drw_h.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                8,
                length[0],
                length[1],
                port_bytes[0],
                port_bytes[1],
                port_bytes[2],
                port_bytes[3],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                gc_bytes[0],
                gc_bytes[1],
                gc_bytes[2],
                gc_bytes[3],
                vid_x_bytes[0],
                vid_x_bytes[1],
                vid_y_bytes[0],
                vid_y_bytes[1],
                vid_w_bytes[0],
                vid_w_bytes[1],
                vid_h_bytes[0],
                vid_h_bytes[1],
                drw_x_bytes[0],
                drw_x_bytes[1],
                drw_y_bytes[0],
                drw_y_bytes[1],
                drw_w_bytes[0],
                drw_w_bytes[1],
                drw_h_bytes[0],
                drw_h_bytes[1],
            ]);
        self.advance_writer(32);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn stop_video(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        drawable: crate::proto::xproto::Drawable,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let port_bytes = port.serialize_fixed();
        let drawable_bytes = drawable.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                9,
                length[0],
                length[1],
                port_bytes[0],
                port_bytes[1],
                port_bytes[2],
                port_bytes[3],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn select_video_notify(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        onoff: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                10,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                onoff,
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

    fn select_port_notify(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        onoff: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let port_bytes = port.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                11,
                length[0],
                length[1],
                port_bytes[0],
                port_bytes[1],
                port_bytes[2],
                port_bytes[3],
                onoff,
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

    fn query_best_size(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        vid_w: u16,
        vid_h: u16,
        drw_w: u16,
        drw_h: u16,
        motion: u8,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xv::QueryBestSizeReply, 12>> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (5u16).to_ne_bytes();
        let port_bytes = port.serialize_fixed();
        let vid_w_bytes = vid_w.serialize_fixed();
        let vid_h_bytes = vid_h.serialize_fixed();
        let drw_w_bytes = drw_w.serialize_fixed();
        let drw_h_bytes = drw_h.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                12,
                length[0],
                length[1],
                port_bytes[0],
                port_bytes[1],
                port_bytes[2],
                port_bytes[3],
                vid_w_bytes[0],
                vid_w_bytes[1],
                vid_h_bytes[0],
                vid_h_bytes[1],
                drw_w_bytes[0],
                drw_w_bytes[1],
                drw_h_bytes[0],
                drw_h_bytes[1],
                motion,
                0,
                0,
                0,
            ]);
        self.advance_writer(20);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn set_port_attribute(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        attribute: u32,
        value: i32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let port_bytes = port.serialize_fixed();
        let attribute_bytes = attribute.serialize_fixed();
        let value_bytes = value.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                13,
                length[0],
                length[1],
                port_bytes[0],
                port_bytes[1],
                port_bytes[2],
                port_bytes[3],
                attribute_bytes[0],
                attribute_bytes[1],
                attribute_bytes[2],
                attribute_bytes[3],
                value_bytes[0],
                value_bytes[1],
                value_bytes[2],
                value_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_port_attribute(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        attribute: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xv::GetPortAttributeReply, 12>> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let port_bytes = port.serialize_fixed();
        let attribute_bytes = attribute.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                14,
                length[0],
                length[1],
                port_bytes[0],
                port_bytes[1],
                port_bytes[2],
                port_bytes[3],
                attribute_bytes[0],
                attribute_bytes[1],
                attribute_bytes[2],
                attribute_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn query_port_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xv::QueryPortAttributesReply>> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let port_bytes = port.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                15,
                length[0],
                length[1],
                port_bytes[0],
                port_bytes[1],
                port_bytes[2],
                port_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn list_image_formats(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xv::ListImageFormatsReply>> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let port_bytes = port.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                16,
                length[0],
                length[1],
                port_bytes[0],
                port_bytes[1],
                port_bytes[2],
                port_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn query_image_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        id: u32,
        width: u16,
        height: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xv::QueryImageAttributesReply>> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let port_bytes = port.serialize_fixed();
        let id_bytes = id.serialize_fixed();
        let width_bytes = width.serialize_fixed();
        let height_bytes = height.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                17,
                length[0],
                length[1],
                port_bytes[0],
                port_bytes[1],
                port_bytes[2],
                port_bytes[3],
                id_bytes[0],
                id_bytes[1],
                id_bytes[2],
                id_bytes[3],
                width_bytes[0],
                width_bytes[1],
                height_bytes[0],
                height_bytes[1],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn put_image(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        id: u32,
        src_x: i16,
        src_y: i16,
        src_w: u16,
        src_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        width: u16,
        height: u16,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let buf_ptr = self.apply_offset(socket_buffer);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&port.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&gc.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&id.serialize_fixed());
        buf_ptr
            .get_mut(20..22)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_x.serialize_fixed());
        buf_ptr
            .get_mut(22..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_y.serialize_fixed());
        buf_ptr
            .get_mut(24..26)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_w.serialize_fixed());
        buf_ptr
            .get_mut(26..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&src_h.serialize_fixed());
        buf_ptr
            .get_mut(28..30)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drw_x.serialize_fixed());
        buf_ptr
            .get_mut(30..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drw_y.serialize_fixed());
        buf_ptr
            .get_mut(32..34)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drw_w.serialize_fixed());
        buf_ptr
            .get_mut(34..36)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drw_h.serialize_fixed());
        buf_ptr
            .get_mut(36..38)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&width.serialize_fixed());
        buf_ptr
            .get_mut(38..40)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&height.serialize_fixed());
        let list_len = data.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(40..)
                .ok_or(crate::error::Error::Serialize)?,
            data,
        )?;
        let mut offset = list_len + 40;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 18]);
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

    fn shm_put_image(
        &mut self,
        socket_buffer: &mut [u8],
        port: crate::proto::xv::Port,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        shmseg: crate::proto::shm::Seg,
        id: u32,
        offset: u32,
        src_x: i16,
        src_y: i16,
        src_w: u16,
        src_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        width: u16,
        height: u16,
        send_event: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::xv::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::xv::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (13u16).to_ne_bytes();
        let port_bytes = port.serialize_fixed();
        let drawable_bytes = drawable.serialize_fixed();
        let gc_bytes = gc.serialize_fixed();
        let shmseg_bytes = shmseg.serialize_fixed();
        let id_bytes = id.serialize_fixed();
        let offset_bytes = offset.serialize_fixed();
        let src_x_bytes = src_x.serialize_fixed();
        let src_y_bytes = src_y.serialize_fixed();
        let src_w_bytes = src_w.serialize_fixed();
        let src_h_bytes = src_h.serialize_fixed();
        let drw_x_bytes = drw_x.serialize_fixed();
        let drw_y_bytes = drw_y.serialize_fixed();
        let drw_w_bytes = drw_w.serialize_fixed();
        let drw_h_bytes = drw_h.serialize_fixed();
        let width_bytes = width.serialize_fixed();
        let height_bytes = height.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..52)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                19,
                length[0],
                length[1],
                port_bytes[0],
                port_bytes[1],
                port_bytes[2],
                port_bytes[3],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                gc_bytes[0],
                gc_bytes[1],
                gc_bytes[2],
                gc_bytes[3],
                shmseg_bytes[0],
                shmseg_bytes[1],
                shmseg_bytes[2],
                shmseg_bytes[3],
                id_bytes[0],
                id_bytes[1],
                id_bytes[2],
                id_bytes[3],
                offset_bytes[0],
                offset_bytes[1],
                offset_bytes[2],
                offset_bytes[3],
                src_x_bytes[0],
                src_x_bytes[1],
                src_y_bytes[0],
                src_y_bytes[1],
                src_w_bytes[0],
                src_w_bytes[1],
                src_h_bytes[0],
                src_h_bytes[1],
                drw_x_bytes[0],
                drw_x_bytes[1],
                drw_y_bytes[0],
                drw_y_bytes[1],
                drw_w_bytes[0],
                drw_w_bytes[1],
                drw_h_bytes[0],
                drw_h_bytes[1],
                width_bytes[0],
                width_bytes[1],
                height_bytes[0],
                height_bytes[1],
                send_event,
                0,
                0,
                0,
            ]);
        self.advance_writer(52);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }
}
