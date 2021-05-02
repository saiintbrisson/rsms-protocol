use std::io;

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::{ProtocolSupportDecoder, ProtocolSupportEncoder};

impl ProtocolSupportEncoder for bool {
    fn calculate_len(&self, _: &crate::ProtocolVersion) -> usize {
        1
    }

    fn encode<W: std::io::Write>(
        &self,
        dst: &mut W,
        _: &crate::ProtocolVersion,
    ) -> std::io::Result<()> {
        dst.write_u8(if *self { 1 } else { 0 })
    }
}

impl ProtocolSupportDecoder for bool {
    fn decode<R: std::io::Read + AsRef<[u8]>>(
        src: &mut std::io::Cursor<R>,
        _: &crate::ProtocolVersion,
    ) -> std::io::Result<Self> {
        Ok(match src.read_u8()? {
            0 => false,
            1 => true,
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid bool value",
            ))?,
        })
    }
}
