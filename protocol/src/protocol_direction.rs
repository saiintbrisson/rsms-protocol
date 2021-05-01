#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ProtocolDirection {
    ClientBound,
    ServerBound
}

impl ProtocolDirection {
    fn opposite(&self) -> Self {
        match self {
            Self::ClientBound => Self::ServerBound,
            Self::ServerBound => Self::ClientBound,
        }
    }
}
