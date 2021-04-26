use crate::packets::macros::{packet_enum, proto_enum};

packet_enum! {
    ClientBound {
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
            records: Vec<MultiBlockChangeRecord>;
            items {
                #[derive(Clone, Debug, protocol_derive::ProtocolSupport)]
                pub struct MultiBlockChangeRecord {
                    pub horizontal_position: u8,
                    pub y_coordinate: u8,
                    #[protocol_field(varnum)]
                    pub block_id: i32,
                }
            }
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
            meta: Vec<ChunkMeta>,
            data: Vec<Vec<u8>>;
            items {
                #[derive(Clone, Debug, Default, protocol_derive::ProtocolSupport)]
                pub struct ChunkMeta {
                    pub position: ChunkPosition,
                    pub primary_bit_mask: u16,
                }
            }
        },
        0x38 => PlayerListItem,
        0x3B => ScoreboardObjective {
            objective_name: String,
            mode: ScoreboardObjectiveMode;
            items {
                #[derive(Clone, Debug, protocol_derive::ProtocolSupport)]
                pub struct ScoreboardObjectiveInfo {
                    pub objective_value: String,
                    pub objective_type: String,
                }
            }
        },
        0x3C => UpdateScore {
            score_name: String,
            action: UpdateScoreAction
        },
        0x3D => DisplayScoreboard {
            position: DisplayScoreboardPosition,
            score_name: String
        },
        0x3E => Teams {
            team_name: String,
            mode: TeamsMode;
            items {
                #[derive(Clone, Debug, protocol_derive::ProtocolSupport)]
                pub struct TeamInfo {
                    pub team_display_name: String,
                    pub team_prefix: String,
                    pub team_suffix: String,
                    pub friendly_fire: FriendlyFire,
                    pub name_tag_visibility: String,
                    pub color: misc::prelude::ChatColor,
                }
            }
        },
        0x3F => PluginMessage {
            channel: String,
            #[protocol_field(dynarray)]
            data: Vec<u8>
        },
        0x40 => Disconnect {
            reason: ChatComponent<'static>
        },
        0x45 => Title,
        0x47 => PlayerListHeaderAndFooter {
            header: ChatComponent<'static>,
            footer: ChatComponent<'static>
        }
    }
}

#[derive(Clone, Debug)]
pub enum PlayerListItem {
    AddPlayer(Vec<(Uuid, PlayerListItemAddPlayer)>),
    UpdateGameMode(Vec<(Uuid, GameMode)>),
    UpdateLatency(Vec<(Uuid, i32)>),
    UpdateDisplayName(Vec<(Uuid, Option<ChatComponent<'static>>)>),
    RemovePlayer(Vec<Uuid>),
}

impl ProtocolSupportSerializer for PlayerListItem {
    fn calculate_len(&self) -> usize {
        todo!()
    }

    fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
        todo!()
    }
}

impl ProtocolSupportDeserializer for PlayerListItem {
    fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
        todo!()
    }
}

#[derive(Clone, Debug, Default, protocol_derive::ProtocolSupport)]
pub struct PlayerListItemAddPlayer {
    pub name: String,
    pub properties: Vec<Property>,
    pub game_mode: GameMode,
    #[protocol_field(varnum)]
    pub ping: i32,
    pub display_name: Option<String>
}

proto_enum! {
    ScoreboardObjectiveMode (u8) {
        Create {
            info: ScoreboardObjectiveInfo
        } = 0,
        Remove = 1,
        Update {
            info: ScoreboardObjectiveInfo
        } = 2
    }
    default Self::Remove
}

proto_enum! {
    UpdateScoreAction (u8) {
        Create_Update {
            objective_name: String,
            value: i32
        } = 0,
        Remove {
            objective_name: String
        } = 1
    }
    default Self::Remove { objective_name: String::new() }
}

proto_enum! {
    DisplayScoreboardPosition (u8) {
        List = 0,
        Sidebar = 1,
        BelowName = 2
    }
    default Self::Sidebar
}

proto_enum! {
    TeamsMode (u8) {
        Create {
            info: TeamInfo,
            players: Vec<String>
        } = 0,
        Remove = 1,
        InfoUpdate {
            info: TeamInfo
        } = 2,
        AddPlayers {
            players: Vec<String>
        } = 3,
        RemovePlayers {
            players: Vec<String>
        } = 4
    }
    default Self::Remove
}

proto_enum! {
    FriendlyFire (u8) {
        Off = 0,
        On = 1,
        ShowInvisible = 3
    }
    default Self::Off
}

proto_enum! {
    Title (u8) {
        SetTitle {
            title: ChatComponent<'static>
        } = 0,
        SetSubTitle {
            sub_title: ChatComponent<'static>
        } = 1,
        SetTimesAndDisplay {
            fade_in: i32,
            stay: i32,
            fade_out: i32
        } = 2,
        Hide = 3,
        Reset = 4
    }
    default Self::Reset
}

