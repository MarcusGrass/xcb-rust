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
pub trait XvmcConnection {
    fn query_version(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xvmc::QueryVersionReply, 16>>;

    fn list_surface_types(
        &mut self,
        socket_buffer: &mut [u8],
        port_id: crate::proto::xv::Port,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xvmc::ListSurfaceTypesReply>>;

    fn create_context(
        &mut self,
        socket_buffer: &mut [u8],
        context_id: crate::proto::xvmc::Context,
        port_id: crate::proto::xv::Port,
        surface_id: crate::proto::xvmc::Surface,
        width: u16,
        height: u16,
        flags: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xvmc::CreateContextReply>>;

    fn destroy_context(
        &mut self,
        socket_buffer: &mut [u8],
        context_id: crate::proto::xvmc::Context,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn create_surface(
        &mut self,
        socket_buffer: &mut [u8],
        surface_id: crate::proto::xvmc::Surface,
        context_id: crate::proto::xvmc::Context,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xvmc::CreateSurfaceReply>>;

    fn destroy_surface(
        &mut self,
        socket_buffer: &mut [u8],
        surface_id: crate::proto::xvmc::Surface,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn create_subpicture(
        &mut self,
        socket_buffer: &mut [u8],
        subpicture_id: crate::proto::xvmc::Subpicture,
        context: crate::proto::xvmc::Context,
        xvimage_id: u32,
        width: u16,
        height: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xvmc::CreateSubpictureReply>>;

    fn destroy_subpicture(
        &mut self,
        socket_buffer: &mut [u8],
        subpicture_id: crate::proto::xvmc::Subpicture,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn list_subpicture_types(
        &mut self,
        socket_buffer: &mut [u8],
        port_id: crate::proto::xv::Port,
        surface_id: crate::proto::xvmc::Surface,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xvmc::ListSubpictureTypesReply>>;
}
impl<C> XvmcConnection for C
where
    C: crate::con::XcbConnection,
{
    fn query_version(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xvmc::QueryVersionReply, 16>> {
        let major_opcode = self
            .major_opcode(crate::proto::xvmc::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xvmc::EXTENSION_NAME,
            ))?;
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

    fn list_surface_types(
        &mut self,
        socket_buffer: &mut [u8],
        port_id: crate::proto::xv::Port,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xvmc::ListSurfaceTypesReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xvmc::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xvmc::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let port_id_bytes = port_id.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                1,
                length[0],
                length[1],
                port_id_bytes[0],
                port_id_bytes[1],
                port_id_bytes[2],
                port_id_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn create_context(
        &mut self,
        socket_buffer: &mut [u8],
        context_id: crate::proto::xvmc::Context,
        port_id: crate::proto::xv::Port,
        surface_id: crate::proto::xvmc::Surface,
        width: u16,
        height: u16,
        flags: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xvmc::CreateContextReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xvmc::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xvmc::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (6u16).to_ne_bytes();
        let context_id_bytes = context_id.serialize_fixed();
        let port_id_bytes = port_id.serialize_fixed();
        let surface_id_bytes = surface_id.serialize_fixed();
        let width_bytes = width.serialize_fixed();
        let height_bytes = height.serialize_fixed();
        let flags_bytes = flags.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                2,
                length[0],
                length[1],
                context_id_bytes[0],
                context_id_bytes[1],
                context_id_bytes[2],
                context_id_bytes[3],
                port_id_bytes[0],
                port_id_bytes[1],
                port_id_bytes[2],
                port_id_bytes[3],
                surface_id_bytes[0],
                surface_id_bytes[1],
                surface_id_bytes[2],
                surface_id_bytes[3],
                width_bytes[0],
                width_bytes[1],
                height_bytes[0],
                height_bytes[1],
                flags_bytes[0],
                flags_bytes[1],
                flags_bytes[2],
                flags_bytes[3],
            ]);
        self.advance_writer(24);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn destroy_context(
        &mut self,
        socket_buffer: &mut [u8],
        context_id: crate::proto::xvmc::Context,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xvmc::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xvmc::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_id_bytes = context_id.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
                length[0],
                length[1],
                context_id_bytes[0],
                context_id_bytes[1],
                context_id_bytes[2],
                context_id_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn create_surface(
        &mut self,
        socket_buffer: &mut [u8],
        surface_id: crate::proto::xvmc::Surface,
        context_id: crate::proto::xvmc::Context,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xvmc::CreateSurfaceReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xvmc::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xvmc::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let surface_id_bytes = surface_id.serialize_fixed();
        let context_id_bytes = context_id.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                4,
                length[0],
                length[1],
                surface_id_bytes[0],
                surface_id_bytes[1],
                surface_id_bytes[2],
                surface_id_bytes[3],
                context_id_bytes[0],
                context_id_bytes[1],
                context_id_bytes[2],
                context_id_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn destroy_surface(
        &mut self,
        socket_buffer: &mut [u8],
        surface_id: crate::proto::xvmc::Surface,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xvmc::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xvmc::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let surface_id_bytes = surface_id.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                5,
                length[0],
                length[1],
                surface_id_bytes[0],
                surface_id_bytes[1],
                surface_id_bytes[2],
                surface_id_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn create_subpicture(
        &mut self,
        socket_buffer: &mut [u8],
        subpicture_id: crate::proto::xvmc::Subpicture,
        context: crate::proto::xvmc::Context,
        xvimage_id: u32,
        width: u16,
        height: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xvmc::CreateSubpictureReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xvmc::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xvmc::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (5u16).to_ne_bytes();
        let subpicture_id_bytes = subpicture_id.serialize_fixed();
        let context_bytes = context.serialize_fixed();
        let xvimage_id_bytes = xvimage_id.serialize_fixed();
        let width_bytes = width.serialize_fixed();
        let height_bytes = height.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                6,
                length[0],
                length[1],
                subpicture_id_bytes[0],
                subpicture_id_bytes[1],
                subpicture_id_bytes[2],
                subpicture_id_bytes[3],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
                xvimage_id_bytes[0],
                xvimage_id_bytes[1],
                xvimage_id_bytes[2],
                xvimage_id_bytes[3],
                width_bytes[0],
                width_bytes[1],
                height_bytes[0],
                height_bytes[1],
            ]);
        self.advance_writer(20);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn destroy_subpicture(
        &mut self,
        socket_buffer: &mut [u8],
        subpicture_id: crate::proto::xvmc::Subpicture,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::xvmc::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xvmc::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let subpicture_id_bytes = subpicture_id.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                7,
                length[0],
                length[1],
                subpicture_id_bytes[0],
                subpicture_id_bytes[1],
                subpicture_id_bytes[2],
                subpicture_id_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn list_subpicture_types(
        &mut self,
        socket_buffer: &mut [u8],
        port_id: crate::proto::xv::Port,
        surface_id: crate::proto::xvmc::Surface,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xvmc::ListSubpictureTypesReply>> {
        let major_opcode = self
            .major_opcode(crate::proto::xvmc::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::xvmc::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let port_id_bytes = port_id.serialize_fixed();
        let surface_id_bytes = surface_id.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                8,
                length[0],
                length[1],
                port_id_bytes[0],
                port_id_bytes[1],
                port_id_bytes[2],
                port_id_bytes[3],
                surface_id_bytes[0],
                surface_id_bytes[1],
                surface_id_bytes[2],
                surface_id_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }
}
