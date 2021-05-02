use crate::{ProtocolSupportDecoder, ProtocolSupportEncoder};

impl<T> ProtocolSupportEncoder for Option<T>
where
    T: ProtocolSupportEncoder,
{
    fn calculate_len(&self, version: &crate::ProtocolVersion) -> usize {
        1 + self
            .as_ref()
            .map(|e| e.calculate_len(version))
            .unwrap_or_default()
    }

    fn encode<W: std::io::Write>(
        &self,
        dst: &mut W,
        version: &crate::ProtocolVersion,
    ) -> std::io::Result<()> {
        self.is_some().encode(dst, version)?;
        if let Some(t) = self {
            t.encode(dst, version)?;
        }

        Ok(())
    }
}

impl<T> ProtocolSupportDecoder for Option<T>
where
    T: ProtocolSupportDecoder,
{
    fn decode<R: std::io::Read + AsRef<[u8]>>(
        src: &mut std::io::Cursor<R>,
        version: &crate::ProtocolVersion,
    ) -> std::io::Result<Self> {
        if <bool as ProtocolSupportDecoder>::decode(src, version)? {
            return Ok(Some(T::decode(src, version)?));
        }

        Ok(None)
    }
}
