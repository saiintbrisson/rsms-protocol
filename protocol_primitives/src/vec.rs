use std::io::{Error, ErrorKind, Read, Result, Write};

use crate::{CodecContext, Constraints, Decoder, Encoder, Varint};

impl<T> Decoder for [T]
where
    T: Decoder,
{
    type Output = Vec<T::Output>;

    fn decode<R: Read>(src: &mut R, c: &Constraints, ctx: &CodecContext) -> Result<Self::Output> {
        let len = <Varint<i32> as Decoder>::decode(src, &Constraints::DEFAULT, ctx)? as isize;

        if len < c.range.0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("vec len {} is out of bounds (min: {})", len, c.range.0),
            ));
        } else if len > c.range.1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("vec len {} is out of bounds (max: {})", len, c.range.1),
            ));
        } else if len == 0 {
            return Ok(Vec::new());
        }

        let mut vec = Vec::with_capacity(len as usize);

        for _ in 0..len {
            vec.push(<T as Decoder>::decode(src, &Constraints::DEFAULT, ctx)?);
        }

        Ok(vec)
    }
}

impl<T> Encoder for [T]
where
    T: Encoder,
{
    fn encode<W: Write>(dst: &mut W, i: &[T], ctx: &CodecContext) -> Result<usize> {
        let mut written = <Varint<i32> as Encoder<i32>>::encode(dst, &(i.len() as i32), ctx)?;

        for e in i {
            written += <T as Encoder<T>>::encode(dst, e, ctx)?;
        }

        Ok(written)
    }
}

impl<T> Decoder for Vec<T>
where
    T: Decoder,
{
    type Output = Vec<T::Output>;

    fn decode<R: Read>(src: &mut R, c: &Constraints, ctx: &CodecContext) -> Result<Self::Output> {
        <[T] as Decoder>::decode(src, c, ctx)
    }
}

impl<T> Encoder<Vec<T>> for Vec<T>
where
    T: Encoder<T>,
{
    fn encode<W: Write>(dst: &mut W, i: &Vec<T>, ctx: &CodecContext) -> Result<usize> {
        <[T] as Encoder>::encode(dst, i, ctx)
    }
}

pub mod dyn_array {
    use super::*;

    pub struct DynArray<T>(std::marker::PhantomData<T>);
    impl<T> Decoder for DynArray<Vec<T>>
    where
        T: Decoder,
    {
        type Output = Vec<T::Output>;

        fn decode<R: Read>(
            src: &mut R,
            c: &Constraints,
            ctx: &CodecContext,
        ) -> Result<Self::Output> {
            let mut vec = if c.range.0 > 0 {
                Vec::with_capacity(c.range.0 as usize)
            } else {
                Vec::new()
            };

            loop {
                match <T as Decoder>::decode(src, &Constraints::DEFAULT, ctx) {
                    Ok(out) => vec.push(out),
                    Err(err) if err.kind() == ErrorKind::UnexpectedEof => break,
                    Err(err) => return Err(err),
                }
            }

            if (vec.len() as isize) < c.range.0 {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!(
                        "dyn array size {} is out of bounds (min: {})",
                        vec.len(),
                        c.range.0
                    ),
                ));
            } else if (vec.len() as isize) > c.range.1 {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!(
                        "dyn array size {} is out of bounds (max: {})",
                        vec.len(),
                        c.range.1
                    ),
                ));
            }

            Ok(vec)
        }
    }

    impl<T> Encoder<Vec<T>> for DynArray<Vec<T>>
    where
        T: Encoder,
    {
        fn encode<W: Write>(dst: &mut W, i: &Vec<T>, ctx: &CodecContext) -> Result<usize> {
            let mut written = 0;
            for e in i {
                written += <T as Encoder<T>>::encode(dst, e, ctx)?;
            }
            Ok(written)
        }
    }
}
