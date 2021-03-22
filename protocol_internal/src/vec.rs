use crate::{
    ProtocolSupportDeserializer, ProtocolSupportSerializer, RangeValidatedSupport, VarNum,
};

impl<T: ProtocolSupportSerializer> ProtocolSupportSerializer for Vec<T> {
    fn calculate_len(&self) -> usize {
        self.iter()
            .map(<T as ProtocolSupportSerializer>::calculate_len)
            .fold(0, |acc, x| acc + x)
            + VarNum::<i32>::calculate_len(&(self.len() as i32))
    }

    fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
        VarNum::<i32>::serialize(&(self.len() as i32), dst)?;

        for e in self {
            <T as ProtocolSupportSerializer>::serialize(e, dst)?;
        }

        Ok(())
    }
}

impl<T: ProtocolSupportDeserializer> ProtocolSupportDeserializer for Vec<T> {
    fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
        let len = VarNum::<i32>::deserialize(src)? as usize;

        let mut buf = Vec::with_capacity(len);
        for _ in 0..len {
            buf.push(<T as ProtocolSupportDeserializer>::deserialize(src)?);
        }

        Ok(buf)
    }
}

impl<T: ProtocolSupportDeserializer> RangeValidatedSupport for Vec<T> {
    fn deserialize<R: std::io::Read>(src: &mut R, min: usize, max: usize) -> std::io::Result<Self> {
        let len = <VarNum<i32> as RangeValidatedSupport<i32>>::deserialize(src, min, max)? as usize;

        let mut buf = Vec::with_capacity(len);
        for _ in 0..len {
            buf.push(<T as ProtocolSupportDeserializer>::deserialize(src)?);
        }

        Ok(buf)
    }
}
