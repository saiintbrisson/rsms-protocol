use misc::prelude::{
    BlockPosition, ChatComponent, ChatMode, ChatPosition, ChunkPosition, Difficulty, Dimension,
    DisplayedSkinParts, EntityLocation, GameMode, Property, Vec2D, Vec3D,
};
use protocol_internal::{ProtocolSupportDecoder, ProtocolSupportEncoder};
use uuid::Uuid;

pub mod client_bound;
pub mod server_bound;

pub use client_bound::ClientBound;
pub use server_bound::ServerBound;

#[cfg(test)]
mod test {
    use misc::prelude::*;
    use protocol_internal::{PacketEncoder, ProtocolSupportEncoder, ProtocolVersionEnum};

    #[test]
    fn test_join_game() {
        let join_game = super::ClientBound::JoinGame(super::client_bound::JoinGame {
            entity_id: 0,
            game_mode: GameMode::Creative,
            dimension: Dimension::Overworld,
            difficulty: Difficulty::Normal,
            max_players: 100,
            level_type: "".into(),
            reduced_debug_info: false,
        });

        assert_eq!(
            ProtocolSupportEncoder::calculate_len(&join_game, &ProtocolVersionEnum::V1_8.into()),
            10
        );
        assert_eq!(
            PacketEncoder::calculate_len(&join_game, &ProtocolVersionEnum::V1_8.into()),
            11
        );
    }
}
