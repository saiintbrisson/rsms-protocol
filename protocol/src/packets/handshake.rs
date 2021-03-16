#[derive(protocol_derive::ProtocolSupport)]
#[packet(0x00)]
pub struct Handshake {
    #[protocol_field(varnum)]
    protocol_version: i32,
    #[protocol_field(range(max = 255))]
    server_address: String,
    server_port: u16,
    next_state: NextState,
}

#[repr(i32)]
#[derive(Clone, Copy, protocol_derive::ProtocolSupport)]
#[protocol_field(varnum)]
pub enum NextState {
    Status = 1,
    Login = 2,
}

#[cfg(test)]
mod test {
    use protocol_internal::{Packet, ProtocolSupport};

    #[test]
    fn test_handshake_len() {
        let handshake = super::Handshake {
            protocol_version: 47,
            server_address: "localhost".into(),
            server_port: 25565,
            next_state: super::NextState::Status,
        };

        assert_eq!(ProtocolSupport::calculate_len(&handshake), 14);
        assert_eq!(Packet::calculate_len(&handshake), 15);
    }
}
