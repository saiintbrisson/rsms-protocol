use std::borrow::Cow;

use crate::{ProtocolSupportDecoder, ProtocolSupportEncoder};

impl<'a, T> ProtocolSupportEncoder for Cow<'a, T>
where
    T: ProtocolSupportEncoder + ToOwned + ?Sized,
    T::Owned: ProtocolSupportEncoder,
{
    fn calculate_len(&self, version: &crate::ProtocolVersion) -> usize {
        match self {
            Cow::Borrowed(b) => b.calculate_len(version),
            Cow::Owned(o) => o.calculate_len(version),
        }
    }

    fn encode<W: std::io::Write>(
        &self,
        dst: &mut W,
        version: &crate::ProtocolVersion,
    ) -> std::io::Result<()> {
        match self {
            Cow::Borrowed(b) => b.encode(dst, version),
            Cow::Owned(o) => o.encode(dst, version),
        }
    }
}

impl<'a, T> ProtocolSupportDecoder for Cow<'a, T>
where
    T: ToOwned + ?Sized,
    T::Owned: ProtocolSupportDecoder,
{
    fn decode<R: std::io::Read>(
        src: &mut R,
        version: &crate::ProtocolVersion,
    ) -> std::io::Result<Self> {
        T::Owned::decode(src, version).map(Cow::Owned)
    }
}
