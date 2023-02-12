//! Utility functions for working with X11 properties

use crate::con::{SocketIo, XcbState};
use crate::connection::xproto::{change_property, get_property};
use crate::cookie::{Cookie, VoidCookie};
use crate::proto::xproto;
use crate::proto::xproto::{Atom, AtomEnum, GetPropertyReply, GetPropertyTypeEnum, Window};
use crate::util::{FixedLengthFromBytes, FixedLengthSerialize, VariableLengthFromBytes};
use crate::Error;

macro_rules! property_cookie {
    {
        $(#[$meta:meta])*
        pub struct $cookie_name:ident: $struct_name:ident,
        $from_reply:expr,
    } => {
        $(#[$meta])*
        #[derive(Debug)]
        pub struct $cookie_name(Cookie<GetPropertyReply>);

        impl $cookie_name
        {
            /// Get the reply that the server sent.
            pub fn reply<IO: SocketIo, XS: XcbState>(self, io: &mut IO, state: &mut XS) -> Result<$struct_name, Error> {
                $from_reply(self.0.reply(io, state)?)
            }
        }
    }
}

// WM_CLASS

property_cookie! {
    /// A cookie for getting a window's `WM_CLASS` property.
    ///
    /// See `WmClass`.
    pub struct WmClassCookie: WmClass,
    WmClass::from_reply,
}

impl WmClassCookie {
    /// Write a `GetProperty` request for the `WM_CLASS` property of the given window
    pub fn new<IO, XS>(io: &mut IO, state: &mut XS, window: Window) -> Result<Self, Error>
    where
        IO: SocketIo,
        XS: XcbState,
    {
        Ok(Self(get_property(
            io,
            state,
            0,
            window,
            AtomEnum::WM_CLASS.0,
            GetPropertyTypeEnum(AtomEnum::STRING.0),
            0,
            2048,
            false,
        )?))
    }
}

/// The value of a window's `WM_CLASS` property.
#[derive(Debug)]
pub struct WmClass(GetPropertyReply, usize);

impl WmClass {
    /// Send a `GetProperty` request for the `WM_CLASS` property of the given window
    pub fn get<IO, XS>(io: &mut IO, state: &mut XS, window: Window) -> Result<WmClassCookie, Error>
    where
        IO: SocketIo,
        XS: XcbState,
    {
        WmClassCookie::new(io, state, window)
    }

    /// Construct a new `WmClass` instance from a `GetPropertyReply`.
    ///
    /// The original `GetProperty` request must have been for a `WM_CLASS` property for this
    /// function to return sensible results.
    pub fn from_reply(reply: GetPropertyReply) -> Result<Self, Error> {
        if reply.r#type != AtomEnum::STRING.0 || reply.format != 8 {
            return Err(Error::FromBytes);
        }
        // Find the first zero byte in the value
        let offset = reply
            .value
            .iter()
            .position(|&v| v == 0)
            .unwrap_or(reply.value.len());
        Ok(WmClass(reply, offset))
    }

    /// Get the instance contained in this `WM_CLASS` property
    #[must_use]
    pub fn instance(&self) -> &[u8] {
        &self.0.value[0..self.1]
    }

    /// Get the class contained in this `WM_CLASS` property
    #[must_use]
    pub fn class(&self) -> &[u8] {
        let start = self.1 + 1;
        if start >= self.0.value.len() {
            return &[];
        };
        let end = if self.0.value.last() == Some(&0) {
            self.0.value.len() - 1
        } else {
            self.0.value.len()
        };
        &self.0.value[start..end]
    }
}

// WM_SIZE_HINTS

/// Representation of whether some part of `WM_SIZE_HINTS` was user/program specified.
#[derive(Debug, Copy, Clone)]
pub enum WmSizeHintsSpecification {
    /// The user specified the values.
    UserSpecified,
    /// The program specified the values.
    ProgramSpecified,
}

property_cookie! {
    /// A cookie for getting a window's `WM_SIZE_HINTS` property.
    pub struct WmSizeHintsCookie: WmSizeHints,
    |reply| WmSizeHints::from_reply(&reply),
}

const NUM_WM_SIZE_HINTS_ELEMENTS: u32 = 18;

impl WmSizeHintsCookie {
    /// Send a `GetProperty` request for the given property of the given window
    pub fn new<IO: SocketIo, XS: XcbState>(
        io: &mut IO,
        state: &mut XS,
        window: Window,
        property: Atom,
    ) -> Result<Self, Error> {
        Ok(Self(get_property(
            io,
            state,
            0,
            window,
            property,
            GetPropertyTypeEnum(AtomEnum::WM_SIZE_HINTS.0),
            0,
            NUM_WM_SIZE_HINTS_ELEMENTS,
            false,
        )?))
    }
    pub fn forget<XS>(self, state: &mut XS)
    where
        XS: XcbState,
    {
        self.0.forget(state);
    }
}

// Possible flags for `WM_SIZE_HINTS`.
const U_S_POSITION: u32 = 1;
const U_S_SIZE: u32 = 1 << 1;
const P_S_POSITION: u32 = 1 << 2;
const P_S_SIZE: u32 = 1 << 3;
const P_MIN_SIZE: u32 = 1 << 4;
const P_MAX_SIZE: u32 = 1 << 5;
const P_RESIZE_INCREMENT: u32 = 1 << 6;
const P_ASPECT: u32 = 1 << 7;
const P_BASE_SIZE: u32 = 1 << 8;
const P_WIN_GRAVITY: u32 = 1 << 9;

/// An aspect ratio `numerator` / `denominator`.
#[derive(Debug, Copy, Clone)]
pub struct AspectRatio {
    /// The numerator of the aspect ratio.
    pub numerator: i32,
    /// The denominator of the aspect ratio.
    pub denominator: i32,
}

impl AspectRatio {
    /// Create a new<C aspect ratio with the given values.
    #[must_use]
    pub fn new(numerator: i32, denominator: i32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

#[allow(clippy::many_single_char_names)]
impl FixedLengthSerialize<8> for AspectRatio {
    fn serialize_fixed(self) -> [u8; 8] {
        let [a, b, c, d] = self.numerator.serialize_fixed();
        let [e, f, g, h] = self.denominator.serialize_fixed();
        [a, b, c, d, e, f, g, h]
    }
}

impl FixedLengthFromBytes<8> for AspectRatio {
    fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        let numerator = bytes
            .get(0..4)
            .map(|b| i32::from_ne_bytes(b.try_into().unwrap()))
            .ok_or(Error::FromBytes)?;
        let denominator = bytes
            .get(4..8)
            .map(|b| i32::from_ne_bytes(b.try_into().unwrap()))
            .ok_or(Error::FromBytes)?;
        Ok(Self {
            numerator,
            denominator,
        })
    }
}

/// A structure representing a `WM_SIZE_HINTS` property.
#[derive(Debug, Default, Copy, Clone)]
pub struct WmSizeHints {
    /// The position that the window should be assigned.
    ///
    /// Note that current versions of ICCCM only make use of the `WmSizeHintsSpecification` field.
    /// The later two fields exist only for backwards compatibility.
    pub position: Option<(WmSizeHintsSpecification, i32, i32)>,
    /// The size that the window should be assigned.
    ///
    /// Note that current versions of ICCCM only make use of the `WmSizeHintsSpecification` field.
    /// The later two fields exist only for backwards compatibility.
    pub size: Option<(WmSizeHintsSpecification, i32, i32)>,
    /// The minimum size that the window may be assigned.
    pub min_size: Option<(i32, i32)>,
    /// The maximum size that the window may be assigned.
    pub max_size: Option<(i32, i32)>,
    /// The increment to be used for sizing the window together with `base_size`.
    pub size_increment: Option<(i32, i32)>,
    /// The minimum and maximum aspect ratio.
    pub aspect: Option<(AspectRatio, AspectRatio)>,
    /// The base size of the window.
    ///
    /// This is used together with `size_increment`.
    pub base_size: Option<(i32, i32)>,
    /// The gravity that is used to make room for window decorations.
    pub win_gravity: Option<xproto::GravityEnum>,
}

impl WmSizeHints {
    /// Get a new, empty `WmSizeHints` structure.
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }

    /// Send a `GetProperty` request for the given property of the given window
    pub fn get<IO: SocketIo, XS: XcbState>(
        io: &mut IO,
        state: &mut XS,
        window: Window,
        property: Atom,
    ) -> Result<WmSizeHintsCookie, Error> {
        WmSizeHintsCookie::new(io, state, window, property)
    }

    /// Send a `GetProperty` request for the `WM_NORMAL_HINTS` property of the given window
    pub fn get_normal_hints<IO: SocketIo, XS: XcbState>(
        io: &mut IO,
        state: &mut XS,
        window: Window,
    ) -> Result<WmSizeHintsCookie, Error> {
        Self::get(io, state, window, AtomEnum::WM_NORMAL_HINTS.0)
    }

    /// Construct a new `WmSizeHints` instance from a `GetPropertyReply`.
    ///
    /// The original `WmSizeHints` request must have been for a `WM_SIZE_HINTS` property for this
    /// function to return sensible results.
    pub fn from_reply(reply: &GetPropertyReply) -> Result<Self, Error> {
        if reply.r#type != AtomEnum::WM_SIZE_HINTS.0 || reply.format != 32 {
            return Err(Error::FromBytes);
        }
        Ok(Self::from_bytes(&reply.value)?.0)
    }

    /// Set these `WM_SIZE_HINTS` on some window as the `WM_NORMAL_HINTS` property.
    pub fn set_normal_hints<IO: SocketIo, XS: XcbState>(
        &self,
        io: &mut IO,
        state: &mut XS,
        window: Window,
        forget: bool,
    ) -> Result<VoidCookie, Error> {
        self.set(io, state, window, AtomEnum::WM_NORMAL_HINTS.0, forget)
    }

    /// Set these `WM_SIZE_HINTS` on some window as the given property.
    pub fn set<IO: SocketIo, XS: XcbState>(
        &self,
        io: &mut IO,
        state: &mut XS,
        window: Window,
        property: Atom,
        forget: bool,
    ) -> Result<VoidCookie, Error> {
        let data = self.serialize_fixed();
        change_property(
            io,
            state,
            xproto::PropModeEnum::REPLACE,
            window,
            property,
            AtomEnum::WM_SIZE_HINTS.0,
            32,
            NUM_WM_HINTS_ELEMENTS,
            &data,
            false,
        )
    }
}

impl<Sub> FixedLengthFromBytes<8> for (Sub, Sub)
where
    Sub: FixedLengthFromBytes<4>,
{
    fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        let b1 = Sub::from_bytes(bytes.get(0..4).ok_or(Error::FromBytes)?)?;
        let b2 = Sub::from_bytes(bytes.get(4..8).ok_or(Error::FromBytes)?)?;
        Ok((b1, b2))
    }
}

impl<Sub> FixedLengthFromBytes<16> for (Sub, Sub)
where
    Sub: FixedLengthFromBytes<8>,
{
    fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        let b1 = Sub::from_bytes(bytes.get(0..8).ok_or(Error::FromBytes)?)?;
        let b2 = Sub::from_bytes(bytes.get(8..16).ok_or(Error::FromBytes)?)?;
        Ok((b1, b2))
    }
}

impl<Sub> FixedLengthSerialize<8> for (Sub, Sub)
where
    Sub: FixedLengthSerialize<4>,
{
    fn serialize_fixed(self) -> [u8; 8] {
        let b1 = self.0.serialize_fixed();
        let b2 = self.1.serialize_fixed();
        let mut join = [0u8; 8];
        join[..4].copy_from_slice(&b1[..4]);
        join[4..8].copy_from_slice(&b2[..4]);
        join
    }
}

impl<Sub> FixedLengthSerialize<16> for (Sub, Sub)
where
    Sub: FixedLengthSerialize<8>,
{
    fn serialize_fixed(self) -> [u8; 16] {
        let b1 = self.0.serialize_fixed();
        let b2 = self.1.serialize_fixed();
        let mut join = [0u8; 16];
        join[..8].copy_from_slice(&b1[..8]);
        join[8..16].copy_from_slice(&b2[..8]);
        join
    }
}

impl VariableLengthFromBytes for WmSizeHints {
    fn from_bytes(bytes: &[u8]) -> crate::Result<(Self, usize)> {
        // Implemented based on what xcb_icccm does. At least a bit. This stuff makes no sense...

        let flags = u32::from_bytes(&bytes[0..4])?;
        let x = i32::from_bytes(&bytes[4..8])?;
        let y = i32::from_bytes(&bytes[8..12])?;
        let width = i32::from_bytes(&bytes[12..16])?;
        let height = i32::from_bytes(&bytes[16..20])?;
        let min_size = parse_with_flag::<8, (i32, i32)>(&bytes[20..28], flags, P_MIN_SIZE)?;
        let max_size = parse_with_flag::<8, (i32, i32)>(&bytes[28..36], flags, P_MAX_SIZE)?;
        let size_increment =
            parse_with_flag::<8, (i32, i32)>(&bytes[36..44], flags, P_RESIZE_INCREMENT)?;
        let aspect =
            parse_with_flag::<16, (AspectRatio, AspectRatio)>(&bytes[44..60], flags, P_ASPECT)?;
        // Apparently, some older version of ICCCM didn't have these...?
        let (base_size, win_gravity, offset) = if bytes[60..].is_empty() {
            (min_size, Some(xproto::GravityEnum::NORTH_WEST), 60)
        } else {
            let base_size = parse_with_flag::<8, (i32, i32)>(&bytes[60..68], flags, P_BASE_SIZE)?;
            let win_gravity = parse_with_flag::<4, u32>(&bytes[68..72], flags, P_WIN_GRAVITY)?;
            (base_size, win_gravity.map(Into::into), 62)
        };

        let position = if flags & U_S_POSITION != 0 {
            Some((WmSizeHintsSpecification::UserSpecified, x, y))
        } else if flags & P_S_POSITION != 0 {
            Some((WmSizeHintsSpecification::ProgramSpecified, x, y))
        } else {
            None
        };
        let size = if flags & U_S_SIZE != 0 {
            Some((WmSizeHintsSpecification::UserSpecified, width, height))
        } else if flags & P_S_SIZE != 0 {
            Some((WmSizeHintsSpecification::ProgramSpecified, width, height))
        } else {
            None
        };

        Ok((
            WmSizeHints {
                position,
                size,
                min_size,
                max_size,
                size_increment,
                aspect,
                base_size,
                win_gravity,
            },
            offset,
        ))
    }
}

impl FixedLengthSerialize<72> for WmSizeHints {
    fn serialize_fixed(self) -> [u8; 72] {
        let mut flags = 0;
        match self.position {
            Some((WmSizeHintsSpecification::UserSpecified, _, _)) => flags |= U_S_POSITION,
            Some((WmSizeHintsSpecification::ProgramSpecified, _, _)) => flags |= P_S_POSITION,
            None => {}
        }
        match self.size {
            Some((WmSizeHintsSpecification::UserSpecified, _, _)) => flags |= U_S_SIZE,
            Some((WmSizeHintsSpecification::ProgramSpecified, _, _)) => flags |= P_S_SIZE,
            None => {}
        }
        flags |= self.min_size.map_or(0, |_| P_MIN_SIZE);
        flags |= self.max_size.map_or(0, |_| P_MAX_SIZE);
        flags |= self.size_increment.map_or(0, |_| P_RESIZE_INCREMENT);
        flags |= self.aspect.map_or(0, |_| P_ASPECT);
        flags |= self.base_size.map_or(0, |_| P_BASE_SIZE);
        flags |= self.win_gravity.map_or(0, |_| P_WIN_GRAVITY);
        let flags_bytes = flags.serialize_fixed();
        let pos_bytes = match self.position {
            Some((_, x, y)) => (x, y),
            None => (0, 0),
        }
        .serialize_fixed();

        let size_bytes = match self.size {
            Some((_, width, height)) => (width, height),
            None => (0, 0),
        }
        .serialize_fixed();
        let min_size_bytes = self.min_size.unwrap_or((0, 0)).serialize_fixed();
        let max_size_bytes = self.max_size.unwrap_or((0, 0)).serialize_fixed();
        let size_increment_bytes = self.size_increment.unwrap_or((0, 0)).serialize_fixed();
        let aspect_bytes = self
            .aspect
            .unwrap_or((AspectRatio::new(0, 0), AspectRatio::new(0, 0)))
            .serialize_fixed();
        let base_size_bytes = self.base_size.unwrap_or((0, 0)).serialize_fixed();
        let win_gravity_bytes = self.win_gravity.map_or(0, |u| u.0).serialize_fixed();
        [
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
            pos_bytes[0],
            pos_bytes[1],
            pos_bytes[2],
            pos_bytes[3],
            pos_bytes[4],
            pos_bytes[5],
            pos_bytes[6],
            pos_bytes[7],
            size_bytes[0],
            size_bytes[1],
            size_bytes[2],
            size_bytes[3],
            size_bytes[4],
            size_bytes[5],
            size_bytes[6],
            size_bytes[7],
            min_size_bytes[0],
            min_size_bytes[1],
            min_size_bytes[2],
            min_size_bytes[3],
            min_size_bytes[4],
            min_size_bytes[5],
            min_size_bytes[6],
            min_size_bytes[7],
            max_size_bytes[0],
            max_size_bytes[1],
            max_size_bytes[2],
            max_size_bytes[3],
            max_size_bytes[4],
            max_size_bytes[5],
            max_size_bytes[6],
            max_size_bytes[7],
            size_increment_bytes[0],
            size_increment_bytes[1],
            size_increment_bytes[2],
            size_increment_bytes[3],
            size_increment_bytes[4],
            size_increment_bytes[5],
            size_increment_bytes[6],
            size_increment_bytes[7],
            aspect_bytes[0],
            aspect_bytes[1],
            aspect_bytes[2],
            aspect_bytes[3],
            aspect_bytes[4],
            aspect_bytes[5],
            aspect_bytes[6],
            aspect_bytes[7],
            aspect_bytes[8],
            aspect_bytes[9],
            aspect_bytes[10],
            aspect_bytes[11],
            aspect_bytes[12],
            aspect_bytes[13],
            aspect_bytes[14],
            aspect_bytes[15],
            base_size_bytes[0],
            base_size_bytes[1],
            base_size_bytes[2],
            base_size_bytes[3],
            base_size_bytes[4],
            base_size_bytes[5],
            base_size_bytes[6],
            base_size_bytes[7],
            win_gravity_bytes[0],
            win_gravity_bytes[1],
            win_gravity_bytes[2],
            win_gravity_bytes[3],
        ]
    }
}

// WM_HINTS
//
property_cookie! {
    /// A cookie for getting a window's `WM_HINTS` property.
    ///
    /// See `WmHints`.
    pub struct WmHintsCookie: WmHints,
    |reply| WmHints::from_reply(&reply),
}

const NUM_WM_HINTS_ELEMENTS: u32 = 9;

impl WmHintsCookie {
    /// Send a `GetProperty` request for the `WM_CLASS` property of the given window
    pub fn new<IO, XS>(io: &mut IO, state: &mut XS, window: Window) -> Result<Self, Error>
    where
        IO: SocketIo,
        XS: XcbState,
    {
        Ok(Self(get_property(
            io,
            state,
            0,
            window,
            AtomEnum::WM_HINTS.0,
            GetPropertyTypeEnum(AtomEnum::WM_HINTS.0),
            0,
            NUM_WM_HINTS_ELEMENTS,
            false,
        )?))
    }

    pub fn forget<XS: XcbState>(self, state: &mut XS) {
        self.0.forget(state);
    }
}

/// The possible values for a `WM_STATE`'s state field.
#[derive(Debug, Copy, Clone)]
pub enum WmHintsState {
    /// The window should be in Normal state.
    Normal,
    /// The window should be in Iconic state.
    Iconic,
}

// Possible flags for `WM_HINTS`.
const HINT_INPUT: u32 = 1;
const HINT_STATE: u32 = 1 << 1;
const HINT_ICON_PIXMAP: u32 = 1 << 2;
const HINT_ICON_WINDOW: u32 = 1 << 3;
const HINT_ICON_POSITION: u32 = 1 << 4;
const HINT_ICON_MASK: u32 = 1 << 5;
const HINT_WINDOW_GROUP: u32 = 1 << 6;
// This bit is obsolete, according to ICCCM
//const HINT_MESSAGE: u32 = 1 << 7;
const HINT_URGENCY: u32 = 1 << 8;

/// A structure representing a `WM_HINTS` property.
#[derive(Debug, Default, Copy, Clone)]
pub struct WmHints {
    /// Whether the window manager may set the input focus to this window.
    ///
    /// See ICCCM §4.1.7 for details.
    pub input: Option<bool>,

    /// The state that the window should be when it leaves the Withdrawn state.
    pub initial_state: Option<WmHintsState>,

    /// A pixmap that represents the icon of this window.
    pub icon_pixmap: Option<xproto::Pixmap>,

    /// A window that should be used as icon.
    pub icon_window: Option<Window>,

    /// The position where the icon should be shown.
    pub icon_position: Option<(i32, i32)>,

    /// A mask for `icon_pixmap`.
    ///
    /// This allows nonrectangular icons.
    pub icon_mask: Option<xproto::Pixmap>,

    /// A window that represents a group of windows.
    ///
    /// The specified window is called the "group leader". All windows with the same group leader
    /// are part of the same group.
    pub window_group: Option<Window>,

    /// Indication that the window contents are urgent.
    ///
    /// Urgency means that a timely response of the user is required. The window manager must make
    /// some effort to draw the user's attention to this window while this flag is set.
    pub urgent: bool,
}

impl WmHints {
    /// Get a new, empty `WmSizeHints` structure.
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }

    /// Send a `GetProperty` request for the `WM_HINTS` property of the given window
    pub fn get<IO, XS>(io: &mut IO, state: &mut XS, window: Window) -> Result<WmHintsCookie, Error>
    where
        IO: SocketIo,
        XS: XcbState,
    {
        WmHintsCookie::new(io, state, window)
    }

    /// Construct a new `WmHints` instance from a `GetPropertyReply`.
    ///
    /// The original `WmHints` request must have been for a `WM_HINTS` property for this
    /// function to return sensible results.
    pub fn from_reply(reply: &GetPropertyReply) -> Result<Self, Error> {
        if reply.r#type != AtomEnum::WM_HINTS.0 || reply.format != 32 {
            return Err(Error::FromBytes);
        }

        Ok(Self::from_bytes(&reply.value)?.0)
    }

    /// Set these `WM_HINTS` on some window.
    pub fn set<IO, XS>(
        &self,
        io: &mut IO,
        state: &mut XS,
        window: Window,
        forget: bool,
    ) -> Result<VoidCookie, Error>
    where
        IO: SocketIo,
        XS: XcbState,
    {
        let data = self.serialize_fixed();
        change_property(
            io,
            state,
            xproto::PropModeEnum::REPLACE,
            window,
            AtomEnum::WM_HINTS.0,
            AtomEnum::WM_HINTS.0,
            32,
            NUM_WM_HINTS_ELEMENTS,
            &data,
            forget,
        )
    }
}

impl VariableLengthFromBytes for WmHints {
    fn from_bytes(bytes: &[u8]) -> crate::Result<(Self, usize)> {
        let flags = u32::from_bytes(&bytes[0..4])?;
        let input = parse_with_flag::<4, u32>(&bytes[4..8], flags, HINT_INPUT)?;
        let initial_state = parse_with_flag::<4, u32>(&bytes[8..12], flags, HINT_STATE)?;
        let icon_pixmap = parse_with_flag::<4, u32>(&bytes[12..16], flags, HINT_ICON_PIXMAP)?;
        let icon_window = parse_with_flag::<4, u32>(&bytes[16..20], flags, HINT_ICON_WINDOW)?;
        let icon_position =
            parse_with_flag::<8, (i32, i32)>(&bytes[20..28], flags, HINT_ICON_POSITION)?;
        let icon_mask = parse_with_flag::<4, u32>(&bytes[28..32], flags, HINT_ICON_MASK)?;
        // Apparently, some older version of ICCCM didn't have this...?
        let (window_group, offset) = if bytes[32..].is_empty() {
            (None, 32)
        } else {
            let window_group = parse_with_flag::<4, u32>(&bytes[32..36], flags, HINT_WINDOW_GROUP)?;
            (window_group, 36)
        };

        let input = input.map(|input| input != 0);

        let initial_state = match initial_state {
            None => None,
            Some(1) => Some(WmHintsState::Normal),
            Some(3) => Some(WmHintsState::Iconic),
            _ => return Err(Error::FromBytes),
        };

        let urgent = flags & HINT_URGENCY != 0;

        Ok((
            WmHints {
                input,
                initial_state,
                icon_pixmap,
                icon_window,
                icon_position,
                icon_mask,
                window_group,
                urgent,
            },
            offset,
        ))
    }
}

impl FixedLengthSerialize<36> for WmHints {
    fn serialize_fixed(self) -> [u8; 36] {
        let mut flags = 0;
        flags |= self.input.map_or(0, |_| HINT_INPUT);
        flags |= self.initial_state.map_or(0, |_| HINT_STATE);
        flags |= self.icon_pixmap.map_or(0, |_| HINT_ICON_PIXMAP);
        flags |= self.icon_window.map_or(0, |_| HINT_ICON_WINDOW);
        flags |= self.icon_position.map_or(0, |_| HINT_ICON_POSITION);
        flags |= self.icon_mask.map_or(0, |_| HINT_ICON_MASK);
        flags |= self.window_group.map_or(0, |_| HINT_WINDOW_GROUP);
        if self.urgent {
            flags |= HINT_URGENCY;
        }
        let flags_bytes = flags.serialize_fixed();
        let input_bytes = u32::from(self.input.unwrap_or(false)).serialize_fixed();
        let initial_state_bytes = match self.initial_state {
            Some(WmHintsState::Normal) => 1,
            Some(WmHintsState::Iconic) => 3,
            None => 0,
        }
        .serialize_fixed();
        let icon_pixmap_bytes = self.icon_pixmap.unwrap_or(0).serialize_fixed();
        let icon_window_bytes = self.icon_window.unwrap_or(0).serialize_fixed();
        let icon_position_bytes = self.icon_position.unwrap_or((0, 0)).serialize_fixed();
        let icon_mask_bytes = self.icon_mask.unwrap_or(0).serialize_fixed();
        let window_group_bytes = self.window_group.unwrap_or(0).serialize_fixed();
        [
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
            input_bytes[0],
            input_bytes[1],
            input_bytes[2],
            input_bytes[3],
            initial_state_bytes[0],
            initial_state_bytes[1],
            initial_state_bytes[2],
            initial_state_bytes[3],
            icon_pixmap_bytes[0],
            icon_pixmap_bytes[1],
            icon_pixmap_bytes[2],
            icon_pixmap_bytes[3],
            icon_window_bytes[0],
            icon_window_bytes[1],
            icon_window_bytes[2],
            icon_window_bytes[3],
            icon_position_bytes[0],
            icon_position_bytes[1],
            icon_position_bytes[2],
            icon_position_bytes[3],
            icon_position_bytes[4],
            icon_position_bytes[5],
            icon_position_bytes[6],
            icon_position_bytes[7],
            icon_mask_bytes[0],
            icon_mask_bytes[1],
            icon_mask_bytes[2],
            icon_mask_bytes[3],
            window_group_bytes[0],
            window_group_bytes[1],
            window_group_bytes[2],
            window_group_bytes[3],
        ]
    }
}

/// Parse an element of type `T` and turn it into an `Option` by checking if the given `bit` is set
/// in `flags`.
fn parse_with_flag<const N: usize, T: FixedLengthFromBytes<N>>(
    remaining: &[u8],
    flags: u32,
    bit: u32,
) -> Result<Option<T>, Error> {
    let v = T::from_bytes(remaining)?;
    if flags & bit == 0 {
        Ok(None)
    } else {
        Ok(Some(v))
    }
}

#[cfg(test)]
mod test {
    use alloc::vec::Vec;

    use crate::proto::xproto::{Atom, AtomEnum, GetPropertyReply, GravityEnum};
    use crate::util::FixedLengthSerialize;

    use super::{WmClass, WmHints, WmHintsState, WmSizeHints};

    fn get_property_reply(value: &[u8], format: u8, type_: Atom) -> GetPropertyReply {
        GetPropertyReply {
            response_type: 0,
            format,
            sequence: 0,
            length: 0,
            r#type: type_,
            bytes_after: 0,
            value: value.to_vec(),
        }
    }

    #[test]
    fn test_wm_class() {
        for (input, instance, class) in &[
            (&b""[..], &b""[..], &b""[..]),
            (b"\0", b"", b""),
            (b"\0\0", b"", b""),
            (b"\0\0\0", b"", b"\0"),
            (b"Hello World", b"Hello World", b""),
            (b"Hello World\0", b"Hello World", b""),
            (b"Hello\0World\0", b"Hello", b"World"),
            (b"Hello\0World", b"Hello", b"World"),
            (b"Hello\0World\0Good\0Day", b"Hello", b"World\0Good\0Day"),
        ] {
            let wm_class =
                WmClass::from_reply(get_property_reply(input, 8, AtomEnum::STRING.0)).unwrap();
            assert_eq!((wm_class.instance(), wm_class.class()), (*instance, *class));
        }
    }

    #[test]
    fn test_wm_normal_hints() {
        // This is the value of some random xterm window.
        // It was acquired via 'xtrace xprop WM_NORMAL_HINTS'.
        let input = [
            0x0000_0350,
            0x0000_0000,
            0x0000_0000,
            0x0000_0000,
            0x0000_0000,
            0x0000_0015,
            0x0000_0017,
            0x0000_0000,
            0x0000_0000,
            0x0000_000a,
            0x0000_0013,
            0x0000_0000,
            0x0000_0000,
            0x0000_0000,
            0x0000_0000,
            0x0000_000b,
            0x0000_0004,
            0x0000_0001,
        ];
        let input = input
            .iter()
            .flat_map(|v| (*v as u32).serialize_fixed().to_vec())
            .collect::<Vec<u8>>();
        let wm_size_hints =
            WmSizeHints::from_reply(&get_property_reply(&input, 32, AtomEnum::WM_SIZE_HINTS.0))
                .unwrap();

        assert!(
            wm_size_hints.position.is_none(),
            "{:?}",
            wm_size_hints.position,
        );
        assert!(wm_size_hints.size.is_none(), "{:?}", wm_size_hints.size);
        assert_eq!(wm_size_hints.min_size, Some((21, 23)));
        assert_eq!(wm_size_hints.max_size, None);
        assert_eq!(wm_size_hints.size_increment, Some((10, 19)));
        assert!(wm_size_hints.aspect.is_none(), "{:?}", wm_size_hints.aspect);
        assert_eq!(wm_size_hints.base_size, Some((11, 4)));
        assert_eq!(wm_size_hints.win_gravity, Some(GravityEnum::NORTH_WEST));

        assert_eq!(input, wm_size_hints.serialize_fixed().to_vec());
    }

    #[test]
    fn test_wm_hints() {
        // This is the value of some random xterm window.
        // It was acquired via 'xtrace xprop WM_HINTS'.
        let input = [
            0x0000_0043,
            0x0000_0001,
            0x0000_0001,
            0x0000_0000,
            0x0000_0000,
            0x0000_0000,
            0x0000_0000,
            0x0000_0000,
            0x0060_0009,
        ];
        let input = input
            .iter()
            .flat_map(|v| (*v as u32).serialize_fixed().to_vec())
            .collect::<Vec<u8>>();
        let wm_hints =
            WmHints::from_reply(&get_property_reply(&input, 32, AtomEnum::WM_HINTS.0)).unwrap();

        assert_eq!(wm_hints.input, Some(true));
        match wm_hints.initial_state {
            Some(WmHintsState::Normal) => {}
            value => panic!("Expected Some(Normal), but got {:?}", value),
        }
        assert_eq!(wm_hints.icon_pixmap, None);
        assert_eq!(wm_hints.icon_window, None);
        assert_eq!(wm_hints.icon_position, None);
        assert_eq!(wm_hints.icon_mask, None);
        assert_eq!(wm_hints.window_group, Some(0x0060_0009));
        assert!(!wm_hints.urgent);

        assert_eq!(input, wm_hints.serialize_fixed().to_vec());
    }
}
