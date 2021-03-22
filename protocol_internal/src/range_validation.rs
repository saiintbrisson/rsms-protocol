use crate::ProtocolSupportDeserializer;

pub trait RangeValidatedSupport<T = Self>
where
    T: ProtocolSupportDeserializer + Sized,
{
    fn deserialize<R: std::io::Read>(src: &mut R, min: usize, max: usize) -> std::io::Result<T>;
}

#[macro_export]
macro_rules! impl_range_validated_numeral {
    ($n:ty, VarNum) => {
        impl $crate::RangeValidatedSupport<$n> for $crate::VarNum<$n> {
            #[inline(always)]
            fn deserialize<R: std::io::Read>(
                src: &mut R,
                min: usize,
                max: usize,
            ) -> std::io::Result<$n> {
                let value = $crate::VarNum::<$n>::deserialize(src)?;

                if min != 0 && min as $n > value {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("number is smaller than min {}", min),
                    ));
                }

                if max != 0 && value > max as $n {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("number is bigger than max {}", max),
                    ));
                }

                Ok(value)
            }
        }
    };
    ($n:ty) => {
        impl $crate::RangeValidatedSupport for $n {
            #[inline(always)]
            fn deserialize<R: std::io::Read>(
                src: &mut R,
                min: usize,
                max: usize,
            ) -> std::io::Result<Self> {
                let value = <$n as $crate::ProtocolSupportDeserializer>::deserialize(src)?;

                if min != 0 && min as $n > value {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("number is smaller than min {}", min),
                    ));
                }

                if max != 0 && value > max as $n {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("number is bigger than max {}", max),
                    ));
                }

                Ok(value)
            }
        }
    };
}
