use alloc::collections::VecDeque;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};
use core::time::Duration;
use rusl::error::Errno;
use rusl::platform::{PollEvents, PollFd, TimeSpec};
use rusl::string::unix_str::{UnixStr, UnixString};

use smallmap::{Map, Set};
use tiny_std::fs::read;
use tiny_std::io::{Read, Write};
use tiny_std::net::UnixStream;
use tiny_std::time::MonotonicInstant;
use tiny_std::unix::fd::{AsRawFd, OwnedFd, RawFd};

use xcb_rust_protocol::con::{SocketIo, XcbBuffer, XcbBuffers, XcbState};
use xcb_rust_protocol::connection::bigreq::enable;
use xcb_rust_protocol::connection::xproto::{
    change_property, get_input_focus, list_extensions, query_extension,
};
use xcb_rust_protocol::cookie::VoidCookie;
use xcb_rust_protocol::proto::find_extension;
use xcb_rust_protocol::proto::xc_misc::GetXIDRangeReply;
use xcb_rust_protocol::proto::xproto::{Atom, ListExtensionsReply, PropModeEnum, Setup, Window};
use xcb_rust_protocol::util::{
    parse_error, ExtensionInfoProvider, ExtensionInformation, VariableLengthFromBytes,
    XcbErrorHandler,
};
use xcb_rust_protocol::{Error, XcbConnection, XcbEnv};

use crate::helpers::basic_info_provider::BasicExtensionInfoProvider;
use crate::helpers::connect::{get_setup_length, parse_setup, setup_request, Connect};
use crate::helpers::id_allocator::IdAllocator;
use crate::helpers::parse_display::ParsedDisplay;
use crate::helpers::xauth::Family;
use crate::{ConnectError, ConnectionError};

#[derive(Debug)]
pub struct XcbEventState {
    setup: Setup,
    seq_count: SeqCount,
    event_cache: VecDeque<Vec<u8>>,
    reply_cache: Map<u16, Vec<u8>>,
    keep_seqs: Set<u16>,
    id_allocator: IdAllocator,
    max_request_length: usize,
    pub extensions: BasicExtensionInfoProvider,
}

impl XcbEventState {
    #[must_use]
    pub fn new(setup: Setup) -> Self {
        Self {
            max_request_length: setup.maximum_request_length as usize, // It's the length in 32bit words
            id_allocator: IdAllocator::new(setup.resource_id_base, setup.resource_id_mask).unwrap(),
            setup,
            seq_count: SeqCount::new(),
            event_cache: VecDeque::new(),
            reply_cache: Map::new(),
            keep_seqs: Set::new(),
            extensions: BasicExtensionInfoProvider::default(),
        }
    }

    #[inline]
    pub(crate) fn extension_information(&self, name: &'static str) -> Option<ExtensionInformation> {
        self.extensions.get_by_name(name)
    }

    #[cfg(feature = "debug")]
    pub fn clear_cache<IO: SocketIo>(&mut self, io: &mut IO) -> Result<(), ConnectionError> {
        if self.keep_seqs.is_empty() && self.reply_cache.is_empty() {
            return Ok(());
        }
        if !self.keep_seqs.is_empty() {
            let _ = get_input_focus(io, self, false)?.reply(io, self)?;
        }
        for (seq, _) in self.keep_seqs.iter() {
            crate::debug!("Dropped voidcookie {seq}");
        }
        for (seq, reply) in self.reply_cache.iter() {
            if reply[0] == ERROR {
                let err = parse_error(reply, &self.extensions)?;
                crate::debug!("Dropped error on seq {seq}! {:?}", err);
            } else {
                crate::debug!("Dropped reply on seq {seq}!");
            }
        }
        crate::debug!("Panicking because of leak!");
        panic!("Leaked replies;")
    }
}

#[derive(Copy, Clone, Debug)]
struct SeqCount {
    cur: u16,
    seen: u16,
}

impl SeqCount {
    fn new() -> Self {
        Self { cur: 1, seen: 1 }
    }

    #[inline]
    // A strictly less than here is kind of dubious as sequences wrap.
    // However, this is only used to potentially skip a sync so it doesn't really matter
    // since it only has false negatives.
    fn sequence_has_been_seen(self, seq: u16) -> bool {
        seq < self.seen
    }

    #[inline]
    fn get_and_increment(&mut self) -> u16 {
        let last = self.cur;
        self.cur = self.cur.overflowing_add(1).0;
        last
    }

    #[inline]
    // Events are sequential so this shouldn't be callable out of order messing with
    // sequence has been seen logic
    fn record_seen(&mut self, seq: u16) {
        self.seen = seq;
    }
}

pub fn find_socket_path(
    dpy_name: Option<&str>,
) -> Result<(UnixString, ParsedDisplay), ConnectError> {
    let parsed_display = crate::helpers::parse_display::parse_display(dpy_name)
        .ok_or(ConnectError::DisplayParsingError)?;
    let screen: usize = parsed_display.screen.into();
    if let Some(path) = parsed_display.connect_instruction() {
        Ok((path, parsed_display))
    } else {
        Err(ConnectError::DisplayParsingError)
    }
}

pub fn setup<IO: SocketIo>(
    io: &mut IO,
    xcb_env: XcbEnv,
    dpy: ParsedDisplay,
) -> Result<XcbEventState, ConnectError> {
    let family = Family::LOCAL;
    let host = tiny_std::unix::host_name::host_name().unwrap_or_else(|_| "localhost".to_string());
    let setup_req = setup_request(xcb_env, family, host.as_bytes(), dpy.display)?;
    io.use_write_buffer(|buf| {
        buf[..setup_req.len()].copy_from_slice(&setup_req);
        Ok::<usize, ConnectError>(setup_req.len())
    })?;
    let mut read_bytes = 0;
    while read_bytes < 8 {
        io.use_read_buffer(|buf| {
            read_bytes = buf.len();
            Ok::<usize, ConnectError>(0)
        })?;
        if read_bytes < 8 {
            io.block_for_more_data().unwrap();
        }
    }
    let mut required_length = 0;
    io.use_read_buffer(|buf| {
        required_length = get_setup_length(buf);
        Ok::<usize, ConnectError>(0)
    })?;
    while read_bytes < required_length {
        io.use_read_buffer(|buf| {
            read_bytes = buf.len();
            Ok::<usize, ConnectError>(0)
        })?;
        if read_bytes < required_length {
            io.block_for_more_data().unwrap();
        }
    }
    let mut setup = None;
    io.use_read_buffer(|buf| {
        setup = Some(parse_setup(buf)?);
        Ok::<usize, ConnectError>(required_length)
    })?;
    let setup = setup.unwrap();

    // resolve the setup
    crate::debug!("Setup completed.");

    // Check that we got a valid screen number
    if dpy.screen >= setup.roots.len() as u16 {
        return Err(ConnectError::InvalidScreen);
    }
    let mut state = XcbEventState::new(setup);
    init_extensions(io, &mut state).map_err(|e| {
        crate::debug!("Error init exts {e}");
        ConnectError::UnknownError
    })?;
    check_for_big_req(io, &mut state).map_err(|e| {
        crate::debug!("Error check big_req {e}");
        ConnectError::UnknownError
    })?;
    Ok(state)
}

// Preload all extensions immediately
fn init_extensions<IO: SocketIo>(
    io: &mut IO,
    state: &mut XcbEventState,
) -> Result<(), ConnectionError> {
    let listed = list_extensions(io, state, false)?;
    let r = state.block_for_reply(io, listed.seq)?;
    let (reply, offset) = ListExtensionsReply::from_bytes(&r)?;
    let mut extensions = vec![];
    for name in reply.names {
        let cookie = query_extension(io, state, &name.name, false)?;
        extensions.push((name.name, cookie));
    }
    crate::debug!("Pushed all {} ext requests", extensions.len());
    for (name, cookie) in extensions {
        let response = cookie.reply(io, state)?;
        let name = String::from_utf8(name).map_err(|e| {
            crate::debug!("Failed string convert {e}");
            ConnectionError::UnsupportedExtension(format!("Failed to convert extension name {e}"))
        })?;
        if let Some(ext) = find_extension(&name) {
            crate::debug!("Registered extension: '{ext}'");
            if response.present == 1 {
                state.extensions.extensions.push((
                    ext,
                    ExtensionInformation {
                        major_opcode: response.major_opcode,
                        first_event: response.first_event,
                        first_error: response.first_error,
                    },
                ));
            }
        }
    }
    Ok(())
}

fn check_for_big_req<IO: SocketIo>(
    io: &mut IO,
    state: &mut XcbEventState,
) -> Result<(), ConnectionError> {
    if state
        .extension_information(xcb_rust_protocol::proto::bigreq::EXTENSION_NAME)
        .is_some()
    {
        let reply = enable(io, state, false)?.reply(io, state)?;
        state.max_request_length = reply.maximum_request_length as usize;
        crate::debug!(
            "Got max_request_length = {} words from bigreq",
            state.max_request_length
        );
    }

    Ok(())
}
pub fn change_property8<IO: SocketIo, XS: XcbState>(
    io: &mut IO,
    state: &mut XS,
    mode: PropModeEnum,
    window: Window,
    property: Atom,
    type_: Atom,
    data: &[u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    Ok(change_property(
        io,
        state,
        mode,
        window,
        property,
        type_,
        8,
        data.len().try_into().expect("`data` has too many elements"),
        data,
        forget,
    )?)
}

/// Change a property on a window with format 16.
pub fn change_property16<IO: SocketIo, XS: XcbState>(
    io: &mut IO,
    state: &mut XS,
    mode: PropModeEnum,
    window: Window,
    property: Atom,
    type_: Atom,
    data: &[u16],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let mut data_u8 = Vec::with_capacity(data.len() * 2);
    for item in data {
        data_u8.extend(item.to_ne_bytes());
    }
    Ok(change_property(
        io,
        state,
        mode,
        window,
        property,
        type_,
        16,
        data.len().try_into().expect("`data` has too many elements"),
        &data_u8,
        forget,
    )?)
}

/// Change a property on a window with format 32.
pub fn change_property32<IO: SocketIo, XS: XcbState>(
    io: &mut IO,
    state: &mut XS,
    mode: PropModeEnum,
    window: Window,
    property: Atom,
    type_: Atom,
    data: &[u32],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let mut data_u8 = Vec::with_capacity(data.len() * 4);
    for item in data {
        data_u8.extend(item.to_ne_bytes());
    }
    Ok(change_property(
        io,
        state,
        mode,
        window,
        property,
        type_,
        32,
        data.len().try_into().expect("`data` has too many elements"),
        &data_u8,
        forget,
    )?)
}

pub fn try_drain<IO: SocketIo>(
    io: &mut IO,
    state: &mut XcbEventState,
) -> Result<Vec<Vec<u8>>, ConnectionError> {
    let mut events = vec![];
    while let Some(next) = state.event_cache.pop_front() {
        events.push(next);
    }
    for rr in do_drain(io) {
        match rr {
            ReadResult::Event(e) => {
                events.push(e);
            }
            ReadResult::Reply(got_seq, buf) => {
                if state.keep_seqs.remove(&got_seq).is_some() {
                    state.reply_cache.insert(got_seq, buf);
                }
                state.seq_count.record_seen(got_seq);
            }
            ReadResult::Error(got_seq, buf) => {
                crate::debug!("Got err {:?}", parse_error(&buf, &state.extensions)?);
                if state.keep_seqs.remove(&got_seq).is_some() {
                    state.reply_cache.insert(got_seq, buf);
                }
                state.seq_count.record_seen(got_seq);
            }
        }
    }

    Ok(events)
}

impl XcbState for XcbEventState {
    #[inline]
    fn major_opcode(&self, extension_name: &'static str) -> Option<u8> {
        self.extension_information(extension_name)
            .map(|ei| ei.major_opcode)
    }

    #[inline]
    fn next_seq(&mut self) -> u16 {
        self.seq_count.get_and_increment()
    }

    #[inline]
    fn keep_and_return_next_seq(&mut self) -> u16 {
        let next = self.seq_count.get_and_increment();
        self.keep_seqs.insert(next, ());
        next
    }

    #[inline]
    fn max_request_size(&self) -> usize {
        self.max_request_length
    }

    #[inline]
    fn setup(&self) -> &Setup {
        &self.setup
    }

    #[inline]
    fn generate_id<IO: SocketIo>(&mut self, io: &mut IO) -> Result<u32, Error> {
        if let Some(id) = self.id_allocator.generate_id() {
            Ok(id)
        } else if self
            .extension_information(xcb_rust_protocol::proto::xc_misc::EXTENSION_NAME)
            .is_none()
        {
            // IDs are exhausted and XC-MISC is not available
            Err(Error::Connection("Ids exhausted and xc-misc not available"))
        } else {
            let range = xcb_rust_protocol::connection::xc_misc::get_x_i_d_range(io, self, false)?
                .reply(io, self)?;

            self.id_allocator
                .update_xid_range(&range)
                .map_err(|_| Error::Connection("Ids exhausted on server"))?;
            self.id_allocator
                .generate_id()
                .ok_or(Error::Connection("Ids exhausted"))
        }
    }

    #[inline]
    fn block_for_reply<IO: SocketIo>(&mut self, io: &mut IO, seq: u16) -> Result<Vec<u8>, Error> {
        if let Some(reply) = self.reply_cache.remove(&seq) {
            Ok(reply)
        } else {
            let mut target = None;
            self.keep_seqs.remove(&seq);
            while target.is_none() {
                for rr in do_drain(io) {
                    match rr {
                        ReadResult::Event(e) => {
                            self.event_cache.push_back(e);
                        }
                        ReadResult::Reply(got_seq, buf) => {
                            if got_seq == seq {
                                target = Some(buf);
                            } else if self.keep_seqs.remove(&got_seq).is_some() {
                                self.reply_cache.insert(got_seq, buf);
                            }
                            self.seq_count.record_seen(got_seq);
                        }
                        ReadResult::Error(got_seq, buf) => {
                            crate::debug!("Got err {:?}", parse_error(&buf, &self.extensions)?);
                            if got_seq == seq {
                                target = Some(buf);
                            } else if self.keep_seqs.remove(&got_seq).is_some() {
                                self.reply_cache.insert(got_seq, buf);
                            }
                            self.seq_count.record_seen(got_seq);
                        }
                    }
                }
                if target.is_some() {
                    continue;
                }
                crate::debug!("No drain in current buffer, try read.");
                for rr in read_next(io).map_err(|e| {
                    crate::debug!("Failed to read next {e}");
                    Error::Connection("Failed to read next")
                })? {
                    match rr {
                        ReadResult::Event(e) => {
                            self.event_cache.push_back(e);
                        }
                        ReadResult::Reply(got_seq, buf) => {
                            if got_seq == seq {
                                target = Some(buf);
                            } else if self.keep_seqs.remove(&got_seq).is_some() {
                                self.reply_cache.insert(got_seq, buf);
                            }
                            self.seq_count.record_seen(got_seq);
                        }
                        ReadResult::Error(got_seq, buf) => {
                            crate::debug!("Got err {:?}", parse_error(&buf, &self.extensions)?);
                            if got_seq == seq {
                                target = Some(buf);
                            } else if self.keep_seqs.remove(&got_seq).is_some() {
                                self.reply_cache.insert(got_seq, buf);
                            }
                            self.seq_count.record_seen(got_seq);
                        }
                    }
                }
            }
            Ok(unsafe { target.unwrap_unchecked() })
        }
    }

    #[inline]
    fn block_check_err<IO: SocketIo>(&mut self, io: &mut IO, seq: u16) -> Result<(), Error> {
        if !self.seq_count.sequence_has_been_seen(seq) {
            get_input_focus(io, self, false)?.reply(io, self)?;
        }
        if let Some(err) = self.reply_cache.remove(&seq) {
            Err(Error::XcbError(parse_error(&err, &self.extensions)?))
        } else {
            self.keep_seqs.remove(&seq);
            Ok(())
        }
    }

    #[inline]
    fn forget(&mut self, seq: u16) {
        self.keep_seqs.remove(&seq);
        self.reply_cache.remove(&seq);
    }
}

#[inline]
fn read_next<IO: SocketIo>(io: &mut IO) -> Result<Vec<ReadResult>, ConnectionError> {
    io.block_for_more_data().map_err(ConnectionError::Io)?;
    Ok(do_drain(io))
}

#[inline]
fn do_drain<IO: SocketIo>(io: &mut IO) -> Vec<ReadResult> {
    let mut read_results = vec![];
    io.use_read_buffer(|read_buf| {
        let mut offset = 0;
        while let Some((new_offset, rr)) = drain_next(read_buf, offset) {
            read_results.push(rr);
            offset = new_offset;
        }
        Ok::<usize, ()>(offset)
    });

    read_results
}

#[allow(clippy::match_on_vec_items)]
#[inline]
fn drain_next(in_buffer: &[u8], offset: usize) -> Option<(usize, ReadResult)> {
    let has_length_field = match in_buffer.get(offset) {
        Some(&REPLY) => true,
        Some(x) if x & 0x7f == xcb_rust_protocol::proto::xproto::GE_GENERIC_EVENT => true,
        _ => false,
    };
    let additional_length = if has_length_field {
        if let Some(length_field) = in_buffer.get(offset + 4..offset + 8) {
            let length_field = u32::from_ne_bytes(length_field.try_into().unwrap());
            let length_field = usize::try_from(length_field).unwrap();
            debug_assert!(length_field <= usize::MAX / 4);
            4 * length_field
        } else {
            0
        }
    } else {
        0
    };
    // All packets are at least 32 bytes
    let packet_length = 32 + additional_length;
    if in_buffer[offset..].len() < packet_length {
        // Need more data
        None
    } else {
        // Got at least one full packet
        let end_at = offset + packet_length;
        let slice = &in_buffer[offset..end_at];
        let read_result = match in_buffer[offset] {
            ERROR => ReadResult::Error(
                parse_seq(&in_buffer[offset..]),
                in_buffer[offset..end_at].to_vec(),
            ),
            REPLY => ReadResult::Reply(
                parse_seq(&in_buffer[offset..]),
                in_buffer[offset..end_at].to_vec(),
            ),
            _ => ReadResult::Event(in_buffer[offset..end_at].to_vec()),
        };
        Some((end_at, read_result))
    }
}

const ERROR: u8 = 0;
const REPLY: u8 = 1;

enum ReadResult {
    Event(Vec<u8>),
    Reply(u16, Vec<u8>),
    Error(u16, Vec<u8>),
}

#[inline]
fn parse_seq(raw_reply: &[u8]) -> u16 {
    // The seq is at the same byte offset for both replies and errors
    u16::from_ne_bytes(raw_reply[2..4].try_into().unwrap())
}
