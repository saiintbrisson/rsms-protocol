use uuid::Uuid;

use crate::{ProtocolSupportDecoder, ProtocolSupportEncoder};

impl ProtocolSupportEncoder for Uuid {
    fn calculate_len(&self, _: &crate::ProtocolVersion) -> usize {
        16
    }

    fn encode<W: std::io::Write>(
        &self,
        dst: &mut W,
        version: &crate::ProtocolVersion,
    ) -> std::io::Result<()> {
        self.as_u128().encode(dst, version)
    }
}

impl ProtocolSupportDecoder for Uuid {
    fn decode<R: std::io::Read>(
        src: &mut R,
        version: &crate::ProtocolVersion,
    ) -> std::io::Result<Self> {
        Ok(Uuid::from_u128(ProtocolSupportDecoder::decode(
            src, version,
        )?))
    }
}
