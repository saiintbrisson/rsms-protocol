use uuid::Uuid;

use crate::{ProtocolSupportDecoder, ProtocolSupportEncoder};

impl ProtocolSupportEncoder for Uuid {
    fn calculate_len(&self) -> usize {
        16
    }

    fn encode<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
        self.as_u128().encode(dst)
    }
}

impl ProtocolSupportDecoder for Uuid {
    fn decode<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
        Ok(Uuid::from_u128(ProtocolSupportDecoder::decode(
            src,
        )?))
    }
}
