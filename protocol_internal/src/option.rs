use crate::{ProtocolSupportDecoder, ProtocolSupportEncoder};

impl<T> ProtocolSupportEncoder for Option<T>
where
    T: ProtocolSupportEncoder,
{
    fn calculate_len(&self) -> usize {
        1 + self.as_ref().map(|e| e.calculate_len()).unwrap_or_default()
    }

    fn encode<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
        self.is_some().encode(dst)?;
        if let Some(t) = self {
            t.encode(dst)?;
        }

        Ok(())
    }
}

impl<T> ProtocolSupportDecoder for Option<T>
where
    T: ProtocolSupportDecoder,
{
    fn decode<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
        if <bool as ProtocolSupportDecoder>::decode(src)? {
            return Ok(Some(T::decode(src)?));
        }

        Ok(None)
    }
}
