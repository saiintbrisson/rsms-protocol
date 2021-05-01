mod bool;
mod cow;
mod dyn_array;
mod fixed_vec;
mod numeral {
    mod int;
    pub(crate) mod varnum;
}
mod option;
mod position;
mod range_validation;
mod regex;
mod string;
mod uuid;
mod vec;

pub use crate::regex::Regex;
pub use dyn_array::DynArray;
pub use fixed_vec::FixedVec;
pub use numeral::varnum::VarNum;
pub use position::{ProtocolPosition, ProtocolPositionSupport};
pub use range_validation::RangeValidatedSupport;

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
