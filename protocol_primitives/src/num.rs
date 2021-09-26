use std::io::{Error, ErrorKind, Read, Result, Write};

use crate::{CodecContext, Constraints};

macro_rules! impl_codec {
    ($($t:ty),+) => {
        $(impl $crate::Decoder for $t {
            type Output = $t;

            fn decode<R: ::std::io::Read>(
                src: &mut R,
                c: &$crate::Constraints,
                _: &$crate::CodecContext,
            ) -> ::std::io::Result<Self::Output> {
                let mut buf = [0u8; <$t>::BITS as usize / 8];
                ::std::io::Read::read_exact(src, &mut buf)?;
                let s = <$t>::from_be_bytes(buf);

                if (s as isize) < c.range.0 {
                    return ::std::result::Result::Err(::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidInput,
                        ::std::format!("{} is out of bounds (min: {})", s, c.range.0),
                    ));
                } else if (s as isize) > c.range.1 {
                    return ::std::result::Result::Err(::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidInput,
                        ::std::format!("{} is out of bounds (max: {})", s, c.range.1),
                    ));
                }

                ::std::result::Result::Ok(s)
            }
        }

        impl $crate::Encoder<$t> for $t {
            fn encode<W: ::std::io::Write>(dst: &mut W, i: &$t, _: &$crate::CodecContext) -> ::std::io::Result<usize> {
                ::std::io::Write::write_all(dst, &i.to_be_bytes())?;
                ::std::result::Result::Ok(<$t>::BITS as usize / 8)
            }
        })+
    };
}

impl_codec! {
    i8, u8,
    i16, u16,
    i32, u32,
    i64, u64
}

pub struct Varint<T>(std::marker::PhantomData<T>);
impl<T> Varint<T> {
    const NUM_SHIFT: [u8; 10] = [0, 7, 14, 21, 28, 35, 42, 49, 56, 63];
}

impl crate::Decoder for Varint<i32> {
    type Output = i32;
    fn decode<R>(src: &mut R, c: &Constraints, _: &CodecContext) -> Result<Self::Output>
    where
        R: Read,
    {
        let mut result = 0i32;
        for i in &Varint::<i32>::NUM_SHIFT[..5] {
            let mut byte = [0];
            src.read_exact(&mut byte[..])?;
            let byte = byte[0];

            result |= ((byte as i32 & 0x7F) << i) as i32;

            if byte & 0x80 == 0 {
                if (result as isize) < c.range.0 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("varint {} is out of bounds (min: {})", result, c.range.0),
                    ));
                } else if (result as isize) > c.range.1 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("varint {} is out of bounds (max: {})", result, c.range.1),
                    ));
                }

                return Ok(result as i32);
            }
        }

        Err(Error::new(ErrorKind::InvalidInput, "varint is too big"))
    }
}

impl crate::Encoder<i32> for Varint<i32> {
    fn encode<W>(dst: &mut W, i: &i32, _: &crate::CodecContext) -> Result<usize>
    where
        W: Write,
    {
        let mut i = *i as u32;
        let mut j = 0;

        loop {
            j += 1;
            let byte = (i & 0x7F) as u8;
            i >>= 7;

            if i != 0 {
                dst.write(&[byte | 0x80])?;
            } else {
                dst.write(&[byte])?;
                break;
            }
        }

        Ok(j)
    }
}

#[cfg(test)]
mod tests {
    use crate::{num::Varint, CodecContext, Constraints, Decoder, Encoder, Version};
    const CONTEXT: crate::CodecContext = CodecContext {
        version: Version(0),
    };
    const CONSTRAINTS: crate::Constraints = Constraints::DEFAULT;

    #[test]
    fn test_roundtrip_i32() {
        let mut buf = [0u8; 4];
        <i32 as Encoder<i32>>::encode(&mut &mut buf[..], &-65535, &CONTEXT).unwrap();
        assert_eq!(
            <i32 as Decoder>::decode(&mut &buf[..], &CONSTRAINTS, &CONTEXT).unwrap(),
            -65535
        );
    }

    #[test]
    fn test_roundtrip_varnum() {
        let mut buf = [0u8; 3];
        <Varint<i32> as Encoder<i32>>::encode(&mut &mut buf[..], &65535, &CONTEXT).unwrap();
        assert_eq!(
            <Varint<i32> as Decoder>::decode(&mut &buf[..], &CONSTRAINTS, &CONTEXT).unwrap(),
            65535
        );
    }

    #[test]
    fn test_roundtrip_varnum_negative() {
        let mut buf = [0u8; 5];
        <Varint<i32> as Encoder<i32>>::encode(&mut &mut buf[..], &-65535, &CONTEXT).unwrap();
        assert_eq!(
            <Varint<i32> as Decoder>::decode(&mut &buf[..], &CONSTRAINTS, &CONTEXT).unwrap(),
            -65535
        );
    }
}
