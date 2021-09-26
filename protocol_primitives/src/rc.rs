use std::{rc::Rc, sync::Arc};

use crate::{Decoder, Encoder};

impl<T> Decoder for Arc<T>
where
    T: Decoder,
{
    type Output = Arc<T::Output>;

    fn decode<R: std::io::Read>(
        src: &mut R,
        c: &crate::Constraints,
        ctx: &crate::CodecContext,
    ) -> std::io::Result<Self::Output> {
        T::decode(src, c, ctx).map(Arc::new)
    }
}

impl<T> Encoder for Arc<T>
where
    T: Encoder,
{
    fn encode<W: std::io::Write>(
        dst: &mut W,
        i: &Self,
        ctx: &crate::CodecContext,
    ) -> std::io::Result<usize> {
        T::encode(dst, i, ctx)
    }
}

impl<T> Decoder for Rc<T>
where
    T: Decoder,
{
    type Output = Rc<T::Output>;

    fn decode<R: std::io::Read>(
        src: &mut R,
        c: &crate::Constraints,
        ctx: &crate::CodecContext,
    ) -> std::io::Result<Self::Output> {
        T::decode(src, c, ctx).map(Rc::new)
    }
}

impl<T> Encoder for Rc<T>
where
    T: Encoder,
{
    fn encode<W: std::io::Write>(
        dst: &mut W,
        i: &Self,
        ctx: &crate::CodecContext,
    ) -> std::io::Result<usize> {
        T::encode(dst, i, ctx)
    }
}
