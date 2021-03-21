use misc::misc::chat::ChatComponent;

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x00)]
pub struct LoginStart {
    #[protocol_field(range(min = 1, max = 16))]
    pub username: String,
}

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x01)]
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
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x02)]
pub struct LoginSuccess {
    #[protocol_field(range(eq = 36))]
    pub uuid: String,
    #[protocol_field(range(min = 1, max = 16))]
    pub username: String,
}

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x03)]
pub struct SetCompression {
    #[protocol_field(varnum)]
    pub threshold: i32,
}

#[cfg(test)]
mod test {
    use protocol_internal::ProtocolSupport;

    #[test]
    fn test_login_success_len() {
        let login_success = super::LoginSuccess {
            uuid: "".into(),
            username: "SaiintBrisson".into(),
        };

        assert_eq!(login_success.calculate_len(), 15)
    }
}
