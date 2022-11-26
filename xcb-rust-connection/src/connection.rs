use alloc::collections::VecDeque;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};
use core::time::Duration;
use rusl::platform::TimeSpec;
use rusl::select::{PollEvents, PollFd};
use rusl::EAGAIN;

use smallmap::{Map, Set};
use tiny_std::io::{AsRawFd, Read, Write};
use tiny_std::net::UnixStream;
use tiny_std::time::MonotonicInstant;
use tiny_std::unix::fd::{OwnedFd, RawFd};

use xcb_rust_protocol::connection::xc_misc::XcMiscConnection;
use xcb_rust_protocol::connection::xproto::XprotoConnection;
use xcb_rust_protocol::cookie::VoidCookie;
use xcb_rust_protocol::proto::xproto::{Atom, ListExtensionsReply, PropModeEnum, Setup, Window};
use xcb_rust_protocol::util::{
    parse_error, ExtensionInfoProvider, ExtensionInformation, VariableLengthFromBytes,
    XcbErrorHandler,
};
use xcb_rust_protocol::{Error, XcbConnection, XcbEnv};

use crate::helpers::basic_info_provider::BasicExtensionInfoProvider;
use crate::helpers::connect::Connect;
use crate::helpers::id_allocator::IdAllocator;
use crate::helpers::xauth::Family;
use crate::{ConnectError, ConnectionError};

#[derive(Debug)]
pub struct SocketConnection {
    buf: SocketBuffer,
    setup: Setup,
    seq_count: SeqCount,
    event_cache: VecDeque<Vec<u8>>,
    reply_cache: Map<u16, Vec<u8>>,
    keep_seqs: Set<u16>,
    id_allocator: IdAllocator,
    max_request_length: usize,
    pub extensions: BasicExtensionInfoProvider,
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

impl SocketConnection {
    pub fn connect(dpy_name: Option<&str>, xcb_env: XcbEnv) -> Result<(Self, usize), ConnectError> {
        // Parse display information
        let parsed_display = crate::helpers::parse_display::parse_display(dpy_name, xcb_env)
            .ok_or(ConnectError::DisplayParsingError)?;
        let screen: usize = parsed_display.screen.into();
        // Establish connection by iterating over ConnectAddresses until we find one that
        // works.
        let mut error = None;

        if let Some(path) = parsed_display.connect_instruction() {
            match tiny_std::net::UnixStream::connect(&path, false) {
                Ok(mut socket) => {
                    crate::debug!("Socket connected");
                    let family = Family::LOCAL;
                    let host =
                        tiny_std::env::host_name().unwrap_or_else(|_| "localhost".to_string());
                    let (mut connect, setup_request) =
                        Connect::new(xcb_env, family, host.as_bytes(), parsed_display.display)?;
                    // write the connect() setup request
                    let mut nwritten = 0;
                    socket
                        .write_all(&setup_request)
                        .map_err(ConnectError::StdError)?;
                    crate::debug!("Sent setup request");
                    // read the setup
                    while connect.needs_bytes() {
                        if !poll_readable(socket.as_raw_fd(), Duration::from_secs(10))? {
                            crate::debug!("Failed to read bytes out of socket, not readable within 10 seconds");
                            return Err(ConnectError::UnknownError);
                        }
                        let read = socket.read_exact(connect.buffer())?;
                        connect.advance();
                    }

                    // resolve the setup
                    let setup = connect.into_setup()?;
                    crate::debug!("Setup completed.");

                    // Check that we got a valid screen number
                    if screen >= setup.roots.len() {
                        return Err(ConnectError::InvalidScreen);
                    }
                    let mut con = SocketConnection::new(SocketBuffer::new(socket), setup);
                    con.init_extensions().map_err(|e| {
                        crate::debug!("Error init exts {e}");
                        ConnectError::UnknownError
                    })?;
                    con.check_for_big_req().map_err(|e| {
                        crate::debug!("Error check big_req {e}");
                        ConnectError::UnknownError
                    })?;
                    return Ok((con, screen));
                }
                Err(e) => {
                    error = Some(e);
                }
            }
        }
        // none of the addresses worked
        Err(match error {
            Some(e) => ConnectError::StdError(e),
            None => ConnectError::DisplayParsingError,
        })
    }

    #[cfg(feature = "debug")]
    pub fn clear_cache(&mut self) -> Result<(), ConnectionError> {
        if self.keep_seqs.is_empty() && self.reply_cache.is_empty() {
            return Ok(());
        }
        if !self.keep_seqs.is_empty() {
            let _ = self.get_input_focus(false)?.reply(self)?;
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

    pub fn read_next_event(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<Vec<u8>>, ConnectionError> {
        if let Some(cached) = self.event_cache.pop_front() {
            Ok(Some(cached))
        } else {
            let start = MonotonicInstant::now();
            let mut got_event = false;
            while start.elapsed() < timeout && !got_event {
                let timeout = if let Some(time) = timeout.checked_sub(start.elapsed()) {
                    time
                } else {
                    return Ok(None);
                };
                if poll_readable(self.buf.sock.as_raw_fd(), timeout)? {
                    for rr in self.buf.read_next()? {
                        match rr {
                            ReadResult::Event(e) => {
                                got_event = true;
                                self.event_cache.push_back(e);
                            }
                            ReadResult::Reply(seq, buf) => {
                                crate::debug!("Got reply on seq {seq}");
                                if self.keep_seqs.remove(&seq).is_some() {
                                    self.reply_cache.insert(seq, buf);
                                }
                                self.seq_count.record_seen(seq);
                            }
                            ReadResult::Error(seq, buf) => {
                                crate::debug!("Got err {:?}", parse_error(&buf, &self.extensions));
                                if self.keep_seqs.remove(&seq).is_some() {
                                    self.reply_cache.insert(seq, buf);
                                }
                                self.seq_count.record_seen(seq);
                            }
                        }
                    }
                }
            }
            Ok(self.event_cache.pop_front())
        }
    }

    pub(crate) fn extension_information(&self, name: &'static str) -> Option<ExtensionInformation> {
        self.extensions.get_by_name(name)
    }

    // Preload all extensions immediately
    fn init_extensions(&mut self) -> Result<(), ConnectionError> {
        let listed = self.list_extensions(false)?;
        let r = self.block_for_reply(listed.seq)?;
        let (reply, offset) = ListExtensionsReply::from_bytes(&r)?;
        let mut extensions = vec![];
        for name in reply.names {
            let cookie = xcb_rust_protocol::connection::xproto::XprotoConnection::query_extension(
                self, &name.name, false,
            )?;
            extensions.push((name.name, cookie));
        }
        crate::debug!("Pushed all {} ext requests", extensions.len());
        self.buf.flush()?;
        for (name, cookie) in extensions {
            let response = cookie.reply(self)?;
            let name = String::from_utf8(name).map_err(|e| {
                crate::debug!("Failed string convert {e}");
                ConnectionError::UnsupportedExtension(format!(
                    "Failed to convert extension name {e}"
                ))
            })?;
            if let Some(ext) = xcb_rust_protocol::proto::find_extension(&name) {
                crate::debug!("Registered extension: '{ext}'");
                if response.present == 1 {
                    self.extensions.extensions.push((
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

    fn check_for_big_req(&mut self) -> Result<(), ConnectionError> {
        if self
            .extension_information(xcb_rust_protocol::proto::bigreq::EXTENSION_NAME)
            .is_some()
        {
            let reply =
                xcb_rust_protocol::connection::bigreq::BigreqConnection::enable(self, false)?
                    .reply(self)
                    .unwrap();
            self.max_request_length = reply.maximum_request_length as usize;
            crate::debug!(
                "Got max_request_length = {} words from bigreq",
                self.max_request_length
            );
        }

        Ok(())
    }
    pub fn change_property8(
        &mut self,
        mode: PropModeEnum,
        window: Window,
        property: Atom,
        type_: Atom,
        data: &[u8],
        forget: bool,
    ) -> Result<VoidCookie, ConnectionError> {
        Ok(self.change_property(
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
    pub fn change_property16(
        &mut self,
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
        Ok(self.change_property(
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
    pub fn change_property32(
        &mut self,
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
        Ok(self.change_property(
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

    #[inline]
    pub fn flush(&mut self) -> Result<(), ConnectionError> {
        self.buf.flush()
    }

    #[inline]
    pub fn sync(&mut self) -> Result<(), ConnectionError> {
        crate::debug!("Syncing");
        self.get_input_focus(false)?.reply(self)?;
        Ok(())
    }

    fn new(buf: SocketBuffer, setup: Setup) -> Self {
        Self {
            max_request_length: setup.maximum_request_length as usize, // It's the length in 32bit words
            buf,
            id_allocator: IdAllocator::new(setup.resource_id_base, setup.resource_id_mask).unwrap(),
            setup,
            seq_count: SeqCount::new(),
            event_cache: VecDeque::new(),
            reply_cache: Map::new(),
            keep_seqs: Set::new(),
            extensions: BasicExtensionInfoProvider::default(),
        }
    }
}

impl XcbConnection for SocketConnection {
    #[inline]
    fn write_buf(&mut self) -> &mut [u8] {
        &mut self.buf.out_buf[self.buf.out_offset..]
    }

    #[inline]
    fn max_request_size(&self) -> usize {
        self.max_request_length
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
    fn forget(&mut self, seq: u16) {
        self.keep_seqs.remove(&seq);
        self.reply_cache.remove(&seq);
    }

    #[inline]
    fn advance_reader(&mut self, step: usize) {
        crate::debug!("Reader advanced {step}");
        self.buf.in_write_offset += step;
    }

    #[inline]
    fn advance_writer(&mut self, step: usize) {
        self.buf.out_offset += step;
    }

    #[inline]
    fn generate_id(&mut self) -> Result<u32, Error> {
        if let Some(id) = self.id_allocator.generate_id() {
            Ok(id)
        } else if self
            .extension_information(xcb_rust_protocol::proto::xc_misc::EXTENSION_NAME)
            .is_none()
        {
            // IDs are exhausted and XC-MISC is not available
            Err(Error::Connection("Ids exhausted and xc-misk not available"))
        } else {
            let range = self.get_x_i_d_range(false)?.reply(self)?;

            self.id_allocator
                .update_xid_range(&range)
                .map_err(|_| Error::Connection("Ids exhausted on server"))?;
            self.id_allocator
                .generate_id()
                .ok_or(Error::Connection("Ids exhausted"))
        }
    }

    fn block_for_reply(&mut self, seq: u16) -> Result<Vec<u8>, Error> {
        if let Some(reply) = self.reply_cache.remove(&seq) {
            Ok(reply)
        } else {
            self.buf.flush().map_err(|e| {
                crate::debug!("Failed to flush, {e}");
                Error::Connection("Failed flush")
            })?;
            let mut target = None;
            self.keep_seqs.remove(&seq);
            while target.is_none() {
                for rr in self.buf.read_next().map_err(|e| {
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
            Ok(target.unwrap())
        }
    }

    fn block_check_for_err(&mut self, seq: u16) -> Result<(), Error> {
        if !self.seq_count.sequence_has_been_seen(seq) {
            self.get_input_focus(false)?.reply(self)?;
        }
        if let Some(err) = self.reply_cache.remove(&seq) {
            Err(Error::XcbError(parse_error(&err, &self.extensions)?))
        } else {
            self.keep_seqs.remove(&seq);
            Ok(())
        }
    }

    #[inline]
    fn major_opcode(&self, extension_name: &'static str) -> Option<u8> {
        self.extension_information(extension_name)
            .map(|ei| ei.major_opcode)
    }

    fn setup(&self) -> &Setup {
        &self.setup
    }
}

const BUF_SIZE: usize = 65536;

#[derive(Debug)]
struct SocketBuffer {
    sock: UnixStream,
    in_buf: Vec<u8>,
    in_read_offset: usize,
    in_write_offset: usize,
    out_buf: Vec<u8>,
    out_offset: usize,
}

const ERROR: u8 = 0;
const REPLY: u8 = 1;

enum ReadResult {
    Event(Vec<u8>),
    Reply(u16, Vec<u8>),
    Error(u16, Vec<u8>),
}

impl SocketBuffer {
    pub fn flush(&mut self) -> Result<(), ConnectionError> {
        if self.out_offset != 0 {
            let mut written = 0;
            loop {
                return match self.sock.write_all(&self.out_buf[written..self.out_offset]) {
                    Ok(_) => {
                        self.out_offset = 0;
                        Ok(())
                    }
                    Err(e) => {
                        match &e {
                            tiny_std::error::Error::Uncategorized(_) => {}
                            tiny_std::error::Error::Os { code, .. } => {
                                if *code == EAGAIN {
                                    continue;
                                }
                            }
                        }
                        Err(ConnectionError::StdErr(e))
                    }
                };
            }
        }
        Ok(())
    }

    fn read_next(&mut self) -> Result<Vec<ReadResult>, ConnectionError> {
        let mut start = self.in_write_offset;
        // drain
        loop {
            match self.sock.read(&mut self.in_buf[self.in_write_offset..]) {
                Ok(o) => {
                    self.in_write_offset += o;
                    break;
                }
                Err(e) => {
                    match &e {
                        tiny_std::error::Error::Uncategorized(e) => {}
                        tiny_std::error::Error::Os { code, msg } => match *code {
                            rusl::EINTR => continue,
                            rusl::EAGAIN => break,
                            _ => {}
                        },
                    }
                    return Err(ConnectionError::StdErr(e));
                }
            }
        }

        let mut read_results = vec![];
        while let Some(rr) = self.drain_next() {
            read_results.push(rr);
        }
        let remainder_len = self.in_write_offset - self.in_read_offset;
        self.in_buf
            .copy_within(self.in_read_offset..self.in_write_offset, 0);
        self.in_read_offset = 0;
        self.in_write_offset = remainder_len;
        Ok(read_results)
    }

    #[allow(clippy::match_on_vec_items)]
    fn drain_next(&mut self) -> Option<ReadResult> {
        let has_length_field = match self.in_buf.get(self.in_read_offset) {
            Some(&REPLY) => true,
            Some(x) if x & 0x7f == xcb_rust_protocol::proto::xproto::GE_GENERIC_EVENT => true,
            _ => false,
        };
        let additional_length = if has_length_field {
            if let Some(length_field) = self
                .in_buf
                .get(self.in_read_offset + 4..self.in_read_offset + 8)
            {
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
        if self.in_write_offset - self.in_read_offset < packet_length {
            // Need more data
            None
        } else {
            // Got at least one full packet
            let end_at = self.in_read_offset + packet_length;
            let read_result = match self.in_buf[self.in_read_offset] {
                ERROR => ReadResult::Error(
                    parse_seq(&self.in_buf[self.in_read_offset..]),
                    self.in_buf[self.in_read_offset..end_at].to_vec(),
                ),
                REPLY => ReadResult::Reply(
                    parse_seq(&self.in_buf[self.in_read_offset..]),
                    self.in_buf[self.in_read_offset..end_at].to_vec(),
                ),
                _ => ReadResult::Event(self.in_buf[self.in_read_offset..end_at].to_vec()),
            };
            self.in_read_offset = end_at;
            Some(read_result)
        }
    }

    pub fn new(sock: UnixStream) -> Self {
        Self {
            sock,
            in_buf: vec![0; BUF_SIZE],
            in_read_offset: 0,
            in_write_offset: 0,
            out_buf: vec![0; BUF_SIZE],
            out_offset: 0,
        }
    }
}

#[inline]
fn parse_seq(raw_reply: &[u8]) -> u16 {
    // The seq is at the same byte offset for both replies and errors
    u16::from_ne_bytes(raw_reply[2..4].try_into().unwrap())
}

#[inline]
fn poll_readable(fd: RawFd, deadline: Duration) -> Result<bool, rusl::Error> {
    let start_instant = MonotonicInstant::now();
    let mut pollfds = [PollFd::new(fd, PollEvents::POLLIN)];
    loop {
        if let Some(timeout) = deadline.checked_sub(start_instant.elapsed()) {
            match rusl::select::ppoll(&mut pollfds, Some(&timeout.try_into()?), None) {
                Ok(_) => {
                    return Ok(true);
                }
                Err(ref e) => {
                    if matches!(e.code, Some(rusl::EINTR)) {
                        continue;
                    }
                }
                Err(e) => return Err(e),
            }
        } else {
            return Ok(false);
        }
        if start_instant.elapsed() >= deadline {
            return Ok(false);
        }
    }
}
