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
pub trait XprotoConnection {
    fn get_window_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GetWindowAttributesReply, 44>>;

    fn destroy_window(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn destroy_subwindows(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn change_save_set(
        &mut self,
        socket_buffer: &mut [u8],
        mode: crate::proto::xproto::SetModeEnum,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn reparent_window(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        parent: crate::proto::xproto::Window,
        x: i16,
        y: i16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn map_window(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn map_subwindows(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn unmap_window(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn unmap_subwindows(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn configure_window(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        configure_window_value_list: crate::proto::xproto::ConfigureWindowValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn circulate_window(
        &mut self,
        socket_buffer: &mut [u8],
        direction: crate::proto::xproto::CirculateEnum,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_geometry(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GetGeometryReply, 24>>;

    fn query_tree(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::QueryTreeReply>>;

    fn intern_atom(
        &mut self,
        socket_buffer: &mut [u8],
        only_if_exists: u8,
        name: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::InternAtomReply, 12>>;

    fn get_atom_name(
        &mut self,
        socket_buffer: &mut [u8],
        atom: crate::proto::xproto::Atom,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::GetAtomNameReply>>;

    fn change_property(
        &mut self,
        socket_buffer: &mut [u8],
        mode: crate::proto::xproto::PropModeEnum,
        window: crate::proto::xproto::Window,
        property: crate::proto::xproto::Atom,
        r#type: crate::proto::xproto::Atom,
        format: u8,
        data_len: u32,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn delete_property(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        property: crate::proto::xproto::Atom,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_property(
        &mut self,
        socket_buffer: &mut [u8],
        delete: u8,
        window: crate::proto::xproto::Window,
        property: crate::proto::xproto::Atom,
        r#type: crate::proto::xproto::GetPropertyTypeEnum,
        long_offset: u32,
        long_length: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::GetPropertyReply>>;

    fn list_properties(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::ListPropertiesReply>>;

    fn set_selection_owner(
        &mut self,
        socket_buffer: &mut [u8],
        owner: crate::proto::xproto::WindowEnum,
        selection: crate::proto::xproto::Atom,
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_selection_owner(
        &mut self,
        socket_buffer: &mut [u8],
        selection: crate::proto::xproto::Atom,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GetSelectionOwnerReply, 12>>;

    fn convert_selection(
        &mut self,
        socket_buffer: &mut [u8],
        requestor: crate::proto::xproto::Window,
        selection: crate::proto::xproto::Atom,
        target: crate::proto::xproto::Atom,
        property: crate::proto::xproto::AtomEnum,
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn send_event(
        &mut self,
        socket_buffer: &mut [u8],
        propagate: u8,
        destination: crate::proto::xproto::SendEventDestEnum,
        event_mask: crate::proto::xproto::EventMask,
        event: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn grab_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        owner_events: u8,
        grab_window: crate::proto::xproto::Window,
        event_mask: crate::proto::xproto::EventMask,
        pointer_mode: crate::proto::xproto::GrabModeEnum,
        keyboard_mode: crate::proto::xproto::GrabModeEnum,
        confine_to: crate::proto::xproto::WindowEnum,
        cursor: crate::proto::xproto::CursorEnum,
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GrabPointerReply, 8>>;

    fn ungrab_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn grab_button(
        &mut self,
        socket_buffer: &mut [u8],
        owner_events: u8,
        grab_window: crate::proto::xproto::Window,
        event_mask: crate::proto::xproto::EventMask,
        pointer_mode: crate::proto::xproto::GrabModeEnum,
        keyboard_mode: crate::proto::xproto::GrabModeEnum,
        confine_to: crate::proto::xproto::WindowEnum,
        cursor: crate::proto::xproto::CursorEnum,
        button: crate::proto::xproto::ButtonIndexEnum,
        modifiers: crate::proto::xproto::ModMask,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn ungrab_button(
        &mut self,
        socket_buffer: &mut [u8],
        button: crate::proto::xproto::ButtonIndexEnum,
        grab_window: crate::proto::xproto::Window,
        modifiers: crate::proto::xproto::ModMask,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn change_active_pointer_grab(
        &mut self,
        socket_buffer: &mut [u8],
        cursor: crate::proto::xproto::CursorEnum,
        time: crate::proto::xproto::TimeEnum,
        event_mask: crate::proto::xproto::EventMask,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn grab_keyboard(
        &mut self,
        socket_buffer: &mut [u8],
        owner_events: u8,
        grab_window: crate::proto::xproto::Window,
        time: crate::proto::xproto::TimeEnum,
        pointer_mode: crate::proto::xproto::GrabModeEnum,
        keyboard_mode: crate::proto::xproto::GrabModeEnum,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GrabKeyboardReply, 8>>;

    fn ungrab_keyboard(
        &mut self,
        socket_buffer: &mut [u8],
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn grab_key(
        &mut self,
        socket_buffer: &mut [u8],
        owner_events: u8,
        grab_window: crate::proto::xproto::Window,
        modifiers: crate::proto::xproto::ModMask,
        key: crate::proto::xproto::GrabEnum,
        pointer_mode: crate::proto::xproto::GrabModeEnum,
        keyboard_mode: crate::proto::xproto::GrabModeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn ungrab_key(
        &mut self,
        socket_buffer: &mut [u8],
        key: crate::proto::xproto::GrabEnum,
        grab_window: crate::proto::xproto::Window,
        modifiers: crate::proto::xproto::ModMask,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn allow_events(
        &mut self,
        socket_buffer: &mut [u8],
        mode: crate::proto::xproto::AllowEnum,
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn grab_server(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn ungrab_server(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn query_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::QueryPointerReply, 28>>;

    fn get_motion_events(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        start: crate::proto::xproto::TimeEnum,
        stop: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::GetMotionEventsReply>>;

    fn translate_coordinates(
        &mut self,
        socket_buffer: &mut [u8],
        src_window: crate::proto::xproto::Window,
        dst_window: crate::proto::xproto::Window,
        src_x: i16,
        src_y: i16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::TranslateCoordinatesReply, 16>>;

    fn warp_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        src_window: crate::proto::xproto::WindowEnum,
        dst_window: crate::proto::xproto::WindowEnum,
        src_x: i16,
        src_y: i16,
        src_width: u16,
        src_height: u16,
        dst_x: i16,
        dst_y: i16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn set_input_focus(
        &mut self,
        socket_buffer: &mut [u8],
        revert_to: crate::proto::xproto::InputFocusEnum,
        focus: crate::proto::xproto::InputFocusEnum,
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_input_focus(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GetInputFocusReply, 12>>;

    fn query_keymap(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::QueryKeymapReply, 40>>;

    fn open_font(
        &mut self,
        socket_buffer: &mut [u8],
        fid: crate::proto::xproto::Font,
        name: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn close_font(
        &mut self,
        socket_buffer: &mut [u8],
        font: crate::proto::xproto::Font,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn query_font(
        &mut self,
        socket_buffer: &mut [u8],
        font: crate::proto::xproto::Fontable,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::QueryFontReply>>;

    fn query_text_extents(
        &mut self,
        socket_buffer: &mut [u8],
        font: crate::proto::xproto::Fontable,
        string: &[crate::proto::xproto::Char2b],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::QueryTextExtentsReply, 28>>;

    fn list_fonts(
        &mut self,
        socket_buffer: &mut [u8],
        max_names: u16,
        pattern: &[u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::ListFontsReply>>;

    fn list_fonts_with_info(
        &mut self,
        socket_buffer: &mut [u8],
        max_names: u16,
        pattern: &[u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::ListFontsWithInfoReply>>;

    fn set_font_path(
        &mut self,
        socket_buffer: &mut [u8],
        font: alloc::vec::Vec<crate::proto::xproto::Str>,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_font_path(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::GetFontPathReply>>;

    fn create_pixmap(
        &mut self,
        socket_buffer: &mut [u8],
        depth: u8,
        pid: crate::proto::xproto::Pixmap,
        drawable: crate::proto::xproto::Drawable,
        width: u16,
        height: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn free_pixmap(
        &mut self,
        socket_buffer: &mut [u8],
        pixmap: crate::proto::xproto::Pixmap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn copy_g_c(
        &mut self,
        socket_buffer: &mut [u8],
        src_gc: crate::proto::xproto::Gcontext,
        dst_gc: crate::proto::xproto::Gcontext,
        value_mask: crate::proto::xproto::Gc,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn set_dashes(
        &mut self,
        socket_buffer: &mut [u8],
        gc: crate::proto::xproto::Gcontext,
        dash_offset: u16,
        dashes: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn set_clip_rectangles(
        &mut self,
        socket_buffer: &mut [u8],
        ordering: crate::proto::xproto::ClipOrderingEnum,
        gc: crate::proto::xproto::Gcontext,
        clip_x_origin: i16,
        clip_y_origin: i16,
        rectangles: &[crate::proto::xproto::Rectangle],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn free_g_c(
        &mut self,
        socket_buffer: &mut [u8],
        gc: crate::proto::xproto::Gcontext,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn clear_area(
        &mut self,
        socket_buffer: &mut [u8],
        exposures: u8,
        window: crate::proto::xproto::Window,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn copy_area(
        &mut self,
        socket_buffer: &mut [u8],
        src_drawable: crate::proto::xproto::Drawable,
        dst_drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        src_x: i16,
        src_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn copy_plane(
        &mut self,
        socket_buffer: &mut [u8],
        src_drawable: crate::proto::xproto::Drawable,
        dst_drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        src_x: i16,
        src_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
        bit_plane: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn poly_point(
        &mut self,
        socket_buffer: &mut [u8],
        coordinate_mode: crate::proto::xproto::CoordModeEnum,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        points: &[crate::proto::xproto::Point],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn poly_line(
        &mut self,
        socket_buffer: &mut [u8],
        coordinate_mode: crate::proto::xproto::CoordModeEnum,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        points: &[crate::proto::xproto::Point],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn poly_segment(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        segments: &[crate::proto::xproto::Segment],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn poly_rectangle(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        rectangles: &[crate::proto::xproto::Rectangle],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn poly_arc(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        arcs: &[crate::proto::xproto::Arc],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn fill_poly(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        shape: crate::proto::xproto::PolyShapeEnum,
        coordinate_mode: crate::proto::xproto::CoordModeEnum,
        points: &[crate::proto::xproto::Point],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn poly_fill_rectangle(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        rectangles: &[crate::proto::xproto::Rectangle],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn poly_fill_arc(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        arcs: &[crate::proto::xproto::Arc],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn put_image(
        &mut self,
        socket_buffer: &mut [u8],
        format: crate::proto::xproto::ImageFormatEnum,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        width: u16,
        height: u16,
        dst_x: i16,
        dst_y: i16,
        left_pad: u8,
        depth: u8,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_image(
        &mut self,
        socket_buffer: &mut [u8],
        format: crate::proto::xproto::ImageFormatEnum,
        drawable: crate::proto::xproto::Drawable,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        plane_mask: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::GetImageReply>>;

    fn poly_text8(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        x: i16,
        y: i16,
        items: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn poly_text16(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        x: i16,
        y: i16,
        items: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn image_text8(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        x: i16,
        y: i16,
        string: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn image_text16(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        x: i16,
        y: i16,
        string: &[crate::proto::xproto::Char2b],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn create_colormap(
        &mut self,
        socket_buffer: &mut [u8],
        alloc: crate::proto::xproto::ColormapAllocEnum,
        mid: crate::proto::xproto::Colormap,
        window: crate::proto::xproto::Window,
        visual: crate::proto::xproto::Visualid,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn free_colormap(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn copy_colormap_and_free(
        &mut self,
        socket_buffer: &mut [u8],
        mid: crate::proto::xproto::Colormap,
        src_cmap: crate::proto::xproto::Colormap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn install_colormap(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn uninstall_colormap(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn list_installed_colormaps(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::ListInstalledColormapsReply>>;

    fn alloc_color(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        red: u16,
        green: u16,
        blue: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::AllocColorReply, 20>>;

    fn alloc_named_color(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        name: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::AllocNamedColorReply, 24>>;

    fn alloc_color_cells(
        &mut self,
        socket_buffer: &mut [u8],
        contiguous: u8,
        cmap: crate::proto::xproto::Colormap,
        colors: u16,
        planes: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::AllocColorCellsReply>>;

    fn alloc_color_planes(
        &mut self,
        socket_buffer: &mut [u8],
        contiguous: u8,
        cmap: crate::proto::xproto::Colormap,
        colors: u16,
        reds: u16,
        greens: u16,
        blues: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::AllocColorPlanesReply>>;

    fn free_colors(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        plane_mask: u32,
        pixels: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn store_colors(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        items: &[crate::proto::xproto::Coloritem],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn store_named_color(
        &mut self,
        socket_buffer: &mut [u8],
        flags: crate::proto::xproto::ColorFlag,
        cmap: crate::proto::xproto::Colormap,
        pixel: u32,
        name: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn query_colors(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        pixels: &[u32],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::QueryColorsReply>>;

    fn lookup_color(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        name: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::LookupColorReply, 20>>;

    fn create_cursor(
        &mut self,
        socket_buffer: &mut [u8],
        cid: crate::proto::xproto::Cursor,
        source: crate::proto::xproto::Pixmap,
        mask: crate::proto::xproto::PixmapEnum,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
        x: u16,
        y: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn create_glyph_cursor(
        &mut self,
        socket_buffer: &mut [u8],
        cid: crate::proto::xproto::Cursor,
        source_font: crate::proto::xproto::Font,
        mask_font: crate::proto::xproto::FontEnum,
        source_char: u16,
        mask_char: u16,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn free_cursor(
        &mut self,
        socket_buffer: &mut [u8],
        cursor: crate::proto::xproto::Cursor,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn recolor_cursor(
        &mut self,
        socket_buffer: &mut [u8],
        cursor: crate::proto::xproto::Cursor,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn query_best_size(
        &mut self,
        socket_buffer: &mut [u8],
        class: crate::proto::xproto::QueryShapeOfEnum,
        drawable: crate::proto::xproto::Drawable,
        width: u16,
        height: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::QueryBestSizeReply, 12>>;

    fn query_extension(
        &mut self,
        socket_buffer: &mut [u8],
        name: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::QueryExtensionReply, 12>>;

    fn list_extensions(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::ListExtensionsReply>>;

    fn change_keyboard_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        keycode_count: u8,
        first_keycode: crate::proto::xproto::Keycode,
        keysyms_per_keycode: u8,
        keysyms: &[crate::proto::xproto::Keysym],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_keyboard_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        first_keycode: crate::proto::xproto::Keycode,
        count: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::GetKeyboardMappingReply>>;

    fn change_keyboard_control(
        &mut self,
        socket_buffer: &mut [u8],
        change_keyboard_control_value_list: crate::proto::xproto::ChangeKeyboardControlValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_keyboard_control(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GetKeyboardControlReply, 52>>;

    fn bell(
        &mut self,
        socket_buffer: &mut [u8],
        percent: i8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn change_pointer_control(
        &mut self,
        socket_buffer: &mut [u8],
        acceleration_numerator: i16,
        acceleration_denominator: i16,
        threshold: i16,
        do_acceleration: u8,
        do_threshold: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_pointer_control(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GetPointerControlReply, 32>>;

    fn set_screen_saver(
        &mut self,
        socket_buffer: &mut [u8],
        timeout: i16,
        interval: i16,
        prefer_blanking: crate::proto::xproto::BlankingEnum,
        allow_exposures: crate::proto::xproto::ExposuresEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn get_screen_saver(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GetScreenSaverReply, 32>>;

    fn change_hosts(
        &mut self,
        socket_buffer: &mut [u8],
        mode: crate::proto::xproto::HostModeEnum,
        family: crate::proto::xproto::FamilyEnum,
        address: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn set_access_control(
        &mut self,
        socket_buffer: &mut [u8],
        mode: crate::proto::xproto::AccessControlEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn set_close_down_mode(
        &mut self,
        socket_buffer: &mut [u8],
        mode: crate::proto::xproto::CloseDownEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn kill_client(
        &mut self,
        socket_buffer: &mut [u8],
        resource: crate::proto::xproto::KillEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn rotate_properties(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        delta: i16,
        atoms: &[crate::proto::xproto::Atom],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn force_screen_saver(
        &mut self,
        socket_buffer: &mut [u8],
        mode: crate::proto::xproto::ScreenSaverEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn set_pointer_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        map: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::SetPointerMappingReply, 8>>;

    fn get_pointer_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::GetPointerMappingReply>>;

    fn set_modifier_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        keycodes_per_modifier: u8,
        keycodes: &[crate::proto::xproto::Keycode],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::SetModifierMappingReply, 8>>;

    fn get_modifier_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::GetModifierMappingReply>>;

    fn no_operation(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn create_window(
        &mut self,
        socket_buffer: &mut [u8],
        depth: u8,
        wid: crate::proto::xproto::Window,
        parent: crate::proto::xproto::Window,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: crate::proto::xproto::WindowClassEnum,
        visual: crate::proto::xproto::Visualid,
        create_window_value_list: crate::proto::xproto::CreateWindowValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn change_window_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        change_window_attributes_value_list: crate::proto::xproto::ChangeWindowAttributesValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn create_g_c(
        &mut self,
        socket_buffer: &mut [u8],
        cid: crate::proto::xproto::Gcontext,
        drawable: crate::proto::xproto::Drawable,
        create_g_c_value_list: crate::proto::xproto::CreateGCValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn change_g_c(
        &mut self,
        socket_buffer: &mut [u8],
        gc: crate::proto::xproto::Gcontext,
        change_g_c_value_list: crate::proto::xproto::ChangeGCValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn list_hosts(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::ListHostsReply>>;
}
impl<C> XprotoConnection for C
where
    C: crate::con::XcbConnection,
{
    fn get_window_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GetWindowAttributesReply, 44>> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                3,
                0,
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

    fn destroy_window(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                4,
                0,
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
        Ok(VoidCookie::new(seq))
    }

    fn destroy_subwindows(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                5,
                0,
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
        Ok(VoidCookie::new(seq))
    }

    fn change_save_set(
        &mut self,
        socket_buffer: &mut [u8],
        mode: crate::proto::xproto::SetModeEnum,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                6,
                mode.0 as u8,
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
        Ok(VoidCookie::new(seq))
    }

    fn reparent_window(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        parent: crate::proto::xproto::Window,
        x: i16,
        y: i16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let parent_bytes = parent.serialize_fixed();
        let x_bytes = x.serialize_fixed();
        let y_bytes = y.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                7,
                0,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                parent_bytes[0],
                parent_bytes[1],
                parent_bytes[2],
                parent_bytes[3],
                x_bytes[0],
                x_bytes[1],
                y_bytes[0],
                y_bytes[1],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn map_window(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                8,
                0,
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
        Ok(VoidCookie::new(seq))
    }

    fn map_subwindows(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                9,
                0,
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
        Ok(VoidCookie::new(seq))
    }

    fn unmap_window(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                10,
                0,
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
        Ok(VoidCookie::new(seq))
    }

    fn unmap_subwindows(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                11,
                0,
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
        Ok(VoidCookie::new(seq))
    }

    fn configure_window(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        configure_window_value_list: crate::proto::xproto::ConfigureWindowValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        // Pad 2 bytes
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[12, 0]);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&window.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&configure_window_value_list.switch_expr().serialize_fixed());
        let mut offset = 12
            + configure_window_value_list.serialize_into(
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

    fn circulate_window(
        &mut self,
        socket_buffer: &mut [u8],
        direction: crate::proto::xproto::CirculateEnum,
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                13,
                direction.0 as u8,
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
        Ok(VoidCookie::new(seq))
    }

    fn get_geometry(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GetGeometryReply, 24>> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                14,
                0,
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
        Ok(FixedCookie::new(seq))
    }

    fn query_tree(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::QueryTreeReply>> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                15,
                0,
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

    fn intern_atom(
        &mut self,
        socket_buffer: &mut [u8],
        only_if_exists: u8,
        name: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::InternAtomReply, 12>> {
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
            .copy_from_slice(&[16, only_if_exists]);
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

    fn get_atom_name(
        &mut self,
        socket_buffer: &mut [u8],
        atom: crate::proto::xproto::Atom,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::GetAtomNameReply>> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let atom_bytes = atom.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                17,
                0,
                length[0],
                length[1],
                atom_bytes[0],
                atom_bytes[1],
                atom_bytes[2],
                atom_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn change_property(
        &mut self,
        socket_buffer: &mut [u8],
        mode: crate::proto::xproto::PropModeEnum,
        window: crate::proto::xproto::Window,
        property: crate::proto::xproto::Atom,
        r#type: crate::proto::xproto::Atom,
        format: u8,
        data_len: u32,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 3 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&window.serialize_fixed());
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
            .get_mut(20..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&data_len.serialize_fixed());
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
            .copy_from_slice(&[18, mode.0]);
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

    fn delete_property(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        property: crate::proto::xproto::Atom,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let property_bytes = property.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                19,
                0,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
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

    fn get_property(
        &mut self,
        socket_buffer: &mut [u8],
        delete: u8,
        window: crate::proto::xproto::Window,
        property: crate::proto::xproto::Atom,
        r#type: crate::proto::xproto::GetPropertyTypeEnum,
        long_offset: u32,
        long_length: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::GetPropertyReply>> {
        let length: [u8; 2] = (6u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let property_bytes = property.serialize_fixed();
        let r#type_bytes = (r#type.0 as u32).serialize_fixed();
        let long_offset_bytes = long_offset.serialize_fixed();
        let long_length_bytes = long_length.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                20,
                delete,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
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
            ]);
        self.advance_writer(24);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn list_properties(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::ListPropertiesReply>> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                21,
                0,
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

    fn set_selection_owner(
        &mut self,
        socket_buffer: &mut [u8],
        owner: crate::proto::xproto::WindowEnum,
        selection: crate::proto::xproto::Atom,
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let owner_bytes = (owner.0 as u32).serialize_fixed();
        let selection_bytes = selection.serialize_fixed();
        let time_bytes = (time.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                22,
                0,
                length[0],
                length[1],
                owner_bytes[0],
                owner_bytes[1],
                owner_bytes[2],
                owner_bytes[3],
                selection_bytes[0],
                selection_bytes[1],
                selection_bytes[2],
                selection_bytes[3],
                time_bytes[0],
                time_bytes[1],
                time_bytes[2],
                time_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_selection_owner(
        &mut self,
        socket_buffer: &mut [u8],
        selection: crate::proto::xproto::Atom,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GetSelectionOwnerReply, 12>> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let selection_bytes = selection.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                23,
                0,
                length[0],
                length[1],
                selection_bytes[0],
                selection_bytes[1],
                selection_bytes[2],
                selection_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn convert_selection(
        &mut self,
        socket_buffer: &mut [u8],
        requestor: crate::proto::xproto::Window,
        selection: crate::proto::xproto::Atom,
        target: crate::proto::xproto::Atom,
        property: crate::proto::xproto::AtomEnum,
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (6u16).to_ne_bytes();
        let requestor_bytes = requestor.serialize_fixed();
        let selection_bytes = selection.serialize_fixed();
        let target_bytes = target.serialize_fixed();
        let property_bytes = (property.0 as u32).serialize_fixed();
        let time_bytes = (time.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                24,
                0,
                length[0],
                length[1],
                requestor_bytes[0],
                requestor_bytes[1],
                requestor_bytes[2],
                requestor_bytes[3],
                selection_bytes[0],
                selection_bytes[1],
                selection_bytes[2],
                selection_bytes[3],
                target_bytes[0],
                target_bytes[1],
                target_bytes[2],
                target_bytes[3],
                property_bytes[0],
                property_bytes[1],
                property_bytes[2],
                property_bytes[3],
                time_bytes[0],
                time_bytes[1],
                time_bytes[2],
                time_bytes[3],
            ]);
        self.advance_writer(24);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn send_event(
        &mut self,
        socket_buffer: &mut [u8],
        propagate: u8,
        destination: crate::proto::xproto::SendEventDestEnum,
        event_mask: crate::proto::xproto::EventMask,
        event: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (11u16).to_ne_bytes();
        let destination_bytes = (destination.0 as u32).serialize_fixed();
        let event_mask_bytes = (event_mask.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..44)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                25,
                propagate,
                length[0],
                length[1],
                destination_bytes[0],
                destination_bytes[1],
                destination_bytes[2],
                destination_bytes[3],
                event_mask_bytes[0],
                event_mask_bytes[1],
                event_mask_bytes[2],
                event_mask_bytes[3],
                event[0],
                event[1],
                event[2],
                event[3],
                event[4],
                event[5],
                event[6],
                event[7],
                event[8],
                event[9],
                event[10],
                event[11],
                event[12],
                event[13],
                event[14],
                event[15],
                event[16],
                event[17],
                event[18],
                event[19],
                event[20],
                event[21],
                event[22],
                event[23],
                event[24],
                event[25],
                event[26],
                event[27],
                event[28],
                event[29],
                event[30],
                event[31],
            ]);
        self.advance_writer(44);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn grab_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        owner_events: u8,
        grab_window: crate::proto::xproto::Window,
        event_mask: crate::proto::xproto::EventMask,
        pointer_mode: crate::proto::xproto::GrabModeEnum,
        keyboard_mode: crate::proto::xproto::GrabModeEnum,
        confine_to: crate::proto::xproto::WindowEnum,
        cursor: crate::proto::xproto::CursorEnum,
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GrabPointerReply, 8>> {
        let length: [u8; 2] = (6u16).to_ne_bytes();
        let grab_window_bytes = grab_window.serialize_fixed();
        let event_mask_bytes = (event_mask.0 as u16).serialize_fixed();
        let confine_to_bytes = (confine_to.0 as u32).serialize_fixed();
        let cursor_bytes = (cursor.0 as u32).serialize_fixed();
        let time_bytes = (time.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                26,
                owner_events,
                length[0],
                length[1],
                grab_window_bytes[0],
                grab_window_bytes[1],
                grab_window_bytes[2],
                grab_window_bytes[3],
                event_mask_bytes[0],
                event_mask_bytes[1],
                pointer_mode.0 as u8,
                keyboard_mode.0 as u8,
                confine_to_bytes[0],
                confine_to_bytes[1],
                confine_to_bytes[2],
                confine_to_bytes[3],
                cursor_bytes[0],
                cursor_bytes[1],
                cursor_bytes[2],
                cursor_bytes[3],
                time_bytes[0],
                time_bytes[1],
                time_bytes[2],
                time_bytes[3],
            ]);
        self.advance_writer(24);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn ungrab_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let time_bytes = (time.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                27,
                0,
                length[0],
                length[1],
                time_bytes[0],
                time_bytes[1],
                time_bytes[2],
                time_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn grab_button(
        &mut self,
        socket_buffer: &mut [u8],
        owner_events: u8,
        grab_window: crate::proto::xproto::Window,
        event_mask: crate::proto::xproto::EventMask,
        pointer_mode: crate::proto::xproto::GrabModeEnum,
        keyboard_mode: crate::proto::xproto::GrabModeEnum,
        confine_to: crate::proto::xproto::WindowEnum,
        cursor: crate::proto::xproto::CursorEnum,
        button: crate::proto::xproto::ButtonIndexEnum,
        modifiers: crate::proto::xproto::ModMask,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (6u16).to_ne_bytes();
        let grab_window_bytes = grab_window.serialize_fixed();
        let event_mask_bytes = (event_mask.0 as u16).serialize_fixed();
        let confine_to_bytes = (confine_to.0 as u32).serialize_fixed();
        let cursor_bytes = (cursor.0 as u32).serialize_fixed();
        let modifiers_bytes = (modifiers.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                28,
                owner_events,
                length[0],
                length[1],
                grab_window_bytes[0],
                grab_window_bytes[1],
                grab_window_bytes[2],
                grab_window_bytes[3],
                event_mask_bytes[0],
                event_mask_bytes[1],
                pointer_mode.0 as u8,
                keyboard_mode.0 as u8,
                confine_to_bytes[0],
                confine_to_bytes[1],
                confine_to_bytes[2],
                confine_to_bytes[3],
                cursor_bytes[0],
                cursor_bytes[1],
                cursor_bytes[2],
                cursor_bytes[3],
                button.0 as u8,
                0,
                modifiers_bytes[0],
                modifiers_bytes[1],
            ]);
        self.advance_writer(24);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn ungrab_button(
        &mut self,
        socket_buffer: &mut [u8],
        button: crate::proto::xproto::ButtonIndexEnum,
        grab_window: crate::proto::xproto::Window,
        modifiers: crate::proto::xproto::ModMask,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let grab_window_bytes = grab_window.serialize_fixed();
        let modifiers_bytes = (modifiers.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                29,
                button.0 as u8,
                length[0],
                length[1],
                grab_window_bytes[0],
                grab_window_bytes[1],
                grab_window_bytes[2],
                grab_window_bytes[3],
                modifiers_bytes[0],
                modifiers_bytes[1],
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

    fn change_active_pointer_grab(
        &mut self,
        socket_buffer: &mut [u8],
        cursor: crate::proto::xproto::CursorEnum,
        time: crate::proto::xproto::TimeEnum,
        event_mask: crate::proto::xproto::EventMask,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let cursor_bytes = (cursor.0 as u32).serialize_fixed();
        let time_bytes = (time.0 as u32).serialize_fixed();
        let event_mask_bytes = (event_mask.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                30,
                0,
                length[0],
                length[1],
                cursor_bytes[0],
                cursor_bytes[1],
                cursor_bytes[2],
                cursor_bytes[3],
                time_bytes[0],
                time_bytes[1],
                time_bytes[2],
                time_bytes[3],
                event_mask_bytes[0],
                event_mask_bytes[1],
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

    fn grab_keyboard(
        &mut self,
        socket_buffer: &mut [u8],
        owner_events: u8,
        grab_window: crate::proto::xproto::Window,
        time: crate::proto::xproto::TimeEnum,
        pointer_mode: crate::proto::xproto::GrabModeEnum,
        keyboard_mode: crate::proto::xproto::GrabModeEnum,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GrabKeyboardReply, 8>> {
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let grab_window_bytes = grab_window.serialize_fixed();
        let time_bytes = (time.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                31,
                owner_events,
                length[0],
                length[1],
                grab_window_bytes[0],
                grab_window_bytes[1],
                grab_window_bytes[2],
                grab_window_bytes[3],
                time_bytes[0],
                time_bytes[1],
                time_bytes[2],
                time_bytes[3],
                pointer_mode.0 as u8,
                keyboard_mode.0 as u8,
                0,
                0,
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn ungrab_keyboard(
        &mut self,
        socket_buffer: &mut [u8],
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let time_bytes = (time.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                32,
                0,
                length[0],
                length[1],
                time_bytes[0],
                time_bytes[1],
                time_bytes[2],
                time_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn grab_key(
        &mut self,
        socket_buffer: &mut [u8],
        owner_events: u8,
        grab_window: crate::proto::xproto::Window,
        modifiers: crate::proto::xproto::ModMask,
        key: crate::proto::xproto::GrabEnum,
        pointer_mode: crate::proto::xproto::GrabModeEnum,
        keyboard_mode: crate::proto::xproto::GrabModeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let grab_window_bytes = grab_window.serialize_fixed();
        let modifiers_bytes = (modifiers.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                33,
                owner_events,
                length[0],
                length[1],
                grab_window_bytes[0],
                grab_window_bytes[1],
                grab_window_bytes[2],
                grab_window_bytes[3],
                modifiers_bytes[0],
                modifiers_bytes[1],
                key.0 as u8,
                pointer_mode.0 as u8,
                keyboard_mode.0 as u8,
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

    fn ungrab_key(
        &mut self,
        socket_buffer: &mut [u8],
        key: crate::proto::xproto::GrabEnum,
        grab_window: crate::proto::xproto::Window,
        modifiers: crate::proto::xproto::ModMask,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let grab_window_bytes = grab_window.serialize_fixed();
        let modifiers_bytes = (modifiers.0 as u16).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                34,
                key.0 as u8,
                length[0],
                length[1],
                grab_window_bytes[0],
                grab_window_bytes[1],
                grab_window_bytes[2],
                grab_window_bytes[3],
                modifiers_bytes[0],
                modifiers_bytes[1],
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

    fn allow_events(
        &mut self,
        socket_buffer: &mut [u8],
        mode: crate::proto::xproto::AllowEnum,
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let time_bytes = (time.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                35,
                mode.0 as u8,
                length[0],
                length[1],
                time_bytes[0],
                time_bytes[1],
                time_bytes[2],
                time_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn grab_server(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = 36;
        buf[1] = 0;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn ungrab_server(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = 37;
        buf[1] = 0;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn query_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::QueryPointerReply, 28>> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                38,
                0,
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

    fn get_motion_events(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        start: crate::proto::xproto::TimeEnum,
        stop: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::GetMotionEventsReply>> {
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let start_bytes = (start.0 as u32).serialize_fixed();
        let stop_bytes = (stop.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                39,
                0,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                start_bytes[0],
                start_bytes[1],
                start_bytes[2],
                start_bytes[3],
                stop_bytes[0],
                stop_bytes[1],
                stop_bytes[2],
                stop_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn translate_coordinates(
        &mut self,
        socket_buffer: &mut [u8],
        src_window: crate::proto::xproto::Window,
        dst_window: crate::proto::xproto::Window,
        src_x: i16,
        src_y: i16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::TranslateCoordinatesReply, 16>>
    {
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let src_window_bytes = src_window.serialize_fixed();
        let dst_window_bytes = dst_window.serialize_fixed();
        let src_x_bytes = src_x.serialize_fixed();
        let src_y_bytes = src_y.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                40,
                0,
                length[0],
                length[1],
                src_window_bytes[0],
                src_window_bytes[1],
                src_window_bytes[2],
                src_window_bytes[3],
                dst_window_bytes[0],
                dst_window_bytes[1],
                dst_window_bytes[2],
                dst_window_bytes[3],
                src_x_bytes[0],
                src_x_bytes[1],
                src_y_bytes[0],
                src_y_bytes[1],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn warp_pointer(
        &mut self,
        socket_buffer: &mut [u8],
        src_window: crate::proto::xproto::WindowEnum,
        dst_window: crate::proto::xproto::WindowEnum,
        src_x: i16,
        src_y: i16,
        src_width: u16,
        src_height: u16,
        dst_x: i16,
        dst_y: i16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (6u16).to_ne_bytes();
        let src_window_bytes = (src_window.0 as u32).serialize_fixed();
        let dst_window_bytes = (dst_window.0 as u32).serialize_fixed();
        let src_x_bytes = src_x.serialize_fixed();
        let src_y_bytes = src_y.serialize_fixed();
        let src_width_bytes = src_width.serialize_fixed();
        let src_height_bytes = src_height.serialize_fixed();
        let dst_x_bytes = dst_x.serialize_fixed();
        let dst_y_bytes = dst_y.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                41,
                0,
                length[0],
                length[1],
                src_window_bytes[0],
                src_window_bytes[1],
                src_window_bytes[2],
                src_window_bytes[3],
                dst_window_bytes[0],
                dst_window_bytes[1],
                dst_window_bytes[2],
                dst_window_bytes[3],
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
            ]);
        self.advance_writer(24);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn set_input_focus(
        &mut self,
        socket_buffer: &mut [u8],
        revert_to: crate::proto::xproto::InputFocusEnum,
        focus: crate::proto::xproto::InputFocusEnum,
        time: crate::proto::xproto::TimeEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let focus_bytes = (focus.0 as u32).serialize_fixed();
        let time_bytes = (time.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                42,
                revert_to.0 as u8,
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
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_input_focus(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GetInputFocusReply, 12>> {
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = 43;
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

    fn query_keymap(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::QueryKeymapReply, 40>> {
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = 44;
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

    fn open_font(
        &mut self,
        socket_buffer: &mut [u8],
        fid: crate::proto::xproto::Font,
        name: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        let name_len = u16::try_from(name.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&fid.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(name_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            name,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[45, 0]);
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

    fn close_font(
        &mut self,
        socket_buffer: &mut [u8],
        font: crate::proto::xproto::Font,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let font_bytes = font.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                46,
                0,
                length[0],
                length[1],
                font_bytes[0],
                font_bytes[1],
                font_bytes[2],
                font_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn query_font(
        &mut self,
        socket_buffer: &mut [u8],
        font: crate::proto::xproto::Fontable,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::QueryFontReply>> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let font_bytes = font.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                47,
                0,
                length[0],
                length[1],
                font_bytes[0],
                font_bytes[1],
                font_bytes[2],
                font_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn query_text_extents(
        &mut self,
        socket_buffer: &mut [u8],
        font: crate::proto::xproto::Fontable,
        string: &[crate::proto::xproto::Char2b],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::QueryTextExtentsReply, 28>> {
        let buf_ptr = self.apply_offset(socket_buffer);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&font.serialize_fixed());
        let list_len = string.len() * 2;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            string,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[48, 0]);
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

    fn list_fonts(
        &mut self,
        socket_buffer: &mut [u8],
        max_names: u16,
        pattern: &[u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::ListFontsReply>> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        let pattern_len =
            u16::try_from(pattern.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&max_names.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(pattern_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = pattern.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            pattern,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[49, 0]);
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

    fn list_fonts_with_info(
        &mut self,
        socket_buffer: &mut [u8],
        max_names: u16,
        pattern: &[u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::ListFontsWithInfoReply>> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        let pattern_len =
            u16::try_from(pattern.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&max_names.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(pattern_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = pattern.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            pattern,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[50, 0]);
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

    fn set_font_path(
        &mut self,
        socket_buffer: &mut [u8],
        font: alloc::vec::Vec<crate::proto::xproto::Str>,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        let font_qty = u16::try_from(font.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(font_qty).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let mut offset = crate::util::var_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            font,
        )?;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[51, 0]);
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

    fn get_font_path(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::GetFontPathReply>> {
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = 52;
        buf[1] = 0;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
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
        depth: u8,
        pid: crate::proto::xproto::Pixmap,
        drawable: crate::proto::xproto::Drawable,
        width: u16,
        height: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let pid_bytes = pid.serialize_fixed();
        let drawable_bytes = drawable.serialize_fixed();
        let width_bytes = width.serialize_fixed();
        let height_bytes = height.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                53,
                depth,
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
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn free_pixmap(
        &mut self,
        socket_buffer: &mut [u8],
        pixmap: crate::proto::xproto::Pixmap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let pixmap_bytes = pixmap.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                54,
                0,
                length[0],
                length[1],
                pixmap_bytes[0],
                pixmap_bytes[1],
                pixmap_bytes[2],
                pixmap_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn copy_g_c(
        &mut self,
        socket_buffer: &mut [u8],
        src_gc: crate::proto::xproto::Gcontext,
        dst_gc: crate::proto::xproto::Gcontext,
        value_mask: crate::proto::xproto::Gc,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let src_gc_bytes = src_gc.serialize_fixed();
        let dst_gc_bytes = dst_gc.serialize_fixed();
        let value_mask_bytes = (value_mask.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                57,
                0,
                length[0],
                length[1],
                src_gc_bytes[0],
                src_gc_bytes[1],
                src_gc_bytes[2],
                src_gc_bytes[3],
                dst_gc_bytes[0],
                dst_gc_bytes[1],
                dst_gc_bytes[2],
                dst_gc_bytes[3],
                value_mask_bytes[0],
                value_mask_bytes[1],
                value_mask_bytes[2],
                value_mask_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn set_dashes(
        &mut self,
        socket_buffer: &mut [u8],
        gc: crate::proto::xproto::Gcontext,
        dash_offset: u16,
        dashes: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        let dashes_len = u16::try_from(dashes.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&gc.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&dash_offset.serialize_fixed());
        buf_ptr
            .get_mut(10..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(dashes_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = dashes.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            dashes,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[58, 0]);
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

    fn set_clip_rectangles(
        &mut self,
        socket_buffer: &mut [u8],
        ordering: crate::proto::xproto::ClipOrderingEnum,
        gc: crate::proto::xproto::Gcontext,
        clip_x_origin: i16,
        clip_y_origin: i16,
        rectangles: &[crate::proto::xproto::Rectangle],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&gc.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&clip_x_origin.serialize_fixed());
        buf_ptr
            .get_mut(10..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&clip_y_origin.serialize_fixed());
        let list_len = rectangles.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            rectangles,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[59, ordering.0]);
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

    fn free_g_c(
        &mut self,
        socket_buffer: &mut [u8],
        gc: crate::proto::xproto::Gcontext,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let gc_bytes = gc.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                60,
                0,
                length[0],
                length[1],
                gc_bytes[0],
                gc_bytes[1],
                gc_bytes[2],
                gc_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn clear_area(
        &mut self,
        socket_buffer: &mut [u8],
        exposures: u8,
        window: crate::proto::xproto::Window,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let x_bytes = x.serialize_fixed();
        let y_bytes = y.serialize_fixed();
        let width_bytes = width.serialize_fixed();
        let height_bytes = height.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                61,
                exposures,
                length[0],
                length[1],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                x_bytes[0],
                x_bytes[1],
                y_bytes[0],
                y_bytes[1],
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
        Ok(VoidCookie::new(seq))
    }

    fn copy_area(
        &mut self,
        socket_buffer: &mut [u8],
        src_drawable: crate::proto::xproto::Drawable,
        dst_drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        src_x: i16,
        src_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (7u16).to_ne_bytes();
        let src_drawable_bytes = src_drawable.serialize_fixed();
        let dst_drawable_bytes = dst_drawable.serialize_fixed();
        let gc_bytes = gc.serialize_fixed();
        let src_x_bytes = src_x.serialize_fixed();
        let src_y_bytes = src_y.serialize_fixed();
        let dst_x_bytes = dst_x.serialize_fixed();
        let dst_y_bytes = dst_y.serialize_fixed();
        let width_bytes = width.serialize_fixed();
        let height_bytes = height.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                62,
                0,
                length[0],
                length[1],
                src_drawable_bytes[0],
                src_drawable_bytes[1],
                src_drawable_bytes[2],
                src_drawable_bytes[3],
                dst_drawable_bytes[0],
                dst_drawable_bytes[1],
                dst_drawable_bytes[2],
                dst_drawable_bytes[3],
                gc_bytes[0],
                gc_bytes[1],
                gc_bytes[2],
                gc_bytes[3],
                src_x_bytes[0],
                src_x_bytes[1],
                src_y_bytes[0],
                src_y_bytes[1],
                dst_x_bytes[0],
                dst_x_bytes[1],
                dst_y_bytes[0],
                dst_y_bytes[1],
                width_bytes[0],
                width_bytes[1],
                height_bytes[0],
                height_bytes[1],
            ]);
        self.advance_writer(28);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn copy_plane(
        &mut self,
        socket_buffer: &mut [u8],
        src_drawable: crate::proto::xproto::Drawable,
        dst_drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        src_x: i16,
        src_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
        bit_plane: u32,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (8u16).to_ne_bytes();
        let src_drawable_bytes = src_drawable.serialize_fixed();
        let dst_drawable_bytes = dst_drawable.serialize_fixed();
        let gc_bytes = gc.serialize_fixed();
        let src_x_bytes = src_x.serialize_fixed();
        let src_y_bytes = src_y.serialize_fixed();
        let dst_x_bytes = dst_x.serialize_fixed();
        let dst_y_bytes = dst_y.serialize_fixed();
        let width_bytes = width.serialize_fixed();
        let height_bytes = height.serialize_fixed();
        let bit_plane_bytes = bit_plane.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                63,
                0,
                length[0],
                length[1],
                src_drawable_bytes[0],
                src_drawable_bytes[1],
                src_drawable_bytes[2],
                src_drawable_bytes[3],
                dst_drawable_bytes[0],
                dst_drawable_bytes[1],
                dst_drawable_bytes[2],
                dst_drawable_bytes[3],
                gc_bytes[0],
                gc_bytes[1],
                gc_bytes[2],
                gc_bytes[3],
                src_x_bytes[0],
                src_x_bytes[1],
                src_y_bytes[0],
                src_y_bytes[1],
                dst_x_bytes[0],
                dst_x_bytes[1],
                dst_y_bytes[0],
                dst_y_bytes[1],
                width_bytes[0],
                width_bytes[1],
                height_bytes[0],
                height_bytes[1],
                bit_plane_bytes[0],
                bit_plane_bytes[1],
                bit_plane_bytes[2],
                bit_plane_bytes[3],
            ]);
        self.advance_writer(32);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn poly_point(
        &mut self,
        socket_buffer: &mut [u8],
        coordinate_mode: crate::proto::xproto::CoordModeEnum,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        points: &[crate::proto::xproto::Point],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&gc.serialize_fixed());
        let list_len = points.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            points,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[64, coordinate_mode.0]);
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

    fn poly_line(
        &mut self,
        socket_buffer: &mut [u8],
        coordinate_mode: crate::proto::xproto::CoordModeEnum,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        points: &[crate::proto::xproto::Point],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&gc.serialize_fixed());
        let list_len = points.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            points,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[65, coordinate_mode.0]);
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

    fn poly_segment(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        segments: &[crate::proto::xproto::Segment],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&gc.serialize_fixed());
        let list_len = segments.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            segments,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[66, 0]);
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

    fn poly_rectangle(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        rectangles: &[crate::proto::xproto::Rectangle],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&gc.serialize_fixed());
        let list_len = rectangles.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            rectangles,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[67, 0]);
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

    fn poly_arc(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        arcs: &[crate::proto::xproto::Arc],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&gc.serialize_fixed());
        let list_len = arcs.len() * 12;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            arcs,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[68, 0]);
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

    fn fill_poly(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        shape: crate::proto::xproto::PolyShapeEnum,
        coordinate_mode: crate::proto::xproto::CoordModeEnum,
        points: &[crate::proto::xproto::Point],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&gc.serialize_fixed());
        buf_ptr
            .get_mut(12..13)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&shape.serialize_fixed());
        buf_ptr
            .get_mut(13..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&coordinate_mode.serialize_fixed());
        let list_len = points.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            points,
        )?;
        let mut offset = list_len + 16;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[69, 0]);
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

    fn poly_fill_rectangle(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        rectangles: &[crate::proto::xproto::Rectangle],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&gc.serialize_fixed());
        let list_len = rectangles.len() * 8;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            rectangles,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[70, 0]);
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

    fn poly_fill_arc(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        arcs: &[crate::proto::xproto::Arc],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&gc.serialize_fixed());
        let list_len = arcs.len() * 12;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            arcs,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[71, 0]);
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

    fn put_image(
        &mut self,
        socket_buffer: &mut [u8],
        format: crate::proto::xproto::ImageFormatEnum,
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        width: u16,
        height: u16,
        dst_x: i16,
        dst_y: i16,
        left_pad: u8,
        depth: u8,
        data: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&gc.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&width.serialize_fixed());
        buf_ptr
            .get_mut(14..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&height.serialize_fixed());
        buf_ptr
            .get_mut(16..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&dst_x.serialize_fixed());
        buf_ptr
            .get_mut(18..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&dst_y.serialize_fixed());
        buf_ptr
            .get_mut(20..21)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&left_pad.serialize_fixed());
        buf_ptr
            .get_mut(21..22)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&depth.serialize_fixed());
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
            .copy_from_slice(&[72, format.0]);
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

    fn get_image(
        &mut self,
        socket_buffer: &mut [u8],
        format: crate::proto::xproto::ImageFormatEnum,
        drawable: crate::proto::xproto::Drawable,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        plane_mask: u32,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::GetImageReply>> {
        let length: [u8; 2] = (5u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let x_bytes = x.serialize_fixed();
        let y_bytes = y.serialize_fixed();
        let width_bytes = width.serialize_fixed();
        let height_bytes = height.serialize_fixed();
        let plane_mask_bytes = plane_mask.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                73,
                format.0 as u8,
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
            ]);
        self.advance_writer(20);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn poly_text8(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        x: i16,
        y: i16,
        items: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&gc.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&x.serialize_fixed());
        buf_ptr
            .get_mut(14..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&y.serialize_fixed());
        let list_len = items.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            items,
        )?;
        let mut offset = list_len + 16;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[74, 0]);
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

    fn poly_text16(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        x: i16,
        y: i16,
        items: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&gc.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&x.serialize_fixed());
        buf_ptr
            .get_mut(14..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&y.serialize_fixed());
        let list_len = items.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            items,
        )?;
        let mut offset = list_len + 16;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[75, 0]);
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

    fn image_text8(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        x: i16,
        y: i16,
        string: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        let string_len = u8::try_from(string.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(3..7)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(7..11)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&gc.serialize_fixed());
        buf_ptr
            .get_mut(11..13)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&x.serialize_fixed());
        buf_ptr
            .get_mut(13..15)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&y.serialize_fixed());
        let list_len = string.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(15..)
                .ok_or(crate::error::Error::Serialize)?,
            string,
        )?;
        let mut offset = list_len + 15;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[76, string_len]);
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

    fn image_text16(
        &mut self,
        socket_buffer: &mut [u8],
        drawable: crate::proto::xproto::Drawable,
        gc: crate::proto::xproto::Gcontext,
        x: i16,
        y: i16,
        string: &[crate::proto::xproto::Char2b],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        let string_len = u8::try_from(string.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(3..7)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(7..11)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&gc.serialize_fixed());
        buf_ptr
            .get_mut(11..13)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&x.serialize_fixed());
        buf_ptr
            .get_mut(13..15)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&y.serialize_fixed());
        let list_len = string.len() * 2;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(15..)
                .ok_or(crate::error::Error::Serialize)?,
            string,
        )?;
        let mut offset = list_len + 15;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[77, string_len]);
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

    fn create_colormap(
        &mut self,
        socket_buffer: &mut [u8],
        alloc: crate::proto::xproto::ColormapAllocEnum,
        mid: crate::proto::xproto::Colormap,
        window: crate::proto::xproto::Window,
        visual: crate::proto::xproto::Visualid,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let mid_bytes = mid.serialize_fixed();
        let window_bytes = window.serialize_fixed();
        let visual_bytes = visual.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                78,
                alloc.0 as u8,
                length[0],
                length[1],
                mid_bytes[0],
                mid_bytes[1],
                mid_bytes[2],
                mid_bytes[3],
                window_bytes[0],
                window_bytes[1],
                window_bytes[2],
                window_bytes[3],
                visual_bytes[0],
                visual_bytes[1],
                visual_bytes[2],
                visual_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn free_colormap(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let cmap_bytes = cmap.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                79,
                0,
                length[0],
                length[1],
                cmap_bytes[0],
                cmap_bytes[1],
                cmap_bytes[2],
                cmap_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn copy_colormap_and_free(
        &mut self,
        socket_buffer: &mut [u8],
        mid: crate::proto::xproto::Colormap,
        src_cmap: crate::proto::xproto::Colormap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let mid_bytes = mid.serialize_fixed();
        let src_cmap_bytes = src_cmap.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                80,
                0,
                length[0],
                length[1],
                mid_bytes[0],
                mid_bytes[1],
                mid_bytes[2],
                mid_bytes[3],
                src_cmap_bytes[0],
                src_cmap_bytes[1],
                src_cmap_bytes[2],
                src_cmap_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn install_colormap(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let cmap_bytes = cmap.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                81,
                0,
                length[0],
                length[1],
                cmap_bytes[0],
                cmap_bytes[1],
                cmap_bytes[2],
                cmap_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn uninstall_colormap(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let cmap_bytes = cmap.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                82,
                0,
                length[0],
                length[1],
                cmap_bytes[0],
                cmap_bytes[1],
                cmap_bytes[2],
                cmap_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn list_installed_colormaps(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::ListInstalledColormapsReply>> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let window_bytes = window.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                83,
                0,
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

    fn alloc_color(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        red: u16,
        green: u16,
        blue: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::AllocColorReply, 20>> {
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let cmap_bytes = cmap.serialize_fixed();
        let red_bytes = red.serialize_fixed();
        let green_bytes = green.serialize_fixed();
        let blue_bytes = blue.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                84,
                0,
                length[0],
                length[1],
                cmap_bytes[0],
                cmap_bytes[1],
                cmap_bytes[2],
                cmap_bytes[3],
                red_bytes[0],
                red_bytes[1],
                green_bytes[0],
                green_bytes[1],
                blue_bytes[0],
                blue_bytes[1],
                0,
                0,
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn alloc_named_color(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        name: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::AllocNamedColorReply, 24>> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        let name_len = u16::try_from(name.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&cmap.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(name_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            name,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[85, 0]);
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

    fn alloc_color_cells(
        &mut self,
        socket_buffer: &mut [u8],
        contiguous: u8,
        cmap: crate::proto::xproto::Colormap,
        colors: u16,
        planes: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::AllocColorCellsReply>> {
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let cmap_bytes = cmap.serialize_fixed();
        let colors_bytes = colors.serialize_fixed();
        let planes_bytes = planes.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                86,
                contiguous,
                length[0],
                length[1],
                cmap_bytes[0],
                cmap_bytes[1],
                cmap_bytes[2],
                cmap_bytes[3],
                colors_bytes[0],
                colors_bytes[1],
                planes_bytes[0],
                planes_bytes[1],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn alloc_color_planes(
        &mut self,
        socket_buffer: &mut [u8],
        contiguous: u8,
        cmap: crate::proto::xproto::Colormap,
        colors: u16,
        reds: u16,
        greens: u16,
        blues: u16,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::AllocColorPlanesReply>> {
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let cmap_bytes = cmap.serialize_fixed();
        let colors_bytes = colors.serialize_fixed();
        let reds_bytes = reds.serialize_fixed();
        let greens_bytes = greens.serialize_fixed();
        let blues_bytes = blues.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                87,
                contiguous,
                length[0],
                length[1],
                cmap_bytes[0],
                cmap_bytes[1],
                cmap_bytes[2],
                cmap_bytes[3],
                colors_bytes[0],
                colors_bytes[1],
                reds_bytes[0],
                reds_bytes[1],
                greens_bytes[0],
                greens_bytes[1],
                blues_bytes[0],
                blues_bytes[1],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn free_colors(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        plane_mask: u32,
        pixels: &[u32],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&cmap.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&plane_mask.serialize_fixed());
        let list_len = pixels.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            pixels,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[88, 0]);
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

    fn store_colors(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        items: &[crate::proto::xproto::Coloritem],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&cmap.serialize_fixed());
        let list_len = items.len() * 12;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            items,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[89, 0]);
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

    fn store_named_color(
        &mut self,
        socket_buffer: &mut [u8],
        flags: crate::proto::xproto::ColorFlag,
        cmap: crate::proto::xproto::Colormap,
        pixel: u32,
        name: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        let name_len = u16::try_from(name.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&cmap.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&pixel.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(name_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(16..)
                .ok_or(crate::error::Error::Serialize)?,
            name,
        )?;
        let mut offset = list_len + 16;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[90, flags.0]);
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

    fn query_colors(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        pixels: &[u32],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::QueryColorsReply>> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&cmap.serialize_fixed());
        let list_len = pixels.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            pixels,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[91, 0]);
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

    fn lookup_color(
        &mut self,
        socket_buffer: &mut [u8],
        cmap: crate::proto::xproto::Colormap,
        name: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::LookupColorReply, 20>> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        let name_len = u16::try_from(name.len()).map_err(|_| crate::error::Error::Serialize)?;
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&cmap.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(name_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = name.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            name,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[92, 0]);
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

    fn create_cursor(
        &mut self,
        socket_buffer: &mut [u8],
        cid: crate::proto::xproto::Cursor,
        source: crate::proto::xproto::Pixmap,
        mask: crate::proto::xproto::PixmapEnum,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
        x: u16,
        y: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (8u16).to_ne_bytes();
        let cid_bytes = cid.serialize_fixed();
        let source_bytes = source.serialize_fixed();
        let mask_bytes = (mask.0 as u32).serialize_fixed();
        let fore_red_bytes = fore_red.serialize_fixed();
        let fore_green_bytes = fore_green.serialize_fixed();
        let fore_blue_bytes = fore_blue.serialize_fixed();
        let back_red_bytes = back_red.serialize_fixed();
        let back_green_bytes = back_green.serialize_fixed();
        let back_blue_bytes = back_blue.serialize_fixed();
        let x_bytes = x.serialize_fixed();
        let y_bytes = y.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                93,
                0,
                length[0],
                length[1],
                cid_bytes[0],
                cid_bytes[1],
                cid_bytes[2],
                cid_bytes[3],
                source_bytes[0],
                source_bytes[1],
                source_bytes[2],
                source_bytes[3],
                mask_bytes[0],
                mask_bytes[1],
                mask_bytes[2],
                mask_bytes[3],
                fore_red_bytes[0],
                fore_red_bytes[1],
                fore_green_bytes[0],
                fore_green_bytes[1],
                fore_blue_bytes[0],
                fore_blue_bytes[1],
                back_red_bytes[0],
                back_red_bytes[1],
                back_green_bytes[0],
                back_green_bytes[1],
                back_blue_bytes[0],
                back_blue_bytes[1],
                x_bytes[0],
                x_bytes[1],
                y_bytes[0],
                y_bytes[1],
            ]);
        self.advance_writer(32);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn create_glyph_cursor(
        &mut self,
        socket_buffer: &mut [u8],
        cid: crate::proto::xproto::Cursor,
        source_font: crate::proto::xproto::Font,
        mask_font: crate::proto::xproto::FontEnum,
        source_char: u16,
        mask_char: u16,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (8u16).to_ne_bytes();
        let cid_bytes = cid.serialize_fixed();
        let source_font_bytes = source_font.serialize_fixed();
        let mask_font_bytes = (mask_font.0 as u32).serialize_fixed();
        let source_char_bytes = source_char.serialize_fixed();
        let mask_char_bytes = mask_char.serialize_fixed();
        let fore_red_bytes = fore_red.serialize_fixed();
        let fore_green_bytes = fore_green.serialize_fixed();
        let fore_blue_bytes = fore_blue.serialize_fixed();
        let back_red_bytes = back_red.serialize_fixed();
        let back_green_bytes = back_green.serialize_fixed();
        let back_blue_bytes = back_blue.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                94,
                0,
                length[0],
                length[1],
                cid_bytes[0],
                cid_bytes[1],
                cid_bytes[2],
                cid_bytes[3],
                source_font_bytes[0],
                source_font_bytes[1],
                source_font_bytes[2],
                source_font_bytes[3],
                mask_font_bytes[0],
                mask_font_bytes[1],
                mask_font_bytes[2],
                mask_font_bytes[3],
                source_char_bytes[0],
                source_char_bytes[1],
                mask_char_bytes[0],
                mask_char_bytes[1],
                fore_red_bytes[0],
                fore_red_bytes[1],
                fore_green_bytes[0],
                fore_green_bytes[1],
                fore_blue_bytes[0],
                fore_blue_bytes[1],
                back_red_bytes[0],
                back_red_bytes[1],
                back_green_bytes[0],
                back_green_bytes[1],
                back_blue_bytes[0],
                back_blue_bytes[1],
            ]);
        self.advance_writer(32);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn free_cursor(
        &mut self,
        socket_buffer: &mut [u8],
        cursor: crate::proto::xproto::Cursor,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let cursor_bytes = cursor.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                95,
                0,
                length[0],
                length[1],
                cursor_bytes[0],
                cursor_bytes[1],
                cursor_bytes[2],
                cursor_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn recolor_cursor(
        &mut self,
        socket_buffer: &mut [u8],
        cursor: crate::proto::xproto::Cursor,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (5u16).to_ne_bytes();
        let cursor_bytes = cursor.serialize_fixed();
        let fore_red_bytes = fore_red.serialize_fixed();
        let fore_green_bytes = fore_green.serialize_fixed();
        let fore_blue_bytes = fore_blue.serialize_fixed();
        let back_red_bytes = back_red.serialize_fixed();
        let back_green_bytes = back_green.serialize_fixed();
        let back_blue_bytes = back_blue.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                96,
                0,
                length[0],
                length[1],
                cursor_bytes[0],
                cursor_bytes[1],
                cursor_bytes[2],
                cursor_bytes[3],
                fore_red_bytes[0],
                fore_red_bytes[1],
                fore_green_bytes[0],
                fore_green_bytes[1],
                fore_blue_bytes[0],
                fore_blue_bytes[1],
                back_red_bytes[0],
                back_red_bytes[1],
                back_green_bytes[0],
                back_green_bytes[1],
                back_blue_bytes[0],
                back_blue_bytes[1],
            ]);
        self.advance_writer(20);
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
        class: crate::proto::xproto::QueryShapeOfEnum,
        drawable: crate::proto::xproto::Drawable,
        width: u16,
        height: u16,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::QueryBestSizeReply, 12>> {
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let width_bytes = width.serialize_fixed();
        let height_bytes = height.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                97,
                class.0 as u8,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                width_bytes[0],
                width_bytes[1],
                height_bytes[0],
                height_bytes[1],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn query_extension(
        &mut self,
        socket_buffer: &mut [u8],
        name: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::QueryExtensionReply, 12>> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
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
            .copy_from_slice(&[98, 0]);
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

    fn list_extensions(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::ListExtensionsReply>> {
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = 99;
        buf[1] = 0;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn change_keyboard_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        keycode_count: u8,
        first_keycode: crate::proto::xproto::Keycode,
        keysyms_per_keycode: u8,
        keysyms: &[crate::proto::xproto::Keysym],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 2 bytes
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&first_keycode.serialize_fixed());
        buf_ptr
            .get_mut(5..6)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&keysyms_per_keycode.serialize_fixed());
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
            .copy_from_slice(&[100, keycode_count]);
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

    fn get_keyboard_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        first_keycode: crate::proto::xproto::Keycode,
        count: u8,
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::GetKeyboardMappingReply>> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let first_keycode_bytes = first_keycode.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                101,
                0,
                length[0],
                length[1],
                first_keycode_bytes[0],
                count,
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

    fn change_keyboard_control(
        &mut self,
        socket_buffer: &mut [u8],
        change_keyboard_control_value_list: crate::proto::xproto::ChangeKeyboardControlValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[102, 0]);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &change_keyboard_control_value_list
                    .switch_expr()
                    .serialize_fixed(),
            );
        let mut offset = 8 + change_keyboard_control_value_list
            .serialize_into(buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?)?;
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

    fn get_keyboard_control(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GetKeyboardControlReply, 52>> {
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = 103;
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

    fn bell(
        &mut self,
        socket_buffer: &mut [u8],
        percent: i8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (1u16).to_ne_bytes();
        let percent_bytes = percent.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[104, percent_bytes[0], length[0], length[1]]);
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn change_pointer_control(
        &mut self,
        socket_buffer: &mut [u8],
        acceleration_numerator: i16,
        acceleration_denominator: i16,
        threshold: i16,
        do_acceleration: u8,
        do_threshold: u8,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let acceleration_numerator_bytes = acceleration_numerator.serialize_fixed();
        let acceleration_denominator_bytes = acceleration_denominator.serialize_fixed();
        let threshold_bytes = threshold.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                105,
                0,
                length[0],
                length[1],
                acceleration_numerator_bytes[0],
                acceleration_numerator_bytes[1],
                acceleration_denominator_bytes[0],
                acceleration_denominator_bytes[1],
                threshold_bytes[0],
                threshold_bytes[1],
                do_acceleration,
                do_threshold,
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn get_pointer_control(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GetPointerControlReply, 32>> {
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = 106;
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

    fn set_screen_saver(
        &mut self,
        socket_buffer: &mut [u8],
        timeout: i16,
        interval: i16,
        prefer_blanking: crate::proto::xproto::BlankingEnum,
        allow_exposures: crate::proto::xproto::ExposuresEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let timeout_bytes = timeout.serialize_fixed();
        let interval_bytes = interval.serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                107,
                0,
                length[0],
                length[1],
                timeout_bytes[0],
                timeout_bytes[1],
                interval_bytes[0],
                interval_bytes[1],
                prefer_blanking.0 as u8,
                allow_exposures.0 as u8,
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

    fn get_screen_saver(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::GetScreenSaverReply, 32>> {
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = 108;
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

    fn change_hosts(
        &mut self,
        socket_buffer: &mut [u8],
        mode: crate::proto::xproto::HostModeEnum,
        family: crate::proto::xproto::FamilyEnum,
        address: &[u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        let address_len =
            u16::try_from(address.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..5)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&family.serialize_fixed());
        buf_ptr
            .get_mut(6..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(address_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        let list_len = address.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(8..).ok_or(crate::error::Error::Serialize)?,
            address,
        )?;
        let mut offset = list_len + 8;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[109, mode.0]);
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

    fn set_access_control(
        &mut self,
        socket_buffer: &mut [u8],
        mode: crate::proto::xproto::AccessControlEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (1u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[111, mode.0 as u8, length[0], length[1]]);
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn set_close_down_mode(
        &mut self,
        socket_buffer: &mut [u8],
        mode: crate::proto::xproto::CloseDownEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (1u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[112, mode.0 as u8, length[0], length[1]]);
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn kill_client(
        &mut self,
        socket_buffer: &mut [u8],
        resource: crate::proto::xproto::KillEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let resource_bytes = (resource.0 as u32).serialize_fixed();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                113,
                0,
                length[0],
                length[1],
                resource_bytes[0],
                resource_bytes[1],
                resource_bytes[2],
                resource_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn rotate_properties(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        delta: i16,
        atoms: &[crate::proto::xproto::Atom],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        let atoms_len = u16::try_from(atoms.len()).map_err(|_| crate::error::Error::Serialize)?;
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&window.serialize_fixed());
        buf_ptr
            .get_mut(8..10)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &(u16::try_from(atoms_len).map_err(|_| crate::error::Error::Serialize)?)
                    .serialize_fixed(),
            );
        buf_ptr
            .get_mut(10..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&delta.serialize_fixed());
        let list_len = atoms.len() * 4;
        crate::util::fixed_vec_serialize_into(
            buf_ptr
                .get_mut(12..)
                .ok_or(crate::error::Error::Serialize)?,
            atoms,
        )?;
        let mut offset = list_len + 12;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[114, 0]);
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

    fn force_screen_saver(
        &mut self,
        socket_buffer: &mut [u8],
        mode: crate::proto::xproto::ScreenSaverEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let length: [u8; 2] = (1u16).to_ne_bytes();
        let buf = self.apply_offset(socket_buffer);
        buf.get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[115, mode.0 as u8, length[0], length[1]]);
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn set_pointer_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        map: &[u8],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::SetPointerMappingReply, 8>> {
        let buf_ptr = self.apply_offset(socket_buffer);
        let map_len = u8::try_from(map.len()).map_err(|_| crate::error::Error::Serialize)?;
        let list_len = map.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(0..).ok_or(crate::error::Error::Serialize)?,
            map,
        )?;
        let mut offset = list_len;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[116, map_len]);
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

    fn get_pointer_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::GetPointerMappingReply>> {
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = 117;
        buf[1] = 0;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn set_modifier_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        keycodes_per_modifier: u8,
        keycodes: &[crate::proto::xproto::Keycode],
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::xproto::SetModifierMappingReply, 8>> {
        let buf_ptr = self.apply_offset(socket_buffer);
        let keycodes_per_modifier =
            u8::try_from(keycodes_per_modifier).map_err(|_| crate::error::Error::Serialize)?;
        let list_len = keycodes.len();
        crate::util::u8_vec_serialize_into(
            buf_ptr.get_mut(0..).ok_or(crate::error::Error::Serialize)?,
            keycodes,
        )?;
        let mut offset = list_len;
        let mut offset = offset + (4 - (offset % 4)) % 4;
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[118, keycodes_per_modifier]);
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

    fn get_modifier_mapping(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::GetModifierMappingReply>> {
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = 119;
        buf[1] = 0;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }

    fn no_operation(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = 127;
        buf[1] = 0;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
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
        depth: u8,
        wid: crate::proto::xproto::Window,
        parent: crate::proto::xproto::Window,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: crate::proto::xproto::WindowClassEnum,
        visual: crate::proto::xproto::Visualid,
        create_window_value_list: crate::proto::xproto::CreateWindowValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[1, depth]);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&wid.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&parent.serialize_fixed());
        buf_ptr
            .get_mut(12..14)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&x.serialize_fixed());
        buf_ptr
            .get_mut(14..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&y.serialize_fixed());
        buf_ptr
            .get_mut(16..18)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&width.serialize_fixed());
        buf_ptr
            .get_mut(18..20)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&height.serialize_fixed());
        buf_ptr
            .get_mut(20..22)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&border_width.serialize_fixed());
        buf_ptr
            .get_mut(22..24)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&class.serialize_fixed());
        buf_ptr
            .get_mut(24..28)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&visual.serialize_fixed());
        buf_ptr
            .get_mut(28..32)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&create_window_value_list.switch_expr().serialize_fixed());
        let mut offset = 32
            + create_window_value_list.serialize_into(
                buf_ptr
                    .get_mut(32..)
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

    fn change_window_attributes(
        &mut self,
        socket_buffer: &mut [u8],
        window: crate::proto::xproto::Window,
        change_window_attributes_value_list: crate::proto::xproto::ChangeWindowAttributesValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[2, 0]);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&window.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(
                &change_window_attributes_value_list
                    .switch_expr()
                    .serialize_fixed(),
            );
        let mut offset = 12
            + change_window_attributes_value_list.serialize_into(
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

    fn create_g_c(
        &mut self,
        socket_buffer: &mut [u8],
        cid: crate::proto::xproto::Gcontext,
        drawable: crate::proto::xproto::Drawable,
        create_g_c_value_list: crate::proto::xproto::CreateGCValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[55, 0]);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&cid.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&drawable.serialize_fixed());
        buf_ptr
            .get_mut(12..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&create_g_c_value_list.switch_expr().serialize_fixed());
        let mut offset = 16
            + create_g_c_value_list.serialize_into(
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
        self.advance_writer(offset);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn change_g_c(
        &mut self,
        socket_buffer: &mut [u8],
        gc: crate::proto::xproto::Gcontext,
        change_g_c_value_list: crate::proto::xproto::ChangeGCValueList,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let buf_ptr = self.apply_offset(socket_buffer);
        // Pad 1 bytes
        buf_ptr
            .get_mut(0..2)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[56, 0]);
        buf_ptr
            .get_mut(4..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&gc.serialize_fixed());
        buf_ptr
            .get_mut(8..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&change_g_c_value_list.switch_expr().serialize_fixed());
        let mut offset = 12
            + change_g_c_value_list.serialize_into(
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

    fn list_hosts(
        &mut self,
        socket_buffer: &mut [u8],
        forget: bool,
    ) -> crate::error::Result<Cookie<crate::proto::xproto::ListHostsReply>> {
        let buf = self
            .apply_offset(socket_buffer)
            .get_mut(..4)
            .ok_or(crate::error::Error::Serialize)?;
        buf[0] = 110;
        buf[1] = 0;
        buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        self.advance_writer(4);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(Cookie::new(seq))
    }
}
