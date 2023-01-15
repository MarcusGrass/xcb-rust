use alloc::vec::Vec;

use crate::proto::xproto::Setup;
use crate::Error;

#[derive(Debug)]
pub struct XcbBuffers<'a> {
    pub in_buffer: &'a mut [u8],
    pub out_buffer: &'a mut [u8],
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

