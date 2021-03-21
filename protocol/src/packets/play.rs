#[protocol_derive::packets("ClientBound")]
pub mod client_bound {
    use misc::prelude::{
        BlockPosition, ChatComponent, ChatPosition, ChunkPosition, Difficulty, Dimension,
        EntityLocation, GameMode,
    };

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x00)]
    pub struct KeepAlive {
        #[protocol_field(varnum)]
        pub keep_alive_id: i32,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x01)]
    pub struct JoinGame {
        pub entity_id: i32,
        pub game_mode: GameMode,
        pub dimension: Dimension,
        pub difficulty: Difficulty,
        pub max_players: u8,
        pub level_type: String,
        pub reduced_debug_info: bool,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x02)]
    pub struct ChatMessage {
        pub json_data: ChatComponent<'static>,
        pub position: ChatPosition,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x05)]
    pub struct SpawnPosition {
        #[protocol_field(position)]
        pub location: EntityLocation,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x08)]
    pub struct PlayerPositionAndLook {
        pub entity_location: EntityLocation,
        pub flags: u8,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x21)]
    pub struct ChunkData {
        pub chunk: ChunkPosition,
        pub ground_up_continuous: bool,
        pub primary_bit_mask: u16,
        pub data: Vec<u8>,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x22)]
    pub struct MultiBlockChange {
        pub chunk: ChunkPosition,
        pub records: Vec<MultiBlockChangeRecord>,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    pub struct MultiBlockChangeRecord {
        pub horizontal_position: u8,
        pub y_coordinate: u8,
        #[protocol_field(varnum)]
        pub block_id: i32,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x23)]
    pub struct BlockChange {
        #[protocol_field(position)]
        pub location: BlockPosition,
        #[protocol_field(varnum)]
        pub block_id: i32,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x24)]
    pub struct BlockAction {
        #[protocol_field(position)]
        pub location: BlockPosition,
        pub extra: u16,
        #[protocol_field(varnum)]
        pub block_type: i32,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x25)]
    pub struct BlockBreakAnimation {
        #[protocol_field(varnum)]
        pub entity_id: i32,
        #[protocol_field(position)]
        pub location: BlockPosition,
        pub destroy_stage: i8,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x40)]
    pub struct Disconnect {
        pub reason: ChatComponent<'static>,
    }
}

#[protocol_derive::packets("ServerBound")]
pub mod server_bound {
    use misc::prelude::{ChatMode, EntityLocation, Vec2D, Vec3D};

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x00)]
    pub struct KeepAlive {
        #[protocol_field(varnum)]
        pub keep_alive_id: i32,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x01)]
    pub struct ChatMessage {
        #[protocol_field(range(min = 1, max = 100))]
        pub message: String,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x03)]
    pub struct Player {
        pub on_ground: bool,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x04)]
    pub struct PlayerPosition {
        pub position: Vec3D<f64>,
        pub on_ground: bool,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x05)]
    pub struct PlayerLook {
        pub look: Vec2D<f32>,
        pub on_ground: bool,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x06)]
    pub struct PlayerPositionAndLook {
        pub entity_location: EntityLocation,
        pub on_ground: bool,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x10)]
    pub struct CreativeInventoryAction {
        pub slot: i16,
        #[protocol_field(dynarray)]
        pub clicked_item: Vec<u8>,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x15)]
    pub struct ClientSettings {
        pub locale: String,
        pub view_distance: i8,
        pub chat_mode: ChatMode,
        pub chat_colors: bool,
        pub displayed_skin_parts: u8,
    }

    #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
    #[packet(0x17)]
    pub struct PluginMessage {
        pub channel: String,
        #[protocol_field(dynarray)]
        pub data: Vec<u8>,
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
