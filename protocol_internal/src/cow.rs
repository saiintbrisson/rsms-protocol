use std::borrow::Cow;

use crate::{ProtocolSupportDeserializer, ProtocolSupportSerializer};

impl<'a, T> ProtocolSupportSerializer for Cow<'a, T>
where
    T: ProtocolSupportSerializer + ToOwned + ?Sized,
    T::Owned: ProtocolSupportSerializer 
{
    fn calculate_len(&self) -> usize {
        match self {
            Cow::Borrowed(b) => b.calculate_len(),
            Cow::Owned(o) => o.calculate_len(),
        }
    }

    fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
        match self {
            Cow::Borrowed(b) => b.serialize(dst),
            Cow::Owned(o) => o.serialize(dst),
        }
    }
}

impl<'a, T> ProtocolSupportDeserializer for Cow<'a, T>
where
    T: ToOwned + ?Sized,
    T::Owned: ProtocolSupportDeserializer
{
    fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
        T::Owned::deserialize(src).map(Cow::Owned)
    }
}