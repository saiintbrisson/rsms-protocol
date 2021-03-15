use std::{io, marker::PhantomData};

use crate::impl_range_validated_numeral;

pub struct VarNum<T> {
    _data: PhantomData<T>,
}

impl VarNum<i32> {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn calculate_len(value: i32) -> usize {
        if value as u32 & 0xF0000000 != 0 { 5 }
        else if value as u32 & 0xFFE00000 != 0 { 4 }
        else if value as u32 & 0xFFFFC000 != 0 { 3 }
        else if value as u32 & 0xFFFFFF80 != 0 { 2 }
        else { 1 }
    }

    pub fn deserialize<R: std::io::Read>(_src: &mut R) -> io::Result<i32> {
        Ok(0)
    }

    pub fn serialize<W: std::io::Write>(_value: i32, _dst: &mut W) -> io::Result<()> {
        Ok(())
    }
}

impl VarNum<i64> {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn calculate_len(value: i64) -> usize {
        if value as u32 & 0xF0000000 != 0 { 5 }
        else if value as u32 & 0xFFE00000 != 0 { 4 }
        else if value as u32 & 0xFFFFC000 != 0 { 3 }
        else if value as u32 & 0xFFFFFF80 != 0 { 2 }
        else { 1 }
    }

    pub fn deserialize<R: std::io::Read>(_src: &mut R) -> io::Result<i64> {
        Ok(0)
    }

    pub fn serialize<W: std::io::Write>(_value: i64, _dst: &mut W) -> io::Result<()> {
        Ok(())
    }
}

impl_range_validated_numeral!(i32, VarNum);
impl_range_validated_numeral!(i64, VarNum);
