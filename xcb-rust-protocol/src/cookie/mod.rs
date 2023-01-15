use core::marker::PhantomData;
use crate::con::XcbBuffers;

use crate::util::{FixedLengthFromBytes, VariableLengthFromBytes};
use crate::XcbConnection;

#[derive(Copy, Clone, Debug)]
pub struct Cookie<REPLY>
where
    REPLY: VariableLengthFromBytes,
{
    pub seq: u16,
    pub(crate) _data: PhantomData<REPLY>,
}

impl<REPLY> Cookie<REPLY>
where
    REPLY: VariableLengthFromBytes,
{
    #[inline]
    #[must_use]
    pub(crate) fn new(seq: u16) -> Self {
        Cookie {
            seq,
            _data: PhantomData::default(),
        }
    }

    #[inline]
    pub fn reply<C>(self, con: &mut C, buffers: &mut XcbBuffers) -> crate::Result<REPLY>
    where
        C: XcbConnection,
    {
        let reply_buf = con.block_for_reply(buffers, self.seq)?;
        let (reply, _offset) = REPLY::from_bytes(&reply_buf)?;
        Ok(reply)
    }

    #[inline]
    pub fn forget<C>(self, con: &mut C)
    where
        C: XcbConnection,
    {
        con.forget(self.seq);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FixedCookie<REPLY, const FIXED_SIZE: usize>
where
    REPLY: FixedLengthFromBytes<FIXED_SIZE>,
{
    pub seq: u16,
    pub(crate) _data: PhantomData<REPLY>,
}

impl<REPLY, const FIXED_SIZE: usize> FixedCookie<REPLY, FIXED_SIZE>
where
    REPLY: FixedLengthFromBytes<FIXED_SIZE>,
{
    #[inline]
    #[must_use]
    pub(crate) fn new(seq: u16) -> Self {
        FixedCookie {
            seq,
            _data: PhantomData::default(),
        }
    }

    #[inline]
    pub fn reply<C>(self, con: &mut C, buffers: &mut XcbBuffers) -> crate::Result<REPLY>
    where
        C: XcbConnection,
    {
        let reply_buf = con.block_for_reply(buffers, self.seq)?;
        let reply = REPLY::from_bytes(&reply_buf)?;
        Ok(reply)
    }
    #[inline]
    pub fn forget<C>(self, con: &mut C)
    where
        C: XcbConnection,
    {
        con.forget(self.seq);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct VoidCookie {
    pub seq: u16,
}

impl VoidCookie {
    #[inline]
    #[must_use]
    pub(crate) fn new(seq: u16) -> Self {
        Self { seq }
    }

    #[inline]
    pub fn check<C>(self, con: &mut C, buffers: &mut XcbBuffers) -> crate::Result<()>
    where
        C: XcbConnection,
    {
        con.block_check_for_err(buffers, self.seq)?;
        Ok(())
    }
    #[inline]
    pub fn forget<C>(self, con: &mut C)
    where
        C: XcbConnection,
    {
        con.forget(self.seq);
    }
}
