#[derive(Clone, Debug, protocol_derive::ProtocolSupport)]
pub struct Property {
    name: String,
    value: String,
    signature: Option<String>,
}
