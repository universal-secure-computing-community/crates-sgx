//! Parsers recognizing numbers, streaming version

use crate::branch::alt;
use crate::character::streaming::{char, digit1};
use crate::combinator::{cut, map, opt, recognize};
use crate::error::{ErrorKind, ParseError};
use crate::internal::*;
use crate::lib::std::ops::{RangeFrom, RangeTo};
use crate::sequence::{pair, tuple};
use crate::traits::{AsChar, InputIter, InputLength, InputTakeAtPosition};
use crate::traits::{Offset, Slice};

/// Recognizes an unsigned 1 byte integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_u8;
///
/// let parser = |s| {
///   be_u8::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01abcd"[..]), Ok((&b"\x01abcd"[..], 0x00)));
/// assert_eq!(parser(&b""[..]), Err(Err::Incomplete(Needed::new(1))));
/// ```
#[inline]
pub fn be_u8<I, E: ParseError<I>>(input: I) -> IResult<I, u8, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  let bound: usize = 1;
  if input.input_len() < bound {
    Err(Err::Incomplete(Needed::new(1)))
  } else {
    let res = input.iter_elements().next().unwrap();

    Ok((input.slice(bound..), res))
  }
}

/// Recognizes a big endian unsigned 2 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_u16;
///
/// let parser = |s| {
///   be_u16::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01abcd"[..]), Ok((&b"abcd"[..], 0x0001)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(1))));
/// ```
#[inline]
pub fn be_u16<I, E: ParseError<I>>(input: I) -> IResult<I, u16, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  let bound: usize = 2;
  if input.input_len() < bound {
    Err(Err::Incomplete(Needed::new(bound - input.input_len())))
  } else {
    let mut res = 0u16;
    for byte in input.iter_elements().take(bound) {
      res = (res << 8) + byte as u16;
    }

    Ok((input.slice(bound..), res))
  }
}

/// Recognizes a big endian unsigned 3 byte integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_u24;
///
/// let parser = |s| {
///   be_u24::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02abcd"[..]), Ok((&b"abcd"[..], 0x000102)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(2))));
/// ```
#[inline]
pub fn be_u24<I, E: ParseError<I>>(input: I) -> IResult<I, u32, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  let bound: usize = 3;
  if input.input_len() < bound {
    Err(Err::Incomplete(Needed::new(bound - input.input_len())))
  } else {
    let mut res = 0u32;
    for byte in input.iter_elements().take(bound) {
      res = (res << 8) + byte as u32;
    }

    Ok((input.slice(bound..), res))
  }
}

/// Recognizes a big endian unsigned 4 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_u32;
///
/// let parser = |s| {
///   be_u32::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03abcd"[..]), Ok((&b"abcd"[..], 0x00010203)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(3))));
/// ```
#[inline]
pub fn be_u32<I, E: ParseError<I>>(input: I) -> IResult<I, u32, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  let bound: usize = 4;
  if input.input_len() < bound {
    Err(Err::Incomplete(Needed::new(bound - input.input_len())))
  } else {
    let mut res = 0u32;
    for byte in input.iter_elements().take(bound) {
      res = (res << 8) + byte as u32;
    }

    Ok((input.slice(bound..), res))
  }
}

/// Recognizes a big endian unsigned 8 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_u64;
///
/// let parser = |s| {
///   be_u64::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcd"[..]), Ok((&b"abcd"[..], 0x0001020304050607)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(7))));
/// ```
#[inline]
pub fn be_u64<I, E: ParseError<I>>(input: I) -> IResult<I, u64, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  let bound: usize = 8;
  if input.input_len() < bound {
    Err(Err::Incomplete(Needed::new(bound - input.input_len())))
  } else {
    let mut res = 0u64;
    for byte in input.iter_elements().take(bound) {
      res = (res << 8) + byte as u64;
    }

    Ok((input.slice(bound..), res))
  }
}

/// Recognizes a big endian unsigned 16 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_u128;
///
/// let parser = |s| {
///   be_u128::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15abcd"[..]), Ok((&b"abcd"[..], 0x00010203040506070809101112131415)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(15))));
/// ```
#[inline]
#[cfg(stable_i128)]
pub fn be_u128<I, E: ParseError<I>>(input: I) -> IResult<I, u128, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  let bound: usize = 16;
  if input.input_len() < bound {
    Err(Err::Incomplete(Needed::new(bound - input.input_len())))
  } else {
    let mut res = 0u128;
    for byte in input.iter_elements().take(bound) {
      res = (res << 8) + byte as u128;
    }

    Ok((input.slice(bound..), res))
  }
}

/// Recognizes a signed 1 byte integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_i8;
///
/// let parser = be_i8::<_, (_, ErrorKind)>;
///
/// assert_eq!(parser(&b"\x00\x01abcd"[..]), Ok((&b"\x01abcd"[..], 0x00)));
/// assert_eq!(parser(&b""[..]), Err(Err::Incomplete(Needed::new(1))));
/// ```
#[inline]
pub fn be_i8<I, E: ParseError<I>>(input: I) -> IResult<I, i8, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  map!(input, be_u8, |x| x as i8)
}

/// Recognizes a big endian signed 2 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_i16;
///
/// let parser = be_i16::<_, (_, ErrorKind)>;
///
/// assert_eq!(parser(&b"\x00\x01abcd"[..]), Ok((&b"abcd"[..], 0x0001)));
/// assert_eq!(parser(&b""[..]), Err(Err::Incomplete(Needed::new(2))));
/// ```
#[inline]
pub fn be_i16<I, E: ParseError<I>>(input: I) -> IResult<I, i16, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  map!(input, be_u16, |x| x as i16)
}

/// Recognizes a big endian signed 3 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_i24;
///
/// let parser = be_i24::<_, (_, ErrorKind)>;
///
/// assert_eq!(parser(&b"\x00\x01\x02abcd"[..]), Ok((&b"abcd"[..], 0x000102)));
/// assert_eq!(parser(&b""[..]), Err(Err::Incomplete(Needed::new(3))));
/// ```
#[inline]
pub fn be_i24<I, E: ParseError<I>>(input: I) -> IResult<I, i32, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  // Same as the unsigned version but we need to sign-extend manually here
  map!(input, be_u24, |x| if x & 0x80_00_00 != 0 {
    (x | 0xff_00_00_00) as i32
  } else {
    x as i32
  })
}

/// Recognizes a big endian signed 4 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_i32;
///
/// let parser = be_i32::<_, (_, ErrorKind)>;
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03abcd"[..]), Ok((&b"abcd"[..], 0x00010203)));
/// assert_eq!(parser(&b""[..]), Err(Err::Incomplete(Needed::new(4))));
/// ```
#[inline]
pub fn be_i32<I, E: ParseError<I>>(input: I) -> IResult<I, i32, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  map!(input, be_u32, |x| x as i32)
}

/// Recognizes a big endian signed 8 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_i64;
///
/// let parser = be_i64::<_, (_, ErrorKind)>;
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcd"[..]), Ok((&b"abcd"[..], 0x0001020304050607)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(7))));
/// ```
#[inline]
pub fn be_i64<I, E: ParseError<I>>(input: I) -> IResult<I, i64, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  map!(input, be_u64, |x| x as i64)
}

/// Recognizes a big endian signed 16 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_i128;
///
/// let parser = be_i128::<_, (_, ErrorKind)>;
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15abcd"[..]), Ok((&b"abcd"[..], 0x00010203040506070809101112131415)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(15))));
/// ```
#[inline]
#[cfg(stable_i128)]
pub fn be_i128<I, E: ParseError<I>>(input: I) -> IResult<I, i128, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  map!(input, be_u128, |x| x as i128)
}

/// Recognizes an unsigned 1 byte integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_u8;
///
/// let parser = le_u8::<_, (_, ErrorKind)>;
///
/// assert_eq!(parser(&b"\x00\x01abcd"[..]), Ok((&b"\x01abcd"[..], 0x00)));
/// assert_eq!(parser(&b""[..]), Err(Err::Incomplete(Needed::new(1))));
/// ```
#[inline]
pub fn le_u8<I, E: ParseError<I>>(input: I) -> IResult<I, u8, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  let bound: usize = 1;
  if input.input_len() < bound {
    Err(Err::Incomplete(Needed::new(1)))
  } else {
    let res = input.iter_elements().next().unwrap();

    Ok((input.slice(bound..), res))
  }
}

/// Recognizes a little endian unsigned 2 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_u16;
///
/// let parser = |s| {
///   le_u16::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01abcd"[..]), Ok((&b"abcd"[..], 0x0100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(1))));
/// ```
#[inline]
pub fn le_u16<I, E: ParseError<I>>(input: I) -> IResult<I, u16, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  let bound: usize = 2;
  if input.input_len() < bound {
    Err(Err::Incomplete(Needed::new(bound - input.input_len())))
  } else {
    let mut res = 0u16;
    for (index, byte) in input.iter_indices().take(bound) {
      res = res + ((byte as u16) << 8 * index);
    }

    Ok((input.slice(bound..), res))
  }
}

/// Recognizes a little endian unsigned 3 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_u24;
///
/// let parser = |s| {
///   le_u24::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02abcd"[..]), Ok((&b"abcd"[..], 0x020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(2))));
/// ```
#[inline]
pub fn le_u24<I, E: ParseError<I>>(input: I) -> IResult<I, u32, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  let bound: usize = 3;
  if input.input_len() < bound {
    Err(Err::Incomplete(Needed::new(bound - input.input_len())))
  } else {
    let mut res = 0u32;
    for (index, byte) in input.iter_indices().take(bound) {
      res = res + ((byte as u32) << 8 * index);
    }

    Ok((input.slice(bound..), res))
  }
}

/// Recognizes a little endian unsigned 4 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_u32;
///
/// let parser = |s| {
///   le_u32::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03abcd"[..]), Ok((&b"abcd"[..], 0x03020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(3))));
/// ```
#[inline]
pub fn le_u32<I, E: ParseError<I>>(input: I) -> IResult<I, u32, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  let bound: usize = 4;
  if input.input_len() < bound {
    Err(Err::Incomplete(Needed::new(bound - input.input_len())))
  } else {
    let mut res = 0u32;
    for (index, byte) in input.iter_indices().take(bound) {
      res = res + ((byte as u32) << 8 * index);
    }

    Ok((input.slice(bound..), res))
  }
}

/// Recognizes a little endian unsigned 8 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_u64;
///
/// let parser = |s| {
///   le_u64::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcd"[..]), Ok((&b"abcd"[..], 0x0706050403020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(7))));
/// ```
#[inline]
pub fn le_u64<I, E: ParseError<I>>(input: I) -> IResult<I, u64, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  let bound: usize = 8;
  if input.input_len() < bound {
    Err(Err::Incomplete(Needed::new(bound - input.input_len())))
  } else {
    let mut res = 0u64;
    for (index, byte) in input.iter_indices().take(bound) {
      res = res + ((byte as u64) << 8 * index);
    }

    Ok((input.slice(bound..), res))
  }
}

/// Recognizes a little endian unsigned 16 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_u128;
///
/// let parser = |s| {
///   le_u128::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15abcd"[..]), Ok((&b"abcd"[..], 0x15141312111009080706050403020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(15))));
/// ```
#[inline]
#[cfg(stable_i128)]
pub fn le_u128<I, E: ParseError<I>>(input: I) -> IResult<I, u128, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  let bound: usize = 16;
  if input.input_len() < bound {
    Err(Err::Incomplete(Needed::new(bound - input.input_len())))
  } else {
    let mut res = 0u128;
    for (index, byte) in input.iter_indices().take(bound) {
      res = res + ((byte as u128) << 8 * index);
    }

    Ok((input.slice(bound..), res))
  }
}

/// Recognizes a signed 1 byte integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_i8;
///
/// let parser = le_i8::<_, (_, ErrorKind)>;
///
/// assert_eq!(parser(&b"\x00\x01abcd"[..]), Ok((&b"\x01abcd"[..], 0x00)));
/// assert_eq!(parser(&b""[..]), Err(Err::Incomplete(Needed::new(1))));
/// ```
#[inline]
pub fn le_i8<I, E: ParseError<I>>(input: I) -> IResult<I, i8, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  map!(input, le_u8, |x| x as i8)
}

/// Recognizes a little endian signed 2 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_i16;
///
/// let parser = |s| {
///   le_i16::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01abcd"[..]), Ok((&b"abcd"[..], 0x0100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(1))));
/// ```
#[inline]
pub fn le_i16<I, E: ParseError<I>>(input: I) -> IResult<I, i16, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  map!(input, le_u16, |x| x as i16)
}

/// Recognizes a little endian signed 3 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_i24;
///
/// let parser = |s| {
///   le_i24::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02abcd"[..]), Ok((&b"abcd"[..], 0x020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(2))));
/// ```
#[inline]
pub fn le_i24<I, E: ParseError<I>>(input: I) -> IResult<I, i32, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  // Same as the unsigned version but we need to sign-extend manually here
  map!(input, le_u24, |x| if x & 0x80_00_00 != 0 {
    (x | 0xff_00_00_00) as i32
  } else {
    x as i32
  })
}

/// Recognizes a little endian signed 4 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_i32;
///
/// let parser = |s| {
///   le_i32::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03abcd"[..]), Ok((&b"abcd"[..], 0x03020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(3))));
/// ```
#[inline]
pub fn le_i32<I, E: ParseError<I>>(input: I) -> IResult<I, i32, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  map!(input, le_u32, |x| x as i32)
}

/// Recognizes a little endian signed 8 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_i64;
///
/// let parser = |s| {
///   le_i64::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcd"[..]), Ok((&b"abcd"[..], 0x0706050403020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(7))));
/// ```
#[inline]
pub fn le_i64<I, E: ParseError<I>>(input: I) -> IResult<I, i64, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  map!(input, le_u64, |x| x as i64)
}

/// Recognizes a little endian signed 16 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_i128;
///
/// let parser = |s| {
///   le_i128::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15abcd"[..]), Ok((&b"abcd"[..], 0x15141312111009080706050403020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(15))));
/// ```
#[inline]
#[cfg(stable_i128)]
pub fn le_i128<I, E: ParseError<I>>(input: I) -> IResult<I, i128, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  map!(input, le_u128, |x| x as i128)
}

/// Recognizes a big endian 4 bytes floating point number.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_f32;
///
/// let parser = |s| {
///   be_f32::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&[0x40, 0x29, 0x00, 0x00][..]), Ok((&b""[..], 2.640625)));
/// assert_eq!(parser(&[0x01][..]), Err(Err::Incomplete(Needed::new(3))));
/// ```
#[inline]
pub fn be_f32<I, E: ParseError<I>>(input: I) -> IResult<I, f32, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  match be_u32(input) {
    Err(e) => Err(e),
    Ok((i, o)) => Ok((i, f32::from_bits(o))),
  }
}

/// Recognizes a big endian 8 bytes floating point number.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_f64;
///
/// let parser = |s| {
///   be_f64::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&[0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]), Ok((&b""[..], 12.5)));
/// assert_eq!(parser(&[0x01][..]), Err(Err::Incomplete(Needed::new(7))));
/// ```
#[inline]
pub fn be_f64<I, E: ParseError<I>>(input: I) -> IResult<I, f64, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  match be_u64(input) {
    Err(e) => Err(e),
    Ok((i, o)) => Ok((i, f64::from_bits(o))),
  }
}

/// Recognizes a little endian 4 bytes floating point number.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_f32;
///
/// let parser = |s| {
///   le_f32::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&[0x00, 0x00, 0x48, 0x41][..]), Ok((&b""[..], 12.5)));
/// assert_eq!(parser(&[0x01][..]), Err(Err::Incomplete(Needed::new(3))));
/// ```
#[inline]
pub fn le_f32<I, E: ParseError<I>>(input: I) -> IResult<I, f32, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  match le_u32(input) {
    Err(e) => Err(e),
    Ok((i, o)) => Ok((i, f32::from_bits(o))),
  }
}

/// Recognizes a little endian 8 bytes floating point number.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_f64;
///
/// let parser = |s| {
///   le_f64::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x41][..]), Ok((&b""[..], 3145728.0)));
/// assert_eq!(parser(&[0x01][..]), Err(Err::Incomplete(Needed::new(7))));
/// ```
#[inline]
pub fn le_f64<I, E: ParseError<I>>(input: I) -> IResult<I, f64, E>
where
  I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
  match le_u64(input) {
    Err(e) => Err(e),
    Ok((i, o)) => Ok((i, f64::from_bits(o))),
  }
}

/// Recognizes a hex-encoded integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::hex_u32;
///
/// let parser = |s| {
///   hex_u32(s)
/// };
///
/// assert_eq!(parser(b"01AE;"), Ok((&b";"[..], 0x01AE)));
/// assert_eq!(parser(b"abc"), Err(Err::Incomplete(Needed::new(1))));
/// assert_eq!(parser(b"ggg"), Err(Err::Error((&b"ggg"[..], ErrorKind::IsA))));
/// ```
#[inline]
pub fn hex_u32<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], u32, E> {
  let (i, o) = crate::bytes::streaming::is_a(&b"0123456789abcdefABCDEF"[..])(input)?;

  // Do not parse more than 8 characters for a u32
  let (parsed, remaining) = if o.len() <= 8 {
    (o, i)
  } else {
    (&input[..8], &input[8..])
  };

  let res = parsed
    .iter()
    .rev()
    .enumerate()
    .map(|(k, &v)| {
      let digit = v as char;
      digit.to_digit(16).unwrap_or(0) << (k * 4)
    })
    .sum();

  Ok((remaining, res))
}

/// Recognizes a floating point number in text format and returns the corresponding part of the input.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if it reaches the end of input.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::recognize_float;
///
/// let parser = |s| {
///   recognize_float(s)
/// };
///
/// assert_eq!(parser("11e-1;"), Ok((";", "11e-1")));
/// assert_eq!(parser("123E-02;"), Ok((";", "123E-02")));
/// assert_eq!(parser("123K-01"), Ok(("K-01", "123")));
/// assert_eq!(parser("abc"), Err(Err::Error(("abc", ErrorKind::Char))));
/// ```
#[allow(unused_imports)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn recognize_float<T, E:ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
  T: Clone + Offset,
  T: InputIter,
  <T as InputIter>::Item: AsChar,
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsChar
{
  recognize(
    tuple((
      opt(alt((char('+'), char('-')))),
      alt((
        map(tuple((digit1, opt(pair(char('.'), opt(digit1))))), |_| ()),
        map(tuple((char('.'), digit1)), |_| ())
      )),
      opt(tuple((
        alt((char('e'), char('E'))),
        opt(alt((char('+'), char('-')))),
        cut(digit1)
      )))
    ))
  )(input)
}

/// Recognizes floating point number in a byte string and returns a `f32`.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if it reaches the end of input.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::float;
///
/// let parser = |s| {
///   float(s)
/// };
///
/// assert_eq!(parser("11e-1;"), Ok((";", 1.1)));
/// assert_eq!(parser("123E-02;"), Ok((";", 1.23)));
/// assert_eq!(parser("123K-01"), Ok(("K-01", 123.0)));
/// assert_eq!(parser("abc"), Err(Err::Error(("abc", ErrorKind::Char))));
/// ```
#[cfg(not(feature = "lexical"))]
pub fn float<T, E: ParseError<T>>(input: T) -> IResult<T, f32, E>
where
  T: Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
  T: Clone + Offset,
  T: InputIter + InputLength + crate::traits::ParseTo<f32>,
  <T as InputIter>::Item: AsChar,
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsChar,
{
  match recognize_float(input) {
    Err(e) => Err(e),
    Ok((i, s)) => match s.parse_to() {
      Some(n) => Ok((i, n)),
      None => Err(Err::Error(E::from_error_kind(i, ErrorKind::Float))),
    },
  }
}

/// Recognizes floating point number in a byte string and returns a `f32`.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if it reaches the end of input.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::float;
///
/// let parser = |s| {
///   float(s)
/// };
///
/// assert_eq!(parser("11e-1;"), Ok((";", 1.1)));
/// assert_eq!(parser("123E-02;"), Ok((";", 1.23)));
/// assert_eq!(parser("123K-01"), Ok(("K-01", 123.0)));
/// assert_eq!(parser("abc"), Err(Err::Error(("abc", ErrorKind::Float))));
/// ```
///
/// this function uses the lexical-core crate for float parsing by default, you
/// can deactivate it by removing the "lexical" feature
#[cfg(feature = "lexical")]
pub fn float<T, E: ParseError<T>>(input: T) -> IResult<T, f32, E>
where
  T: crate::traits::AsBytes + InputLength + Slice<RangeFrom<usize>>,
{
  match ::lexical_core::parse_partial(input.as_bytes()) {
    Ok((value, processed)) => {
      if processed == input.input_len() {
        Err(Err::Incomplete(Needed::Unknown))
      } else {
        Ok((input.slice(processed..), value))
      }
    }
    Err(_) => Err(Err::Error(E::from_error_kind(input, ErrorKind::Float))),
  }
}

/// Recognizes floating point number in a byte string and returns a `f64`.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if it reaches the end of input.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::double;
///
/// let parser = |s| {
///   double(s)
/// };
///
/// assert_eq!(parser("11e-1;"), Ok((";", 1.1)));
/// assert_eq!(parser("123E-02;"), Ok((";", 1.23)));
/// assert_eq!(parser("123K-01"), Ok(("K-01", 123.0)));
/// assert_eq!(parser("abc"), Err(Err::Error(("abc", ErrorKind::Char))));
/// ```
#[cfg(not(feature = "lexical"))]
pub fn double<T, E: ParseError<T>>(input: T) -> IResult<T, f64, E>
where
  T: Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
  T: Clone + Offset,
  T: InputIter + InputLength + crate::traits::ParseTo<f64>,
  <T as InputIter>::Item: AsChar,
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsChar,
{
  match recognize_float(input) {
    Err(e) => Err(e),
    Ok((i, s)) => match s.parse_to() {
      Some(n) => Ok((i, n)),
      None => Err(Err::Error(E::from_error_kind(i, ErrorKind::Float))),
    },
  }
}

/// Recognizes floating point number in a byte string and returns a `f64`.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if it reaches the end of input.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::double;
///
/// let parser = |s| {
///   double(s)
/// };
///
/// assert_eq!(parser("11e-1;"), Ok((";", 1.1)));
/// assert_eq!(parser("123E-02;"), Ok((";", 1.23)));
/// assert_eq!(parser("123K-01"), Ok(("K-01", 123.0)));
/// assert_eq!(parser("abc"), Err(Err::Error(("abc", ErrorKind::Float))));
/// ```
///
/// this function uses the lexical-core crate for float parsing by default, you
/// can deactivate it by removing the "lexical" feature
#[cfg(feature = "lexical")]
pub fn double<T, E: ParseError<T>>(input: T) -> IResult<T, f64, E>
where
  T: crate::traits::AsBytes + InputLength + Slice<RangeFrom<usize>>,
{
  match ::lexical_core::parse_partial(input.as_bytes()) {
    Ok((value, processed)) => {
      if processed == input.input_len() {
        Err(Err::Incomplete(Needed::Unknown))
      } else {
        Ok((input.slice(processed..), value))
      }
    }
    Err(_) => Err(Err::Error(E::from_error_kind(input, ErrorKind::Float))),
  }
}

#[cfg(feature = "enclave_unit_test")]
mod tests {
  use super::*;
  use crate::error::ErrorKind;
  use crate::internal::{Err, Needed};
  use std::string::ToString;
  use crates_unittest::test_case;
  macro_rules! assert_parse(
    ($left: expr, $right: expr) => {
      let res: $crate::IResult<_, _, (_, ErrorKind)> = $left;
      assert_eq!(res, $right);
    };
  );

  #[test_case]
  fn i8_tests() {
    assert_parse!(be_i8(&[0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(be_i8(&[0x7f][..]), Ok((&b""[..], 127)));
    assert_parse!(be_i8(&[0xff][..]), Ok((&b""[..], -1)));
    assert_parse!(be_i8(&[0x80][..]), Ok((&b""[..], -128)));
    assert_parse!(be_i8(&[][..]), Err(Err::Incomplete(Needed::new(1))));
  }

  #[test_case]
  fn i16_tests() {
    assert_parse!(be_i16(&[0x00, 0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(be_i16(&[0x7f, 0xff][..]), Ok((&b""[..], 32_767_i16)));
    assert_parse!(be_i16(&[0xff, 0xff][..]), Ok((&b""[..], -1)));
    assert_parse!(be_i16(&[0x80, 0x00][..]), Ok((&b""[..], -32_768_i16)));
    assert_parse!(be_i16(&[][..]), Err(Err::Incomplete(Needed::new(2))));
    assert_parse!(be_i16(&[0x00][..]), Err(Err::Incomplete(Needed::new(1))));
  }

  #[test_case]
  fn u24_tests() {
    assert_parse!(be_u24(&[0x00, 0x00, 0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(be_u24(&[0x00, 0xFF, 0xFF][..]), Ok((&b""[..], 65_535_u32)));
    assert_parse!(
      be_u24(&[0x12, 0x34, 0x56][..]),
      Ok((&b""[..], 1_193_046_u32))
    );
    assert_parse!(be_u24(&[][..]), Err(Err::Incomplete(Needed::new(3))));
    assert_parse!(be_u24(&[0x00][..]), Err(Err::Incomplete(Needed::new(2))));
    assert_parse!(
      be_u24(&[0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(1)))
    );
  }

  #[test_case]
  fn i24_tests() {
    assert_parse!(be_i24(&[0xFF, 0xFF, 0xFF][..]), Ok((&b""[..], -1_i32)));
    assert_parse!(be_i24(&[0xFF, 0x00, 0x00][..]), Ok((&b""[..], -65_536_i32)));
    assert_parse!(
      be_i24(&[0xED, 0xCB, 0xAA][..]),
      Ok((&b""[..], -1_193_046_i32))
    );
    assert_parse!(be_i24(&[][..]), Err(Err::Incomplete(Needed::new(3))));
    assert_parse!(be_i24(&[0x00][..]), Err(Err::Incomplete(Needed::new(2))));
    assert_parse!(
      be_i24(&[0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(1)))
    );
  }

  #[test_case]
  fn i32_tests() {
    assert_parse!(be_i32(&[0x00, 0x00, 0x00, 0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(
      be_i32(&[0x7f, 0xff, 0xff, 0xff][..]),
      Ok((&b""[..], 2_147_483_647_i32))
    );
    assert_parse!(be_i32(&[0xff, 0xff, 0xff, 0xff][..]), Ok((&b""[..], -1)));
    assert_parse!(
      be_i32(&[0x80, 0x00, 0x00, 0x00][..]),
      Ok((&b""[..], -2_147_483_648_i32))
    );
    assert_parse!(be_i32(&[][..]), Err(Err::Incomplete(Needed::new(4))));
    assert_parse!(be_i32(&[0x00][..]), Err(Err::Incomplete(Needed::new(3))));
    assert_parse!(
      be_i32(&[0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(2)))
    );
    assert_parse!(
      be_i32(&[0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(1)))
    );
  }

  #[test_case]
  fn i64_tests() {
    assert_parse!(
      be_i64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Ok((&b""[..], 0))
    );
    assert_parse!(
      be_i64(&[0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff][..]),
      Ok((&b""[..], 9_223_372_036_854_775_807_i64))
    );
    assert_parse!(
      be_i64(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff][..]),
      Ok((&b""[..], -1))
    );
    assert_parse!(
      be_i64(&[0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Ok((&b""[..], -9_223_372_036_854_775_808_i64))
    );
    assert_parse!(be_i64(&[][..]), Err(Err::Incomplete(Needed::new(8))));
    assert_parse!(be_i64(&[0x00][..]), Err(Err::Incomplete(Needed::new(7))));
    assert_parse!(
      be_i64(&[0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(6)))
    );
    assert_parse!(
      be_i64(&[0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(5)))
    );
    assert_parse!(
      be_i64(&[0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(4)))
    );
    assert_parse!(
      be_i64(&[0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(3)))
    );
    assert_parse!(
      be_i64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(2)))
    );
    assert_parse!(
      be_i64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(1)))
    );
  }

  #[test_case]
  #[cfg(stable_i128)]
  fn i128_tests() {
    assert_parse!(
      be_i128(
        &[
          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x00
        ][..]
      ),
      Ok((&b""[..], 0))
    );
    assert_parse!(
      be_i128(
        &[
          0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
          0xff
        ][..]
      ),
      Ok((
        &b""[..],
        170_141_183_460_469_231_731_687_303_715_884_105_727_i128
      ))
    );
    assert_parse!(
      be_i128(
        &[
          0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
          0xff
        ][..]
      ),
      Ok((&b""[..], -1))
    );
    assert_parse!(
      be_i128(
        &[
          0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x00
        ][..]
      ),
      Ok((
        &b""[..],
        -170_141_183_460_469_231_731_687_303_715_884_105_728_i128
      ))
    );
    assert_parse!(be_i128(&[][..]), Err(Err::Incomplete(Needed::new(16))));
    assert_parse!(be_i128(&[0x00][..]), Err(Err::Incomplete(Needed::new(15))));
    assert_parse!(
      be_i128(&[0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(14)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(13)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(12)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(11)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(10)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(9)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(8)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(7)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(6)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(5)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(4)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(3)))
    );
    assert_parse!(
      be_i128(
        &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]
      ),
      Err(Err::Incomplete(Needed::new(2)))
    );
    assert_parse!(
      be_i128(
        &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
          [..]
      ),
      Err(Err::Incomplete(Needed::new(1)))
    );
  }

  #[test_case]
  fn le_i8_tests() {
    assert_parse!(le_i8(&[0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(le_i8(&[0x7f][..]), Ok((&b""[..], 127)));
    assert_parse!(le_i8(&[0xff][..]), Ok((&b""[..], -1)));
    assert_parse!(le_i8(&[0x80][..]), Ok((&b""[..], -128)));
  }

  #[test_case]
  fn le_i16_tests() {
    assert_parse!(le_i16(&[0x00, 0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(le_i16(&[0xff, 0x7f][..]), Ok((&b""[..], 32_767_i16)));
    assert_parse!(le_i16(&[0xff, 0xff][..]), Ok((&b""[..], -1)));
    assert_parse!(le_i16(&[0x00, 0x80][..]), Ok((&b""[..], -32_768_i16)));
  }

  #[test_case]
  fn le_u24_tests() {
    assert_parse!(le_u24(&[0x00, 0x00, 0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(le_u24(&[0xFF, 0xFF, 0x00][..]), Ok((&b""[..], 65_535_u32)));
    assert_parse!(
      le_u24(&[0x56, 0x34, 0x12][..]),
      Ok((&b""[..], 1_193_046_u32))
    );
  }

  #[test_case]
  fn le_i24_tests() {
    assert_parse!(le_i24(&[0xFF, 0xFF, 0xFF][..]), Ok((&b""[..], -1_i32)));
    assert_parse!(le_i24(&[0x00, 0x00, 0xFF][..]), Ok((&b""[..], -65_536_i32)));
    assert_parse!(
      le_i24(&[0xAA, 0xCB, 0xED][..]),
      Ok((&b""[..], -1_193_046_i32))
    );
  }

  #[test_case]
  fn le_i32_tests() {
    assert_parse!(le_i32(&[0x00, 0x00, 0x00, 0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(
      le_i32(&[0xff, 0xff, 0xff, 0x7f][..]),
      Ok((&b""[..], 2_147_483_647_i32))
    );
    assert_parse!(le_i32(&[0xff, 0xff, 0xff, 0xff][..]), Ok((&b""[..], -1)));
    assert_parse!(
      le_i32(&[0x00, 0x00, 0x00, 0x80][..]),
      Ok((&b""[..], -2_147_483_648_i32))
    );
  }

  #[test_case]
  fn le_i64_tests() {
    assert_parse!(
      le_i64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Ok((&b""[..], 0))
    );
    assert_parse!(
      le_i64(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f][..]),
      Ok((&b""[..], 9_223_372_036_854_775_807_i64))
    );
    assert_parse!(
      le_i64(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff][..]),
      Ok((&b""[..], -1))
    );
    assert_parse!(
      le_i64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80][..]),
      Ok((&b""[..], -9_223_372_036_854_775_808_i64))
    );
  }

  #[test_case]
  #[cfg(stable_i128)]
  fn le_i128_tests() {
    assert_parse!(
      le_i128(
        &[
          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x00
        ][..]
      ),
      Ok((&b""[..], 0))
    );
    assert_parse!(
      le_i128(
        &[
          0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
          0x7f
        ][..]
      ),
      Ok((
        &b""[..],
        170_141_183_460_469_231_731_687_303_715_884_105_727_i128
      ))
    );
    assert_parse!(
      le_i128(
        &[
          0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
          0xff
        ][..]
      ),
      Ok((&b""[..], -1))
    );
    assert_parse!(
      le_i128(
        &[
          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x80
        ][..]
      ),
      Ok((
        &b""[..],
        -170_141_183_460_469_231_731_687_303_715_884_105_728_i128
      ))
    );
  }

  #[test_case]
  fn be_f32_tests() {
    assert_parse!(be_f32(&[0x00, 0x00, 0x00, 0x00][..]), Ok((&b""[..], 0_f32)));
    assert_parse!(
      be_f32(&[0x4d, 0x31, 0x1f, 0xd8][..]),
      Ok((&b""[..], 185_728_392_f32))
    );
  }

  #[test_case]
  fn be_f64_tests() {
    assert_parse!(
      be_f64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Ok((&b""[..], 0_f64))
    );
    assert_parse!(
      be_f64(&[0x41, 0xa6, 0x23, 0xfb, 0x10, 0x00, 0x00, 0x00][..]),
      Ok((&b""[..], 185_728_392_f64))
    );
  }

  #[test_case]
  fn le_f32_tests() {
    assert_parse!(le_f32(&[0x00, 0x00, 0x00, 0x00][..]), Ok((&b""[..], 0_f32)));
    assert_parse!(
      le_f32(&[0xd8, 0x1f, 0x31, 0x4d][..]),
      Ok((&b""[..], 185_728_392_f32))
    );
  }

  #[test_case]
  fn le_f64_tests() {
    assert_parse!(
      le_f64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Ok((&b""[..], 0_f64))
    );
    assert_parse!(
      le_f64(&[0x00, 0x00, 0x00, 0x10, 0xfb, 0x23, 0xa6, 0x41][..]),
      Ok((&b""[..], 185_728_392_f64))
    );
  }

  #[test_case]
  fn hex_u32_tests() {
    assert_parse!(
      hex_u32(&b";"[..]),
      Err(Err::Error(error_position!(&b";"[..], ErrorKind::IsA)))
    );
    assert_parse!(hex_u32(&b"ff;"[..]), Ok((&b";"[..], 255)));
    assert_parse!(hex_u32(&b"1be2;"[..]), Ok((&b";"[..], 7138)));
    assert_parse!(hex_u32(&b"c5a31be2;"[..]), Ok((&b";"[..], 3_315_801_058)));
    assert_parse!(hex_u32(&b"C5A31be2;"[..]), Ok((&b";"[..], 3_315_801_058)));
    assert_parse!(hex_u32(&b"00c5a31be2;"[..]), Ok((&b"e2;"[..], 12_952_347)));
    assert_parse!(
      hex_u32(&b"c5a31be201;"[..]),
      Ok((&b"01;"[..], 3_315_801_058))
    );
    assert_parse!(hex_u32(&b"ffffffff;"[..]), Ok((&b";"[..], 4_294_967_295)));
    assert_parse!(hex_u32(&b"0x1be2;"[..]), Ok((&b"x1be2;"[..], 0)));
    assert_parse!(hex_u32(&b"12af"[..]), Err(Err::Incomplete(Needed::new(1))));
  }

  #[test_case]
  #[cfg(feature = "std")]
  fn float_test() {
    let mut test_cases = vec![
      "+3.14",
      "3.14",
      "-3.14",
      "0",
      "0.0",
      "1.",
      ".789",
      "-.5",
      "1e7",
      "-1E-7",
      ".3e-2",
      "1.e4",
      "1.2e4",
      "12.34",
      "-1.234E-12",
      "-1.234e-12",
    ];

    for test in test_cases.drain(..) {
      let expected32 = str::parse::<f32>(test).unwrap();
      let expected64 = str::parse::<f64>(test).unwrap();

      println!("now parsing: {} -> {}", test, expected32);

      let larger = format!("{};", test);
      assert_parse!(recognize_float(&larger[..]), Ok((";", test)));

      assert_parse!(float(larger.as_bytes()), Ok((&b";"[..], expected32)));
      assert_parse!(float(&larger[..]), Ok((";", expected32)));

      assert_parse!(double(larger.as_bytes()), Ok((&b";"[..], expected64)));
      assert_parse!(double(&larger[..]), Ok((";", expected64)));
    }

    let remaining_exponent = "-1.234E-";
    assert_parse!(
      recognize_float(remaining_exponent),
      Err(Err::Incomplete(Needed::new(1)))
    );
  }
}
