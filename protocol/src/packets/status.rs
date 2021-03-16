#[derive(protocol_derive::ProtocolSupportDerive)]
// #[packet(0x00)]
pub struct Request;

#[derive(protocol_derive::ProtocolSupportDerive)]
// #[packet(0x01)]
pub struct Ping {
    payload: i64,
}

#[derive(protocol_derive::ProtocolSupportDerive)]
// #[packet(0x00)]
pub struct Response {
    json_response: String,
}

#[derive(protocol_derive::ProtocolSupportDerive)]
// #[packet(0x01)]
pub struct Pong {
    payload: i64,
}
