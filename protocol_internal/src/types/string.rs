use crate::{
    ProtocolSupportDecoder, ProtocolSupportEncoder, RangeValidatedSupport, VarNum,
};

impl ProtocolSupportEncoder for String {
    fn calculate_len(&self) -> usize {
        VarNum::<i32>::calculate_len(&(self.len() as i32)) + self.len()
    }

    fn encode<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
        VarNum::<i32>::encode(&(self.len() as i32), dst)?;
        dst.write(self.as_bytes()).map(|_| ())
    }
}

impl ProtocolSupportDecoder for String {
    fn decode<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
        <String as RangeValidatedSupport>::decode(src, 0, 32767)
    }
}

impl RangeValidatedSupport for String {
    #[inline(always)]
    fn decode<R: std::io::Read>(src: &mut R, min: usize, max: usize) -> std::io::Result<Self> {
        let len = <VarNum<i32> as RangeValidatedSupport<i32>>::decode(src, min, max * 4)? as usize;

        let mut buf = vec![0u8; len];
        if src.read(&mut buf)? != len {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("string is smaller than expected {}", len),
            ));
        }

        let string = String::from_utf8(buf)
                    .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
        
        if string.len() > max {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("string is bigger than max {}", max),
            ));
        }

        Ok(string)
    }
}

impl ProtocolSupportEncoder for str {
    fn calculate_len(&self) -> usize {
        VarNum::<i32>::calculate_len(&(self.len() as i32)) + self.len()
    }

    fn encode<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
        VarNum::<i32>::encode(&(self.len() as i32), dst)?;
        dst.write(self.as_bytes()).map(|_| ())
    }
}
