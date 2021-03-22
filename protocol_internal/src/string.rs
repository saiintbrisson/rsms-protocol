use crate::{
    ProtocolSupportDeserializer, ProtocolSupportSerializer, RangeValidatedSupport, VarNum,
};

impl ProtocolSupportSerializer for String {
    fn calculate_len(&self) -> usize {
        VarNum::<i32>::calculate_len(&(self.len() as i32)) + self.len()
    }

    fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
        VarNum::<i32>::serialize(&(self.len() as i32), dst)?;
        dst.write(self.as_bytes()).map(|_| ())
    }
}

impl ProtocolSupportDeserializer for String {
    fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
        <String as RangeValidatedSupport>::deserialize(src, 0, 32767)
    }
}

impl RangeValidatedSupport for String {
    #[inline(always)]
    fn deserialize<R: std::io::Read>(src: &mut R, min: usize, max: usize) -> std::io::Result<Self> {
        let len = <VarNum<i32> as RangeValidatedSupport<i32>>::deserialize(src, min, max)? as usize;

        let mut buf = vec![0u8; len];
        if src.read(&mut buf)? != len {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("string is smaller than expected {}", len),
            ));
        }

        String::from_utf8(buf)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
    }
}
