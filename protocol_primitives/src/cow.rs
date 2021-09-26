use std::io::{Read, Result, Write};

use crate::{CodecContext, Constraints, Decoder, Encoder};

impl<'a, T> Decoder for std::borrow::Cow<'a, T>
where
    T: Decoder<Output = <T as ToOwned>::Owned> + ToOwned + ?Sized,
{
    type Output = Self;

    fn decode<R: Read>(src: &mut R, c: &Constraints, ctx: &CodecContext) -> Result<Self::Output> {
        Ok(std::borrow::Cow::Owned(<T as Decoder>::decode(
            src, c, ctx,
        )?))
    }
}

impl<'a, T> Encoder for std::borrow::Cow<'a, T>
where
    T: Encoder<T> + ToOwned + ?Sized,
{
    fn encode<W: Write>(dst: &mut W, i: &Self, ctx: &CodecContext) -> Result<usize> {
        <T as Encoder<T>>::encode(dst, i, ctx)
    }
}
