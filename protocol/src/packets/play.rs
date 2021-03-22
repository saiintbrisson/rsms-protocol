use misc::prelude::{
    BlockPosition, ChatComponent, ChatMode, ChatPosition, ChunkPosition, Difficulty, Dimension,
    EntityLocation, GameMode, Vec2D, Vec3D
};

use super::macros::packet_enum;

packet_enum!(client_bound, ClientBound =>
    0x00 => KeepAlive {
        #[protocol_field(varnum)]
        keep_alive_id: i32
    },
    0x01 => JoinGame {
        entity_id: i32,
        game_mode: GameMode,
        dimension: Dimension,
        difficulty: Difficulty,
        max_players: u8,
        level_type: String,
        reduced_debug_info: bool
    },
    0x02 => ChatMessage {
        json_data: ChatComponent<'static>,
        position: ChatPosition
    },
    0x05 => SpawnPosition {
        #[protocol_field(position)]
        location: EntityLocation
    },
    0x08 => PlayerPositionAndLook {
        entity_location: EntityLocation,
        flags: u8
    },
    0x21 => ChunkData {
        position: ChunkPosition,
        ground_up_continuous: bool,
        primary_bit_mask: u16,
        data: Vec<u8>
    },
    0x22 => MultiBlockChange {
        chunk: ChunkPosition,
        records: Vec<MultiBlockChangeRecord>
    },
    0x23 => BlockChange {
        #[protocol_field(position)]
        location: BlockPosition,
        #[protocol_field(varnum)]
        block_id: i32
    },
    0x24 => BlockAction {
        #[protocol_field(position)]
        location: BlockPosition,
        extra: u16,
        #[protocol_field(varnum)]
        block_type: i32
    },
    0x25 => BlockBreakAnimation {
        #[protocol_field(varnum)]
        entity_id: i32,
        #[protocol_field(position)]
        location: BlockPosition,
        destroy_stage: i8
    },
    0x26 => MapChunkBulk {
        sky_light_sent: bool,
        location: Vec<ChunkMeta>

    },
    0x40 => Disconnect {
        reason: ChatComponent<'static>
    }
);

#[derive(Debug, Default, protocol_derive::ProtocolSupport)]
pub struct MultiBlockChangeRecord {
    pub horizontal_position: u8,
    pub y_coordinate: u8,
    #[protocol_field(varnum)]
    pub block_id: i32,
}

#[derive(Debug, Default, protocol_derive::ProtocolSupport)]
pub struct ChunkMeta {
    pub position: ChunkPosition,
    pub primary_bit_mask: u16,
}

packet_enum!(server_bound, ServerBound =>
    0x00 => KeepAlive { 
        #[protocol_field(varnum)]
        keep_alive_id: i32
    },
    0x01 => ChatMessage { 
        #[protocol_field(range(min = 1, max = 100))]
        message: String
    },
    0x03 => Player { 
        on_ground: bool
    },
    0x04 => PlayerPosition { 
        position: Vec3D<f64>,
        on_ground: bool
    },
    0x05 => PlayerLook { 
        look: Vec2D<f32>,
        on_ground: bool
    },
    0x06 => PlayerPositionAndLook { 
        entity_location: EntityLocation,
        on_ground: bool
    },
    0x10 => CreativeInventoryAction { 
        slot: i16,
        #[protocol_field(dynarray)]
        clicked_item: Vec<u8>
    },
    0x15 => ClientSettings { 
        locale: String,
        view_distance: i8,
        chat_mode: ChatMode,
        chat_colors: bool,
        displayed_skin_parts: u8
    },
    0x17 => PluginMessage { 
        channel: String,
        #[protocol_field(dynarray)]
        data: Vec<u8>
    }
);

#[cfg(test)]
mod test {
    use protocol_internal::{PacketSerializer, ProtocolSupportSerializer};

    #[test]
    fn test_join_game() {
        let join_game = super::ClientBound::JoinGame(super::client_bound::JoinGame::default());
        assert_eq!(ProtocolSupportSerializer::calculate_len(&join_game), 10);
        assert_eq!(PacketSerializer::calculate_len(&join_game), 11);
    }
}
