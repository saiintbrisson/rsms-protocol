use crate::{ProtocolSupportDecoder, ProtocolSupportEncoder, RangeValidatedSupport, VarNum};

impl<T: ProtocolSupportEncoder> ProtocolSupportEncoder for Vec<T> {
    fn calculate_len(&self, version: &crate::ProtocolVersion) -> usize {
        self.iter()
            .map(|e| <T as ProtocolSupportEncoder>::calculate_len(e, version))
            .fold(0, |acc, x| acc + x)
            + VarNum::<i32>::calculate_len(&(self.len() as i32))
    }

    fn encode<W: std::io::Write>(
        &self,
        dst: &mut W,
        version: &crate::ProtocolVersion,
    ) -> std::io::Result<()> {
        VarNum::<i32>::encode(&(self.len() as i32), dst)?;

        for e in self {
            <T as ProtocolSupportEncoder>::encode(e, dst, version)?;
        }

        Ok(())
    }
}

impl<T: ProtocolSupportDecoder> ProtocolSupportDecoder for Vec<T> {
    fn decode<R: std::io::Read>(
        src: &mut R,
        version: &crate::ProtocolVersion,
    ) -> std::io::Result<Self> {
        let len = VarNum::<i32>::decode(src)? as usize;

        let mut buf = Vec::with_capacity(len);
        while buf.len() < buf.capacity() {
            buf.push(<T as ProtocolSupportDecoder>::decode(src, version)?);
        }

        Ok(buf)
    }
}

impl<T: ProtocolSupportDecoder> RangeValidatedSupport for Vec<T> {
    fn decode<R: std::io::Read>(
        src: &mut R,
        version: &crate::ProtocolVersion,
        min: usize,
        max: usize,
    ) -> std::io::Result<Self> {
        let len =
            <VarNum<i32> as RangeValidatedSupport<i32>>::decode(src, version, min, max)? as usize;

        let mut buf = Vec::with_capacity(len);
        while buf.len() < buf.capacity() {
            buf.push(<T as ProtocolSupportDecoder>::decode(src, version)?);
        }

        Ok(buf)
    }
}
