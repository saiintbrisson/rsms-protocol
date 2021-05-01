use misc::misc::chat::ChatComponent;

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x00)]
#[packet_size(max = 17)]
pub struct LoginStart {
    #[protocol_field(regex(r"^([\w]{1,16})$"))]
    pub username: String,
}

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x01)]
#[packet_size(min = 261, max = 1282)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
}

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x00)]
pub struct Disconnect {
    pub reason: ChatComponent<'static>,
}

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x01)]
#[packet_size(eq = 260)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x02)]
#[packet_size(max = 54)]
pub struct LoginSuccess {
    #[protocol_field(range(eq = 36))]
    pub uuid: String,
    #[protocol_field(range(min = 1, max = 16))]
    pub username: String,
}

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x03)]
#[packet_size(max = 5)]
pub struct SetCompression {
    #[protocol_field(varnum)]
    pub threshold: i32,
}

#[cfg(test)]
mod test {
    use protocol_internal::ProtocolSupportEncoder;

    #[test]
    fn test_login_success_len() {
        let login_success = super::LoginSuccess {
            uuid: "".into(),
            username: "SaiintBrisson".into(),
        };

        assert_eq!(login_success.calculate_len(), 15)
    }
}
