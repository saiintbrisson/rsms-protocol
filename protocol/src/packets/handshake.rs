use std::fmt::Display;

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x00)]
pub struct Handshake {
    #[protocol_field(varnum)]
    pub protocol_version: i32,
    #[protocol_field(range(max = 255))]
    pub server_address: String,
    pub server_port: u16,
    pub next_state: NextState,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, protocol_derive::ProtocolSupport)]
#[protocol_field(varnum)]
pub enum NextState {
    Status = 1,
    Login = 2,
}

impl Display for NextState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self)
    }
}

#[cfg(test)]
mod test {
    use protocol_internal::{PacketEncoder, ProtocolSupportEncoder};

    #[test]
    fn test_handshake_len() {
        let handshake = super::Handshake {
            protocol_version: 47,
            server_address: "localhost".into(),
            server_port: 25565,
            next_state: super::NextState::Status,
        };

        assert_eq!(ProtocolSupportEncoder::calculate_len(&handshake), 14);
        assert_eq!(PacketEncoder::calculate_len(&handshake), 15);
    }
}
