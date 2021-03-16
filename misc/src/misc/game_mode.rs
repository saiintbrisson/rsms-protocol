#[repr(u8)]
#[derive(Copy, Clone, Debug, protocol_derive::ProtocolSupport)]
pub enum GameMode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

impl Default for GameMode {
    fn default() -> Self {
        Self::Survival
    }
}
