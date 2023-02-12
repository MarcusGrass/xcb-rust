use crate::con::{SocketIo, XcbState};
use core::marker::PhantomData;

use crate::util::{FixedLengthFromBytes, VariableLengthFromBytes};

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
    pub fn reply<IO: SocketIo, XS: XcbState>(
        self,
        io: &mut IO,
        state: &mut XS,
    ) -> crate::Result<REPLY> {
        let reply_buf = state.block_for_reply(io, self.seq)?;
        let (reply, _offset) = REPLY::from_bytes(&reply_buf)?;
        Ok(reply)
    }

    #[inline]
    pub fn forget<XS>(self, state: &mut XS)
    where
        XS: XcbState,
    {
        state.forget(self.seq);
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
    pub fn reply<IO: SocketIo, XS: XcbState>(
        self,
        io: &mut IO,
        state: &mut XS,
    ) -> crate::Result<REPLY> {
        let reply_buf = state.block_for_reply(io, self.seq)?;
        let reply = REPLY::from_bytes(&reply_buf)?;
        Ok(reply)
    }

    #[inline]
    pub fn forget<XS: XcbState>(self, state: &mut XS) {
        state.forget(self.seq);
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
    pub fn check<IO: SocketIo, XS: XcbState>(
        self,
        io: &mut IO,
        state: &mut XS,
    ) -> crate::Result<()> {
        state.block_check_err(io, self.seq)?;
        Ok(())
    }
    #[inline]
    pub fn forget<XS>(self, state: &mut XS)
    where
        XS: XcbState,
    {
        state.forget(self.seq);
    }
}
