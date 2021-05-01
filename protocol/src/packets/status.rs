#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x00)]
#[packet_size(eq = 0)]
pub struct Request;

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x01)]
#[packet_size(eq = 8)]
pub struct Ping {
    pub payload: i64,
}

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x00)]
pub struct Response {
    pub json_response: String,
}

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x01)]
#[packet_size(eq = 8)]
pub struct Pong {
    pub payload: i64,
}
