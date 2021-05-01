#[cfg(feature = "types")]
pub mod types;
#[cfg(feature = "types")]
pub use types::*;

pub mod protocol_direction;
pub mod protocol_state;
pub mod protocol_version;

use std::io;

pub trait PacketEncoder: std::fmt::Debug + ProtocolSupportEncoder {
    fn calculate_len(&self) -> usize;
    fn encode<W: io::Write>(&self, dst: &mut W) -> io::Result<()>;
}

pub trait PacketDecoder: std::fmt::Debug + ProtocolSupportDecoder {
    fn decode<R: io::Read>(src: &mut R) -> io::Result<Self>;
    fn min_size() -> i32 {
        -1
    }
    fn max_size() -> i32 {
        -1
    }
}

pub trait ProtocolSupportEncoder {
    fn calculate_len(&self) -> usize;
    fn encode<W: io::Write>(&self, dst: &mut W) -> io::Result<()>;
}

pub trait ProtocolSupportDecoder: Sized {
    fn decode<R: io::Read>(src: &mut R) -> io::Result<Self>;
}
