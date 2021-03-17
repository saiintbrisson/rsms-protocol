use std::marker::PhantomData;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub trait ProtocolPosition {
    fn to_position(&self) -> i64;
    fn from_position(position: i64) -> Self
    where
        Self: Sized;
}

pub struct ProtocolPositionSupport<T> {
    _data: PhantomData<T>,
}

impl<T: ProtocolPosition> ProtocolPositionSupport<T> {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn calculate_len(_: &T) -> usize {
        8
    }

    pub fn serialize<W: std::io::Write>(value: &T, dst: &mut W) -> std::io::Result<()> {
        dst.write_i64::<BigEndian>(ProtocolPosition::to_position(value))
    }

    pub fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<T> {
        Ok(ProtocolPosition::from_position(
            src.read_i64::<BigEndian>()?,
        ))
    }
}
