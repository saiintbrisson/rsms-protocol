#[repr(u8)]
#[derive(Copy, Clone, Debug, protocol_derive::ProtocolSupport)]
pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}

impl Default for Difficulty {
    fn default() -> Self {
        Self::Normal
    }
}
