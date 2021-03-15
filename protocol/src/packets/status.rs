#[derive(protocol_derive::ProtocolSupportDerive)]
pub struct Request;

#[derive(protocol_derive::ProtocolSupportDerive)]
pub struct Ping {
    payload: i64,
}

#[derive(protocol_derive::ProtocolSupportDerive)]
pub struct Response {
    json_response: String,
}

#[derive(protocol_derive::ProtocolSupportDerive)]
pub struct Pong {
    payload: i64,
}
