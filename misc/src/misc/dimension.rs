#[repr(i8)]
#[derive(Clone, Copy, Debug, protocol_derive::ProtocolSupport)]
pub enum Dimension {
    Nether = -1,
    Overworld = 0,
    End = 1,
}

impl Default for Dimension {
    fn default() -> Self {
        Self::Overworld
    }
}
