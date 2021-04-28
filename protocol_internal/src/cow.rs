use std::borrow::Cow;

use crate::{ProtocolSupportDecoder, ProtocolSupportEncoder};

impl<'a, T> ProtocolSupportEncoder for Cow<'a, T>
where
    T: ProtocolSupportEncoder + ToOwned + ?Sized,
    T::Owned: ProtocolSupportEncoder 
{
    fn calculate_len(&self) -> usize {
        match self {
            Cow::Borrowed(b) => b.calculate_len(),
            Cow::Owned(o) => o.calculate_len(),
        }
    }

    fn encode<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
        match self {
            Cow::Borrowed(b) => b.encode(dst),
            Cow::Owned(o) => o.encode(dst),
        }
    }
}

impl<'a, T> ProtocolSupportDecoder for Cow<'a, T>
where
    T: ToOwned + ?Sized,
    T::Owned: ProtocolSupportDecoder
{
    fn decode<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
        T::Owned::decode(src).map(Cow::Owned)
    }
}