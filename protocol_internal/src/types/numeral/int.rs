use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::impl_range_validated_numeral;

macro_rules! impl_numeral {
    ($n:ty, 1, $r:ident, $w:ident) => {
        impl $crate::ProtocolSupportEncoder for $n {
            fn calculate_len(&self, _: &crate::ProtocolVersion) -> usize {
                1
            }

            fn encode<W: std::io::Write>(
                &self,
                dst: &mut W,
                _: &crate::ProtocolVersion,
            ) -> std::io::Result<()> {
                dst.$w(*self)
            }
        }

        impl $crate::ProtocolSupportDecoder for $n {
            fn decode<R: std::io::Read + AsRef<[u8]>>(
                src: &mut std::io::Cursor<R>,
                _: &$crate::ProtocolVersion,
            ) -> std::io::Result<$n> {
                src.$r()
            }
        }
    };
    ($n:ty, $s:expr, $r:ident, $w:ident) => {
        impl $crate::ProtocolSupportEncoder for $n {
            fn calculate_len(&self, _: &crate::ProtocolVersion) -> usize {
                $s
            }

            fn encode<W: std::io::Write>(
                &self,
                dst: &mut W,
                _: &crate::ProtocolVersion,
            ) -> std::io::Result<()> {
                dst.$w::<BigEndian>(*self)
            }
        }

        impl $crate::ProtocolSupportDecoder for $n {
            fn decode<R: std::io::Read + AsRef<[u8]>>(
                src: &mut std::io::Cursor<R>,
                _: &$crate::ProtocolVersion,
            ) -> std::io::Result<$n> {
                src.$r::<BigEndian>()
            }
        }
    };
}

impl_numeral!(u8, 1, read_u8, write_u8);
impl_numeral!(i8, 1, read_i8, write_i8);
impl_numeral!(u16, 2, read_u16, write_u16);
impl_numeral!(i16, 2, read_i16, write_i16);
impl_numeral!(u32, 4, read_u32, write_u32);
impl_numeral!(i32, 4, read_i32, write_i32);
impl_numeral!(u64, 8, read_u64, write_u64);
impl_numeral!(i64, 8, read_i64, write_i64);
impl_numeral!(u128, 6, read_u128, write_u128);
impl_numeral!(i128, 16, read_i128, write_i128);

impl_numeral!(f32, 4, read_f32, write_f32);
impl_numeral!(f64, 8, read_f64, write_f64);

impl_range_validated_numeral!(u8);
impl_range_validated_numeral!(i8);
impl_range_validated_numeral!(u16);
impl_range_validated_numeral!(i16);
impl_range_validated_numeral!(u32);
impl_range_validated_numeral!(i32);
impl_range_validated_numeral!(u64);
impl_range_validated_numeral!(i64);
impl_range_validated_numeral!(u128);
impl_range_validated_numeral!(i128);

impl_range_validated_numeral!(f32);
impl_range_validated_numeral!(f64);
