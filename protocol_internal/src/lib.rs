#[cfg(feature = "types")]
pub mod types;
#[cfg(feature = "types")]
pub use types::*;

pub mod protocol_direction;
pub mod protocol_state;
pub mod protocol_version;

pub use protocol_direction::ProtocolDirection;
pub use protocol_state::ProtocolState;
pub use protocol_version::{ProtocolVersion, ProtocolVersionEnum};

use std::io;

pub trait PacketEncoder: std::fmt::Debug + ProtocolSupportEncoder {
    fn calculate_len(&self, version: &ProtocolVersion) -> usize;
    fn encode<W: io::Write>(&self, dst: &mut W, version: &ProtocolVersion) -> io::Result<()>;
}

pub trait PacketDecoder: std::fmt::Debug + ProtocolSupportDecoder {
    fn decode<R: io::Read + AsRef<[u8]>>(
        src: &mut io::Cursor<R>,
        version: &ProtocolVersion,
    ) -> io::Result<Self>;
}

pub trait PacketSizer {
    fn min_size(_: &protocol_version::ProtocolVersion) -> i32 {
        -1
    }
    fn max_size(_: &protocol_version::ProtocolVersion) -> i32 {
        -1
    }
}

pub trait ProtocolSupportEncoder {
    fn calculate_len(&self, version: &ProtocolVersion) -> usize;
    fn encode<W: io::Write>(&self, dst: &mut W, version: &ProtocolVersion) -> io::Result<()>;
}

pub trait ProtocolSupportDecoder: Sized {
    fn decode<R: io::Read + AsRef<[u8]>>(
        src: &mut io::Cursor<R>,
        version: &ProtocolVersion,
    ) -> io::Result<Self>;
}
