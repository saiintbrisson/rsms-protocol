#[cfg(feature = "derive")]
pub use protocol_derive::{packets, ProtocolSupport};
pub use protocol_internal::{
    DynArray, PacketDecoder, PacketEncoder, PacketSizer, ProtocolSupportDecoder,
    ProtocolSupportEncoder, ProtocolVersion, ProtocolVersionEnum, RangeValidatedSupport, VarNum,
};

#[cfg(feature = "packets")]
pub mod packets {
    pub mod handshake;
    pub mod login;
    pub mod macros;
    pub mod play;
    pub mod status;
}
