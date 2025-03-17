use alloc::vec::Vec;
use core::fmt::Debug;

use crate::proto::xproto::Setup;
use crate::Error;

#[derive(Debug)]
pub struct XcbBuffers {
    pub in_buffer: XcbBuffer,
    pub out_buffer: XcbBuffer,
}

#[derive(Debug)]
pub struct XcbBuffer {
    pub bytes: Vec<u8>,
    pub consumed_offset: usize,
    pub committed_offset: usize,
}

impl XcbBuffer {
    #[inline]
    pub fn at_offset(&mut self) -> &mut [u8] {
        &mut self.bytes[self.committed_offset..]
    }

    #[inline]
    pub fn advance_committed(&mut self, step: usize) {
        self.committed_offset += step;
    }

    #[inline]
    pub fn advance_consumed(&mut self, step: usize) {
        self.consumed_offset += step;
    }

    #[inline]
    pub fn unwritten_space(&mut self) -> usize {
        self.committed_offset - self.consumed_offset
    }

    #[inline]
    pub fn sync_offsets(&mut self) {
        self.consumed_offset = self.committed_offset;
    }

    #[inline]
    pub fn clear_offsets(&mut self) {
        self.consumed_offset = 0;
        self.committed_offset = 0;
    }

    #[inline]
    #[must_use]
    pub fn space_left(&self) -> usize {
        self.bytes.len() - self.committed_offset
    }

    #[inline]
    #[must_use]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            consumed_offset: 0,
            committed_offset: 0,
        }
    }
}

pub trait SocketIo {
    /// Trigger a wait for new data
    fn block_for_more_data(&mut self) -> Result<(), &'static str>;
    /// Get a reference to a read buffer, function will return consumed bytes which can
    /// then be considered spent
    fn use_read_buffer<E, F: FnOnce(&[u8]) -> Result<usize, E>>(
        &mut self,
        read_op: F,
    ) -> Result<(), E>
    where
        E: Debug;
    /// Get a mutable reference to a buffer that should be dumped to the x11-socket
    /// function returns number of bytes written onto it.
    ///
    fn use_write_buffer<E, F: FnOnce(&mut [u8]) -> Result<usize, E>>(
        &mut self,
        write_op: F,
    ) -> Result<(), E>
    where
        E: Debug;
}

pub trait XcbState {
    fn major_opcode(&self, extension_name: &'static str) -> Option<u8>;
    fn next_seq(&mut self) -> u16;
    fn keep_and_return_next_seq(&mut self) -> u16;
    fn max_request_size(&self) -> usize;
    fn setup(&self) -> &Setup;
    fn generate_id<IO: SocketIo>(&mut self, io: &mut IO) -> Result<u32, Error>;
    fn block_for_reply<IO: SocketIo>(&mut self, io: &mut IO, seq: u16) -> Result<Vec<u8>, Error>;
    fn block_check_err<IO: SocketIo>(&mut self, io: &mut IO, seq: u16) -> Result<(), Error>;
    fn forget(&mut self, seq: u16);
}

/// A pre-sized buffer that can hold bytes
pub trait XcbConnection {
    /// It's up to the implementor to make sure that enough space exists in the buffer to write the bytes.
    fn apply_offset<'a>(&mut self, in_buffer: &'a mut [u8]) -> &'a mut [u8];
    /// With some large requests we need to rewrite the header (`BigReq`)
    fn max_request_size(&self) -> usize;
    /// Function that allows us to get a sequence number to be written into a request
    fn next_seq(&mut self) -> u16;
    fn keep_and_return_next_seq(&mut self) -> u16;
    fn forget(&mut self, seq: u16);
    /// Function that advances to buffer by `step` bytes
    fn advance_reader(&mut self, step: usize);
    fn advance_writer(&mut self, step: usize);
    fn generate_id(&mut self, buffers: &mut XcbBuffers) -> Result<u32, Error>;
    fn block_for_reply(&mut self, buffers: &mut XcbBuffers, seq: u16) -> Result<Vec<u8>, Error>;
    fn block_check_for_err(&mut self, buffers: &mut XcbBuffers, seq: u16) -> Result<(), Error>;
    fn major_opcode(&self, extension_name: &'static str) -> Option<u8>;
    fn setup(&self) -> &Setup;
}
