#[cfg(feature = "derive")]
pub use protocol_derive::ProtocolSupportDerive;
pub use protocol_internal::{DynArray, Packet, ProtocolSupport, RangeValidatedSupport, VarNum};

#[cfg(feature = "packets")]
pub mod packets {
    pub mod handshake;
    pub mod login;
}
