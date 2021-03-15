#[repr(i8)]
#[derive(Copy, Clone, protocol_derive::ProtocolSupportDerive)]
pub enum Dimension {
    Nether = -1,
    Overworld = 0,
    End = 1,
}
