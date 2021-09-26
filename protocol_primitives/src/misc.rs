use uuid::Uuid;

use crate::{CodecContext, Constraints, Decoder, Encoder};

impl Decoder for Uuid {
    type Output = Uuid;

    fn decode<R: std::io::Read>(
        src: &mut R,
        _: &Constraints,
        _: &CodecContext,
    ) -> std::io::Result<Self::Output> {
        let mut buf = [0; 16];
        src.read_exact(&mut buf)?;
        Ok(Uuid::from_bytes(buf))
    }
}

impl Encoder for Uuid {
    fn encode<W: std::io::Write>(
        dst: &mut W,
        i: &Self,
        _: &CodecContext,
    ) -> std::io::Result<usize> {
        dst.write_all(i.as_bytes())?;
        Ok(16)
    }
}
