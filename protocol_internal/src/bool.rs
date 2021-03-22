use std::io;

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::{ProtocolSupportDeserializer, ProtocolSupportSerializer};

impl ProtocolSupportSerializer for bool {
    fn calculate_len(&self) -> usize {
        1
    }

    fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
        dst.write_u8(if *self { 1 } else { 0 })
    }
}

impl ProtocolSupportDeserializer for bool {
    fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
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
