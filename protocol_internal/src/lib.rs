mod bool;
mod dyn_array;
mod numeral {
    mod int;
    pub(crate) mod varnum;
}
mod range_validation;
mod string;
mod uuid;
mod vec;

pub use dyn_array::DynArray;
pub use numeral::varnum::VarNum;
pub use range_validation::RangeValidatedSupport;

use std::io;

pub trait Packet: ProtocolSupport {}

pub trait ProtocolSupport: Sized {
    fn calculate_len(&self) -> usize;

    fn deserialize<R: io::Read>(src: &mut R) -> io::Result<Self>;
    fn serialize<W: io::Write>(&self, dst: &mut W) -> io::Result<()>;
}
