use crate::{ProtocolSupportDeserializer, ProtocolSupportSerializer};

impl<T> ProtocolSupportSerializer for Option<T>
where
    T: ProtocolSupportSerializer,
{
    fn calculate_len(&self) -> usize {
        1 + self.as_ref().map(|e| e.calculate_len()).unwrap_or_default()
    }

    fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
        self.is_some().serialize(dst)?;
        if let Some(t) = self {
            t.serialize(dst)?;
        }

        Ok(())
    }
}

impl<T> ProtocolSupportDeserializer for Option<T>
where
    T: ProtocolSupportDeserializer,
{
    fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
        if <bool as ProtocolSupportDeserializer>::deserialize(src)? {
            return Ok(Some(T::deserialize(src)?));
        }

        Ok(None)
    }
}
