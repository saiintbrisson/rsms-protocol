use std::io::{Error, ErrorKind, Read, Result, Write};

use crate::{CodecContext, Constraints, Decoder, Encoder, Varint};

impl Decoder for str {
    type Output = String;

    fn decode<R: Read>(src: &mut R, c: &Constraints, ctx: &CodecContext) -> Result<Self::Output> {
        let len = <Varint<i32> as Decoder>::decode(
            src,
            &Constraints {
                range: (c.range.0, c.range.1 * 4),
            },
            ctx,
        )? as isize;

        let mut str = vec![0; len as usize];
        src.read_exact(&mut str[..])?;

        let str = String::from_utf8(str)
            .map_err(|err| Error::new(ErrorKind::InvalidInput, format!("invalid utf8: {}", err)))?;

        if (str.len() as isize) < c.range.0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("str len {} is out of bounds (min: {})", len, c.range.0),
            ));
        } else if (str.len() as isize) > c.range.1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("str len {} is out of bounds (max: {})", len, c.range.1),
            ));
        }

        Ok(str)
    }
}

impl Encoder for str {
    fn encode<W: Write>(dst: &mut W, i: &str, ctx: &CodecContext) -> Result<usize> {
        let buf = i.as_bytes();

        let len = <Varint<i32> as Encoder<i32>>::encode(dst, &(buf.len() as i32), ctx)? + buf.len();
        dst.write_all(buf)?;

        Ok(len)
    }
}

impl Decoder for String {
    type Output = String;

    fn decode<R: Read>(src: &mut R, c: &Constraints, ctx: &CodecContext) -> Result<Self::Output> {
        <str as Decoder>::decode(src, c, ctx)
    }
}

impl Encoder for String {
    fn encode<W: Write>(dst: &mut W, i: &String, ctx: &CodecContext) -> Result<usize> {
        <str as Encoder>::encode(dst, i, ctx)
    }
}
