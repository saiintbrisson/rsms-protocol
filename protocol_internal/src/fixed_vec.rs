use crate::ProtocolSupportDeserializer;

pub struct FixedVec;

impl FixedVec {
    pub fn deserialize<R: std::io::Read, T: ProtocolSupportDeserializer>(
        src: &mut R,
        len: usize,
    ) -> std::io::Result<Vec<T>> {
        let mut buf = Vec::with_capacity(len);

        while buf.len() < buf.capacity() {
            buf.push(<T as ProtocolSupportDeserializer>::deserialize(src)?);
        }

        Ok(buf)
    }
}
