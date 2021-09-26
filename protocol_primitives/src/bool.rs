use crate::{Decoder, Encoder};

impl Decoder for bool {
    type Output = bool;

    fn decode<R: std::io::Read>(
        src: &mut R,
        _: &crate::Constraints,
        _: &crate::CodecContext,
    ) -> std::io::Result<Self::Output> {
        let mut buf = [0u8; 1];
        src.read_exact(&mut buf)?;
        Ok(buf[0] != 0)
    }
}

impl Encoder for bool {
    fn encode<W: std::io::Write>(
        dst: &mut W,
        i: &Self,
        _: &crate::CodecContext,
    ) -> std::io::Result<usize> {
        dst.write_all(&[if *i { 1 } else { 0 }]).map(|_| 1)
    }
}
