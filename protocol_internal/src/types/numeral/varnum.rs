use std::{io, marker::PhantomData};

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::impl_range_validated_numeral;

pub struct VarNum<T> {
    _data: PhantomData<T>,
}

impl<T> VarNum<T> {
    const NUM_SHIFT: [u8; 10] = [0, 7, 14, 21, 28, 35, 42, 49, 56, 63];
}

impl VarNum<i32> {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn calculate_len(value: &i32) -> usize {
        let value = *value;

        if value as u32 & 0xF0000000 != 0 { 5 }
        else if value as u32 & 0xFFE00000 != 0 { 4 }
        else if value as u32 & 0xFFFFC000 != 0 { 3 }
        else if value as u32 & 0xFFFFFF80 != 0 { 2 }
        else { 1 }
    }

    pub fn encode<W: std::io::Write>(value: &i32, dst: &mut W) -> io::Result<()> {
        let mut temp = *value;

        loop {
            let byte = (temp & 0x7F) as u8;
            temp >>= 7;

            if temp != 0 {
                dst.write_u8(byte | 0x80)?;
            } else {
                dst.write_u8(byte)?;
                break;
            }
        }

        Ok(())
    }

    pub fn decode<R: std::io::Read>(src: &mut R) -> io::Result<i32> {
        let mut result = 0i32;

        for i in &VarNum::<i32>::NUM_SHIFT[..5] {
            let byte = src.read_u8()?;
            result |= ((byte as i32 & 0x7F) << i) as i32;

            if byte & 0x80 == 0 {
                return Ok(result.into());
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "varint is too big",
        ))
    }
}

impl VarNum<i64> {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn calculate_len(value: &i64) -> usize {
        let value = *value;

        if value as u32 & 0xF0000000 != 0 { 5 }
        else if value as u32 & 0xFFE00000 != 0 { 4 }
        else if value as u32 & 0xFFFFC000 != 0 { 3 }
        else if value as u32 & 0xFFFFFF80 != 0 { 2 }
        else { 1 }
    }

    pub fn encode<W: std::io::Write>(_value: &i64, _dst: &mut W) -> io::Result<()> {
        Ok(())
    }

    pub fn decode<R: std::io::Read>(_src: &mut R) -> io::Result<i64> {
        Ok(0)
    }
}

impl_range_validated_numeral!(i32, VarNum);
impl_range_validated_numeral!(i64, VarNum);

impl VarNum<Vec<i32>> {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn calculate_len(value: &Vec<i32>) -> usize {
        value.iter().fold(0, |acc, e| acc + VarNum::<i32>::calculate_len(e))
    }

    pub fn encode<W: std::io::Write>(value: &Vec<i32>, dst: &mut W) -> io::Result<()> {
        for e in value {
            VarNum::<i32>::encode(e, dst)?;
        }

        Ok(())
    }

    pub fn decode<R: std::io::Read>(src: &mut R) -> io::Result<Vec<i32>> {
        let len = VarNum::<i32>::decode(src)? as usize;

        let mut buf = Vec::with_capacity(len);
        while buf.len() < buf.capacity() {
            buf.push(VarNum::<i32>::decode(src)?);
        }

        Ok(buf)
    }
}
