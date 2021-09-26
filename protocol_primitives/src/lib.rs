mod bool;
mod cow;
mod misc;
mod num;
mod str;
mod vec;
mod version;

use std::io::{Read, Result, Write};

pub use num::Varint;
pub use vec::dyn_array::DynArray;
pub use version::{Version, VersionEnum};

pub struct CodecContext {
    pub version: Version,
}

impl Default for CodecContext {
    fn default() -> CodecContext {
        CodecContext {
            version: Version::new(0),
        }
    }
}

pub struct Constraints {
    pub range: (isize, isize),
}

impl Constraints {
    pub const DEFAULT: Constraints = Constraints {
        range: (i32::MIN as isize, i32::MAX as isize),
    };
}

impl Default for Constraints {
    fn default() -> Constraints {
        Constraints {
            range: (i32::MIN as isize, i32::MAX as isize),
        }
    }
}

pub trait Decoder {
    type Output;

    fn decode<R: Read>(src: &mut R, c: &Constraints, ctx: &CodecContext) -> Result<Self::Output>;
}

pub trait Encoder<T: ?Sized = Self> {
    fn encode<W: Write>(dst: &mut W, i: &T, ctx: &CodecContext) -> Result<usize>;
}

pub trait Codec<T: ?Sized = Self>: Decoder + Encoder<T> {}
impl<C> Codec for C where C: Decoder + Encoder {}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ProtocolDirection {
    ClientBound,
    ServerBound,
}

impl ProtocolDirection {
    pub fn opposite(&self) -> Self {
        match self {
            Self::ClientBound => Self::ServerBound,
            Self::ServerBound => Self::ClientBound,
        }
    }
}

#[repr(u8)]
pub enum ProtocolState {
    Handshake = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}
