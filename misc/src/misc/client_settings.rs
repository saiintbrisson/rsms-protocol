#[repr(i8)]
#[derive(Clone, Copy, Debug, protocol_derive::ProtocolSupport)]
pub enum ChatMode {
    Enabled = 0,
    Commands = 1,
    Hidden = 2,
}

bitflags::bitflags! {
    #[derive(protocol_derive::ProtocolSupport)]
    pub struct DisplayedSkinParts: u8 {
        const CAPE = 0x01;
        const JACKET = 0x02;
        const LEFT_SLEEVE = 0x04;
        const RIGHT_SLEEVE = 0x08;
        const LEFT_PANTS = 0x10;
        const RIGHT_PANTS = 0x20;
        const HAT = 0x40;
    }
}

impl Default for DisplayedSkinParts {
    fn default() -> Self {
        Self::all()
    }
}

impl Default for ChatMode {
    fn default() -> Self {
        Self::Enabled
    }
}
