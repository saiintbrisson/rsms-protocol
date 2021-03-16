#[derive(protocol_derive::ProtocolSupportDerive)]
#[packet(0x00)]
pub struct LoginStart {
    #[protocol_field(range(min = 1, max = 16))]
    username: String,
}

#[derive(protocol_derive::ProtocolSupportDerive)]
#[packet(0x01)]
pub struct EncryptionRequest {
    server_id: String,
    public_key: Vec<u8>,
    verify_token: Vec<u8>,
}

#[derive(protocol_derive::ProtocolSupportDerive)]
#[packet(0x00)]
pub struct Disconnect {
    reason: String,
}

#[derive(protocol_derive::ProtocolSupportDerive)]
#[packet(0x01)]
pub struct EncryptionResponse {
    shared_secret: Vec<u8>,
    verify_token: Vec<u8>,
}

#[derive(protocol_derive::ProtocolSupportDerive)]
#[packet(0x02)]
pub struct LoginSuccess {
    #[protocol_field(range(eq = 36))]
    uuid: String,
    #[protocol_field(range(min = 1, max = 16))]
    username: String,
}

#[derive(protocol_derive::ProtocolSupportDerive)]
#[packet(0x03)]
pub struct SetCompression {
    #[protocol_field(varnum)]
    threshold: i32,
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
