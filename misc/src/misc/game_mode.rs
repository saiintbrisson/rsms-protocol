#[repr(u8)]
#[derive(Copy, Clone, protocol_derive::ProtocolSupportDerive)]
pub enum GameMode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

#[test]
fn test() {
    use protocol_internal::ProtocolSupport;
    let game_mode = GameMode::Survival;
    assert_eq!(game_mode.calculate_len(), 1);
}
