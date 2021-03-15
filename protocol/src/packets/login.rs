#[derive(protocol_derive::ProtocolSupportDerive)]
pub struct LoginStart {
    #[protocol_field(range(eq = 36))]
    uuid: String,
    #[protocol_field(range(min = 1, max = 16))]
    username: String,
}

#[cfg(test)]
mod test {
    use protocol_internal::ProtocolSupport;

    #[test]
    fn test() {
        let handshake = super::LoginStart {
            uuid: "".into(),
            username: "SaiintBrisson".into(),
        };

        assert_eq!(handshake.calculate_len(), 15)
    }
}
