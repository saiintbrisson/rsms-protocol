use crate::packets::macros::{packet_enum, proto_enum};

packet_enum! {
    ServerBound {
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
        0x07 => PlayerDigging {
            status: PlayerDiggingStatus,
            #[protocol_field(position)]
            location: Vec3D<i32>,
            face: i8
        },
        0x09 => HeldItemChange {
            slot: i16
        },
        0x0A => Animation {},
        0x0B => EntityAction {
            #[protocol_field(varnum)]
            entity_id: i32,
            action: EntityActionType,
            #[protocol_field(varnum)]
            action_parameter: i32
        },
        0x0D => CloseWindow {
            window_id: u8
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
        0x16 => ClientStatus,
        0x17 => PluginMessage {
            channel: String,
            #[protocol_field(dynarray)]
            data: Vec<u8>
        }
    }
}

proto_enum! {
    PlayerDiggingStatus (u8) {
        StartedDigging = 0,
        CancelledDigging = 1,
        FinishedDigging = 2,
        DropItemStack = 3,
        DropItem = 4,
        ShootArrow_FinishEating = 5
    }
}

proto_enum! {
    EntityActionType (u8) {
        StartSneaking = 0,
        StopSneaking = 1,
        LeaveBed = 2,
        StartSprinting = 3,
        StopSprinting = 4,
        JumpWithHorse = 5,
        OpenRiddenHorseInventory = 6
    }
}

proto_enum! {
    ClientStatus (u8) {
        PerformRespawn = 0,
        RequestStats = 1,
        TakingInventoryAchievement = 2
    }
}
