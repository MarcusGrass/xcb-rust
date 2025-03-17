//! Utility functions for working with X11 cursors
//!
//! The code in this module is only available when the `cursor` feature of the library is enabled.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::con::{SocketIo, XcbState};
use crate::connection::render::{create_cursor, create_picture, free_picture};
use crate::connection::xproto::{create_pixmap, put_image};
use crate::helpers::cursor::parse_cursor::VecBuffer;
use crate::helpers::resource_manager::protocol::Database;
use crate::helpers::XcbEnv;
use crate::proto::render::{CreatePictureValueList, Pictformat};
use crate::proto::xproto::{CreateGCValueList, Font, FontEnum, Window};
use crate::proto::{render, xproto};
use crate::{Error, NONE};

mod find_cursor;
mod parse_cursor;

/// The level of cursor support of the X11 server
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RenderSupport {
    /// Render extension not available
    None,

    /// Static cursor support (`CreateCursor` added in RENDER 0.5)
    StaticCursor,

    /// Animated cursor support (`CreateAnimCursor` added in RENDER 0.8)
    AnimatedCursor,
}

/// A handle necessary for loading cursors
#[derive(Debug)]
pub struct Handle {
    root: Window,
    cursor_font: Font,
    picture_format: Pictformat,
    render_support: RenderSupport,
    theme: Option<String>,
    cursor_size: u32,
}

impl Handle {
    /// Create a new cursor handle for creating cursors on the given screen.
    ///
    /// The `resource_database` is used to look up settings like the current cursor theme and the
    /// cursor size to use.
    ///
    /// This function returns a cookie that can be used to later get the actual handle.
    ///
    /// If you want this function not to block, you should prefetch the RENDER extension's data on
    /// the connection.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<IO: SocketIo, XS: XcbState>(
        io: &mut IO,
        xcb_state: &mut XS,
        screen: usize,
        resource_database: &Database,
        env: XcbEnv,
    ) -> Result<Self, Error> {
        let screen = xcb_state.setup().roots[screen].clone();
        let render_info = if xcb_state.major_opcode(render::EXTENSION_NAME).is_some() {
            let render_version =
                crate::connection::render::query_version(io, xcb_state, 0, 8, false)?;
            let render_pict_format =
                crate::connection::render::query_pict_formats(io, xcb_state, false)?;
            Some((render_version, render_pict_format))
        } else {
            None
        };
        let mut render_version = (0, 0);
        let mut picture_format = NONE;
        if let Some((version, formats)) = render_info {
            let version = version.reply(io, xcb_state)?;
            render_version = (version.major_version, version.minor_version);
            picture_format = find_format(&formats.reply(io, xcb_state)?);
        }
        let render_support = if render_version.0 >= 1 || render_version.1 >= 8 {
            RenderSupport::AnimatedCursor
        } else if render_version.0 >= 1 || render_version.1 >= 5 {
            RenderSupport::StaticCursor
        } else {
            RenderSupport::None
        };
        let theme = resource_database
            .get_string("Xcursor.theme", "")
            .map(alloc::string::ToString::to_string);
        let cursor_size = match resource_database.get_value("Xcursor.size", "") {
            Ok(Some(value)) => value,
            _ => 0,
        };
        let xft_dpi = match resource_database.get_value("Xft.dpi", "") {
            Ok(Some(value)) => value,
            _ => 0,
        };
        let cursor_size = get_cursor_size(cursor_size, xft_dpi, &screen, env);
        let cursor_font = xcb_state.generate_id(io)?;

        let _ = crate::connection::xproto::open_font(io, xcb_state, cursor_font, b"cursor", true)?;
        Ok(Handle {
            root: screen.root,
            cursor_font,
            picture_format,
            render_support,
            theme,
            cursor_size,
        })
    }

    /// Loads the specified cursor, either from the cursor theme or by falling back to the X11
    /// "cursor" font.
    #[inline]
    pub fn load_cursor<IO, XS>(
        &self,
        io: &mut IO,
        state: &mut XS,
        name: &str,
        env: XcbEnv,
    ) -> Result<xproto::Cursor, Error>
    where
        IO: SocketIo,
        XS: XcbState,
    {
        load_cursor(io, state, self, name, env)
    }
}

fn open_cursor(
    theme: Option<&str>,
    name: &str,
    env: XcbEnv,
) -> Option<find_cursor::Cursor<Vec<u8>>> {
    if let Some(theme) = theme {
        if let Ok(cursor) = find_cursor::find_cursor(theme, name, env) {
            return Some(cursor);
        }
    }
    if let Ok(cursor) = find_cursor::find_cursor("default", name, env) {
        Some(cursor)
    } else {
        None
    }
}

fn create_core_cursor<IO, XS>(
    io: &mut IO,
    state: &mut XS,
    cursor_font: Font,
    cursor: u16,
) -> Result<xproto::Cursor, Error>
where
    IO: SocketIo,
    XS: XcbState,
{
    let result = state.generate_id(io)?;
    crate::connection::xproto::create_glyph_cursor(
        io,
        state,
        result,
        cursor_font,
        FontEnum(cursor_font),
        cursor,
        cursor + 1,
        // foreground color
        0,
        0,
        0,
        // background color
        u16::MAX,
        u16::MAX,
        u16::MAX,
        true,
    )?;
    Ok(result)
}

fn create_render_cursor<IO, XS>(
    io: &mut IO,
    state: &mut XS,
    handle: &Handle,
    image: &parse_cursor::Image,
    storage: &mut Option<(xproto::Pixmap, xproto::Gcontext, u16, u16)>,
) -> Result<render::Animcursorelt, Error>
where
    IO: SocketIo,
    XS: XcbState,
{
    let (cursor, picture) = (state.generate_id(io)?, state.generate_id(io)?);

    // Get a pixmap of the right size and a gc for it
    let (pixmap, gc) = if storage.map(|(_, _, w, h)| (w, h)) == Some((image.width, image.height)) {
        storage.map(|(pixmap, gc, _, _)| (pixmap, gc)).unwrap()
    } else {
        let (pixmap, gc) = if let Some((pixmap, gc, _, _)) = storage {
            crate::connection::xproto::free_g_c(io, state, *gc, true)?;
            crate::connection::xproto::free_pixmap(io, state, *pixmap, true)?;
            (*pixmap, *gc)
        } else {
            (state.generate_id(io)?, state.generate_id(io)?)
        };
        create_pixmap(
            io,
            state,
            32,
            pixmap,
            handle.root,
            image.width,
            image.height,
            true,
        )?;
        crate::connection::xproto::create_g_c(
            io,
            state,
            gc,
            pixmap,
            CreateGCValueList::default(),
            true,
        )?;

        *storage = Some((pixmap, gc, image.width, image.height));
        (pixmap, gc)
    };

    // Sigh. We need the pixel data as a bunch of bytes.
    let mut pixels = vec![0; image.pixels.len() * 4];
    crate::util::fixed_vec_serialize_into(&mut pixels, &image.pixels)?;
    let _ = put_image(
        io,
        state,
        xproto::ImageFormatEnum::Z_PIXMAP,
        pixmap,
        gc,
        image.width,
        image.height,
        0,
        0,
        0,
        32,
        &pixels,
        true,
    )?;

    let _ = create_picture(
        io,
        state,
        picture,
        pixmap,
        handle.picture_format,
        CreatePictureValueList::default(),
        true,
    )?;
    create_cursor(io, state, cursor, picture, image.x_hot, image.y_hot, true)?;
    let _ = free_picture(io, state, picture, true)?;

    Ok(render::Animcursorelt {
        cursor,
        delay: image.delay,
    })
}

fn load_cursor<IO, XS>(
    io: &mut IO,
    state: &mut XS,
    handle: &Handle,
    name: &str,
    env: XcbEnv,
) -> Result<xproto::Cursor, Error>
where
    IO: SocketIo,
    XS: XcbState,
{
    // Find the right cursor, load it directly if it is a core cursor
    let cursor_file = match open_cursor(handle.theme.as_deref(), name, env) {
        None => return Ok(NONE),
        Some(find_cursor::Cursor::CoreChar(c)) => {
            return create_core_cursor(io, state, handle.cursor_font, c);
        }
        Some(find_cursor::Cursor::File(f)) => f,
    };

    // We have to load a file and use RENDER to create a cursor
    if handle.render_support == RenderSupport::None {
        return Ok(NONE);
    }

    let mut vb = VecBuffer::new(cursor_file);
    // Load the cursor from the file
    let images = parse_cursor::parse_cursor(&mut vb, handle.cursor_size)
        .or(Err(crate::error::Error::FromBytes))?;
    let mut images = &images[..];

    // No animated cursor support? Only use the first image
    if handle.render_support == RenderSupport::StaticCursor {
        images = &images[0..1];
    }

    // Now transfer the cursors to the X11 server
    let mut storage = None;
    let cursors = images
        .iter()
        .map(|image| create_render_cursor(io, state, handle, image, &mut storage))
        .collect::<Result<Vec<_>, _>>()?;
    if let Some((pixmap, gc, _, _)) = storage {
        let _ = crate::connection::xproto::free_g_c(io, state, gc, true)?;
        let _ = crate::connection::xproto::free_pixmap(io, state, pixmap, true)?;
    }

    if cursors.len() == 1 {
        Ok(cursors[0].cursor)
    } else {
        let result = state.generate_id(io)?;
        let _ = crate::connection::render::create_anim_cursor(io, state, result, &cursors, true)?;
        for elem in cursors {
            let _ = crate::connection::xproto::free_cursor(io, state, elem.cursor, true)?;
        }
        Ok(result)
    }
}

fn find_format(reply: &render::QueryPictFormatsReply) -> Pictformat {
    reply
        .formats
        .iter()
        .filter(|format| {
            format.r#type == render::PictTypeEnum::DIRECT
                && format.depth == 32
                && format.direct.red_shift == 16
                && format.direct.red_mask == 0xff
                && format.direct.green_shift == 8
                && format.direct.green_mask == 0xff
                && format.direct.blue_shift == 0
                && format.direct.blue_mask == 0xff
                && format.direct.alpha_shift == 24
                && format.direct.alpha_mask == 0xff
        })
        .map(|format| format.id)
        .next()
        .expect("The X11 server is missing the RENDER ARGB_32 standard format!")
}

fn get_cursor_size(
    rm_cursor_size: u32,
    rm_xft_dpi: u32,
    screen: &xproto::Screen,
    env: XcbEnv,
) -> u32 {
    if let Some(size) = env
        .x_cursor_size
        .and_then(|s| s.as_str().ok().and_then(|s| s.parse().ok()))
    {
        return size;
    }
    if rm_cursor_size > 0 {
        return rm_cursor_size;
    }
    if rm_xft_dpi > 0 {
        return rm_xft_dpi * 16 / 72;
    }
    u32::from(screen.height_in_pixels.min(screen.width_in_pixels) / 48)
}
