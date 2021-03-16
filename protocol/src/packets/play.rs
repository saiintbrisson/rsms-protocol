#[protocol_derive::packets("ClientBound")]
mod client_bound {
    use misc::prelude::{Difficulty, Dimension, GameMode};

    #[derive(protocol_derive::ProtocolSupportDerive)]
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
