//! Utility functions for working with X11 cursors
//!
//! The code in this module is only available when the `cursor` feature of the library is enabled.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::connection::render::RenderConnection;
use crate::connection::xproto::XprotoConnection;
use crate::helpers::cursor::parse_cursor::VecBuffer;
use crate::helpers::resource_manager::protocol::Database;
use crate::helpers::XcbEnv;
use crate::proto::render::{CreatePictureValueList, Pictformat};
use crate::proto::xproto::{CreateGCValueList, Font, FontEnum, Window};
use crate::proto::{render, xproto};
use crate::{Error, XcbConnection, NONE};
use crate::con::XcbBuffers;

mod find_cursor;
mod parse_cursor;

/// The level of cursor support of the X11 server
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RenderSupport {
    /// Render extension not available
    None,

    /// Static cursor support (CreateCursor added in RENDER 0.5)
    StaticCursor,

    /// Animated cursor support (CreateAnimCursor added in RENDER 0.8)
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
    pub fn new<C>(
        conn: &mut C,
        buffers: &mut XcbBuffers,
        screen: usize,
        resource_database: &Database,
        env: XcbEnv,
    ) -> Result<Self, Error>
    where
        C: XcbConnection,
    {
        let screen = conn.setup().roots[screen].clone();
        let render_info = if conn.major_opcode(render::EXTENSION_NAME).is_some() {
            let render_version = crate::connection::render::RenderConnection::query_version(
                conn, buffers.out_buffer, 0, 8, false,
            )?;
            let render_pict_format =
                crate::connection::render::RenderConnection::query_pict_formats(
                    conn, buffers.out_buffer, false,
                )?;
            Some((render_version, render_pict_format))
        } else {
            None
        };
        let mut render_version = (0, 0);
        let mut picture_format = NONE;
        if let Some((version, formats)) = render_info {
            let version = version.reply(conn, buffers)?;
            render_version = (version.major_version, version.minor_version);
            picture_format = find_format(&formats.reply(conn, buffers)?);
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
        let cursor_font = conn.generate_id(buffers)?;

        let _ = conn.open_font(buffers.out_buffer, cursor_font, b"cursor", true)?;
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
    pub fn load_cursor<C>(
        &self,
        buffers: &mut XcbBuffers,
        conn: &mut C,
        name: &str,
        env: XcbEnv,
    ) -> Result<xproto::Cursor, Error>
    where
        C: XcbConnection,
    {
        load_cursor(conn, buffers, self, name, env)
    }
}

fn open_cursor(
    theme: &Option<String>,
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

fn create_core_cursor<C>(
    conn: &mut C,
    buffers: &mut XcbBuffers,
    cursor_font: Font,
    cursor: u16,
) -> Result<xproto::Cursor, Error>
where
    C: XcbConnection,
{
    let result = conn.generate_id(buffers)?;
    conn.create_glyph_cursor(
        buffers.out_buffer,
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

fn create_render_cursor<C>(
    conn: &mut C,
    buffers: &mut XcbBuffers,
    handle: &Handle,
    image: &parse_cursor::Image,
    storage: &mut Option<(xproto::Pixmap, xproto::Gcontext, u16, u16)>,
) -> Result<render::Animcursorelt, Error>
where
    C: XcbConnection,
{
    let (cursor, picture) = (conn.generate_id(buffers)?, conn.generate_id(buffers)?);

    // Get a pixmap of the right size and a gc for it
    let (pixmap, gc) = if storage.map(|(_, _, w, h)| (w, h)) == Some((image.width, image.height)) {
        storage.map(|(pixmap, gc, _, _)| (pixmap, gc)).unwrap()
    } else {
        let (pixmap, gc) = if let Some((pixmap, gc, _, _)) = storage {
            conn.free_g_c(buffers.out_buffer, *gc, true)?;
            conn.free_pixmap(buffers.out_buffer, *pixmap, true)?;
            (*pixmap, *gc)
        } else {
            (conn.generate_id(buffers)?, conn.generate_id(buffers)?)
        };
        conn.create_pixmap(
            buffers.out_buffer,
            32,
            pixmap,
            handle.root,
            image.width,
            image.height,
            true,
        )?;
        conn.create_g_c(buffers.out_buffer, gc, pixmap, CreateGCValueList::default(), true)?;

        *storage = Some((pixmap, gc, image.width, image.height));
        (pixmap, gc)
    };

    // Sigh. We need the pixel data as a bunch of bytes.
    let mut pixels = vec![0; image.pixels.len() * 4];
    crate::util::fixed_vec_serialize_into(&mut pixels, &image.pixels)?;
    let _ = XprotoConnection::put_image(
        conn,
        buffers.out_buffer,
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

    let _ = conn.create_picture(
        buffers.out_buffer,
        picture,
        pixmap,
        handle.picture_format,
        CreatePictureValueList::default(),
        true,
    )?;
    RenderConnection::create_cursor(
        conn,
        buffers.out_buffer,
        cursor,
        picture,
        image.x_hot,
        image.y_hot,
        true,
    )?;
    let _ = conn.free_picture(buffers.out_buffer, picture, true)?;

    Ok(render::Animcursorelt {
        cursor,
        delay: image.delay,
    })
}

fn load_cursor<C>(
    conn: &mut C,
    buffers: &mut XcbBuffers,
    handle: &Handle,
    name: &str,
    env: XcbEnv,
) -> Result<xproto::Cursor, Error>
where
    C: XcbConnection,
{
    // Find the right cursor, load it directly if it is a core cursor
    let cursor_file = match open_cursor(&handle.theme, name, env) {
        None => return Ok(NONE),
        Some(find_cursor::Cursor::CoreChar(c)) => {
            return create_core_cursor(conn, buffers, handle.cursor_font, c);
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
        .map(|image| create_render_cursor(conn, buffers, handle, image, &mut storage))
        .collect::<Result<Vec<_>, _>>()?;
    if let Some((pixmap, gc, _, _)) = storage {
        let _ = conn.free_g_c(buffers.out_buffer, gc, true)?;
        let _ = conn.free_pixmap(buffers.out_buffer, pixmap, true)?;
    }

    if cursors.len() == 1 {
        Ok(cursors[0].cursor)
    } else {
        let result = conn.generate_id(buffers)?;
        let _ = conn.create_anim_cursor(buffers.out_buffer, result, &cursors, true)?;
        for elem in cursors {
            let _ = conn.free_cursor(buffers.out_buffer, elem.cursor, true)?;
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
    if let Some(size) = env.x_cursor_size.and_then(|s| s.parse().ok()) {
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
