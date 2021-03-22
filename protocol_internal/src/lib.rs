mod bool;
mod dyn_array;
mod numeral {
    mod int;
    pub(crate) mod varnum;
}
mod position;
mod range_validation;
mod string;
mod uuid;
mod vec;

pub use dyn_array::DynArray;
pub use numeral::varnum::VarNum;
pub use position::{ProtocolPosition, ProtocolPositionSupport};
pub use range_validation::RangeValidatedSupport;

use std::io;

// pub trait Packet: std::fmt::Debug + ProtocolSupport {
//     fn calculate_len(&self) -> usize;

//     fn serialize<W: io::Write>(&self, dst: &mut W) -> io::Result<()>;
//     fn deserialize<R: io::Read>(src: &mut R) -> io::Result<Self>;
// }

// pub trait ProtocolSupport: Sized {
//     fn calculate_len(&self) -> usize;

//     fn serialize<W: io::Write>(&self, dst: &mut W) -> io::Result<()>;
//     fn deserialize<R: io::Read>(src: &mut R) -> io::Result<Self>;
// }

pub trait PacketSerializer: std::fmt::Debug + ProtocolSupportSerializer {
    fn calculate_len(&self) -> usize;
    fn serialize<W: io::Write>(&self, dst: &mut W) -> io::Result<()>;
}

pub trait PacketDeserializer: std::fmt::Debug + ProtocolSupportDeserializer {
    fn deserialize<R: io::Read>(src: &mut R) -> io::Result<Self>;
}

pub trait ProtocolSupportSerializer {
    fn calculate_len(&self) -> usize;
    fn serialize<W: io::Write>(&self, dst: &mut W) -> io::Result<()>;
}

pub trait ProtocolSupportDeserializer: Sized {
    fn deserialize<R: io::Read>(src: &mut R) -> io::Result<Self>;
}
