use std::borrow::Cow;

use crate::packets::macros::{packet_enum, proto_enum};

packet_enum! {
    ClientBound<'a> {
        0x00 => KeepAlive {
            #[protocol_field(varnum)]
            keep_alive_id: i32
        },
        0x01 => JoinGame<'a> {
            entity_id: i32,
            game_mode: GameMode,
            dimension: Dimension,
            difficulty: Difficulty,
            max_players: u8,
            level_type: Cow<'a, str>,
            reduced_debug_info: bool
        },
        0x02 => ChatMessage<'a> {
            json_data: ChatComponent<'a>,
            position: ChatPosition
        },
        0x05 => SpawnPosition {
            #[protocol_field(position)]
            location: EntityLocation
        },
        0x08 => PlayerPositionAndLook {
            entity_location: EntityLocation,
            flags: PlayerPositionAndLookFlags
        },
        0x09 => HeldItemChange {
            slot: i8
        },
        0x0B => Animation {
            #[protocol_field(varnum)]
            entity_id: i32,
            animation: AnimationAction
        },
        0x0D => CollectItem {
            #[protocol_field(varnum)]
            collected_entity_id: i32,
            #[protocol_field(varnum)]
            collector_entity_id: i32
        },
        0x13 => DestroyEntities {
            #[protocol_field(varnum)]
            entities: Vec<i32>
        },
        0x14 => Entity {
            #[protocol_field(varnum)]
            entity_id: i32
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
        0x2D => OpenWindow<'a> {
            window_id: u8,
            window_type: Cow<'a, str>,
            window_title: ChatComponent<'a>,
            number_of_slots: u8
        },
        0x2E => CloseWindow {
            window_id: u8
        },
        0x38 => PlayerListItem<'a>,
        0x3B => ScoreboardObjective<'a> {
            objective_name: Cow<'a, str>,
            mode: ScoreboardObjectiveMode<'a>;
            items {
                #[derive(Clone, Debug, protocol_derive::ProtocolSupport)]
                pub struct ScoreboardObjectiveInfo<'a> {
                    pub objective_value: Cow<'a, str>,
                    pub objective_type: Cow<'a, str>,
                }
            }
        },
        0x3C => UpdateScore<'a> {
            score_name: Cow<'a, str>,
            action: UpdateScoreAction<'a>
        },
        0x3D => DisplayScoreboard<'a> {
            position: DisplayScoreboardPosition,
            score_name: Cow<'a, str>
        },
        0x3E => Teams<'a> {
            team_name: Cow<'a, str>,
            mode: TeamsMode<'a>;
            items {
                #[derive(Clone, Debug, protocol_derive::ProtocolSupport)]
                pub struct TeamInfo<'a> {
                    pub team_display_name: Cow<'a, str>,
                    pub team_prefix: Cow<'a, str>,
                    pub team_suffix: Cow<'a, str>,
                    pub friendly_fire: FriendlyFire,
                    pub name_tag_visibility: Cow<'a, str>,
                    pub color: misc::prelude::ChatColor,
                }
            }
        },
        0x3F => PluginMessage<'a> {
            channel: Cow<'a, str>,
            #[protocol_field(dynarray)]
            data: Vec<u8>
        },
        0x40 => Disconnect<'a> {
            reason: ChatComponent<'a>
        },
        0x45 => Title<'a>,
        0x47 => PlayerListHeaderAndFooter<'a> {
            header: ChatComponent<'a>,
            footer: ChatComponent<'a>
        }
    }
}

bitflags::bitflags! {
    #[derive(protocol_derive::ProtocolSupport)]
    pub struct PlayerPositionAndLookFlags: u8 {
        const X = 0x01;
        const Y = 0x02;
        const Z = 0x04;
        const Y_ROT = 0x08;
        const X_ROT = 0x10;
    }
}

impl Default for PlayerPositionAndLookFlags {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Clone, Debug)]
pub enum PlayerListItem<'a> {
    AddPlayer(Vec<(Uuid, PlayerListItemAddPlayer<'a>)>),
    UpdateGameMode(Vec<(Uuid, GameMode)>),
    UpdateLatency(Vec<(Uuid, i32)>),
    UpdateDisplayName(Vec<(Uuid, Option<ChatComponent<'a>>)>),
    RemovePlayer(Vec<Uuid>),
}

impl<'a> ProtocolSupportEncoder for PlayerListItem<'a> {
    fn calculate_len(&self, _version: &::protocol_internal::ProtocolVersion) -> usize {
        todo!()
    }

    fn encode<W: std::io::Write>(
        &self,
        _dst: &mut W,
        _version: &::protocol_internal::ProtocolVersion,
    ) -> std::io::Result<()> {
        todo!()
    }
}

impl<'a> ProtocolSupportDecoder for PlayerListItem<'a> {
    fn decode<R: std::io::Read>(
        _src: &mut R,
        _version: &protocol_internal::ProtocolVersion,
    ) -> std::io::Result<Self> {
        todo!()
    }
}

#[derive(Clone, Debug, Default, protocol_derive::ProtocolSupport)]
pub struct PlayerListItemAddPlayer<'a> {
    pub name: Cow<'a, str>,
    pub properties: Vec<Property>,
    pub game_mode: GameMode,
    #[protocol_field(varnum)]
    pub ping: i32,
    pub display_name: Option<Cow<'a, str>>,
}

proto_enum! {
    AnimationAction (u8) {
        SwingArm = 0,
        TakeDamage = 1,
        LeaveBed = 2,
        EatFood = 3,
        CriticalEffect = 4,
        MagicCriticalEffect = 5
    }
    default Self::SwingArm
}

proto_enum! {
    ScoreboardObjectiveMode<'a> (u8) {
        Create {
            info: ScoreboardObjectiveInfo<'a>
        } = 0,
        Remove = 1,
        Update {
            info: ScoreboardObjectiveInfo<'a>
        } = 2
    }
    default Self::Remove
}

proto_enum! {
    UpdateScoreAction<'a> (u8) {
        Create_Update {
            objective_name: Cow<'a, str>,
            value: i32
        } = 0,
        Remove {
            objective_name: Cow<'a, str>
        } = 1
    }
    default Self::Remove { objective_name: "".into() }
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
    TeamsMode<'a> (u8) {
        Create {
            info: TeamInfo<'a>,
            players: Vec<Cow<'a, str>>
        } = 0,
        Remove = 1,
        InfoUpdate {
            info: TeamInfo<'a>
        } = 2,
        AddPlayers {
            players: Vec<Cow<'a, str>>
        } = 3,
        RemovePlayers {
            players: Vec<Cow<'a, str>>
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
    Title<'a> (u8) {
        SetTitle {
            title: ChatComponent<'a>
        } = 0,
        SetSubTitle {
            sub_title: ChatComponent<'a>
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
