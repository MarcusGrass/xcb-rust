use alloc::vec::Vec;

use crate::proto::request_name;
use crate::Error;

pub trait VariableLengthFromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> crate::Result<(Self, usize)>;
}

pub trait VariableLengthSerialize {
    fn serialize_into(self, buf: &mut [u8]) -> crate::Result<usize>;
}

pub trait FixedLengthFromBytes<const N: usize>: Sized {
    fn from_bytes(bytes: &[u8]) -> crate::Result<Self>;
}

impl<const N: usize> FixedLengthFromBytes<N> for [u8; N] {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        unsafe {
            Ok(bytes
                .get(..N)
                .ok_or(Error::FromBytes)?
                .try_into()
                .unwrap_unchecked())
        }
    }
}

pub trait FixedLengthSerialize<const N: usize> {
    fn serialize_fixed(self) -> [u8; N];
}

impl<const N: usize> FixedLengthSerialize<N> for [u8; N] {
    #[inline]
    fn serialize_fixed(self) -> [u8; N] {
        self
    }
}

impl<const N: usize, FLS> FixedLengthSerialize<N> for &FLS
where
    FLS: FixedLengthSerialize<N> + Copy,
{
    #[inline]
    fn serialize_fixed(self) -> [u8; N] {
        (*self).serialize_fixed()
    }
}

#[inline]
pub fn u8_vec_from_bytes(bytes: &[u8], count: usize) -> crate::Result<Vec<u8>> {
    Ok(bytes.get(..count).ok_or(Error::FromBytes)?.to_vec())
}

#[macro_export]
macro_rules! vec_from_bytes_fixed {
    ($buf: expr, $entity: ty, $count: expr, $size: expr) => {
        {
            let mut __v = alloc::vec::Vec::with_capacity($count);
            for i in 0..$count {
                __v.push(<$entity>::from_bytes($buf.get(i * $size..).ok_or($crate::error::Error::FromBytes)?)?);
            }
            __v
        }
    };
    ($buf: expr, $entity: ty, $count: expr, $size: expr, $($arg: expr)*) => {
        {
            let mut __v = alloc::vec::Vec::with_capacity($count);
            for i in 0..$count {
                __v.push(<$entity>::from_bytes($buf.get(i * $size..).ok_or($crate::error::Error::FromBytes)?,
                    $($arg,)*
                )?);
            }
            __v
        }
    };
}

#[inline]
pub fn u8_vec_serialize_into(buf: &mut [u8], bytes: &[u8]) -> crate::Result<()> {
    buf.get_mut(0..bytes.len())
        .ok_or(Error::Serialize)?
        .copy_from_slice(bytes);
    Ok(())
}

#[inline]
pub fn fixed_vec_serialize_into<const N: usize, FLS: FixedLengthSerialize<N>>(
    buf: &mut [u8],
    items: impl IntoIterator<Item = FLS>,
) -> crate::Result<()> {
    for (i, item) in items.into_iter().enumerate() {
        buf.get_mut(i * N..i * N + N)
            .ok_or(Error::Serialize)?
            .copy_from_slice(&item.serialize_fixed());
    }
    Ok(())
}

#[macro_export]
macro_rules! vec_from_bytes_var {
    ($buf: expr, $entity: ty, $offset: expr, $count: expr) => {
        {
            let mut __v = alloc::vec::Vec::with_capacity($count);
            for _ in 0..$count {
                let (item, __new_offset) = <$entity>::from_bytes($buf.get($offset..).ok_or($crate::error::Error::FromBytes)?)?;
                $offset += __new_offset;
                __v.push(item);
            }
            (__v)
        }
    };
    ($buf: expr, $entity: ty, $offset: expr, $count: expr, $($arg: expr)*) => {
        {
            let mut __v = alloc::vec::Vec::with_capacity($count);
            for _ in 0..$count {
                let (item, __new_offset) = <$entity>::from_bytes($buf.get($offset..).ok_or($crate::error::Error::FromBytes)?,
                    $($arg,)*
                )?;
                $offset += __new_offset;
                __v.push(item);
            }
            __v
        }
    };
}

#[inline]
pub fn var_vec_serialize_into<VLS: VariableLengthSerialize>(
    buf: &mut [u8],
    items: impl IntoIterator<Item = VLS>,
) -> crate::Result<usize> {
    let mut offset = 0;
    for item in items {
        offset += item.serialize_into(buf.get_mut(offset..).ok_or(crate::Error::Serialize)?)?;
    }
    Ok(offset)
}

impl FixedLengthSerialize<0> for () {
    fn serialize_fixed(self) -> [u8; 0] {
        []
    }
}

impl FixedLengthFromBytes<0> for () {
    fn from_bytes(_: &[u8]) -> crate::Result<Self> {
        Ok(())
    }
}

macro_rules! implement_num_serde {
    ($num_type: ty, $byte_size: expr) => {
        impl FixedLengthSerialize<$byte_size> for $num_type {
            #[inline]
            fn serialize_fixed(self) -> [u8; $byte_size] {
                self.to_ne_bytes()
            }
        }
        impl FixedLengthFromBytes<$byte_size> for $num_type {
            #[inline]
            fn from_bytes(bytes: &[u8]) -> $crate::Result<Self> {
                #[cfg(feature = "debug")]
                let byte_buf = bytes
                    .get(..$byte_size)
                    .ok_or_else(|| crate::Error::FromBytes)?;
                #[cfg(not(feature = "debug"))]
                let byte_buf = bytes.get(..$byte_size).ok_or(crate::Error::FromBytes)?;
                Ok(Self::from_ne_bytes(
                    bytes
                        .get(..$byte_size)
                        .ok_or(crate::Error::FromBytes)?
                        .try_into()
                        .unwrap(),
                ))
            }
        }
    };
}

implement_num_serde!(u8, 1);
implement_num_serde!(u16, 2);
implement_num_serde!(u32, 4);
implement_num_serde!(u64, 8);
implement_num_serde!(i8, 1);
implement_num_serde!(i16, 2);
implement_num_serde!(i32, 4);
implement_num_serde!(i64, 8);

implement_num_serde!(f32, 4);
implement_num_serde!(f64, 8);

#[macro_export]
macro_rules! implement_bit_ops {
    ($kind: ty) => {
        impl core::ops::BitAnd for $kind {
            type Output = Self;

            #[inline]
            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl core::ops::BitOr for $kind {
            type Output = Self;

            #[inline]
            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl core::ops::BitAndAssign for $kind {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl core::ops::BitOrAssign for $kind {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0
            }
        }
    };
}

pub trait XcbErrorHandler {
    fn parse_error(&mut self) -> crate::Result<XcbError>;
}

pub fn parse_error<E>(buf: &[u8], ext: &E) -> crate::Result<XcbError>
where
    E: ExtensionInfoProvider,
{
    let opcode = *buf.get(1).ok_or(Error::FromBytes)?;
    let sequence = u16::from_ne_bytes(buf.get(2..4).ok_or(Error::FromBytes)?.try_into()?);
    let bad_value = u32::from_ne_bytes(buf.get(4..8).ok_or(Error::FromBytes)?.try_into()?);
    let minor_opcode = u16::from_ne_bytes(buf.get(8..10).ok_or(Error::FromBytes)?.try_into()?);
    let major_opcode = *buf.get(10).ok_or(Error::FromBytes)?;
    let ext_name = ext.get_from_major_opcode(major_opcode).map(|e| e.0);
    let err = XcbError {
        opcode,
        sequence,
        bad_value,
        minor_opcode,
        major_opcode,
        extension_name: ext_name,
        request_name: request_name(ext_name, major_opcode, minor_opcode),
    };
    Ok(err)
}

#[derive(Debug, Copy, Clone)]
pub struct XcbError {
    pub opcode: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
    pub extension_name: Option<&'static str>,
    pub request_name: Option<&'static str>,
}

pub trait ExtensionInfoProvider {
    fn get_by_name(&self, name: &str) -> Option<ExtensionInformation>;

    fn get_from_major_opcode(
        &self,
        major_opcode: u8,
    ) -> Option<(&'static str, ExtensionInformation)>;

    fn get_from_event_code(&self, event_code: u8) -> Option<(&'static str, ExtensionInformation)>;

    fn get_from_error_code(&self, error_code: u8) -> Option<(&'static str, ExtensionInformation)>;
}

/// Information about a X11 extension.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ExtensionInformation {
    /// Major opcode used in request
    pub major_opcode: u8,
    /// Lowest event number used by the extension.
    pub first_event: u8,
    /// Lowest error number used by the extension.
    pub first_error: u8,
}

/// Get the response type out of the raw bytes of an X11 error or event.
#[inline]
pub fn response_type(raw_bytes: &[u8]) -> Result<u8, Error> {
    raw_bytes.first().map(|x| x & 0x7f).ok_or(Error::FromBytes)
}

pub struct Iter32<'a> {
    ind: usize,
    inner: &'a [u8],
}

impl<'a> Iterator for Iter32<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let target = self.ind * 4;
        self.inner.get(target..target + 4).map(|buf| {
            self.ind += 1;
            unsafe { u32::from_ne_bytes(buf.try_into().unwrap_unchecked()) }
        })
    }
}

pub trait AsIter32 {
    fn as_iter_32(&self) -> Iter32;
}

impl<const N: usize> AsIter32 for [u8; N] {
    fn as_iter_32(&self) -> Iter32 {
        Iter32 {
            ind: 0,
            inner: self,
        }
    }
}

impl<'a> AsIter32 for &'a [u8] {
    fn as_iter_32(&self) -> Iter32 {
        Iter32 {
            ind: 0,
            inner: self,
        }
    }
}
