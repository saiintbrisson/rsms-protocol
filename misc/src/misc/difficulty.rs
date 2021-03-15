#[repr(u8)]
#[derive(Copy, Clone, protocol_derive::ProtocolSupportDerive)]
pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}
