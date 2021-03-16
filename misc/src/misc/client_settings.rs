#[repr(i8)]
#[derive(Clone, Copy, Debug, protocol_derive::ProtocolSupport)]
pub enum ChatMode {
    Enabled = 0,
    Commands = 1,
    Hidden = 2,
}

impl Default for ChatMode {
    fn default() -> Self {
        Self::Enabled
    }
}
