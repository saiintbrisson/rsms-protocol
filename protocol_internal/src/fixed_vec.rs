use crate::ProtocolSupportDecoder;

pub struct FixedVec;

impl FixedVec {
    pub fn decode<R: std::io::Read, T: ProtocolSupportDecoder>(
        src: &mut R,
        len: usize,
    ) -> std::io::Result<Vec<T>> {
        let mut buf = Vec::with_capacity(len);

        while buf.len() < buf.capacity() {
            buf.push(<T as ProtocolSupportDecoder>::decode(src)?);
        }

        Ok(buf)
    }
}
