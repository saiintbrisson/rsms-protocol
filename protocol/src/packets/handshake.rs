#[derive(protocol_derive::ProtocolSupportDerive)]
pub struct Handshake {
    #[protocol_field(varnum)]
    protocol_version: i32,
    #[protocol_field(range(max = 255))]
    server_address: String,
    server_port: u16,
    next_state: NextState,
}

#[repr(i32)]
#[protocol_field(varnum)]
#[derive(Clone, Copy, protocol_derive::ProtocolSupportDerive)]
pub enum NextState {
    Status = 1,
    Login = 2,
}

mod test {
    use protocol_internal::ProtocolSupport;

    #[test]
    fn test() {
        let handshake = super::Handshake {
            protocol_version: 47,
            server_address: "localhost".into(),
            server_port: 25565,
            next_state: super::NextState::Status,
        };

        assert_eq!(handshake.calculate_len(), 14)
    }
}
