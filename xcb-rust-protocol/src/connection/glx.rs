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
pub trait GlxConnection {
    fn render(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn render_large(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        request_num: u16,
        request_total: u16,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn create_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::glx::Context,
        visual: crate::proto::xproto::Visualid,
        screen: u32,
        share_list: crate::proto::glx::Context,
        is_direct: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn destroy_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::glx::Context,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn make_current(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::glx::Drawable,
        context: crate::proto::glx::Context,
        old_context_tag: crate::proto::glx::ContextTag,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::MakeCurrentReply, 32>>;

    fn is_direct(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::glx::Context,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::IsDirectReply, 32>>;

    fn query_version(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u32,
        minor_version: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::QueryVersionReply, 32>>;

    fn wait_g_l(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn wait_x(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn copy_context(
        &mut self,
        socket_buffer: &mut [u8],
        src: crate::proto::glx::Context,
        dest: crate::proto::glx::Context,
        mask: u32,
        src_context_tag: crate::proto::glx::ContextTag,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn swap_buffers(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        drawable: crate::proto::glx::Drawable,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn use_x_font(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        font: crate::proto::xproto::Font,
        first: u32,
        count: u32,
        list_base: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn create_g_l_x_pixmap(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        visual: crate::proto::xproto::Visualid,
        pixmap: crate::proto::xproto::Pixmap,
        glx_pixmap: crate::proto::glx::Pixmap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_visual_configs(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetVisualConfigsReply>>;

    fn destroy_g_l_x_pixmap(
        &mut self,
        socket_buffer: &mut [u8],
        glx_pixmap: crate::proto::glx::Pixmap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn vendor_private(
        &mut self,
        socket_buffer: &mut [u8],
        vendor_code: u32,
        context_tag: crate::proto::glx::ContextTag,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn vendor_private_with_reply(
        &mut self,
        socket_buffer: &mut [u8],
        vendor_code: u32,
        context_tag: crate::proto::glx::ContextTag,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::VendorPrivateWithReplyReply>>;

    fn query_extensions_string(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::QueryExtensionsStringReply, 32>>;

    fn query_server_string(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        name: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::QueryServerStringReply>>;

    fn client_info(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u32,
        minor_version: u32,
        string: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_f_b_configs(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetFBConfigsReply>>;

    fn create_pixmap(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        fbconfig: crate::proto::glx::Fbconfig,
        pixmap: crate::proto::xproto::Pixmap,
        glx_pixmap: crate::proto::glx::Pixmap,
        num_attribs: u32,
        attribs: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn destroy_pixmap(
        &mut self,
        socket_buffer: &mut [u8],
        glx_pixmap: crate::proto::glx::Pixmap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn create_new_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::glx::Context,
        fbconfig: crate::proto::glx::Fbconfig,
        screen: u32,
        render_type: u32,
        share_list: crate::proto::glx::Context,
        is_direct: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn query_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::glx::Context,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::QueryContextReply>>;

    fn make_context_current(
        &mut self,
        socket_buffer: &mut [u8],
        old_context_tag: crate::proto::glx::ContextTag,
        drawable: crate::proto::glx::Drawable,
        read_drawable: crate::proto::glx::Drawable,
        context: crate::proto::glx::Context,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::MakeContextCurrentReply, 32>>;

    fn create_pbuffer(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        fbconfig: crate::proto::glx::Fbconfig,
        pbuffer: crate::proto::glx::Pbuffer,
        num_attribs: u32,
        attribs: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn destroy_pbuffer(
        &mut self,
        socket_buffer: &mut [u8],
        pbuffer: crate::proto::glx::Pbuffer,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_drawable_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::glx::Drawable,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetDrawableAttributesReply>>;

    fn change_drawable_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::glx::Drawable,
        num_attribs: u32,
        attribs: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn create_window(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        fbconfig: crate::proto::glx::Fbconfig,
        window: crate::proto::xproto::Window,
        glx_window: crate::proto::glx::Window,
        num_attribs: u32,
        attribs: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn delete_window(
        &mut self,
        socket_buffer: &mut [u8],
        glxwindow: crate::proto::glx::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn set_client_info_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u32,
        minor_version: u32,
        num_versions: u32,
        gl_str_len: u32,
        glx_str_len: u32,
        gl_versions: &[u32],
        gl_extension_string: &[u8],
        glx_extension_string: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn create_context_attribs_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::glx::Context,
        fbconfig: crate::proto::glx::Fbconfig,
        screen: u32,
        share_list: crate::proto::glx::Context,
        is_direct: u8,
        num_attribs: u32,
        attribs: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn set_client_info2_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u32,
        minor_version: u32,
        num_versions: u32,
        gl_str_len: u32,
        glx_str_len: u32,
        gl_versions: &[u32],
        gl_extension_string: &[u8],
        glx_extension_string: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn new_list(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        list: u32,
        mode: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn end_list(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn delete_lists(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        list: u32,
        range: i32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn gen_lists(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        range: i32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::GenListsReply, 12>>;

    fn feedback_buffer(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        size: i32,
        r#type: i32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn select_buffer(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        size: i32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn render_mode(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        mode: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::RenderModeReply>>;

    fn finish(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::FinishReply, 8>>;

    fn pixel_storef(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        pname: u32,
        datum: crate::proto::glx::Float32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn pixel_storei(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        pname: u32,
        datum: i32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn read_pixels(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        r#type: u32,
        swap_bytes: u8,
        lsb_first: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::ReadPixelsReply>>;

    fn get_booleanv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        pname: i32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetBooleanvReply>>;

    fn get_clip_plane(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        plane: i32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetClipPlaneReply>>;

    fn get_doublev(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetDoublevReply>>;

    fn get_error(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::GetErrorReply, 12>>;

    fn get_floatv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetFloatvReply>>;

    fn get_integerv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetIntegervReply>>;

    fn get_lightfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        light: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetLightfvReply>>;

    fn get_lightiv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        light: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetLightivReply>>;

    fn get_mapdv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        query: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetMapdvReply>>;

    fn get_mapfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        query: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetMapfvReply>>;

    fn get_mapiv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        query: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetMapivReply>>;

    fn get_materialfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        face: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetMaterialfvReply>>;

    fn get_materialiv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        face: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetMaterialivReply>>;

    fn get_pixel_mapfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        map: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetPixelMapfvReply>>;

    fn get_pixel_mapuiv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        map: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetPixelMapuivReply>>;

    fn get_pixel_mapusv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        map: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetPixelMapusvReply>>;

    fn get_polygon_stipple(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        lsb_first: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetPolygonStippleReply>>;

    fn get_string(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        name: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetStringReply>>;

    fn get_tex_envfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexEnvfvReply>>;

    fn get_tex_enviv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexEnvivReply>>;

    fn get_tex_gendv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        coord: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexGendvReply>>;

    fn get_tex_genfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        coord: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexGenfvReply>>;

    fn get_tex_geniv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        coord: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexGenivReply>>;

    fn get_tex_image(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        level: i32,
        format: u32,
        r#type: u32,
        swap_bytes: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexImageReply>>;

    fn get_tex_parameterfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexParameterfvReply>>;

    fn get_tex_parameteriv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexParameterivReply>>;

    fn get_tex_level_parameterfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        level: i32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexLevelParameterfvReply>>;

    fn get_tex_level_parameteriv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        level: i32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexLevelParameterivReply>>;

    fn is_enabled(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        capability: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::IsEnabledReply, 12>>;

    fn is_list(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        list: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::IsListReply, 12>>;

    fn flush(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn are_textures_resident(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        textures: &[u32],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::AreTexturesResidentReply>>;

    fn delete_textures(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        textures: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn gen_textures(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        n: i32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GenTexturesReply>>;

    fn is_texture(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        texture: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::IsTextureReply, 12>>;

    fn get_color_table(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        format: u32,
        r#type: u32,
        swap_bytes: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetColorTableReply>>;

    fn get_color_table_parameterfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetColorTableParameterfvReply>>;

    fn get_color_table_parameteriv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetColorTableParameterivReply>>;

    fn get_convolution_filter(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        format: u32,
        r#type: u32,
        swap_bytes: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetConvolutionFilterReply>>;

    fn get_convolution_parameterfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetConvolutionParameterfvReply>>;

    fn get_convolution_parameteriv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetConvolutionParameterivReply>>;

    fn get_separable_filter(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        format: u32,
        r#type: u32,
        swap_bytes: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetSeparableFilterReply>>;

    fn get_histogram(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        format: u32,
        r#type: u32,
        swap_bytes: u8,
        reset: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetHistogramReply>>;

    fn get_histogram_parameterfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetHistogramParameterfvReply>>;

    fn get_histogram_parameteriv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetHistogramParameterivReply>>;

    fn get_minmax(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        format: u32,
        r#type: u32,
        swap_bytes: u8,
        reset: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetMinmaxReply>>;

    fn get_minmax_parameterfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetMinmaxParameterfvReply>>;

    fn get_minmax_parameteriv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetMinmaxParameterivReply>>;

    fn get_compressed_tex_image_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        level: i32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetCompressedTexImageARBReply>>;

    fn delete_queries_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        ids: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn gen_queries_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        n: i32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GenQueriesARBReply>>;

    fn is_query_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        id: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::IsQueryARBReply, 12>>;

    fn get_queryiv_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetQueryivARBReply>>;

    fn get_query_objectiv_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        id: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetQueryObjectivARBReply>>;

    fn get_query_objectuiv_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        id: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetQueryObjectuivARBReply>>;
}
impl<C> GlxConnection for C
where
    C: crate::con::XcbConnection,
{
    fn render(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let buf_ptr = self.apply_offset(socket_buffer);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context_tag.serialize_fixed());
        let list_len = data.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            data,
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
        Ok(VoidCookie::new(seq))
    }

    fn render_large(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        request_num: u16,
        request_total: u16,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let data_len = u32::try_from(data.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context_tag.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&request_num.serialize_fixed());
        buf_ptr
            .get_mut(10..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&request_total.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(data_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = data.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            data,
        )?;
        let mut offset = list_len + 16;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 2]);
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

    fn create_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::glx::Context,
        visual: crate::proto::xproto::Visualid,
        screen: u32,
        share_list: crate::proto::glx::Context,
        is_direct: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (6u16).to_ne_bytes();
        let context_bytes = context.serialize_fixed();
        let visual_bytes = visual.serialize_fixed();
        let screen_bytes = screen.serialize_fixed();
        let share_list_bytes = share_list.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
                visual_bytes[0],
                visual_bytes[1],
                visual_bytes[2],
                visual_bytes[3],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
                share_list_bytes[0],
                share_list_bytes[1],
                share_list_bytes[2],
                share_list_bytes[3],
                is_direct,
                0,
                0,
                0,
            ]);
        self.advance_writer(24);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn destroy_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::glx::Context,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_bytes = context.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                4,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn make_current(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::glx::Drawable,
        context: crate::proto::glx::Context,
        old_context_tag: crate::proto::glx::ContextTag,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::MakeCurrentReply, 32>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let context_bytes = context.serialize_fixed();
        let old_context_tag_bytes = old_context_tag.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
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
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
                old_context_tag_bytes[0],
                old_context_tag_bytes[1],
                old_context_tag_bytes[2],
                old_context_tag_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn is_direct(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::glx::Context,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::IsDirectReply, 32>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_bytes = context.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                6,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn query_version(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u32,
        minor_version: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::QueryVersionReply, 32>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let major_version_bytes = major_version.serialize_fixed();
        let minor_version_bytes = minor_version.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                7,
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

    fn wait_g_l(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                8,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn wait_x(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                9,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn copy_context(
        &mut self,
        socket_buffer: &mut [u8],
        src: crate::proto::glx::Context,
        dest: crate::proto::glx::Context,
        mask: u32,
        src_context_tag: crate::proto::glx::ContextTag,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (5u16).to_ne_bytes();
        let src_bytes = src.serialize_fixed();
        let dest_bytes = dest.serialize_fixed();
        let mask_bytes = mask.serialize_fixed();
        let src_context_tag_bytes = src_context_tag.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                10,
                length[0],
                length[1],
                src_bytes[0],
                src_bytes[1],
                src_bytes[2],
                src_bytes[3],
                dest_bytes[0],
                dest_bytes[1],
                dest_bytes[2],
                dest_bytes[3],
                mask_bytes[0],
                mask_bytes[1],
                mask_bytes[2],
                mask_bytes[3],
                src_context_tag_bytes[0],
                src_context_tag_bytes[1],
                src_context_tag_bytes[2],
                src_context_tag_bytes[3],
            ]);
        self.advance_writer(20);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn swap_buffers(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        drawable: crate::proto::glx::Drawable,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let drawable_bytes = drawable.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                11,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
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

    fn use_x_font(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        font: crate::proto::xproto::Font,
        first: u32,
        count: u32,
        list_base: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (6u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let font_bytes = font.serialize_fixed();
        let first_bytes = first.serialize_fixed();
        let count_bytes = count.serialize_fixed();
        let list_base_bytes = list_base.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                12,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                font_bytes[0],
                font_bytes[1],
                font_bytes[2],
                font_bytes[3],
                first_bytes[0],
                first_bytes[1],
                first_bytes[2],
                first_bytes[3],
                count_bytes[0],
                count_bytes[1],
                count_bytes[2],
                count_bytes[3],
                list_base_bytes[0],
                list_base_bytes[1],
                list_base_bytes[2],
                list_base_bytes[3],
            ]);
        self.advance_writer(24);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn create_g_l_x_pixmap(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        visual: crate::proto::xproto::Visualid,
        pixmap: crate::proto::xproto::Pixmap,
        glx_pixmap: crate::proto::glx::Pixmap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (5u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let visual_bytes = visual.serialize_fixed();
        let pixmap_bytes = pixmap.serialize_fixed();
        let glx_pixmap_bytes = glx_pixmap.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                13,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
                visual_bytes[0],
                visual_bytes[1],
                visual_bytes[2],
                visual_bytes[3],
                pixmap_bytes[0],
                pixmap_bytes[1],
                pixmap_bytes[2],
                pixmap_bytes[3],
                glx_pixmap_bytes[0],
                glx_pixmap_bytes[1],
                glx_pixmap_bytes[2],
                glx_pixmap_bytes[3],
            ]);
        self.advance_writer(20);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_visual_configs(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetVisualConfigsReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                14,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn destroy_g_l_x_pixmap(
        &mut self,
        socket_buffer: &mut [u8],
        glx_pixmap: crate::proto::glx::Pixmap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let glx_pixmap_bytes = glx_pixmap.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                15,
                length[0],
                length[1],
                glx_pixmap_bytes[0],
                glx_pixmap_bytes[1],
                glx_pixmap_bytes[2],
                glx_pixmap_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn vendor_private(
        &mut self,
        socket_buffer: &mut [u8],
        vendor_code: u32,
        context_tag: crate::proto::glx::ContextTag,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let buf_ptr = self.apply_offset(socket_buffer);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vendor_code.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context_tag.serialize_fixed());
        let list_len = data.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            data,
        )?;
        let mut offset = list_len + 12;
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
        Ok(VoidCookie::new(seq))
    }

    fn vendor_private_with_reply(
        &mut self,
        socket_buffer: &mut [u8],
        vendor_code: u32,
        context_tag: crate::proto::glx::ContextTag,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::VendorPrivateWithReplyReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let buf_ptr = self.apply_offset(socket_buffer);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&vendor_code.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context_tag.serialize_fixed());
        let list_len = data.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            data,
        )?;
        let mut offset = list_len + 12;
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
        Ok(Cookie::new(seq))
    }

    fn query_extensions_string(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::QueryExtensionsStringReply, 32>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                18,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn query_server_string(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        name: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::QueryServerStringReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let name_bytes = name.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                19,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
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
        Ok(Cookie::new(seq))
    }

    fn client_info(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u32,
        minor_version: u32,
        string: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let str_len = u32::try_from(string.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&major_version.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&minor_version.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(str_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = string.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            string,
        )?;
        let mut offset = list_len + 16;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 20]);
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

    fn get_f_b_configs(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetFBConfigsReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let screen_bytes = screen.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                21,
                length[0],
                length[1],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn create_pixmap(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        fbconfig: crate::proto::glx::Fbconfig,
        pixmap: crate::proto::xproto::Pixmap,
        glx_pixmap: crate::proto::glx::Pixmap,
        num_attribs: u32,
        attribs: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_attribs = u32::try_from(num_attribs).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&screen.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&fbconfig.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&pixmap.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&glx_pixmap.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(core::ops::Div::div(num_attribs as u32, 2u32 as u32))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = attribs.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            attribs,
        )?;
        let mut offset = list_len + 24;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 22]);
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

    fn destroy_pixmap(
        &mut self,
        socket_buffer: &mut [u8],
        glx_pixmap: crate::proto::glx::Pixmap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let glx_pixmap_bytes = glx_pixmap.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                23,
                length[0],
                length[1],
                glx_pixmap_bytes[0],
                glx_pixmap_bytes[1],
                glx_pixmap_bytes[2],
                glx_pixmap_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn create_new_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::glx::Context,
        fbconfig: crate::proto::glx::Fbconfig,
        screen: u32,
        render_type: u32,
        share_list: crate::proto::glx::Context,
        is_direct: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (7u16).to_ne_bytes();
        let context_bytes = context.serialize_fixed();
        let fbconfig_bytes = fbconfig.serialize_fixed();
        let screen_bytes = screen.serialize_fixed();
        let render_type_bytes = render_type.serialize_fixed();
        let share_list_bytes = share_list.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                24,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
                fbconfig_bytes[0],
                fbconfig_bytes[1],
                fbconfig_bytes[2],
                fbconfig_bytes[3],
                screen_bytes[0],
                screen_bytes[1],
                screen_bytes[2],
                screen_bytes[3],
                render_type_bytes[0],
                render_type_bytes[1],
                render_type_bytes[2],
                render_type_bytes[3],
                share_list_bytes[0],
                share_list_bytes[1],
                share_list_bytes[2],
                share_list_bytes[3],
                is_direct,
                0,
                0,
                0,
            ]);
        self.advance_writer(28);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn query_context(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::glx::Context,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::QueryContextReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_bytes = context.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                25,
                length[0],
                length[1],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn make_context_current(
        &mut self,
        socket_buffer: &mut [u8],
        old_context_tag: crate::proto::glx::ContextTag,
        drawable: crate::proto::glx::Drawable,
        read_drawable: crate::proto::glx::Drawable,
        context: crate::proto::glx::Context,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::MakeContextCurrentReply, 32>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (5u16).to_ne_bytes();
        let old_context_tag_bytes = old_context_tag.serialize_fixed();
        let drawable_bytes = drawable.serialize_fixed();
        let read_drawable_bytes = read_drawable.serialize_fixed();
        let context_bytes = context.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                26,
                length[0],
                length[1],
                old_context_tag_bytes[0],
                old_context_tag_bytes[1],
                old_context_tag_bytes[2],
                old_context_tag_bytes[3],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                read_drawable_bytes[0],
                read_drawable_bytes[1],
                read_drawable_bytes[2],
                read_drawable_bytes[3],
                context_bytes[0],
                context_bytes[1],
                context_bytes[2],
                context_bytes[3],
            ]);
        self.advance_writer(20);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn create_pbuffer(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        fbconfig: crate::proto::glx::Fbconfig,
        pbuffer: crate::proto::glx::Pbuffer,
        num_attribs: u32,
        attribs: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_attribs = u32::try_from(num_attribs).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&screen.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&fbconfig.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&pbuffer.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(core::ops::Div::div(num_attribs as u32, 2u32 as u32))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = attribs.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(20..)
                .ok_or(crate::error::Error::Serialize)?,
            attribs,
        )?;
        let mut offset = list_len + 20;
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
        Ok(VoidCookie::new(seq))
    }

    fn destroy_pbuffer(
        &mut self,
        socket_buffer: &mut [u8],
        pbuffer: crate::proto::glx::Pbuffer,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let pbuffer_bytes = pbuffer.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                28,
                length[0],
                length[1],
                pbuffer_bytes[0],
                pbuffer_bytes[1],
                pbuffer_bytes[2],
                pbuffer_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_drawable_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::glx::Drawable,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetDrawableAttributesReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                29,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn change_drawable_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::glx::Drawable,
        num_attribs: u32,
        attribs: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_attribs = u32::try_from(num_attribs).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(core::ops::Div::div(num_attribs as u32, 2u32 as u32))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = attribs.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            attribs,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 30]);
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

    fn create_window(
        &mut self,
        socket_buffer: &mut [u8],
        screen: u32,
        fbconfig: crate::proto::glx::Fbconfig,
        window: crate::proto::xproto::Window,
        glx_window: crate::proto::glx::Window,
        num_attribs: u32,
        attribs: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_attribs = u32::try_from(num_attribs).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&screen.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&fbconfig.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&window.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&glx_window.serialize_fixed());
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(core::ops::Div::div(num_attribs as u32, 2u32 as u32))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = attribs.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            attribs,
        )?;
        let mut offset = list_len + 24;
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

    fn delete_window(
        &mut self,
        socket_buffer: &mut [u8],
        glxwindow: crate::proto::glx::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let glxwindow_bytes = glxwindow.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                32,
                length[0],
                length[1],
                glxwindow_bytes[0],
                glxwindow_bytes[1],
                glxwindow_bytes[2],
                glxwindow_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn set_client_info_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u32,
        minor_version: u32,
        num_versions: u32,
        gl_str_len: u32,
        glx_str_len: u32,
        gl_versions: &[u32],
        gl_extension_string: &[u8],
        glx_extension_string: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_versions =
            u32::try_from(num_versions).map_err(|_| crate::error::Error::Serialize)?;
        let gl_str_len = u32::try_from(gl_str_len).map_err(|_| crate::error::Error::Serialize)?;
        let glx_str_len = u32::try_from(glx_str_len).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&major_version.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&minor_version.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(core::ops::Div::div(num_versions as u32, 2u32 as u32))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(gl_str_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(glx_str_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = gl_versions.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            gl_versions,
        )?;
        let mut offset = list_len + 24;
        let list_len = gl_extension_string.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            gl_extension_string,
        )?;
        offset += list_len;
        offset += (4 - (offset % 4)) % 4;
        let list_len = glx_extension_string.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            glx_extension_string,
        )?;
        offset += list_len;
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
        Ok(VoidCookie::new(seq))
    }

    fn create_context_attribs_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        context: crate::proto::glx::Context,
        fbconfig: crate::proto::glx::Fbconfig,
        screen: u32,
        share_list: crate::proto::glx::Context,
        is_direct: u8,
        num_attribs: u32,
        attribs: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 3 bytes
        let num_attribs = u32::try_from(num_attribs).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&fbconfig.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&screen.serialize_fixed());
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&share_list.serialize_fixed());
        buf_ptr
            .get_mut(20..21)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&is_direct.serialize_fixed());
        buf_ptr
            .get_mut(24..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(core::ops::Div::div(num_attribs as u32, 2u32 as u32))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        let list_len = attribs.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(28..)
                .ok_or(crate::error::Error::Serialize)?,
            attribs,
        )?;
        let mut offset = list_len + 28;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 34]);
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

    fn set_client_info2_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        major_version: u32,
        minor_version: u32,
        num_versions: u32,
        gl_str_len: u32,
        glx_str_len: u32,
        gl_versions: &[u32],
        gl_extension_string: &[u8],
        glx_extension_string: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let num_versions =
            u32::try_from(num_versions).map_err(|_| crate::error::Error::Serialize)?;
        let gl_str_len = u32::try_from(gl_str_len).map_err(|_| crate::error::Error::Serialize)?;
        let glx_str_len = u32::try_from(glx_str_len).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&major_version.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&minor_version.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(core::ops::Div::div(num_versions as u32, 3u32 as u32))
                    .map_err(|_| crate::error::Error::Serialize)?)
                .serialize_fixed(),
            );
        buf_ptr
            .get_mut(16..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(gl_str_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(glx_str_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = gl_versions.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(24..)
                .ok_or(crate::error::Error::Serialize)?,
            gl_versions,
        )?;
        let mut offset = list_len + 24;
        let list_len = gl_extension_string.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            gl_extension_string,
        )?;
        offset += list_len;
        offset += (4 - (offset % 4)) % 4;
        let list_len = glx_extension_string.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(offset..)
                .ok_or(crate::error::Error::Serialize)?,
            glx_extension_string,
        )?;
        offset += list_len;
        let mut offset = offset + (4 - (offset % 4)) % 4;
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
        Ok(VoidCookie::new(seq))
    }

    fn new_list(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        list: u32,
        mode: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let list_bytes = list.serialize_fixed();
        let mode_bytes = mode.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                101,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                list_bytes[0],
                list_bytes[1],
                list_bytes[2],
                list_bytes[3],
                mode_bytes[0],
                mode_bytes[1],
                mode_bytes[2],
                mode_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn end_list(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                102,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn delete_lists(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        list: u32,
        range: i32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let list_bytes = list.serialize_fixed();
        let range_bytes = range.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                103,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                list_bytes[0],
                list_bytes[1],
                list_bytes[2],
                list_bytes[3],
                range_bytes[0],
                range_bytes[1],
                range_bytes[2],
                range_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn gen_lists(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        range: i32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::GenListsReply, 12>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let range_bytes = range.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                104,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                range_bytes[0],
                range_bytes[1],
                range_bytes[2],
                range_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn feedback_buffer(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        size: i32,
        r#type: i32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let size_bytes = size.serialize_fixed();
        let r#type_bytes = r#type.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                105,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                size_bytes[0],
                size_bytes[1],
                size_bytes[2],
                size_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn select_buffer(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        size: i32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let size_bytes = size.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                106,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                size_bytes[0],
                size_bytes[1],
                size_bytes[2],
                size_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn render_mode(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        mode: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::RenderModeReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let mode_bytes = mode.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                107,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
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
        Ok(Cookie::new(seq))
    }

    fn finish(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::FinishReply, 8>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                108,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn pixel_storef(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        pname: u32,
        datum: crate::proto::glx::Float32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let datum_bytes = datum.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                109,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
                datum_bytes[0],
                datum_bytes[1],
                datum_bytes[2],
                datum_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn pixel_storei(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        pname: u32,
        datum: i32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let datum_bytes = datum.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                110,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
                datum_bytes[0],
                datum_bytes[1],
                datum_bytes[2],
                datum_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn read_pixels(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        r#type: u32,
        swap_bytes: u8,
        lsb_first: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::ReadPixelsReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (9u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let x_bytes = x.serialize_fixed();
        let y_bytes = y.serialize_fixed();
        let width_bytes = width.serialize_fixed();
        let height_bytes = height.serialize_fixed();
        let format_bytes = format.serialize_fixed();
        let r#type_bytes = r#type.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..36)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                111,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                x_bytes[0],
                x_bytes[1],
                x_bytes[2],
                x_bytes[3],
                y_bytes[0],
                y_bytes[1],
                y_bytes[2],
                y_bytes[3],
                width_bytes[0],
                width_bytes[1],
                width_bytes[2],
                width_bytes[3],
                height_bytes[0],
                height_bytes[1],
                height_bytes[2],
                height_bytes[3],
                format_bytes[0],
                format_bytes[1],
                format_bytes[2],
                format_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                swap_bytes,
                lsb_first,
                0,
                0,
            ]);
        self.advance_writer(36);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_booleanv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        pname: i32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetBooleanvReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                112,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_clip_plane(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        plane: i32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetClipPlaneReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let plane_bytes = plane.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                113,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                plane_bytes[0],
                plane_bytes[1],
                plane_bytes[2],
                plane_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_doublev(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetDoublevReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                114,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_error(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::GetErrorReply, 12>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                115,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn get_floatv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetFloatvReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                116,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_integerv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetIntegervReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                117,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_lightfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        light: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetLightfvReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let light_bytes = light.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                118,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                light_bytes[0],
                light_bytes[1],
                light_bytes[2],
                light_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_lightiv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        light: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetLightivReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let light_bytes = light.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                119,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                light_bytes[0],
                light_bytes[1],
                light_bytes[2],
                light_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_mapdv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        query: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetMapdvReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let query_bytes = query.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                120,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                query_bytes[0],
                query_bytes[1],
                query_bytes[2],
                query_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_mapfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        query: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetMapfvReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let query_bytes = query.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                121,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                query_bytes[0],
                query_bytes[1],
                query_bytes[2],
                query_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_mapiv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        query: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetMapivReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let query_bytes = query.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                122,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                query_bytes[0],
                query_bytes[1],
                query_bytes[2],
                query_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_materialfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        face: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetMaterialfvReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let face_bytes = face.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                123,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                face_bytes[0],
                face_bytes[1],
                face_bytes[2],
                face_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_materialiv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        face: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetMaterialivReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let face_bytes = face.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                124,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                face_bytes[0],
                face_bytes[1],
                face_bytes[2],
                face_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_pixel_mapfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        map: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetPixelMapfvReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let map_bytes = map.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                125,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                map_bytes[0],
                map_bytes[1],
                map_bytes[2],
                map_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_pixel_mapuiv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        map: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetPixelMapuivReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let map_bytes = map.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                126,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                map_bytes[0],
                map_bytes[1],
                map_bytes[2],
                map_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_pixel_mapusv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        map: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetPixelMapusvReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let map_bytes = map.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                127,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                map_bytes[0],
                map_bytes[1],
                map_bytes[2],
                map_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_polygon_stipple(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        lsb_first: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetPolygonStippleReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                128,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                lsb_first,
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

    fn get_string(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        name: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetStringReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let name_bytes = name.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                129,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
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
        Ok(Cookie::new(seq))
    }

    fn get_tex_envfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexEnvfvReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                130,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_tex_enviv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexEnvivReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                131,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_tex_gendv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        coord: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexGendvReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let coord_bytes = coord.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                132,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                coord_bytes[0],
                coord_bytes[1],
                coord_bytes[2],
                coord_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_tex_genfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        coord: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexGenfvReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let coord_bytes = coord.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                133,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                coord_bytes[0],
                coord_bytes[1],
                coord_bytes[2],
                coord_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_tex_geniv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        coord: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexGenivReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let coord_bytes = coord.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                134,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                coord_bytes[0],
                coord_bytes[1],
                coord_bytes[2],
                coord_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_tex_image(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        level: i32,
        format: u32,
        r#type: u32,
        swap_bytes: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexImageReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (7u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let level_bytes = level.serialize_fixed();
        let format_bytes = format.serialize_fixed();
        let r#type_bytes = r#type.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                135,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                level_bytes[0],
                level_bytes[1],
                level_bytes[2],
                level_bytes[3],
                format_bytes[0],
                format_bytes[1],
                format_bytes[2],
                format_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                swap_bytes,
                0,
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

    fn get_tex_parameterfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexParameterfvReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                136,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_tex_parameteriv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexParameterivReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                137,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_tex_level_parameterfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        level: i32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexLevelParameterfvReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (5u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let level_bytes = level.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                138,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                level_bytes[0],
                level_bytes[1],
                level_bytes[2],
                level_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(20);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_tex_level_parameteriv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        level: i32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetTexLevelParameterivReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (5u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let level_bytes = level.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                139,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                level_bytes[0],
                level_bytes[1],
                level_bytes[2],
                level_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(20);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn is_enabled(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        capability: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::IsEnabledReply, 12>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let capability_bytes = capability.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                140,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                capability_bytes[0],
                capability_bytes[1],
                capability_bytes[2],
                capability_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn is_list(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        list: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::IsListReply, 12>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let list_bytes = list.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                141,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                list_bytes[0],
                list_bytes[1],
                list_bytes[2],
                list_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn flush(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                142,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn are_textures_resident(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        textures: &[u32],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::AreTexturesResidentReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let n = u32::try_from(textures.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context_tag.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(n).map_err(|_| crate::error::Error::Serialize)?).serialize_fixed(),
            );
        let list_len = textures.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            textures,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 143]);
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

    fn delete_textures(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        textures: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let n = u32::try_from(textures.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context_tag.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(n).map_err(|_| crate::error::Error::Serialize)?).serialize_fixed(),
            );
        let list_len = textures.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            textures,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 144]);
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

    fn gen_textures(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        n: i32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GenTexturesReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let n_bytes = n.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                145,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                n_bytes[0],
                n_bytes[1],
                n_bytes[2],
                n_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn is_texture(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        texture: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::IsTextureReply, 12>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let texture_bytes = texture.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                146,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                texture_bytes[0],
                texture_bytes[1],
                texture_bytes[2],
                texture_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn get_color_table(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        format: u32,
        r#type: u32,
        swap_bytes: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetColorTableReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (6u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let format_bytes = format.serialize_fixed();
        let r#type_bytes = r#type.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                147,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                format_bytes[0],
                format_bytes[1],
                format_bytes[2],
                format_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                swap_bytes,
                0,
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

    fn get_color_table_parameterfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetColorTableParameterfvReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                148,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_color_table_parameteriv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetColorTableParameterivReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                149,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_convolution_filter(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        format: u32,
        r#type: u32,
        swap_bytes: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetConvolutionFilterReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (6u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let format_bytes = format.serialize_fixed();
        let r#type_bytes = r#type.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                150,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                format_bytes[0],
                format_bytes[1],
                format_bytes[2],
                format_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                swap_bytes,
                0,
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

    fn get_convolution_parameterfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetConvolutionParameterfvReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                151,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_convolution_parameteriv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetConvolutionParameterivReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                152,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_separable_filter(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        format: u32,
        r#type: u32,
        swap_bytes: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetSeparableFilterReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (6u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let format_bytes = format.serialize_fixed();
        let r#type_bytes = r#type.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                153,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                format_bytes[0],
                format_bytes[1],
                format_bytes[2],
                format_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                swap_bytes,
                0,
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

    fn get_histogram(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        format: u32,
        r#type: u32,
        swap_bytes: u8,
        reset: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetHistogramReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (6u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let format_bytes = format.serialize_fixed();
        let r#type_bytes = r#type.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                154,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                format_bytes[0],
                format_bytes[1],
                format_bytes[2],
                format_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                swap_bytes,
                reset,
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

    fn get_histogram_parameterfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetHistogramParameterfvReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                155,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_histogram_parameteriv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetHistogramParameterivReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                156,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_minmax(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        format: u32,
        r#type: u32,
        swap_bytes: u8,
        reset: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetMinmaxReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (6u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let format_bytes = format.serialize_fixed();
        let r#type_bytes = r#type.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                157,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                format_bytes[0],
                format_bytes[1],
                format_bytes[2],
                format_bytes[3],
                r#type_bytes[0],
                r#type_bytes[1],
                r#type_bytes[2],
                r#type_bytes[3],
                swap_bytes,
                reset,
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

    fn get_minmax_parameterfv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetMinmaxParameterfvReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                158,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_minmax_parameteriv(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetMinmaxParameterivReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                159,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_compressed_tex_image_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        level: i32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetCompressedTexImageARBReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let level_bytes = level.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                160,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                level_bytes[0],
                level_bytes[1],
                level_bytes[2],
                level_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn delete_queries_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        ids: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let buf_ptr = self.apply_offset(socket_buffer);
        let n = u32::try_from(ids.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&context_tag.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u32::try_from(n).map_err(|_| crate::error::Error::Serialize)?).serialize_fixed(),
            );
        let list_len = ids.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            ids,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[major_opcode, 161]);
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

    fn gen_queries_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        n: i32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GenQueriesARBReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let n_bytes = n.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                162,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                n_bytes[0],
                n_bytes[1],
                n_bytes[2],
                n_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn is_query_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        id: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::glx::IsQueryARBReply, 12>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let id_bytes = id.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                163,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                id_bytes[0],
                id_bytes[1],
                id_bytes[2],
                id_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn get_queryiv_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        target: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetQueryivARBReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                164,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_query_objectiv_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        id: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetQueryObjectivARBReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let id_bytes = id.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                165,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                id_bytes[0],
                id_bytes[1],
                id_bytes[2],
                id_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn get_query_objectuiv_a_r_b(
        &mut self,
        socket_buffer: &mut [u8],
        context_tag: crate::proto::glx::ContextTag,
        id: u32,
        pname: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::glx::GetQueryObjectuivARBReply>> {
        let major_opcode = self.major_opcode(crate::proto::glx::EXTENSION_NAME).ok_or(
            crate::error::Error::MissingExtension(crate::proto::glx::EXTENSION_NAME),
        )?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let context_tag_bytes = context_tag.serialize_fixed();
        let id_bytes = id.serialize_fixed();
        let pname_bytes = pname.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                166,
                length[0],
                length[1],
                context_tag_bytes[0],
                context_tag_bytes[1],
                context_tag_bytes[2],
                context_tag_bytes[3],
                id_bytes[0],
                id_bytes[1],
                id_bytes[2],
                id_bytes[3],
                pname_bytes[0],
                pname_bytes[1],
                pname_bytes[2],
                pname_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }
}
