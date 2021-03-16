#[protocol_derive::packets("ClientBound")]
pub mod client_bound {
    use misc::prelude::{Difficulty, Dimension, GameMode};

    #[derive(Default, protocol_derive::ProtocolSupport)]
    #[packet(0x01)]
    pub struct JoinGame {
        entity_id: i32,
        game_mode: GameMode,
        dimension: Dimension,
        difficulty: Difficulty,
        max_players: u8,
        level_type: String,
        reduced_debug_info: bool,
    }
}

#[cfg(test)]
mod test {
    use protocol_internal::{Packet, ProtocolSupport};

    #[test]
    fn test_join_game() {
        let join_game = super::ClientBound::JoinGame(super::client_bound::JoinGame::default());
        assert_eq!(ProtocolSupport::calculate_len(&join_game), 10);
        assert_eq!(Packet::calculate_len(&join_game), 11);
    }
}
