#[repr(u8)]
pub enum ProtocolState {
    Handshake = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}
